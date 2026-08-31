# Blockoria

> **Gerenciador de backups de mundos Minecraft Bedrock Edition para Windows 10/11.**
> Interface gráfica nativa, backups versionados, restauração com preview.

> ⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
> Este é um projeto open-source independente, desenvolvido como hobby/estudo.

[![License: AGPL-3.0-only](https://img.shields.io/badge/License-AGPL%203.0--only-blue.svg?style=for-the-badge)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-4f46e5?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-4f46e5?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078d6?style=for-the-badge&logo=windows&logoColor=white)](https://microsoft.com/windows)

---

## 🏗️ Status Atual

**Em desenvolvimento ativo** — **Domain, Application e Infrastructure implementadas**.

| Camada | Status |
|--------|--------|
| **Domain** (`blockoria-domain`) | ✅ Completo — 9 VOs, Entities, Aggregates, 66 testes + 12 doctests |
| **Application** (`blockoria-application`) | ✅ Implementado — 5 use cases, 15 testes |
| **Infrastructure** (`blockoria-infrastructure`) | ✅ Implementado — FileWorldRepository, FileBackupRepository, Config (41 testes) |
| **Frontend (Tauri + React)** | ❌ Não iniciado |

---

## 📦 blockoria-domain (Concluído)

Camada de domínio pura, sem dependências externas.

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

```bash
cargo test -p blockoria-domain        # 66 unit tests + 12 doctests = 78
cargo test -p blockoria-application   # 15 use case tests
cargo test -p blockoria-infrastructure # 21 unit + 20 integration = 41
# Total: 134 passing
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
│   ├── blockoria-domain/      # ✅ Completo (9 VOs, 66 testes)
│   ├── blockoria-application/ # ✅ Implementado (5 use cases, 15 testes)
│   └── blockoria-infrastructure/ # ✅ Implementado (repos, config, 41 testes)
├── docs/                      # Documentação (Zensical)
└── LICENSE                    # AGPL-3.0-or-later
```

---

## 🎯 Próximos Passos

1. **Frontend** — Tauri 2 + React + TypeScript (`src-tauri/`)
2. **Integração** — Tauri commands chamando use cases, composition root em `main.rs`

---

## 📄 Licença

**AGPL-3.0-or-later** — Código aberto, livre para usar, modificar e distribuir.
Consulte [LICENSE](LICENSE) para detalhes.

---

⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
