---
icon: lucide/database-backup
hide:
  - toc
  - navigation
---

# Blockoria

> **Gerenciador de backups de mundos Minecraft Bedrock Edition para Windows 10/11.**
> Interface gráfica nativa, backups versionados, restauração com preview.

!!! warning "⚠️ Aviso Legal"
    **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
    Este é um projeto open-source independente, desenvolvido como hobby/estudo.

[![License: AGPL-3.0-only](https://img.shields.io/badge/License-AGPL%203.0--only-blue.svg?style=for-the-badge)](https://github.com/DandanLeinad/blockoria/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-4f46e5?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-4f46e5?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078d6?style=for-the-badge&logo=windows&logoColor=white)](https://microsoft.com/windows)

---

## 🏗️ Status Atual

**Em desenvolvimento ativo** — **Domain, Application, Infrastructure e Frontend implementados** com testes abrangentes e CI/CD.

| Camada | Status |
|--------|--------|
| **Domain** (`blockoria-domain`) | ✅ Completo — VOs, Entities, Aggregates, 66 testes + 12 doctests |
| **Application** (`blockoria-application`) | ✅ Completo — 5 use cases, **taxonomia `ApplicationError`** (spec 07), 21 testes |
| **Infrastructure** (`blockoria-infrastructure`) | ✅ Completo — FileWorldRepository, FileBackupRepository, Config, **NBT Parser** (115 testes), Test Contracts |
| **Frontend (Tauri + React)** | ✅ Implementado — 4 telas MVP (incl. Settings), 47 testes unitários, React Router v7, Tailwind v4, **shadcn/ui**, **ErrorBoundary + mensagens PT-BR** |

---

## 📦 blockoria-domain (Concluído)

A camada de domínio pura, sem dependências externas, contendo:

### Value Objects (9)

| VO | Descrição | Testes |
|------|-----------|--------|
| `WorldFolderName` | Nome da pasta do mundo (12 chars + `=`) | 7 |
| `LevelName` | Nome de exibição do mundo | 4 |
| `WorldPath` | Caminho do mundo (FS: exists + is_dir) | 5 |
| `AccountId` | ID da conta Microsoft | 4 |
| `WorldVersion` | Versão do mundo `[u16; 5]` ≥ 0 | 4 |
| `WorldIconPath` | Caminho do ícone (`world_icon.jpeg`) | 6 |
| `BackupTimestamp` | Timestamp UTC do backup | 6 |
| `BackupPath` | Caminho do diretório de backup | 5 |
| `WorldLocation` | Localização do mundo: Account(AccountId) \| Shared | 11 |

### Novos métodos / traits (atualizações recentes)

- `BackupTimestamp::from_filename_safe()` — Parse de timestamp do nome do diretório
- `BackupTimestamp: Ord` — Ordenação para listar backups mais recentes primeiro
- `WorldVersion: Default` — Versão padrão [0,0,0,0,0]
- `AccountId: Hash` — Para uso como chave em HashMap

> O `From<io::Error> for DomainError` foi **removido** (spec 07): `DomainError` é puramente validação; I/O vive na aplicação como `ApplicationError::Storage`.

### Entities / Aggregates

| Entity | Tipo | Descrição |
|--------|------|-----------|
| `World` | Aggregate Root | Mundo Minecraft com folder_name, level_name, path, account_id, version, icon_path |
| `Backup` | Aggregate Root | Backup com world_folder_name, world_account_id, world_version, created_at, backup_path |

### DomainError (8 variants)

| Variant | Quando ocorre |
|---------|---------------|
| `InvalidWorldFolderName` | Formato inválido (não 12 chars, não termina com `=`, whitespace) |
| `InvalidWorldPath` | Vazio, não existe, não é diretório |
| `InvalidWorldIconPath` | Nome do arquivo não é `world_icon.jpeg` |
| `InvalidLevelName` | Vazio ou apenas whitespace |
| `InvalidAccountId` | Vazio ou apenas whitespace |
| `InvalidWorldVersion` | Não tem 5 elementos ou contém negativos |
| `InvalidBackupTimestamp` | Anterior a Unix epoch (1970) |
| `InvalidBackupPath` | Vazio, não existe, não é diretório |

---

## ✅ Testes

| Métrica | Valor |
|---------|-------|
| Testes unitários (domain) | 66 |
| Doctests (domain) | 12 |
| Testes unitários (application) | 21 |
| Testes unitários (infrastructure) | 115 |
| Testes integração (infrastructure) | 20 (inclui bateria de contratos dos ports) |
| Testes unitários (adapter Tauri) | 3 |
| Testes frontend (Vitest + RTL) | 47 |
| **Total** | **284 passando** |

```bash
# Backend
cargo test                        # 237 passed (unit + integration + adapter + doc)
cargo test -p blockoria-domain    # 66 passed + 12 doctests
cargo test -p blockoria-application    # 21 passed
cargo test -p blockoria-infrastructure # 135 passed (115 unit + 20 integration)

# Frontend
cd app && bun run test            # 47 passed (Vitest + React Testing Library)
```

---

## 🛡️ Tratamento de Erros (spec 07)

Taxonomia honesta por camada — "não achei", "entrada inválida" e "o disco falhou" são categorias distintas:

| Camada | Tipo | Categorias |
|--------|------|-----------|
| Application | `ApplicationError` | `NotFound { what: World\|Backup }`, `Validation(DomainError)`, `Storage(io::Error)`, `Config` |
| Domain | `DomainError` | 8 variants de validação, **puro** (sem conversão de I/O) |
| IPC (Tauri) | `CommandError` | objeto `{ "code", "message" }` — `not_found\|validation\|storage\|config\|unknown` |
| Frontend | `tauriError.ts` | `getTauriError` (nunca descarta o payload) + `humanMessage` (mapa PT-BR por `code`) |

- **Startup**: falha no `create_state()` → message box nativo (`rfd`) com o caminho procurado antes de encerrar (RF-08) — fim da morte silenciosa em release sem console.
- **Render**: `ErrorBoundary` global (main.tsx) + `errorElement` no router (`RouteError`) — fim da tela branca (RF-06).
- **Degradação de dados parciais**: NBT ilegível → versão default; levelname ilegível → "Unknown World"; ícone ausente → sem ícone. Contrato canônico na spec 07 (RF-09).

### Especificação

- **SDD Spec:** [`docs/specs/07-error-handling.md`](specs/07-error-handling.md)
- **BDD Scenarios:** [`tests/features/07-error-handling.feature`](../tests/features/07-error-handling.feature)
- **ADR:** [`docs/adr/0001-error-taxonomy.md`](adr/0001-error-taxonomy.md)

---

## 📊 NBT Parser (`blockoria-infrastructure/src/nbt/`)

Parser completo **Little-Endian NBT** para arquivos `level.dat` do Minecraft Bedrock.

### Componentes

| Módulo | Descrição |
|--------|-----------|
| `error.rs` | `NbtError` com offsets + limites de segurança (MAX_SIZE=10MB, MAX_DEPTH=128, MAX_COMPOUND_ENTRIES=100k, MAX_LIST_LENGTH=1M) |
| `tag_type.rs` | `NbtTagType` enum (0-12) com `TryFrom<u8>` |
| `value.rs` | `NbtValue` AST + `NbtList` wrapper preserva element_type |
| `reader.rs` | `LeReader<R: Read>` — LE binary reading via `from_le_bytes` (zero deps) |
| `parser.rs` | Parser genérico recursivo com depth tracking |
| `level_dat.rs` | `LevelDatHeader` (8 bytes LE), `LevelDatParser`, `extract_world_version` |
| `json.rs` | `to_json_simple()` + `to_json_typed()` (feature `serde`) |

### Integração

`FileWorldRepository::parse_level_dat_version()` agora usa o parser real (era stub retornando `None`):
```rust
fn parse_level_dat_version(path: &Path) -> Option<WorldVersion> {
    let file = fs::File::open(path).ok()?;
    let mut parser = LevelDatParser::new(file).ok()?;
    let nbt = parser.parse().ok()?;
    extract_world_version(&nbt)
}
```

### Especificação

- **SDD Spec:** [`docs/specs/05-nbt-leveldat-parser.md`](specs/05-nbt-leveldat-parser.md)
- **BDD Scenarios:** [`tests/features/05-nbt-leveldat-parser.feature`](../tests/features/05-nbt-leveldat-parser.feature)

### Limites de Segurança

```rust
const MAX_LEVEL_DAT_SIZE: usize = 10_000_000;      // 10 MB
const MAX_NESTING_DEPTH: usize = 128;
const MAX_COMPOUND_ENTRIES: usize = 100_000;
const MAX_LIST_LENGTH: usize = 1_000_000;
const MAX_STRING_LENGTH: usize = 1_000_000;
```

### Testes NBT

- 104 testes unitários (core parser, reader, level_dat)
- 11 testes JSON serialization (simple + typed)
- 20 testes integração (repositórios)
- 15 testes contract (WorldRepository + BackupRepository)
- **Zero `unwrap()`/`expect()`** na lógica do parser

---

## 🎨 Frontend (Tauri + React + TypeScript)

Implementado com **shadcn/ui** design system, **Toast Notifications v2**, **Settings page** e **tratamento de erros completo** (spec 07).

### Tech Stack

| Item | Versão |
|------|--------|
| Tauri | 2.x |
| React | 19.x |
| TypeScript | 6.x |
| Build | Vite 8.x + Bun |
| Routing | React Router v7 |
| Styling | Tailwind CSS v4 + shadcn/ui |
| Testes Unitários | Vitest + React Testing Library |

### Estrutura

```
app/
├── package.json
├── vite.config.ts
├── vitest.config.ts
├── tsconfig.json
├── components.json          # shadcn/ui config
├── src/
│   ├── main.tsx                    # Entry point + ErrorBoundary global
│   ├── App.tsx                     # RouterProvider + ToastProvider
│   ├── router.tsx                  # React Router v7 + errorElement (RouteError)
│   ├── index.css                   # Tailwind v4 theme (CSS variables)
│   ├── lib/utils.ts                # cn() utility (clsx + tailwind-merge)
│   ├── components/
│   │   ├── WorldList.tsx           # Tela 1: Lista de mundos
│   │   ├── CreateBackup.tsx        # Tela 2: Criar backup
│   │   ├── ListBackups.tsx         # Tela 3: Listar/restaurar/deletar backups
│   │   ├── useToast.ts             # Toast hook (shadcn/ui)
│   │   ├── ToastContext.tsx        # Toast context v2
│   │   ├── ToastContainer.tsx      # Toast stack + animations
│   │   ├── Theme.tsx               # Theme provider + toggle
│   │   ├── ThemeContext.tsx        # Theme context
│   │   ├── AppLayout.tsx           # Layout com sidebar/header
│   │   ├── ErrorBoundary.tsx       # Boundary global de render (spec 07)
│   │   ├── ErrorScreen.tsx         # Fallback compartilhado "Algo deu errado"
│   │   ├── RouteError.tsx          # errorElement do router
│   │   ├── ui/                     # shadcn/ui components
│   │   │   ├── button.tsx
│   │   │   ├── card.tsx
│   │   │   ├── dialog.tsx
│   │   │   ├── toast.tsx
│   │   │   ├── switch.tsx
│   │   │   ├── label.tsx
│   │   │   ├── separator.tsx
│   │   │   └── Icons.tsx
│   ├── pages/
│   │   ├── WorldListPage.tsx       # Página lista de mundos
│   │   ├── CreateBackupPage.tsx    # Página criar backup
│   │   ├── ListBackupsPage.tsx     # Página listar backups
│   │   └── SettingsPage.tsx        # Página configurações (NOVA)
│   ├── test/
│   │   └── setup.ts                # Vitest setup (mocks Tauri)
│   └── utils/
│       ├── format.ts               # formatDate, formatVersion
│       └── tauriError.ts           # getTauriError + humanMessage (mapa PT-BR por code)
└── public/
```

### Telas MVP (4)

| Tela | Rota | Comando Tauri | Status |
|------|------|---------------|--------|
| **WorldList** | `/` | `cmd_list_worlds` | ✅ |
| **CreateBackup** | `/world/:folderName/backup/create` | `cmd_create_backup` | ✅ |
| **ListBackups** | `/world/:folderName/backups` | `cmd_list_backups`, `cmd_restore_backup`, `cmd_delete_backup` | ✅ |
| **Settings** | `/settings` | `cmd_get_config`, `cmd_save_config` | ✅ **NOVA** |

### Funcionalidades Implementadas

#### 🎯 Core Features
- **WorldList**: Cards responsivos com ícones, badges (Compartilhado/Conta), versão, botão "Ver backups", estados loading/empty/error, navegação por teclado, hover visible em dark mode
- **CreateBackup**: Carregamento do mundo, confirmação, loading spinner, toast sucesso/erro, retry, **diretório de destino real via config**
- **ListBackups**: Cards ordenados por data (recente primeiro), botões Restaurar/Deletar com modais de confirmação, estado vazio, **hover visible em dark mode**
- **Settings**: **NOVA** — Seletor de tema (Light/Dark/System), persistido em `%APPDATA%\Blockoria\config.toml`, aplicação imediata via CSS variables

#### 🔔 Toast Notifications v2 (NOVO)
Sistema completo substituindo `alert()` nativo:
- **Stack**: Até 5 toasts simultâneos (FIFO), gap 8px
- **Animações**: Slide-in (300ms ease-out) + Slide-out (200ms ease-in)
- **Progress Bar**: Barra visual topo → anima width 100%→0%, pausa no hover/focus
- **Pause on Hover/Focus**: Countdown pausa, retoma ao sair
- **Close Manual**: Botão X + tecla `Escape`
- **Action Buttons**: Suporte a `{ label, onClick }` (ex: "Desfazer")
- **Rich Content**: Renderiza `ReactNode` customizado
- **Persist**: `persist: true` → sem auto-dismiss, sem progress bar
- **6 Posições**: top-left/center/right, bottom-left/center/right
- **Acessibilidade Completa**: `role="status"/"alert"`, `aria-live="polite"/"assertive"`, `aria-atomic`, focus management, navegação por teclado

#### 🎨 Design System (shadcn/ui + Tailwind v4)
- **Tokens Semânticos** via `@theme` em `index.css` (light/dark automático)
- **Componentes**: Button, Card, Dialog, Toast, Switch, Label, Separator, Icons
- **Utility**: `cn()` (clsx + tailwind-merge), `.bg-overlay` para modais
- **Sem `@layer utilities` redundante** — Tailwind v4 gera tudo automaticamente

#### 🧭 Navegação & Roteamento
- **Sidebar fixa** (desktop) + **Header mobile** com `AppLayout`
- **React Router v7** com nested routes + loaders
- Rotas: `/`, `/world/:folderName/backup/create`, `/world/:folderName/backups`, `/settings`

#### ♿ Acessibilidade
- ARIA labels, focus visible, navegação por teclado (Tab, Enter, Space, Escape)
- Roles semânticos (`role="list"`, `role="listitem"`, `role="status"`, `role="alert"`)
- Screen readers anunciam toasts corretamente

### Testes Frontend

| Arquivo | Testes | Cobre |
|---------|--------|-------|
| ToastContainer.test.tsx | 20 | Stack, animações, progress, hover pause, action, persist |
| ToastContext.test.tsx | 9 | Create, dismiss, dismissAll, types, options |
| useToast.test.tsx | 4 | Hook usage, provider check |
| tauriError.test.ts | 10 | Extração `{code, message}`, nunca descartar payload, mapa PT-BR (spec 07) |
| WorldList.test.tsx | 2 | Regressão RF-05: mensagem real mapeada + payload nunca perdido |
| ErrorBoundary.test.tsx | 2 | RF-06: throw de filho → fallback com Recarregar; filho são passa |
| **Total** | **47** | |

```bash
cd app && bun run test      # 47 passed (Vitest + React Testing Library)
cd app && bun run build     # TypeScript + Vite build OK
```

### Integração Tauri

| Comando | Descrição |
|---------|-----------|
| `cmd_list_worlds` | Lista mundos → `WorldSummaryDto[]` |
| `cmd_create_backup` | Cria backup → `CreateBackupResponseDto` |
| `cmd_list_backups` | Lista backups → `BackupSummaryDto[]` |
| `cmd_restore_backup` | Restaura backup → `RestoreBackupResponseDto` |
| `cmd_delete_backup` | Deleta backup → `DeleteBackupResponseDto` |
| `cmd_get_backup_root` | Retorna backup_root configurado → `String` |
| `cmd_get_config` | **NOVO** Retorna config (theme) → `ConfigDto` |
| `cmd_save_config` | **NOVO** Salva config (theme) → `()` |

> **Erros**: todo command que falha rejeita com objeto `{ "code": "not_found|validation|storage|config|unknown", "message": "..." }` (spec 07, RF-03). O frontend mapeia `code` para mensagens PT-BR via `tauriError.ts`.

### Development

```bash
# Frontend standalone
cd app && bun run dev        # Vite dev server em http://localhost:1420

# Integrado com Tauri
cargo tauri dev              # Compila Rust + inicia frontend + abre janela Tauri

# Build produção
cd app && bun run build      # dist/ gerado para Tauri
cargo tauri build            # Build completo (Rust + frontend)

# Testes
cd app && bun run test         # Vitest unit (47 passed)
cargo test                      # Backend (237 passed)
```

---

## 📁 Estrutura do Projeto

```
blockoria/
├── Cargo.toml                          # Workspace root
├── Cargo.lock
├── src-tauri/                          # Tauri app (driving adapter)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs                     # Entry point → create_state() → run_with_state()
│       └── lib.rs                      # Commands, error handling, AppState
├── crates/
│   ├── blockoria-domain/               # Domain layer (0 deps)
│   ├── blockoria-application/          # Use cases + ports
│   └── blockoria-infrastructure/       # Repositories + NBT parser + Config + Test Contracts
├── app/                                # Frontend React + TypeScript
│   ├── src/
│   │   ├── components/                 # WorldList, CreateBackup, ListBackups, Toast, Theme, ErrorBoundary, UI
│   │   ├── pages/                      # WorldListPage, CreateBackupPage, ListBackupsPage, SettingsPage
│   │   ├── router.tsx                  # React Router v7
│   │   ├── index.css                   # Tailwind v4 theme
│   │   └── test/setup.ts               # Vitest mocks
│   └── ...
├── docs/
│   ├── specs/                          # SDD specs (4 telas + NBT + Toast + Erros)
│   ├── adr/                            # Architecture Decision Records
│   ├── index.md                        # Esta página
│   ├── domain.md
│   ├── application.md
│   └── frontend.md                     # Documentação detalhada do frontend
├── tests/
│   └── features/                       # BDD scenarios (Gherkin)
└── ...
```

---

## 🛠️ Tech Stack Completo

| Camada | Tecnologia |
|--------|------------|
| **Domain** | Rust 1.80+, Edition 2024, `thiserror`, `serde` |
| **Application** | Rust, `thiserror`, ports (traits) |
| **Infrastructure** | Rust, `tokio` (fs), NBT parser custom (zero deps), `toml`, `dirs` |
| **Frontend** | Tauri 2, React 19, TypeScript 6, Vite 8, Bun |
| **UI Components** | shadcn/ui (Radix UI + Tailwind) |
| **Routing** | React Router v7 (nested routes, errorElement) |
| **Styling** | Tailwind CSS v4 (CSS variables, dark mode) |
| **Testes Backend** | `cargo test` (built-in) |
| **Testes Frontend Unit** | Vitest + React Testing Library + jsdom |
| **CI/CD** | GitHub Actions (`cargo test`, `cargo deny`, `cargo fmt`, `pre-commit`) |

---

## 📄 Licença

**AGPL-3.0-or-later** — Código aberto, livre para usar, modificar e distribuir.
Consulte [LICENSE](../LICENSE) para detalhes.

---

## 🔗 Links Úteis

- [Domain Documentation](domain.md)
- [Application Documentation](application.md)
- [Frontend Documentation](frontend.md)
- [Specs](specs/)
- [BDD Features](features/)
- [ADRs](adr/)

---

⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
