// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Delete a backup by its path.
//!
//! This use case deletes a backup by delegating to the
//! `BackupRepository::delete` method with the provided backup path.

use crate::ports::BackupRepository;
use blockoria_domain::{BackupPath, DomainError};

/// Deletes a backup by its backup path.
pub fn delete_backup(
    repo: &dyn BackupRepository,
    backup_path: &BackupPath,
) -> Result<(), DomainError> {
    repo.delete(backup_path.as_path())
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockoria_domain::{Backup, BackupPath, DomainError, WorldFolderName, WorldLocation};
    use tempfile::TempDir;

    struct MockBackupRepo {
        deleted: std::sync::Mutex<Vec<BackupPath>>,
    }

    impl MockBackupRepo {
        fn new() -> Self {
            Self {
                deleted: std::sync::Mutex::new(vec![]),
            }
        }
    }

    impl BackupRepository for MockBackupRepo {
        fn save(&self, _backup: &Backup) -> Result<(), DomainError> {
            Ok(())
        }

        fn list_by_world(
            &self,
            _folder_name: &WorldFolderName,
            _location: &WorldLocation,
        ) -> Result<Vec<Backup>, DomainError> {
            Ok(vec![])
        }

        fn delete(&self, backup_path: &std::path::Path) -> Result<(), DomainError> {
            // Convert Path to BackupPath for storage
            let bp = BackupPath::new(backup_path).unwrap();
            self.deleted.lock().unwrap().push(bp);
            Ok(())
        }
    }

    fn make_backup_path() -> (BackupPath, TempDir) {
        let temp = TempDir::new().unwrap();
        let path = BackupPath::new(temp.path()).unwrap();
        (path, temp)
    }

    #[test]
    fn given_valid_backup_path_when_delete_then_calls_repo_delete() {
        // Given
        let repo = MockBackupRepo::new();
        let (backup_path, _temp) = make_backup_path();

        // When
        let result = delete_backup(&repo, &backup_path);

        // Then
        assert!(result.is_ok());
        let deleted = repo.deleted.lock().unwrap();
        assert_eq!(deleted.len(), 1);
        assert_eq!(deleted[0], backup_path);
    }
}
