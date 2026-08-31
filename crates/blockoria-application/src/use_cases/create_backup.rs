// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Create a new backup of a world.
//!
//! This use case creates a new backup of a Minecraft Bedrock world by copying
//! all files from the world directory to a timestamped backup directory under
//! the backup root.
//!
//! # Operation
//! 1. Generates a timestamp for the backup
//! 2. Creates a sanitized folder name (replaces `=` with `_` for Windows)
//! 3. Creates the backup directory structure
//! 4. Recursively copies all files from the world directory
//! 5. Returns the created `Backup` aggregate

use crate::util::copy_dir_all;
use blockoria_domain::{Backup, BackupPath, BackupTimestamp, DomainError, World};
use std::fs;

/// Creates a backup of the given world.
///
/// # Arguments
/// * `world` - The world to backup
/// * `backup_root` - Root directory where backups are stored
///
/// # Returns
/// The created `Backup` aggregate on success
///
/// # Errors
/// Returns `DomainError` if:
/// - Filesystem operations fail (I/O errors)
/// - Backup path validation fails
pub fn create_backup(world: &World, backup_root: &BackupPath) -> Result<Backup, DomainError> {
    let timestamp = BackupTimestamp::now();
    let timestamp_dir_name = timestamp.to_filename_safe();

    // Sanitize folder name for filesystem (replace = with _ for Windows compatibility)
    let safe_folder_name = world.folder_name().as_str().replace('=', "_");

    let backup_dir = backup_root
        .as_path()
        .join(&safe_folder_name)
        .join(&timestamp_dir_name);

    fs::create_dir_all(&backup_dir).map_err(|e| DomainError::InvalidBackupPath(e.to_string()))?;

    copy_dir_all(world.path().as_path(), &backup_dir)
        .map_err(|e| DomainError::InvalidBackupPath(e.to_string()))?;

    Ok(Backup::new(
        world.folder_name().clone(),
        world
            .account_id()
            .expect("world must have account_id for backup")
            .clone(),
        world.version().clone(),
        timestamp,
        BackupPath::new(&backup_dir)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockoria_domain::{
        AccountId, LevelName, WorldFolderName, WorldIconPath, WorldLocation, WorldPath,
        WorldVersion,
    };
    use std::path::PathBuf;
    use tempfile::TempDir;

    struct TestWorld {
        world: World,
        _temp_dir: TempDir,
    }

    impl TestWorld {
        fn new() -> Self {
            let temp_dir = TempDir::new().unwrap();
            fs::write(temp_dir.path().join("level.dat"), b"test").unwrap();
            fs::write(temp_dir.path().join("levelname.txt"), b"Test World").unwrap();

            let world = World::new(
                WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
                LevelName::new("Test World").unwrap(),
                WorldPath::new(temp_dir.path()).unwrap(),
                WorldLocation::Account(AccountId::new("123456789012345678").unwrap()),
                WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
                WorldIconPath::new(None::<PathBuf>).unwrap(),
            );

            TestWorld {
                world,
                _temp_dir: temp_dir,
            }
        }
    }

    #[test]
    fn given_valid_world_and_backup_root_when_create_backup_then_returns_backup_with_correct_structure()
     {
        // Given
        let test_world = TestWorld::new();
        let backup_root = TempDir::new().unwrap();
        let backup_path = BackupPath::new(backup_root.path()).unwrap();

        // When
        let result = create_backup(&test_world.world, &backup_path);

        // Then
        assert!(result.is_ok());
        let backup = result.unwrap();
        assert!(backup.backup_path().as_path().exists());
        assert_eq!(backup.world_folder_name(), test_world.world.folder_name());
        assert_eq!(backup.world_version(), test_world.world.version());
    }

    #[test]
    fn given_valid_world_when_create_backup_then_copies_all_files() {
        // Given
        let test_world = TestWorld::new();
        let backup_root = TempDir::new().unwrap();
        let backup_path = BackupPath::new(backup_root.path()).unwrap();

        // When
        let backup = create_backup(&test_world.world, &backup_path).unwrap();

        // Then
        let backup_dir = backup.backup_path().as_path();
        assert!(backup_dir.join("level.dat").exists());
        assert!(backup_dir.join("levelname.txt").exists());
    }

    #[test]
    fn given_world_with_subdirectories_when_create_backup_then_copies_recursively() {
        // Given
        let test_world = TestWorld::new();
        let world_path = test_world.world.path().as_path();
        fs::create_dir_all(world_path.join("db")).unwrap();
        fs::write(world_path.join("db").join("file.txt"), b"data").unwrap();
        fs::create_dir_all(world_path.join("region")).unwrap();
        fs::write(world_path.join("region").join("r.0.0.mca"), b"chunk").unwrap();

        let backup_root = TempDir::new().unwrap();
        let backup_path = BackupPath::new(backup_root.path()).unwrap();

        // When
        let backup = create_backup(&test_world.world, &backup_path).unwrap();

        // Then
        let backup_dir = backup.backup_path().as_path();
        assert!(backup_dir.join("db").join("file.txt").exists());
        assert!(backup_dir.join("region").join("r.0.0.mca").exists());
    }

    #[test]
    fn given_backup_created_then_timestamp_is_recent() {
        // Given
        let test_world = TestWorld::new();
        let backup_root = TempDir::new().unwrap();
        let backup_path = BackupPath::new(backup_root.path()).unwrap();
        let before = chrono::Utc::now();

        // When
        let backup = create_backup(&test_world.world, &backup_path).unwrap();

        // Then
        let ts = backup.created_at();
        assert!(ts.as_datetime() >= &before);
        assert!(ts.as_datetime() <= &chrono::Utc::now());
    }

    #[test]
    fn given_backup_created_then_backup_path_has_correct_naming() {
        // Given
        let test_world = TestWorld::new();
        let backup_root = TempDir::new().unwrap();
        let backup_path = BackupPath::new(backup_root.path()).unwrap();

        // When
        let backup = create_backup(&test_world.world, &backup_path).unwrap();

        // Then
        let backup_dir = backup.backup_path().as_path();
        let expected_parent = backup_root
            .path()
            .join(test_world.world.folder_name().as_str().replace('=', "_"));
        // canonicalize both paths for comparison (handles Windows \\?\ prefix)
        assert_eq!(
            backup_dir.parent().unwrap().canonicalize().unwrap(),
            expected_parent.canonicalize().unwrap()
        );
        let dir_name = backup_dir.file_name().unwrap().to_str().unwrap();
        assert!(!dir_name.contains(':'));
        assert!(dir_name.contains('T') || dir_name.contains('-'));
    }
}
