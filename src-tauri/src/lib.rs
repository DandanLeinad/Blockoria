// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Tauri adapter layer for Blockoria.
//!
//! This crate is the **driving adapter** in the hexagonal architecture.
//! It translates Tauri invoke() calls from the React frontend into
//! calls to use cases in `blockoria-application`.

use blockoria_domain::DomainError;
use blockoria_infrastructure::{FileBackupRepository, FileWorldRepository};
use std::sync::Arc;

// ============================================================================
// Error handling
// ============================================================================

mod error {
    use blockoria_domain::DomainError;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum CommandError {
        #[error("Domain error: {0}")]
        Domain(#[from] DomainError),
    }

    impl serde::Serialize for CommandError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    pub type CommandResult<T> = Result<T, CommandError>;
}

pub use error::{CommandError, CommandResult};

// ============================================================================
// Application State
// ============================================================================

/// Application state holding repositories.
#[derive(Clone)]
pub struct AppState {
    world_repo: Arc<FileWorldRepository>,
    #[allow(dead_code)]
    backup_repo: Arc<FileBackupRepository>,
}

/// Create the application state with real repositories.
pub fn create_state() -> Result<AppState, DomainError> {
    let world_repo = Arc::new(FileWorldRepository::new()?);
    let backup_repo = Arc::new(FileBackupRepository::with_default_path()?);
    Ok(AppState {
        world_repo,
        backup_repo,
    })
}

// ============================================================================
// Commands
// ============================================================================

mod commands {
    use super::*;
    use blockoria_application::ports::WorldRepository;
    use blockoria_application::use_cases::{
        create_backup as uc_create_backup, delete_backup as uc_delete_backup,
        list_backups as uc_list_backups, list_worlds as uc_list_worlds,
        restore_backup as uc_restore_backup,
    };
    use blockoria_domain::{AccountId, BackupPath, WorldFolderName, WorldLocation};
    use serde::Serialize;
    use tauri::State;

    #[derive(Serialize)]
    pub struct WorldSummaryDto {
        pub folder_name: String,
        pub level_name: String,
        pub version: [u16; 5],
        pub is_shared: bool,
        pub account_id: Option<String>,
        pub icon_path: Option<String>,
    }

    #[tauri::command]
    pub async fn cmd_list_worlds(
        state: State<'_, AppState>,
    ) -> CommandResult<Vec<WorldSummaryDto>> {
        let repo = &state.world_repo;
        let worlds = uc_list_worlds::list_worlds(repo.as_ref()).map_err(CommandError::from)?;
        let dtos = worlds
            .into_iter()
            .map(|w| WorldSummaryDto {
                folder_name: w.folder_name().as_str().to_string(),
                level_name: w.level_name().as_str().to_string(),
                version: *w.version().as_array(),
                is_shared: w.is_shared(),
                account_id: w.account_id().map(|id| id.as_str().to_string()),
                icon_path: w
                    .icon_path()
                    .as_path()
                    .map(|p| p.to_string_lossy().to_string()),
            })
            .collect();
        Ok(dtos)
    }

    #[derive(Serialize)]
    pub struct BackupSummaryDto {
        pub backup_path: String,
        pub timestamp: String,
        pub world_folder_name: String,
        pub world_version: [u16; 5],
    }

    #[tauri::command]
    pub async fn cmd_list_backups(
        world_folder_name: String,
        account_id: Option<String>,
        state: State<'_, AppState>,
    ) -> CommandResult<Vec<BackupSummaryDto>> {
        let folder_name = WorldFolderName::new(&world_folder_name).map_err(CommandError::from)?;
        let location = match account_id {
            Some(id) => WorldLocation::Account(AccountId::new(&id).map_err(CommandError::from)?),
            None => WorldLocation::Shared,
        };
        let repo = &state.backup_repo;
        let backups = uc_list_backups::list_backups(repo.as_ref(), &folder_name, &location)
            .map_err(CommandError::from)?;
        let dtos = backups
            .into_iter()
            .map(|b| BackupSummaryDto {
                backup_path: b.backup_path().as_path().to_string_lossy().to_string(),
                timestamp: b.created_at().to_iso_string(),
                world_folder_name: b.world_folder_name().as_str().to_string(),
                world_version: *b.world_version().as_array(),
            })
            .collect();
        Ok(dtos)
    }

    #[derive(Serialize)]
    pub struct CreateBackupResponseDto {
        pub backup_path: String,
        pub timestamp: String,
        pub world_folder_name: String,
    }

    #[tauri::command]
    pub async fn cmd_create_backup(
        world_folder_name: String,
        account_id: Option<String>,
        state: State<'_, AppState>,
    ) -> CommandResult<CreateBackupResponseDto> {
        let folder_name = WorldFolderName::new(&world_folder_name).map_err(CommandError::from)?;
        let _location = match account_id {
            Some(id) => WorldLocation::Account(AccountId::new(&id).map_err(CommandError::from)?),
            None => WorldLocation::Shared,
        };
        let world_repo = &state.world_repo;
        let backup_repo = &state.backup_repo;
        let world = world_repo
            .find_by_folder_name(&folder_name)
            .map_err(CommandError::from)?
            .ok_or_else(|| {
                CommandError::from(blockoria_domain::DomainError::InvalidWorldPath(
                    "World not found".into(),
                ))
            })?;
        let backup_root = backup_repo.backup_root();
        let backup =
            uc_create_backup::create_backup(&world, backup_root).map_err(CommandError::from)?;
        Ok(CreateBackupResponseDto {
            backup_path: backup.backup_path().as_path().to_string_lossy().to_string(),
            timestamp: backup.created_at().to_iso_string(),
            world_folder_name: backup.world_folder_name().as_str().to_string(),
        })
    }

    #[derive(Serialize)]
    pub struct RestoreBackupResponseDto {
        pub success: bool,
        pub message: String,
    }

    #[tauri::command]
    pub async fn cmd_restore_backup(
        backup_path: String,
        world_folder_name: String,
        account_id: Option<String>,
        state: State<'_, AppState>,
    ) -> CommandResult<RestoreBackupResponseDto> {
        let backup_path = BackupPath::new(&backup_path).map_err(CommandError::from)?;
        let folder_name = WorldFolderName::new(&world_folder_name).map_err(CommandError::from)?;
        let location = match account_id {
            Some(id) => WorldLocation::Account(AccountId::new(&id).map_err(CommandError::from)?),
            None => WorldLocation::Shared,
        };
        let world_repo = &state.world_repo;
        let backup_repo = &state.backup_repo;
        uc_restore_backup::restore_backup(
            backup_repo.as_ref(),
            world_repo.as_ref(),
            &folder_name,
            &location,
            &backup_path,
        )
        .map_err(CommandError::from)?;
        Ok(RestoreBackupResponseDto {
            success: true,
            message: "Backup restored successfully".to_string(),
        })
    }

    #[derive(Serialize)]
    pub struct DeleteBackupResponseDto {
        pub success: bool,
        pub message: String,
    }

    #[tauri::command]
    pub async fn cmd_delete_backup(
        backup_path: String,
        state: State<'_, AppState>,
    ) -> CommandResult<DeleteBackupResponseDto> {
        let backup_path = BackupPath::new(&backup_path).map_err(CommandError::from)?;
        let repo = &state.backup_repo;
        uc_delete_backup::delete_backup(repo.as_ref(), &backup_path).map_err(CommandError::from)?;
        Ok(DeleteBackupResponseDto {
            success: true,
            message: "Backup deleted successfully".to_string(),
        })
    }
}

/// Entry point called from main.rs with real repositories.
pub fn run_with_state(state: AppState) {
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::cmd_list_worlds,
            commands::cmd_list_backups,
            commands::cmd_create_backup,
            commands::cmd_restore_backup,
            commands::cmd_delete_backup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Blockoria application");
}
