// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Restore a world from a backup.
//!
//! This use case restores a world to a previous state by copying all files
//! from a backup directory to the world directory, overwriting existing files.
//!
//! # Operation
//! 1. Verifies the backup exists and belongs to the specified world/location
//! 2. Retrieves the world to get the destination path
//! 3. Recursively copies all files from backup to world directory
//! 4. Returns success or error

use crate::ports::{BackupRepository, WorldRepository};
use crate::util::copy_dir_all;
use blockoria_domain::{BackupPath, DomainError, WorldFolderName, WorldLocation};

/// Restores a world from a backup.
///
/// Copies all files from the backup directory to the world directory,
/// overwriting existing files.
pub fn restore_backup(
    backup_repo: &dyn BackupRepository,
    world_repo: &dyn WorldRepository,
    folder_name: &WorldFolderName,
    location: &WorldLocation,
    backup_path: &BackupPath,
) -> Result<(), DomainError> {
    // Verify backup exists and belongs to this world/location
    let backups = backup_repo.list_by_world(folder_name, location)?;
    let backup = backups
        .iter()
        .find(|b| b.backup_path() == backup_path)
        .ok_or(DomainError::InvalidBackupPath("Backup not found".into()))?;

    // Get world to find destination path
    let world = world_repo
        .find_by_folder_name(folder_name)?
        .ok_or(DomainError::InvalidWorldPath("World not found".into()))?;

    // Copy backup contents to world path (overwrite)
    copy_dir_all(backup.backup_path().as_path(), world.path().as_path())
        .map_err(|e| DomainError::InvalidBackupPath(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockoria_domain::{
        AccountId, Backup, BackupPath, BackupTimestamp, DomainError, LevelName, World,
        WorldFolderName, WorldIconPath, WorldLocation, WorldPath, WorldVersion,
    };
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;

    struct MockWorldRepo {
        world: Option<World>,
    }

    impl MockWorldRepo {
        fn new(world: World) -> Self {
            Self { world: Some(world) }
        }
    }

    impl WorldRepository for MockWorldRepo {
        fn list_all(&self) -> Result<Vec<World>, DomainError> {
            Ok(self
                .world
                .as_ref()
                .map(|w| vec![w.clone()])
                .unwrap_or_default())
        }

        fn find_by_folder_name(
            &self,
            folder_name: &WorldFolderName,
        ) -> Result<Option<World>, DomainError> {
            if self.world.as_ref().map(|w| w.folder_name()) == Some(folder_name) {
                Ok(self.world.clone())
            } else {
                Ok(None)
            }
        }
    }

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
            location: &WorldLocation,
        ) -> Result<Vec<Backup>, DomainError> {
            Ok(self
                .backups
                .iter()
                .filter(|b| {
                    b.world_folder_name() == folder_name
                        && match location {
                            WorldLocation::Account(account_id) => {
                                b.world_account_id() == account_id
                            }
                            WorldLocation::Shared => false, // Shared worlds don't have account_id in backup
                        }
                })
                .cloned()
                .collect())
        }

        fn delete(&self, _backup_path: &BackupPath) -> Result<(), DomainError> {
            Ok(())
        }
    }

    fn make_world(folder: &str, name: &str, path: &Path, location: WorldLocation) -> World {
        World::new(
            WorldFolderName::new(folder).unwrap(),
            LevelName::new(name).unwrap(),
            WorldPath::new(path).unwrap(),
            location,
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            WorldIconPath::new(None::<PathBuf>).unwrap(),
        )
    }

    fn make_backup(folder: &str, account: &str, path: &Path) -> Backup {
        Backup::new(
            WorldFolderName::new(folder).unwrap(),
            AccountId::new(account).unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            BackupTimestamp::now(),
            BackupPath::new(path).unwrap(),
        )
    }

    #[test]
    fn given_valid_backup_and_world_when_restore_then_copies_files() {
        // Given
        let backup_dir = TempDir::new().unwrap();
        fs::write(backup_dir.path().join("level.dat"), b"restored").unwrap();
        fs::write(backup_dir.path().join("levelname.txt"), b"Restored World").unwrap();

        let world_dir = TempDir::new().unwrap();
        fs::write(world_dir.path().join("level.dat"), b"original").unwrap();

        let backup = make_backup("aaaaaaaaaaa=", "123456789012345678", backup_dir.path());
        let world = make_world(
            "aaaaaaaaaaa=",
            "Test World",
            world_dir.path(),
            WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
        );

        let backup_repo = MockBackupRepo::new(vec![backup.clone()]);
        let world_repo = MockWorldRepo::new(world);

        // When
        let result = restore_backup(
            &backup_repo,
            &world_repo,
            &WorldFolderName::new("aaaaaaaaaaa=").unwrap(),
            &WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
            backup.backup_path(),
        );

        // Then
        assert!(result.is_ok());
        assert_eq!(
            fs::read(world_dir.path().join("level.dat")).unwrap(),
            b"restored"
        );
        assert_eq!(
            fs::read(world_dir.path().join("levelname.txt")).unwrap(),
            b"Restored World"
        );
    }

    #[test]
    fn given_backup_not_found_when_restore_then_returns_error() {
        // Given
        let backup_dir = TempDir::new().unwrap();
        let world_dir = TempDir::new().unwrap();

        let backup = make_backup("aaaaaaaaaaa=", "123456789012345678", backup_dir.path());
        let world = make_world(
            "aaaaaaaaaaa=",
            "Test World",
            world_dir.path(),
            WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
        );

        let backup_repo = MockBackupRepo::new(vec![]); // empty
        let world_repo = MockWorldRepo::new(world);

        // When
        let result = restore_backup(
            &backup_repo,
            &world_repo,
            &WorldFolderName::new("aaaaaaaaaaa=").unwrap(),
            &WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
            backup.backup_path(),
        );

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_world_not_found_when_restore_then_returns_error() {
        // Given
        let backup_dir = TempDir::new().unwrap();
        let backup = make_backup("aaaaaaaaaaa=", "123456789012345678", backup_dir.path());

        let backup_repo = MockBackupRepo::new(vec![backup.clone()]);
        let world_repo = MockWorldRepo::new(make_world(
            "bbbbbbbbbbb=",
            "Other World",
            TempDir::new().unwrap().path(),
            WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
        ));

        // When
        let result = restore_backup(
            &backup_repo,
            &world_repo,
            &WorldFolderName::new("aaaaaaaaaaa=").unwrap(),
            &WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
            backup.backup_path(),
        );

        // Then
        assert!(result.is_err());
    }
}
