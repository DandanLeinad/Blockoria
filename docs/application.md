---
icon: lucide/layers
---

# Aplicação (`blockoria-application`)

Camada de aplicação com use cases e ports (traits). Contém a lógica de negócio orquestrada, sem dependências de infraestrutura.

## Ports (Traits)

### `WorldRepository`
- `list_all()` — Retorna todos os mundos conhecidos
- `find_by_folder_name(folder_name, location)` — Busca mundo pelo folder name + location

### `BackupRepository`
- `save()` — Persiste um backup (reservado para uso futuro)
- `list_by_world(location)` — Lista backups de um mundo por `WorldLocation`
- `delete(path)` — Remove um backup pelo caminho (idempotente)

## Use Cases (5)

| Use Case | Descrição | Testes |
|----------|-----------|--------|
| `create_backup` | Cria backup de um mundo copiando arquivos | 5 |
| `list_worlds` | Lista todos os mundos conhecidos | 2 |
| `list_backups` | Lista backups filtrando por mundo + conta via `WorldLocation` | 4 |
| `restore_backup` | Restaura mundo a partir de backup | 3 |
| `delete_backup` | Remove backup pelo caminho | 1 |

**Total: 15 testes**

## Utilitários

- `util::copy_dir_all` — Cópia recursiva de diretórios (usado por create_backup e restore_backup)

## Testes

| Métrica | Valor |
|---------|-------|
| Testes unitários | 15 |
| **Total** | **15 passando** |

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

## Detalhes dos Use Cases

### `create_backup`
Cria um novo backup de um mundo Minecraft Bedrock.
- **Input**: `World` aggregate + `backup_root` path
- **Processo**: Gera timestamp → cria diretório sanitizado → copia recursivamente todos os arquivos
- **Output**: `Backup` aggregate com path, timestamp, version, folder_name, account_id
- **Erros**: `DomainError::InvalidBackupPath` para falhas de I/O

### `list_worlds`
Lista todos os mundos conhecidos no repositório.
- **Input**: `&dyn WorldRepository`
- **Output**: `Vec<World>`
- **Erros**: Propaga erros do repositório

### `list_backups`
Lista backups filtrando por mundo + conta via `WorldLocation`.
- **Input**: `folder_name: &WorldFolderName`, `location: &WorldLocation`, `&dyn BackupRepository`
- **Output**: `Vec<Backup>` ordenado por timestamp descendente (mais recente primeiro)
- **Erros**: Propaga erros do repositório

### `restore_backup`
Restaura um mundo a partir de um backup específico.
- **Input**: `backup_path`, `folder_name`, `location`, `&dyn BackupRepository`, `&dyn WorldRepository`
- **Processo**: Verifica se backup pertence ao mundo/conta → obtém world path → copia arquivos do backup para world (overwrite)
- **Output**: `()`
- **Erros**: `InvalidBackupPath` (backup não encontrado), `InvalidWorldPath` (mundo não encontrado)

### `delete_backup`
Remove um backup pelo caminho.
- **Input**: `backup_path`, `&dyn BackupRepository`
- **Processo**: `fs::remove_dir_all` (idempotente - OK se não existir)
- **Output**: `()`
- **Erros**: `InvalidBackupPath` para falhas de I/O

## Integração com Tauri (Commands)

Os commands Tauri em `src-tauri/src/lib.rs` chamam diretamente estes use cases:

| Command | Use Case | Parâmetros |
|---------|----------|------------|
| `cmd_list_worlds` | `list_worlds` | `state: State<AppState>` |
| `cmd_create_backup` | `create_backup` | `world_folder_name`, `account_id`, `state` |
| `cmd_list_backups` | `list_backups` | `world_folder_name`, `account_id`, `state` |
| `cmd_restore_backup` | `restore_backup` | `backup_path`, `world_folder_name`, `account_id`, `state` |
| `cmd_delete_backup` | `delete_backup` | `backup_path`, `state` |

> **Nota**: Os commands `cmd_get_config` e `cmd_save_config` acessam `Config` diretamente no `AppState` (não passam por use cases), pois configuração é concern da infraestrutura, não regra de negócio.
