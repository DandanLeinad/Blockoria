# Spec: Tratamento de Erros — Taxonomia, Contrato IPC e UX (07)

## Visão Geral

Erros hoje atravessam o sistema com semântica corrompida: `DomainError` é o tipo
universal (qualquer `io::Error` vira `InvalidBackupPath`), a camada application não tem
vocabulário próprio, o adapter serializa a string de debug
(`"Domain error: InvalidWorldPath: ..."`) direto para o frontend, e o `WorldList` joga a
mensagem real fora. Esta spec define: **categorias honestas por camada**, um **contrato
estruturado no IPC** (`{code, message}`), **superfície correta no frontend** (bugfix +
ErrorBoundary + mensagens PT-BR) e o **fim da morte silenciosa no startup**.

**Causa raiz:** ausência de categorias. "Não achei" (NotFound), "entrada inválida"
(Validation) e "o disco falhou" (Storage) tornam-se indistinguíveis após o primeiro
mapeamento — nenhum boundary consegue decidir nada (tentar de novo? avisar? qual texto?).

---

## Requisitos Funcionais

### RF-01: `ApplicationError` na camada application

- **Dado** qualquer operação de use case ou port
- **Quando** ela falha
- **Então** o erro é `ApplicationError` (thiserror), com exatamente estas categorias:

| Variante | Quando | Origem típica |
|----------|--------|---------------|
| `NotFound { what: What }` | agregado procurado não existe (`What::World`, `What::Backup`) | use cases, repos |
| `Validation(#[from] DomainError)` | entrada viola regra de domínio | value objects |
| `Storage(#[from] io::Error)` | falha de I/O real (permissão, disco, path) | repos, `copy_dir_all`, `fs::create_dir_all` |
| `Config(String)` | ler/gravar/serializar `config.toml` | `Config::load`/`save` |

- **E** os ports (`WorldRepository`, `BackupRepository`) passam a retornar `ApplicationError`
- **E** `Display` é legível por código (inglês, técnico, para log/debug)

### RF-02: `DomainError` vira puramente validação

- **Dado** `DomainError`
- **Quando** inspecionado
- **Então** contém apenas os 8 variants de validação existentes
- **E** o `impl From<io::Error>` é **removido** — I/O nunca mais entra por ali
- **E** todo `?` de `io::Error` que hoje depende daquele `From` passa a mapear para
  `ApplicationError::Storage` (via `#[from]`)

### RF-03: Contrato de erro no IPC

- **Dado** um command que falha
- **Quando** o `CommandError` chega ao frontend via `invoke()`
- **Então** serializa como **objeto** `{ "code": "<code>", "message": "<string>" }`
  (não mais `to_string()`)
- **E** `code` ∈ `not_found` | `validation` | `storage` | `config` | `unknown`
- **E** o mapeamento `ApplicationError → CommandError` é 1:1 por categoria:
  `NotFound → not_found`, `Validation → validation`, `Storage → storage`,
  `Config → config`; demais casos (ex.: lock envenenado) → `unknown`
- **E** `message` continua técnica (inglês) — a tradução mora no frontend (RF-04)

### RF-04: Frontend extrai e traduz o erro

- **Dado** o `invoke()` rejeita
- **Quando** `getTauriError(unknown)` é chamado (novo `app/src/utils/tauriError.ts`)
- **Então** retorna `{ code: ErrorCode, message: string }`
- **E** se o payload não tem o shape conhecido (string crua, objeto sem `code`),
  retorna `{ code: 'unknown', message: <payload como string> }` — **a mensagem bruta
  nunca é descartada**
- **E** um mapa PT-BR por `code` produz a mensagem humana exibida ao usuário:

| code | Mensagem humana (default) |
|------|---------------------------|
| `not_found` | "Mundo ou backup não encontrado" |
| `validation` | "Dados inválidos" |
| `storage` | "Erro ao acessar arquivos (permissões ou disco)" |
| `config` | "Erro nas configurações" |
| `unknown` | "Erro desconhecido" (+ mensagem bruta como detalhe) |

### RF-05: Bugfix — WorldList nunca descarta a mensagem

- **Dado** `cmd_list_worlds` falha
- **Quando** o `WorldList` captura o erro
- **Então** a tela de erro exibe a mensagem extraída (RF-04) — nunca um "Erro
  desconhecido" fixo
- (hoje: `err instanceof Error` é sempre falso para a string do Tauri → mensagem real
  é substituída por genérica)

### RF-06: ErrorBoundary + errorElement no router

- **Dado** qualquer exceção de render não capturada
- **Quando** o React propaga
- **Então** um `ErrorBoundary` renderiza tela de falha com botão "Recarregar"
- **E** o router declara `errorElement` cobrindo falhas de loaders
- **E** nenhum crash vira tela branca

### RF-07: Catchs existentes param de engolir a causa

- `SettingsPage` (load e save): toast exibe mensagem extraída (RF-04) em vez de string fixa
- `CreateBackup` (fallback do `cmd_get_backup_root`): mantém o fallback visual — aceito
  como degradação de rótulo informativo, não de dado

### RF-08: Startup sem morte silenciosa

- **Dado** `create_state()` falha (ex.: `%APPDATA%\Minecraft Bedrock` inexistente)
- **Quando** `main.rs` detecta o erro
- **Então** exibe message box nativo (`rfd::MessageDialog` — crate já transitiva do
  `tauri-plugin-dialog`) com o caminho esperado antes de encerrar
- **E** o processo encerra com código ≠ 0 — release com `windows_subsystem = "windows"`
  não pode simplesmente não abrir sem dizer nada

### RF-09: Política de degradação de dados parciais (contrato explícito)

Falha de **dado** degrada; falha de **operação** é erro. Sempre:

| Situação | Comportamento |
|----------|---------------|
| `level.dat` ilegível / NBT inválido | versão `[0,0,0,0,0]` (default), mundo listado |
| `levelname.txt` ilegível/vazio | `LevelName` "Unknown World" |
| `world_icon.jpeg` ausente/ilegível | mundo sem ícone |
| Pasta de mundo com `folder_name` inválida | pasta pulada, listagem continua |
| Diretório `Users/` inexistente | ERRO — operação de listagem falha (e startup avisa, RF-08) |

- **E** esta tabela é a fonte canônica: nada além dela degrada silenciosamente
- **E** `NbtError` não muda — já é o padrão de qualidade do repo; a degradação no
  repositório é deliberada

---

## Contrato IPC (API proposta)

```rust
// src-tauri/src/lib.rs
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode { NotFound, Validation, Storage, Config, Unknown }

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("{message}")]
    Structured { code: ErrorCode, message: String },
}

// Serialize impl → objeto { "code": "...", "message": "..." }
pub type CommandResult<T> = Result<T, CommandError>;
```

```typescript
// app/src/utils/tauriError.ts
export type ErrorCode = 'not_found' | 'validation' | 'storage' | 'config' | 'unknown'

export interface TauriError {
  code: ErrorCode
  message: string
}

export function getTauriError(error: unknown): TauriError
export function humanMessage(error: TauriError): string // mapa PT-BR (RF-04)
```

---

## Critérios de Aceite (direção BDD)

1. **application (unit, fakes):** restaurar com backup inexistente →
   `Err(ApplicationError::NotFound { what: Backup })` — não `InvalidBackupPath`
2. **application (unit):** falha de cópia em `copy_dir_all` →
   `Err(ApplicationError::Storage(_))` — não variant de validação
3. **infrastructure (contract):** bateria de `test_contract.rs` roda contra
   `ApplicationError` com as mesmas garantias de antes
4. **adapter (unit):** `CommandError` serializa como `{code, message}`;
   `ApplicationError::NotFound` → `code: "not_found"`
5. **front (unit):** `getTauriError` com objeto `{code, message}` preserva ambos; com
   string crua → `{code: 'unknown', message: <string>}`
6. **front (regression):** `cmd_list_worlds` rejeitando → tela de erro do WorldList
   exibe a mensagem, não "Erro desconhecido"
7. **front (unit):** ErrorBoundary captura throw de filho → renderiza fallback com Recarregar
8. **main (integration/manual):** diretório Bedrock ausente → message box com o caminho
   e encerramento sem crash invisível

---

## Decisões

- **Sem feature flag.** O contrato IPC muda de string → objeto, mas os dois lados
  (adapter + frontend) vivem no mesmo PR, na mesma release. Contrato interno, sem
  consumidores externos. A spec registra a ausência e o porquê.
- **`DomainError` mantém payload `String`.** Mensagens de validação são assertivas em
  teste via `PartialEq`; estruturar agora não paga (YAGNI).
- **`NbtError` intocado.** O melhor tipo de erro do repo; a integração degrada por
  decisão (RF-09), não por bug.
- **PT-BR no front, inglês no back.** `message` do IPC permanece técnico; a tradução
  humana mora só no `humanMessage`. Sem framework de i18n.
- **Promoção a ADR:** se a taxonomia `ApplicationError` estabilizar como a fronteira
  permanente de erros dos ports, registrar ADR em `docs/adr/` no fechamento desta feature.

---

## Fora de Escopo

- `anyhow`/`anyhow`-like no core — as categorias já resolvem
- Framework de i18n / multi-idioma
- Telemetria/sentry, logging em arquivo (app local; release sem console)
- Tipo de erro por use case — um `ApplicationError` basta
- Remover `success: bool` redundante dos DTOs de resposta (estável, não é erro)
- Caminho UWP legado do Minecraft Bedrock
