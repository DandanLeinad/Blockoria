# Spec: Feature Flags — Mecanismo e Ciclo de Vida (08)

## Visão Geral

O projeto segue trunk-based development com feature flags (regra do AGENTS.md):
comportamento incompleto é integrado à `main` escondido atrás de uma **flag de
lançamento**, e remover a flag quando a feature estabiliza faz parte da
definition of done. Esta spec define o mecanismo que torna isso operacional —
antes do primeiro consumidor (a spec 09, back-ups órfãos, adicionará a primeira
flag real, `orphan_restore`).

A premissa central: **flag de lançamento não é setting de usuário**. O usuário
final não deve conseguir ligar trabalho incompleto, então flags não podem morar
em arquivo que ele edita (`config.toml`, `%APPDATA%\Blockoria\`) — nem em
variável que o build de release leia. Três decisões resolvem isso:

1. **Default no código** — o enum `Flag` declara cada flag e seu
   `default_enabled()`; em release esse valor é final.
2. **Sobreposição só em desenvolvimento** — em build de debug,
   `BLOCKORIA_FLAGS="flag_a,flag_b"` liga flags por nome; o código do env é
   removido na compilação de release (`#[cfg(debug_assertions)]`).
3. **Remoção guiada pelo compilador** — `name()` e `default_enabled()` são
   matches **exaustivos sem wildcard**: criar variante sem mapear (ou graduar
   uma flag) é erro de compilação que lista cada ponto de toque.

O frontend é consumidor read-only: fonte única no backend, mapa entregue por
`cmd_feature_flags`. Sem `VITE_*`, sem store de flags no front.

---

## Requisitos Funcionais

### RF-01: Port `FeatureFlags` e enum `Flag` na camada application

- **Dado** o crate `blockoria-application`
- **Então** ele define:
  - `trait FeatureFlags: Send + Sync` com um único método **infallível**:
    `is_enabled(&self, flag: Flag) -> bool` — sem `Result`; flag ignorada ou
    sem sobreposição resolve para o default do código
  - `enum Flag` com `name() -> &'static str`, `default_enabled() -> bool` e
    `const ALL: &'static [Flag]`
- **E** o enum **nasce vazio**: cada flag é introduzida pela spec da feature
  que ela esconde (a 09 introduz `OrphanRestore`) — mecanismo sem consumidor
  não inventa flag de mentira
- **E** `name()` e `default_enabled()` usam `match` **exaustivo sem arm
  wildcard `_`** — a regra central do ciclo de vida (RF-05)
- **E** a camada **domain não sabe que flags existem** (regra do AGENTS.md)

Nomenclatura (a primeira flag, chegando com a spec 09):

| Conceito | Valor |
|----------|-------|
| Variante | `Flag::OrphanRestore` |
| `name()` / token do env / chave no mapa IPC | `orphan_restore` (snake_case) |

> O prefixo `feature.` do exemplo de nomeação do AGENTS.md era o caminho no
> `config.toml`, que não existe mais; o nome da flag é o identificador
> snake_case puro.

### RF-02: Impl `CodeFeatureFlags` na camada infrastructure

- **Dado** o crate `blockoria-infrastructure`
- **Então** `CodeFeatureFlags` (unit struct, sem estado) implementa o port:
  - em **debug**, lê `BLOCKORIA_FLAGS` — lista separada por vírgula, com trim,
    match exato contra `name()`; token que casa → `true`
  - em **release**, o `let` do env é **removido na compilação**
    (`#[cfg(not(debug_assertions))]`) — o resultado é sempre o default
- **E** token desconhecido (não casa com nenhuma flag de `Flag::ALL`) emite
  **aviso em stderr** — typo de env var não vira meia hora de debug silencioso
- **E** nenhuma mudança em `Config`/`config.toml`/`cmd_save_config` — flags não
  vivem em arquivo editável pelo usuário

Tabela de resolução (fonte canônica):

| Situação | Resultado |
|----------|-----------|
| Release, qualquer `BLOCKORIA_FLAGS` | `default_enabled()` (código do env não existe no binário) |
| Debug, sem `BLOCKORIA_FLAGS` | `default_enabled()` |
| Debug, token `== name()` | `true` |
| Debug, token não listado / desconhecido | default + aviso em stderr |
| Debug, lista vazia ou só espaços | `default_enabled()` |

- **E** não existe sintaxe de desligar (`flag=false`): todas as flags nascem
  `false`; forçar-off de um default-true é YAGNI
- **E** a lógica inteira mora na função pura `resolve(name, env_list, default)`
  — **alvo dos testes unitários**; a lista de env é passada explicitamente,
  testes nunca leem nem escrevem env de processo (`set_var` é `unsafe` na
  edition 2024 e racy entre threads de teste)

### RF-03: Command IPC `cmd_feature_flags`

- **Dado** o adapter Tauri
- **Quando** o frontend invoca `cmd_feature_flags`
- **Então** recebe o mapa `{ "<name>": bool }` com **uma entrada por flag em
  `Flag::ALL`**, valor resolvido (default aplicado quando ausente no env)
- **E** o command é **read-only**: não existe command de `set` — flags não são
  settings
- **E** hoje, com zero flags registradas, o mapa é vazio; a 09 o preenche

### RF-04: Frontend consumidor read-only (contrato)

- **Dado** o frontend React
- **Então** a única fonte de flags é o backend via `cmd_feature_flags`
- **E** não existe `VITE_*` nem store de flags no front
- **E** quando a primeira UI precisar branchear (spec 09): union type TS
  espelhando `Flag::ALL` + helper — remover um membro da union faz o `tsc`
  listar os usos, a mesma simetria do enum no Rust
- **E** a 08 **não adiciona arquivos frontend** — ainda não há nada a esconder

### RF-05: Ciclo de vida (contrato central)

| Operação | Passos |
|----------|--------|
| **Criar** | 1. A spec da feature declara nome + default (quase sempre `false`). 2. `+1` variante no enum. 3. `+1` arm em `name()` — match exaustivo **obriga**. 4. `+1` arm em `default_enabled()` — idem. 5. `+1` em `Flag::ALL`. 6. Use case ganha `&dyn FeatureFlags`; **o command recusa com `ApplicationError::Config` quando a flag está desligada** — gate no backend, nunca só UI. BDD cobre os dois estados; testes injetam fake, nunca env real. 7. Front: `+1` membro na union TS (quando a UI branchear). |
| **Editar** (dev) | `$env:BLOCKORIA_FLAGS="nome"` + restart do `cargo tauri dev`. Sem UI, sem command de `set`. |
| **Graduar** (flag → código permanente) | 1. Remover a variante → o compilador lista `name()`, `default_enabled()`, `ALL`, use case, command, fakes de teste. 2. Remover a branch desligada + o parâmetro do use case; manter o caminho da feature. 3. Remover o membro da union TS → `tsc` lista os usos. 4. Remover cenários BDD/testes do estado "off" (comportamento transitório). 5. Atualizar spec/ADR se a decisão for arquitetural. É **definition of done**, não faxina opcional. |
| **Cancelar** | Igual a graduar, removendo também o código da feature. |

### RF-06: Cargo features ≠ feature flags

- `[features]` do `Cargo.toml` é compile-time, para variantes de build —
  separação já estabelecida no AGENTS.md, mantida intacta. Flag desta spec é
  runtime, resolvida por `is_enabled`.

---

## Critérios de Aceitação

1. `resolve`: sem env → default
2. `resolve`: env com o nome da flag → `true`
3. `resolve`: lista com múltiplos tokens e espaços → a listada liga
4. `resolve`: token que não casa com nenhuma flag → default
5. `resolve`: lista vazia/só espaços → default
6. `resolve`: default não-falso é respeitado (contrato do parâmetro)
7. Release: caminho de release = `resolve` com env `None` (o `let` do env é
   removido na compilação — verificado por `cargo check --release`)
8. IPC: mapa de flags conhecidas com defaults (vazio hoje)
9. `cargo test`, `cargo fmt` e `cargo clippy -- -D warnings` verdes; **zero
   mudanças** em `Config`/`cmd_save_config`

## Fora de Escopo

- **Hot-reload / toggle a quente** — flag muda ~1× na vida útil; restart
  resolve. Se um dia precisar, re-ler o env por chamada.
- **UI de edição de flags / command de `set`** — flags não são settings de
  usuário.
- **Canal beta distribuído** — custo aceito: em release, ligar flag exige
  recompilar com o default trocado; sem build "beta" para terceiros sem
  instalador separado. Gatilho de revisão: se um canal beta existir um dia,
  mover a sobreposição para config (retrocompatível) e registrar em ADR novo.
- **`strum`/EnumIter** — dependência nova por duas linhas; `Flag::ALL` é
  array manual (`shortcut`: esquecer de listar = front não vê a flag,
  descoberta imediata em dev).
- **Frontend desta spec** — nenhum arquivo; o consumo real chega com a 09.

## Referências

- ADR: [`docs/adr/0002-feature-flags.md`](../adr/0002-feature-flags.md)
- BDD: [`tests/features/08-feature-flags.feature`](../../tests/features/08-feature-flags.feature)
- AGENTS.md — seções "Trunk-based development e feature flags" e "Adapters"
