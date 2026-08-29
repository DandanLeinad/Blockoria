// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! List all backups for a specific world and account.
//!
//! This use case retrieves all backups for a given world (identified by
// folder name and account ID) by delegating to the
// `BackupRepository::list_by_world` method.

use crate::ports::BackupRepository;
use blockoria_domain::{AccountId, Backup, DomainError, WorldFolderName};

/// Returns all backups for a given world.
pub fn list_backups(
    repo: &dyn BackupRepository,
    folder_name: &WorldFolderName,
    account_id: &AccountId,
) -> Result<Vec<Backup>, DomainError> {
    repo.list_by_world(folder_name, account_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockoria_domain::{
        AccountId, Backup, BackupPath, BackupTimestamp, DomainError, WorldFolderName, WorldVersion,
    };
    use tempfile::TempDir;

    struct MockBackupRepo {
        backups: Vec<Backup>,
    }

    impl MockBackupRepo {
        fn new(backups: Vec<Backup>) -> Self {
            Self { backups }
        }
    }

    impl BackupRepository for MockBackupRepo {
        fn save(&self, _backup: &Backup) -> Result<(), DomainError> {
            Ok(())
        }

        fn list_by_world(
            &self,
            folder_name: &WorldFolderName,
            account_id: &AccountId,
        ) -> Result<Vec<Backup>, DomainError> {
            Ok(self
                .backups
                .iter()
                .filter(|b| {
                    b.world_folder_name() == folder_name && b.world_account_id() == account_id
                })
                .cloned()
                .collect())
        }

        fn delete(&self, _backup_path: &BackupPath) -> Result<(), DomainError> {
            Ok(())
        }
    }

    fn make_backup(folder: &str, account: &str) -> Backup {
        let temp = TempDir::new().unwrap();
        std::fs::create_dir_all(temp.path()).unwrap();
        Backup::new(
            WorldFolderName::new(folder).unwrap(),
            AccountId::new(account).unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            BackupTimestamp::now(),
            BackupPath::new(temp.path()).unwrap(),
        )
    }

    #[test]
    fn given_empty_repo_when_list_backups_then_returns_empty_vec() {
        // Given
        let repo = MockBackupRepo::new(vec![]);
        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let account = AccountId::new("123456789012345678").unwrap();

        // When
        let result = list_backups(&repo, &folder, &account);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn given_repo_with_backups_when_list_backups_then_filters_by_world_and_account() {
        // Given
        let b1 = make_backup("aaaaaaaaaaa=", "123456789012345678");
        let b2 = make_backup("aaaaaaaaaaa=", "123456789012345678");
        let b3 = make_backup("bbbbbbbbbbb=", "123456789012345678"); // different world
        let repo = MockBackupRepo::new(vec![b1, b2, b3]);
        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let account = AccountId::new("123456789012345678").unwrap();

        // When
        let result = list_backups(&repo, &folder, &account);

        // Then
        assert!(result.is_ok());
        let backups = result.unwrap();
        assert_eq!(backups.len(), 2);
    }

    #[test]
    fn given_different_account_when_list_backups_then_returns_empty() {
        // Given
        let b1 = make_backup("aaaaaaaaaaa=", "111111111111111111");
        let repo = MockBackupRepo::new(vec![b1]);
        let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
        let account = AccountId::new("222222222222222222").unwrap();

        // When
        let result = list_backups(&repo, &folder, &account);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
