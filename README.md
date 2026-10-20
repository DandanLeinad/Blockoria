# Blockoria

> **Gerenciador de backups de mundos Minecraft Bedrock Edition para Windows 10/11.**
> Interface gráfica nativa, backups versionados, restauração com preview.

> ⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
> Este é um projeto open-source independente, desenvolvido como hobby/estudo.

[![License: AGPL-3.0-or-later](https://img.shields.io/badge/License-AGPL%20v3.0%2B-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-4f46e5?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.x-4f46e5?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app)
[![React](https://img.shields.io/badge/React-19.3-61dafb?style=for-the-badge&logo=react&logoColor=white)](https://react.dev)
[![TypeScript](https://img.shields.io/badge/TypeScript-6.0-3178c6?style=for-the-badge&logo=typescript&logoColor=white)](https://typescriptlang.org)
[![Vite](https://img.shields.io/badge/Vite-8.3-646cff?style=for-the-badge&logo=vite&logoColor=white)](https://vitejs.dev)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078d6?style=for-the-badge&logo=windows&logoColor=white)](https://microsoft.com/windows)

---

## 🏗️ Status Atual

**v0.10.0** — **Domain, Application, Infrastructure e Frontend implementadas** com shadcn/ui, Toast v2, Settings e **tratamento de erros completo** (spec 07: taxonomia por camada, contrato IPC estruturado, mensagens PT-BR, ErrorBoundary, startup dialog).

| Camada                                          | Status                                                                                                                                    |
| ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **Domain** (`blockoria-domain`)                 | ✅ Completo — 9 VOs, Entities, Aggregates, 66 testes + 12 doctests                                                                        |
| **Application** (`blockoria-application`)       | ✅ Implementado — 5 use cases, **taxonomia `ApplicationError`** (spec 07), 21 testes                                                     |
| **Infrastructure** (`blockoria-infrastructure`) | ✅ Implementado — FileWorldRepository, FileBackupRepository, Config, **NBT Parser** (115 testes), Test Contracts, gen_test_worlds example |
| **Frontend (Tauri + React)**                    | ✅ MVP Completo — **4 telas**, **47 testes unitários**, React Router v7, Tailwind v4, **shadcn/ui**, **Toast v2**, **Settings**, **ErrorBoundary + mensagens PT-BR** |

---

## 📦 blockoria-domain (Concluído)

Camada de domínio pura, sem dependências externas.

### Value Objects (9)

| VO                | Descrição                                          | Testes |
| ----------------- | -------------------------------------------------- | ------ |
| `WorldFolderName` | Nome da pasta do mundo (12 chars + `=`)            | 7      |
| `LevelName`       | Nome de exibição do mundo                          | 4      |
| `WorldPath`       | Caminho do mundo (FS: exists + is_dir)             | 5      |
| `AccountId`       | ID da conta Microsoft                              | 4      |
| `WorldVersion`    | Versão do mundo `[u16; 5]` ≥ 0                     | 4      |
| `WorldIconPath`   | Caminho do ícone (`world_icon.jpeg`)               | 6      |
| `BackupTimestamp` | Timestamp UTC do backup                            | 6      |
| `BackupPath`      | Caminho do diretório de backup                     | 5      |
| `WorldLocation`   | Localização do mundo: Account(AccountId) \| Shared | 11     |

### Novos métodos / traits (atualizações recentes)

- `BackupTimestamp::from_filename_safe()` — Parse de timestamp do nome do diretório
- `BackupTimestamp: Ord` — Ordenação para listar backups mais recentes primeiro
- `WorldVersion: Default` — Versão padrão [0,0,0,0,0]
- `AccountId: Hash` — Para uso como chave em HashMap

> `From<io::Error> for DomainError` foi **removido** (spec 07): `DomainError` é puramente validação; I/O vive na aplicação como `ApplicationError::Storage`.

### Entities / Aggregates

| Entity   | Tipo           | Descrição                                                                              |
| -------- | -------------- | -------------------------------------------------------------------------------------- |
| `World`  | Aggregate Root | Mundo Minecraft com folder_name, level_name, path, account_id, version, icon_path      |
| `Backup` | Aggregate Root | Backup com world_folder_name, world_account_id, world_version, created_at, backup_path |

### DomainError (8 variants)

| Variant                  | Quando ocorre                                                    |
| ------------------------ | ---------------------------------------------------------------- |
| `InvalidWorldFolderName` | Formato inválido (não 12 chars, não termina com `=`, whitespace) |
| `InvalidWorldPath`       | Vazio, não existe, não é diretório                               |
| `InvalidWorldIconPath`   | Nome do arquivo não é `world_icon.jpeg`                          |
| `InvalidLevelName`       | Vazio ou apenas whitespace                                       |
| `InvalidAccountId`       | Vazio ou apenas whitespace                                       |
| `InvalidWorldVersion`    | Não tem 5 elementos ou contém negativos                          |
| `InvalidBackupTimestamp` | Anterior a Unix epoch (1970)                                     |
| `InvalidBackupPath`      | Vazio, não existe, não é diretório                               |

---

## ✅ Testes

```bash
cargo test -p blockoria-domain        # 66 unit tests + 12 doctests = 78
cargo test -p blockoria-application   # 21 use case + taxonomy tests
cargo test -p blockoria-infrastructure # 135 unit/integration/contract tests (incl. NBT parser)
cargo test --workspace                # 237 backend (incl. adapter IPC tests)
cd app && bun run test                # 47 frontend tests (Vitest + RTL)
# Total: 284 passing
```

---

## 🛠️ Tech Stack Completo

| Camada                                    | Tecnologia                                                             |
| ----------------------------------------- | ---------------------------------------------------------------------- |
| **Domain / Application / Infrastructure** | Rust 1.80+, Edition 2024, `thiserror`, `serde`, `tokio`                |
| **Frontend**                              | Tauri 2.x, React 19.3, TypeScript 6.0, Vite 8.3, Bun                   |
| **UI Components**                         | shadcn/ui (Radix UI + Tailwind)                                        |
| **Routing**                               | React Router v7 (nested routes, errorElement)                          |
| **Styling**                               | Tailwind CSS v4 (CSS variables, dark mode)                             |
| **Testes Backend**                        | `cargo test` (built-in)                                                |
| **Testes Frontend Unit**                  | Vitest + React Testing Library + jsdom                                 |
| **CI/CD**                                 | GitHub Actions (`cargo test`, `cargo deny`, `cargo fmt`, `pre-commit`) |

---

## 📁 Estrutura do Workspace

```
blockoria/
├── Cargo.toml
├── src-tauri/                 # Tauri app (driving adapter)
├── app/                       # Frontend React + TypeScript
│   ├── src/
│   │   ├── components/        # WorldList, CreateBackup, ListBackups, Toast, Theme, ErrorBoundary, UI
│   │   ├── pages/             # WorldListPage, CreateBackupPage, ListBackupsPage, SettingsPage
│   │   ├── router.tsx         # React Router v7
│   │   ├── index.css          # Tailwind v4 theme
│   │   └── test/              # Vitest setup
│   └── components.json        # shadcn/ui config
├── crates/
│   ├── blockoria-domain/      # ✅ Completo (9 VOs, 66 testes)
│   ├── blockoria-application/ # ✅ Implementado (5 use cases, taxonomia de erros, 21 testes)
│   └── blockoria-infrastructure/
│       ├── repos/             # FileWorldRepository, FileBackupRepository
│       ├── config/            # Config system (config.toml)
│       ├── nbt/               # ✅ LE-NBT parser (level.dat, JSON)
│       └── examples/          # gen_test_worlds (fixtures de teste)
├── docs/                      # Documentação (Zensical)
├── tests/                     # BDD features (Gherkin)
└── LICENSE                    # AGPL-3.0-or-later
```

---

## 📄 Licença

**AGPL-3.0-or-later** — Código aberto, livre para usar, modificar e distribuir.
Consulte [LICENSE](LICENSE) para detalhes.

---

⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
