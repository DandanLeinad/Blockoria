---
icon: lucide/database
---

# Domínio (`blockoria-domain`)

Camada de domínio pura, sem dependências externas.

## Value Objects (9)

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
| `WorldLocation` | Localização do mundo: `Account(AccountId)` \| `Shared` | 11 |

## Novos métodos / traits (atualizações recentes)

- `BackupTimestamp::from_filename_safe()` — Parse de timestamp do nome do diretório (`YYYY-MM-DDTHH-MM-SSZ`)
- `BackupTimestamp: Ord` — Ordenação para listar backups mais recentes primeiro
- `WorldVersion: Default` — Versão padrão `[0,0,0,0,0]`
- `AccountId: Hash` — Para uso como chave em `HashMap`
- `From<io::Error> for DomainError` — Conversão automática de erros de I/O

## Entities / Aggregates

| Entity | Tipo | Descrição |
|--------|------|-----------|
| `World` | Aggregate Root | Mundo Minecraft com folder_name, level_name, path, account_id, version, icon_path |
| `Backup` | Aggregate Root | Backup com world_folder_name, world_account_id, world_version, created_at, backup_path |

## DomainError (8 variants)

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

## Testes

| Métrica | Valor |
|---------|-------|
| Testes unitários | 66 |
| Doctests | 12 |
| **Total** | **78 passando** |

```bash
cargo test -p blockoria-domain        # 66 passed
cargo test -p blockoria-domain --doc  # 12 passed
```

## Estrutura do Crate

```
crates/blockoria-domain/
├── Cargo.toml
└── src/
    ├── lib.rs                  # Re-exports públicos
    ├── error.rs                # DomainError (8 variants)
    ├── world_folder_name.rs
    ├── level_name.rs
    ├── world_path.rs
    ├── account_id.rs
    ├── world_version.rs
    ├── world_icon_path.rs
    ├── backup_timestamp.rs
    ├── backup_path.rs
    ├── world_location.rs       # NOVO: WorldLocation enum
    └── entities.rs             # World, Backup
```

## Próximos Passos

A camada de domínio está **completa**. Próximos passos na camada de aplicação e infraestrutura.
