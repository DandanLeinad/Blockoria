# ADR-0002: Feature flags com default no código e sobreposição só em debug

## Status

Accepted (2026-11-01)

## Context

O projeto segue trunk-based development com feature flags (AGENTS.md):
comportamento incompleto é integrado à `main` escondido atrás de uma flag, e
remover a flag quando a feature estabiliza faz parte da definition of done. O
mecanismo precisava existir antes do primeiro consumidor (spec 09, back-ups
órfãos).

A primeira proposta punha o estado das flags em `config.toml`
(`%APPDATA%\Blockoria\config.toml`). A reavaliação rejeitou o lugar: flag de
lançamento esconde trabalho incompleto, e o usuário final **não deve** conseguir
ligá-la — `config.toml` é exatamente o arquivo que ele edita. Três problemas
concretos selaram a rejeição:

- `cmd_save_config` regrava o arquivo inteiro — poderia apagar as flags;
- um valor TOML inválido nas flags derrubaria o carregamento do config inteiro;
- o app instalado e o de desenvolvimento dividiriam o mesmo arquivo.

## Decision

1. **O default mora no código.** O enum `Flag` (application) declara cada flag;
   `Flag::default_enabled()` devolve o default (todas nascem `false`). Em
   release, esse valor é final — não existe arquivo nem variável que o mude.
2. **Sobreposição só em desenvolvimento.** Em build de debug,
   `BLOCKORIA_FLAGS="nome_a,nome_b"` liga flags por nome. O código do env está
   atrás de `#[cfg(debug_assertions)]`: em release **nem é compilado**. Em
   testes automatizados, a sobreposição é fake injetado (`&dyn FeatureFlags`) —
   testes nunca leem nem escrevem env de processo.
3. **Remoção guiada pelo compilador.** `Flag::name()` e
   `Flag::default_enabled()` são matches **exaustivos sem wildcard**: criar
   variante sem mapear, ou remover variante ainda usada, é erro de compilação
   que lista cada ponto de toque. Graduar uma flag (virar código permanente) é
   deletar a variante e seguir os erros até o fim — use case, command e a union
   TS do frontend incluídos.
4. **Gate duro no backend.** O command da feature flaggada recusa com
   `ApplicationError::Config` quando a flag está desligada — esconder botão na
   UI é cosmético, nunca a única barreira.
5. **Frontend read-only.** `cmd_feature_flags` entrega o mapa resolvido
   (`{ "<name>": bool }`) de todas as flags conhecidas. Sem `VITE_*`, sem store
   de flags no front.
6. **Sem runtime toggle.** Flags não são settings: mudar flag em dev = env var
   + restart. Não existe command de `set`.
7. **Cargo features continuam sendo outra ferramenta** (compile-time,
   variantes de build) — a separação do AGENTS.md se mantém.

## Consequences

- **Positivas:** trunk-based real — o código da feature navega no binário de
  release, inacessível; remoção de flag é compiler-driven (zero grep, zero
  config órfão em máquinas de usuários); testes são funções puras, sem I/O nem
  estado de processo; `Config` e `cmd_save_config` ficam intocados.
- **Negativas / custos:** em release, uma flag só liga recompilando com o
  default trocado — não existe build "beta" para terceiros sem instalador
  separado. Custo aceito para um projeto de contribuidor único. **Gatilho de
  revisão:** se um canal beta distribuído um dia existir, mover a sobreposição
  para config (retrocompatível por construção) e registrar a troca num novo
  ADR.
- **Teto conhecido:** `Flag::ALL` é array manual (sem `strum`/EnumIter —
  dependência nova por duas linhas): esquecer de listar uma variante faz o
  frontend não vê-la; descoberta é imediata em dev.

## Alternatives Considered

| Alternativa | Por que rejeitada |
|-------------|-------------------|
| Estado das flags em `config.toml` (`%APPDATA%`) | Usuário final edita o arquivo; `cmd_save_config` poderia apagar flags; TOML inválido derruba o config inteiro; dev e app instalado dividem o mesmo arquivo |
| Cargo features por flag (`cargo tauri dev --features orphan_restore`) | Matriz no CI, duas compilações para testar on/off, e conflita com a separação do AGENTS.md (Cargo features = compile-time, feature flags = runtime) |
| Editar a constante `default_enabled()` na branch | Zero mecanismo, mas fácil de commitar ligado por engano — o pior erro num fluxo em que a flag existe para nunca ir ligada à `main` |
| Env var lida também em release | Usuário final conseguiria ligar trabalho incompleto |
| `strum`/EnumIter para derivar `Flag::ALL` | Dependência nova por duas linhas |

## Referências

- Spec: [`docs/specs/08-feature-flags.md`](../specs/08-feature-flags.md)
- BDD: [`tests/features/08-feature-flags.feature`](../../tests/features/08-feature-flags.feature)
- AGENTS.md — seção "Trunk-based development e feature flags"
