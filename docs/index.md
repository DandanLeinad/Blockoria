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

**Em desenvolvimento ativo** — **Domain, Application, Infrastructure e Frontend implementados**.

| Camada | Status |
|--------|--------|
| **Domain** (`blockoria-domain`) | ✅ Completo — VOs, Entities, Aggregates, 66 testes + 12 doctests |
| **Application** (`blockoria-application`) | ✅ Completo — 5 use cases, 15 testes |
| **Infrastructure** (`blockoria-infrastructure`) | ✅ Completo — FileWorldRepository, FileBackupRepository, Config, **NBT Parser** (115 testes) |
| **Frontend (Tauri + React)** | ✅ Implementado — 3 telas MVP, 24 testes, React Router v7, Tailwind v4 |

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
- `From<io::Error> for DomainError` — Conversão automática de erros de I/O

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
| Testes unitários (application) | 15 |
| Testes unitários (infrastructure) | 95 |
| Testes integração (infrastructure) | 20 |
| Testes frontend (Vitest + RTL) | 24 |
| **Total** | **232 passando** |

```bash
# Backend
cargo test                        # 208 passed (unit + integration + doc)
cargo test -p blockoria-domain    # 66 passed
cargo test -p blockoria-domain --doc   # 12 passed
cargo test -p blockoria-application    # 15 passed
cargo test -p blockoria-infrastructure # 115 passed

# Frontend
cd app && bun test                # 24 passed (Vitest + React Testing Library)
```

---

## 📊 NBT Parser (`blockoria-infrastructure/src/nbt/`)

Parser completo **Little-Endian NBT** para arquivos `level.dat` do Minecraft Bedrock.

### Componentes

| Módulo | Descrição |
|--------|-----------|
| `error.rs` | `NbtError` com offsets + limites de segurança (MAX_SIZE=10MB, MAX_DEPTH=64, MAX_COMPOUND_ENTRIES=10k, MAX_LIST_LENGTH=1M) |
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

- **SDD Spec:** [`docs/specs/nbt-leveldat-parser.md`](specs/nbt-leveldat-parser.md)
- **BDD Scenarios:** [`tests/features/nbt_leveldat_parser.feature`](../tests/features/nbt_leveldat_parser.feature)

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
- **Zero `unwrap()`/`expect()`** na lógica do parser

---

## 🎨 Frontend (Tauri + React + TypeScript)

Implementado na branch `feat/tauri-setup`, pronto para integração.

### Tech Stack

| Item | Versão |
|------|--------|
| Tauri | 2.x |
| React | 19.x |
| TypeScript | 6.x |
| Build | Vite 8.x + Bun |
| Routing | React Router v7 |
| Styling | Tailwind CSS v4 |
| Testes | Vitest + React Testing Library |

### Estrutura

```
app/
├── package.json
├── vite.config.ts
├── vitest.config.ts
├── tsconfig.json
├── src/
│   ├── main.tsx                    # Entry point
│   ├── App.tsx                     # RouterProvider
│   ├── router.tsx                  # React Router v7 + loaders
│   ├── index.css                   # Tailwind v4 theme (CSS variables)
│   ├── components/
│   │   ├── WorldList.tsx           # Tela 1: Lista de mundos
│   │   ├── CreateBackup.tsx        # Tela 2: Criar backup
│   │   └── ListBackups.tsx         # Tela 3: Listar/restaurar/deletar backups
│   └── test/
│       └── setup.ts                # Vitest setup (mocks Tauri)
└── public/
```

### Telas MVP (3)

| Tela | Rota | Comando Tauri | Status |
|------|------|---------------|--------|
| **WorldList** | `/` | `cmd_list_worlds` | ✅ |
| **CreateBackup** | `/world/:folderName/backup/create` | `cmd_create_backup` | ✅ |
| **ListBackups** | `/world/:folderName/backups` | `cmd_list_backups`, `cmd_restore_backup`, `cmd_delete_backup` | ✅ |

### Funcionalidades Implementadas

- **WorldList**: Cards responsivos com ícones, badges (Compartilhado/Conta), versão, botão "Ver backups", estados loading/empty/error, navegação por teclado
- **CreateBackup**: Carregamento do mundo, confirmação, loading spinner, toast sucesso/erro, retry, diretório de destino
- **ListBackups**: Cards ordenados por data (recente primeiro), botões Restaurar/Deletar com modais de confirmação, estado vazio
- **Navegação**: Sidebar fixa (desktop), header mobile, rotas aninhadas com loaders React Router v7
- **Tema**: Light/Dark mode automático via `prefers-color-scheme`, tokens semânticos CSS (`bg-background`, `text-foreground`, `border-border`, `bg-card`, `bg-muted`, etc.)
- **Acessibilidade**: ARIA labels, focus visible, navegação por teclado, roles semânticos

### Testes Frontend

| Componente | Testes | Cobertura |
|------------|--------|-----------|
| WorldList | 6 | Loading, lista, empty, error, retry, keyboard nav |
| CreateBackup | 10 | Load world, create, success, errors (not found, permission, disk full), retry, keyboard |
| ListBackups | 8 | Load, list, empty, error, restore modal, delete modal, ordering, close |

```bash
cd app && bun test              # 24 passed
cd app && bun run build         # TypeScript + Vite build OK
```

### Integração Tauri

| Comando | Descrição |
|---------|-----------|
| `cmd_list_worlds` | Lista mundos → `WorldSummaryDto[]` |
| `cmd_create_backup` | Cria backup → `CreateBackupResponseDto` |
| `cmd_list_backups` | Lista backups → `BackupSummaryDto[]` |
| `cmd_restore_backup` | Restaura backup → `RestoreBackupResponseDto` |
| `cmd_delete_backup` | Deleta backup → `DeleteBackupResponseDto` |

### Development

```bash
# Frontend standalone
cd app && bun run dev        # Vite dev server em http://localhost:1420

# Integrado com Tauri
cargo tauri dev              # Compila Rust + inicia frontend + abre janela Tauri

# Build produção
cd app && bun run build      # dist/ gerado para Tauri
cargo tauri build            # Build completo (Rust + frontend)
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
│   └── blockoria-infrastructure/       # Repositories + NBT parser
├── app/                                # Frontend React + TypeScript
│   ├── src/
│   │   ├── components/                 # WorldList, CreateBackup, ListBackups
│   │   ├── router.tsx                  # React Router v7
│   │   ├── index.css                   # Tailwind v4 theme
│   │   └── test/setup.ts               # Vitest mocks
│   └── ...
├── docs/
│   ├── specs/                          # SDD specs (4 telas + NBT)
│   ├── features/                       # BDD Gherkin scenarios
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
| **Infrastructure** | Rust, `tokio` (fs), NBT parser custom (zero deps) |
| **Frontend** | Tauri 2, React 19, TypeScript 6, Vite 8, Bun |
| **Routing** | React Router v7 (loaders, nested routes) |
| **Styling** | Tailwind CSS v4 (CSS variables, dark mode) |
| **Testes Backend** | `cargo test` (built-in) |
| **Testes Frontend** | Vitest + React Testing Library + jsdom |
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
