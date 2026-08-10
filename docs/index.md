---
icon: lucide/database-backup
hide:
  - toc
  - navigation
---

# Blockoria

> **Minecraft Bedrock world backup manager for Windows 10/11.**
> Native GUI, versioned backups, restore with preview.

!!! warning "⚠️ Legal Disclaimer"
    **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
    This is an independent open-source project, developed as a hobby/learning project.

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg?style=for-the-badge)](https://github.com/DandanLeinad/blockoria/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-4f46e5?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-4f46e5?style=for-the-badge&logo=tauri&logoColor=white)](https://tauri.app)
[![Windows](https://img.shields.io/badge/Windows-10%2F11-0078d6?style=for-the-badge&logo=windows&logoColor=white)](https://microsoft.com/windows)

---

## 🏗️ Current Status

**Under active development** — Currently implementing only the **domain layer** (`blockoria-domain`).

| Layer | Status |
|-------|--------|
| **Domain** (`blockoria-domain`) | ✅ Complete — VOs, Entities, Aggregates, Tests |
| **Application** (`blockoria-application`) | ❌ Not started |
| **Infrastructure** (`blockoria-infrastructure`) | ❌ Not started |
| **Frontend (Tauri + React)** | ❌ Not started |

---

## 📦 blockoria-domain (Complete)

The pure domain layer, no external dependencies, containing:

### Value Objects (8)
| VO | Description | Tests |
|------|-------------|-------|
| `WorldFolderName` | World folder name (12 chars + `=`) | 7 |
| `LevelName` | World display name | 4 |
| `WorldPath` | World path (FS: exists + is_dir) | 5 |
| `AccountId` | Microsoft account ID | 4 |
| `WorldVersion` | World version `[u16; 5]` ≥ 0 | 4 |
| `WorldIconPath` | Icon path (`world_icon.jpeg`) | 6 |
| `BackupTimestamp` | Backup UTC timestamp | 6 |
| `BackupPath` | Backup directory path | 5 |

### Entities / Aggregates
| Entity | Type | Description |
|--------|------|-------------|
| `World` | Aggregate Root | Minecraft world with folder_name, level_name, path, account_id, version, icon_path |
| `Backup` | Aggregate Root | Backup with world_folder_name, world_account_id, world_version, created_at, backup_path |

### DomainError (8 variants)
| Variant | When it occurs |
|---------|----------------|
| `InvalidWorldFolderName` | Invalid format (not 12 chars, doesn't end with `=`, whitespace) |
| `InvalidWorldPath` | Empty, doesn't exist, not a directory |
| `InvalidWorldIconPath` | Filename isn't `world_icon.jpeg` |
| `InvalidLevelName` | Empty or only whitespace |
| `InvalidAccountId` | Empty or only whitespace |
| `InvalidWorldVersion` | Not 5 elements or contains negatives |
| `InvalidBackupTimestamp` | Before Unix epoch (1970) |
| `InvalidBackupPath` | Empty, doesn't exist, not a directory |

---

## ✅ Tests

| Metric | Value |
|--------|-------|
| Unit tests | 53 |
| Doctests | 11 |
| **Total** | **64 passing** |

```bash
cargo test -p blockoria-domain
# 53 passed
cargo test -p blockoria-domain --doc
# 11 passed
```

---

## 🛠️ Tech Stack (Domain)

| Item | Version |
|------|---------|
| Rust | 1.80+ |
| Edition | 2024 |
| Testing | `cargo test` (built-in) |
| Serialization | `serde` (planned) |

---

## 📁 Crate Structure

```
crates/blockoria-domain/
├── Cargo.toml
└── src/
    ├── lib.rs              # Public re-exports
    ├── error.rs            # DomainError (8 variants)
    ├── world_folder_name.rs
    ├── level_name.rs
    ├── world_path.rs
    ├── account_id.rs
    ├── world_version.rs
    ├── world_icon_path.rs
    ├── backup_timestamp.rs
    ├── backup_path.rs
    └── entities.rs         # World, Backup
```

---

## 🎯 Next Steps

1. **Application Layer** (`blockoria-application`) — Use Cases, Ports (Traits)
2. **Infrastructure** (`blockoria-infrastructure`) — File Repositories, Tauri Commands
3. **Frontend** — Tauri 2 + React + TypeScript

---

## 📄 License

**AGPL-3.0-or-later** — Open source, free to use, modify and distribute.
See [LICENSE](../../LICENSE) for details.

---

⚠️ **NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.**
