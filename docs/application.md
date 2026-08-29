---
icon: lucide/layers
---

# Aplicação (`blockoria-application`)

Camada de aplicação com use cases e ports (traits). Contém a lógica de negócio orquestrada, sem dependências de infraestrutura.

## Ports (Traits)

### `WorldRepository`
- `list_all()` — Retorna todos os mundos conhecidos
- `find_by_folder_name()` — Busca mundo pelo folder name

### `BackupRepository`
- `save()` — Persiste um backup (reservado para uso futuro)
- `list_by_world()` — Lista backups de um mundo (folder_name + account_id)
- `delete()` — Remove um backup pelo caminho

## Use Cases (5)

| Use Case | Descrição | Testes |
|----------|-----------|--------|
| `create_backup` | Cria backup de um mundo copiando arquivos | 5 |
| `list_worlds` | Lista todos os mundos conhecidos | 2 |
| `list_backups` | Lista backups filtrando por mundo + conta | 3 |
| `restore_backup` | Restaura mundo a partir de backup | 3 |
| `delete_backup` | Remove backup pelo caminho | 1 |

## Utilitários

- `util::copy_dir_all` — Cópia recursiva de diretórios (usado por create_backup e restore_backup)

## Testes

| Métrica | Valor |
|---------|-------|
| Testes unitários | 14 |
| **Total** | **14 passando** |

```bash
cargo test -p blockoria-application
```

## Estrutura

```
crates/blockoria-application/
├── Cargo.toml
└── src/
    ├── lib.rs              # Re-exports públicos
    ├── error.rs            # Result<T> = Result<T, DomainError>
    ├── ports/
    │   ├── mod.rs
    │   ├── world_repository.rs
    │   └── backup_repository.rs
    ├── use_cases/
    │   ├── mod.rs
    │   ├── create_backup.rs
    │   ├── list_worlds.rs
    │   ├── list_backups.rs
    │   ├── restore_backup.rs
    │   └── delete_backup.rs
    └── util.rs             # copy_dir_all
```

## Arquitetura

```text
Application Layer
    ├── ports/          # Traits (interfaces)
    ├── use_cases/      # Lógica de negócio
    └── util/           # Utilitários compartilhados
         ↓ depends on
    Domain Layer (blockoria-domain)
```

A camada de aplicação **não depende de infraestrutura** (filesystem, network, etc.). Tudo externo é acessado via ports (traits) implementados na camada de infraestrutura.
