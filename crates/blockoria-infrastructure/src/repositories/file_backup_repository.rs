// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Filesystem implementation of `BackupRepository` for Minecraft Bedrock world backups.
//!
//! Stores backups under a configurable root directory organized as:
//! `<backup_root>/<location>/<sanitized_folder_name>/<timestamp>/`

use blockoria_application::ports::BackupRepository;
use blockoria_domain::{
    AccountId, Backup, BackupPath, BackupTimestamp, DomainError, WorldFolderName, WorldLocation,
    WorldVersion,
};
use std::fs;
use std::path::{Path, PathBuf};

/// Filesystem-based backup repository.
///
/// Backup directory structure:
/// `<backup_root>/<location>/<sanitized_folder_name>/<timestamp>/`
/// where location is either account_id or "Shared"
pub struct FileBackupRepository {
    backup_root: PathBuf,
}

impl FileBackupRepository {
    /// Creates a new repository with a custom backup root path.
    pub fn new(backup_root: impl Into<PathBuf>) -> Self {
        let root = backup_root.into();
        fs::create_dir_all(&root).ok();
        Self { backup_root: root }
    }

    /// Creates a repository with the default backup path.
    ///
    /// Default: `%APPDATA%\Blockoria\backups\`
    pub fn with_default_path() -> Result<Self, DomainError> {
        let root = dirs::data_dir()
            .ok_or_else(|| {
                DomainError::InvalidBackupPath("Could not find APPDATA directory".into())
            })?
            .join("Blockoria")
            .join("backups");
        fs::create_dir_all(&root).ok();
        Ok(Self { backup_root: root })
    }

    /// Returns the backup root path.
    pub fn backup_root(&self) -> &Path {
        &self.backup_root
    }

    /// Computes the world-specific backup directory.
    fn world_backup_dir(&self, location: &WorldLocation, folder_name: &WorldFolderName) -> PathBuf {
        // Sanitize folder name for filesystem (replace '=' with '_' for Windows compatibility)
        let safe_folder_name = folder_name.as_str().replace('=', "_");
        self.backup_root
            .join(location.as_path_segment())
            .join(safe_folder_name)
    }
}

impl BackupRepository for FileBackupRepository {
    fn save(&self, backup: &Backup) -> Result<(), DomainError> {
        // The backup directory should already exist (created by use case)
        // This method is a no-op for filesystem implementation since the use case
        // handles the actual file copying. Kept for interface completeness.
        let _ = backup;
        Ok(())
    }

    fn list_by_world(
        &self,
        folder_name: &WorldFolderName,
        location: &WorldLocation,
    ) -> Result<Vec<Backup>, DomainError> {
        let world_backup_dir = self.world_backup_dir(location, folder_name);

        if !world_backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();

        let entries = fs::read_dir(&world_backup_dir)?;
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }

            let dir_name = entry.file_name().to_string_lossy().to_string();
            let timestamp = match BackupTimestamp::from_filename_safe(&dir_name) {
                Ok(ts) => ts,
                Err(_) => continue, // Skip invalid timestamp directories
            };

            let backup_path = BackupPath::new(entry.path())?;

            let account_id = match location {
                WorldLocation::Account(id) => id.clone(),
                WorldLocation::Shared => {
                    // Shared worlds don't have an account_id in backup context
                    // Return placeholder
                    AccountId::new("shared").unwrap()
                }
            };

            let backup = Backup::new(
                folder_name.clone(),
                account_id,
                WorldVersion::default(),
                timestamp,
                backup_path,
            );

            backups.push(backup);
        }

        // Sort by timestamp descending (newest first)
        backups.sort_by(|a, b| b.created_at().cmp(a.created_at()));

        Ok(backups)
    }

    fn delete(&self, backup_path: &Path) -> Result<(), DomainError> {
        if backup_path.exists() {
            fs::remove_dir_all(backup_path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockoria_domain::{
        Backup, BackupPath, BackupTimestamp, WorldFolderName, WorldLocation, WorldVersion,
    };
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, FileBackupRepository) {
        let temp = TempDir::new().unwrap();
        let repo = FileBackupRepository::new(temp.path());
        (temp, repo)
    }

    #[test]
    fn given_empty_backup_root_when_list_by_world_then_returns_empty() {
        // Given
        let (_temp, repo) = setup_test_env();
        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());

        // When
        let result = repo.list_by_world(&folder, &location);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn given_backups_when_save_and_list_then_returns_correct_backups() {
        // Given
        let (_temp, repo) = setup_test_env();
        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());

        // Create backup in the expected directory structure
        let backup_root = repo.backup_root();
        let backup_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-01T00-00-00Z");
        fs::create_dir_all(&backup_dir).unwrap();

        let backup = Backup::new(
            folder.clone(),
            AccountId::new("111111111111111111").unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            BackupTimestamp::now(),
            BackupPath::new(&backup_dir).unwrap(),
        );

        repo.save(&backup).unwrap();
        let result = repo.list_by_world(&folder, &location);

        // Then
        assert!(result.is_ok());
        let backups = result.unwrap();
        assert_eq!(backups.len(), 1);
        assert_eq!(backups[0].world_folder_name().as_str(), "aaaaaaaaaaa=");
    }

    #[test]
    fn given_multiple_backups_when_list_by_world_then_filters_by_folder_and_account() {
        // Given
        let (_temp, repo) = setup_test_env();
        let backup_root = repo.backup_root();

        // Create real backup directories with timestamp structure (format: YYYY-MM-DDTHH-MM-SSZ)
        let b1_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-01T00-00-00Z");
        let b2_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-02T00-00-00Z");
        let b3_dir = backup_root
            .join("111111111111111111")
            .join("bbbbbbbbbbb_")
            .join("2024-01-01T00-00-00Z");
        let b4_dir = backup_root
            .join("222222222222222222")
            .join("aaaaaaaaaaa_")
            .join("2024-01-01T00-00-00Z");

        fs::create_dir_all(&b1_dir).unwrap();
        fs::create_dir_all(&b2_dir).unwrap();
        fs::create_dir_all(&b3_dir).unwrap();
        fs::create_dir_all(&b4_dir).unwrap();

        let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());
        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();

        // When
        let result = repo.list_by_world(&folder, &location);

        // Then
        assert!(result.is_ok());
        let backups = result.unwrap();
        assert_eq!(backups.len(), 2); // only b1 and b2 (same folder + same account)
    }

    #[test]
    fn given_shared_location_when_list_by_world_then_returns_empty() {
        // Given
        let (_temp, repo) = setup_test_env();
        let backup_root = repo.backup_root();

        let b1_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-01T00-00-00Z");
        fs::create_dir_all(&b1_dir).unwrap();

        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let location = WorldLocation::Shared;

        // When
        let result = repo.list_by_world(&folder, &location);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn given_backup_when_delete_then_removes_directory() {
        // Given
        let (_temp, repo) = setup_test_env();
        let backup_root = repo.backup_root();

        let backup_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-01T00-00-00Z");
        fs::create_dir_all(&backup_dir).unwrap();
        fs::write(backup_dir.join("level.dat"), b"backup").unwrap();

        let backup = Backup::new(
            WorldFolderName::new("aaaaaaaaaaa=").unwrap(),
            AccountId::new("111111111111111111").unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            BackupTimestamp::now(),
            BackupPath::new(&backup_dir).unwrap(),
        );

        // When
        repo.save(&backup).unwrap();
        let result = repo.delete(backup.backup_path().as_path());

        // Then
        assert!(result.is_ok());
        assert!(
            !backup_dir.exists(),
            "Backup directory should have been removed"
        );
    }

    #[test]
    fn given_nonexistent_backup_when_delete_then_succeeds_idempotent() {
        // Given
        let (temp, repo) = setup_test_env();
        let fake_path = temp.path().join("nonexistent");

        // When
        let result = repo.delete(&fake_path);

        // Then
        assert!(result.is_ok(), "Delete should be idempotent");
    }

    #[test]
    fn given_backup_timestamp_ordering_when_list_then_sorted_descending() {
        // Given
        let (_temp, repo) = setup_test_env();
        let backup_root = repo.backup_root();

        // Create backups with different timestamps (format: YYYY-MM-DDTHH-MM-SSZ)
        let b1_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-01T00-00-00Z");
        let b2_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-03T00-00-00Z"); // newest
        let b3_dir = backup_root
            .join("111111111111111111")
            .join("aaaaaaaaaaa_")
            .join("2024-01-02T00-00-00Z");

        fs::create_dir_all(&b1_dir).unwrap();
        fs::create_dir_all(&b2_dir).unwrap();
        fs::create_dir_all(&b3_dir).unwrap();

        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());

        // When
        let result = repo.list_by_world(&folder, &location);

        // Then: should be sorted descending (newest first)
        assert!(result.is_ok());
        let backups = result.unwrap();
        assert_eq!(backups.len(), 3);
        let timestamps: Vec<_> = backups
            .iter()
            .map(|b| b.created_at().to_filename_safe())
            .collect();
        assert!(timestamps[0] > timestamps[1]); // 2024-01-03 > 2024-01-02
        assert!(timestamps[1] > timestamps[2]); // 2024-01-02 > 2024-01-01
    }
}
