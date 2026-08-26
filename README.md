# Blockoria

> **Gerenciador de backups de mundos Minecraft Bedrock Edition para Windows 10/11.**
> Interface gráfica nativa, backups versionados, restauração com preview.

> ⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
> Este é um projeto open-source independente, desenvolvido como hobby/estudo.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-4f46e5?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-4f46e5?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078d6?style=for-the-badge&logo=windows&logoColor=white)](https://microsoft.com/windows)

---

## 🏗️ Status Atual

**Em desenvolvimento ativo** — Camada de domínio completa, **Application iniciada** (`create_backup`).

| Camada | Status |
|--------|--------|
| **Domain** (`blockoria-domain`) | ✅ Completo — VOs, Entities, Aggregates, Testes |
| **Application** (`blockoria-application`) | 🚧 Iniciado — `create_backup` (use case + 5 testes) |
| **Infrastructure** (`blockoria-infrastructure`) | ❌ Não iniciado |
| **Frontend (Tauri + React)** | ❌ Não iniciado |

---

## 📦 blockoria-domain (Concluído)

Camada de domínio pura, sem dependências externas.

### Value Objects (8)
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

```bash
cargo test -p blockoria-domain    # 54 unit tests
cargo test -p blockoria-domain --doc  # 11 doctests
# Total: 65 passing
```

---

## 🛠️ Tech Stack (Domain)

| Item | Versão |
|------|--------|
| Rust | 1.80+ |
| Edition | 2024 |
| Testes | `cargo test` (built-in) |
| Serialização | `serde` (planejado) |

---

## 📁 Estrutura do Workspace

```
blockoria/
├── Cargo.toml
├── crates/
│   ├── blockoria-domain/      # ✅ Completo
│   ├── blockoria-application/ # ❌ Não iniciado
│   └── blockoria-infrastructure/ # ❌ Não iniciado
├── docs/                      # Documentação (Zensical)
└── LICENSE                    # AGPL-3.0-or-later
```

---

## 🎯 Próximos Passos

1. **Application Layer** (`blockoria-application`) — Demais Use Cases, Ports (Traits)
2. **Infrastructure** (`blockoria-infrastructure`) — File Repositories, Tauri Commands
3. **Frontend** — Tauri 2 + React + TypeScript

---

## 📄 Licença

**AGPL-3.0-or-later** — Código aberto, livre para usar, modificar e distribuir.
Consulte [LICENSE](LICENSE) para detalhes.

---

⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
