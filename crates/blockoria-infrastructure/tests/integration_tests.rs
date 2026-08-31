// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Real integration tests (filesystem I/O) for blockoria-infrastructure.
//!
//! Uses tempfile to simulate the real Minecraft Bedrock structure:
//! %APPDATA%\Minecraft Bedrock\Users\<account_id|Shared>\games\com.mojang\minecraftWorlds\

use blockoria_application::ports::{BackupRepository, WorldRepository};
use blockoria_domain::{
    AccountId, Backup, BackupPath, BackupTimestamp, WorldFolderName, WorldLocation, WorldVersion,
};
use blockoria_infrastructure::repositories::{FileBackupRepository, FileWorldRepository};
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create Minecraft Bedrock folder structure in temp dir
struct MinecraftTestEnv {
    temp_dir: TempDir,
    users_dir: PathBuf,
}

impl MinecraftTestEnv {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let users_dir = temp_dir.path().join("Users");
        std::fs::create_dir_all(&users_dir).unwrap();
        Self {
            temp_dir,
            users_dir,
        }
    }

    /// Creates world folder: Users/<account_or_shared>/games/com.mojang/minecraftWorlds/<folder_name>/
    fn create_world(&self, location: &str, folder_name: &str, level_name: &str) -> PathBuf {
        let world_dir = self
            .users_dir
            .join(location)
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join(folder_name);
        std::fs::create_dir_all(&world_dir).unwrap();
        std::fs::write(world_dir.join("level.dat"), b"test").unwrap();
        std::fs::write(world_dir.join("levelname.txt"), level_name.as_bytes()).unwrap();
        std::fs::write(world_dir.join("world_icon.jpeg"), b"fake_icon").unwrap();
        world_dir
    }

    /// Returns base path for FileWorldRepository
    fn worlds_root(&self) -> PathBuf {
        self.temp_dir.path().join("Users")
    }

    /// Returns path for backup root
    fn backup_root(&self) -> PathBuf {
        self.temp_dir.path().join("backups")
    }
}

/// ===================== FILE WORLD REPOSITORY TESTS =====================

#[test]
fn given_empty_users_dir_when_list_all_then_returns_empty() {
    // Given
    let env = MinecraftTestEnv::new();
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn given_single_account_with_world_when_list_all_then_returns_world() {
    // Given
    let env = MinecraftTestEnv::new();
    env.create_world("123456789012345678", "aaaaaaaaaaa=", "My World");
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    let world = &worlds[0];
    assert_eq!(world.folder_name().as_str(), "aaaaaaaaaaa=");
    assert_eq!(world.level_name().as_str(), "My World");
    assert!(!world.is_shared());
    assert_eq!(world.account_id().unwrap().as_str(), "123456789012345678");
    assert!(world.icon_path().is_some());
}

#[test]
fn given_multiple_accounts_when_list_all_then_returns_all_worlds() {
    // Given
    let env = MinecraftTestEnv::new();
    env.create_world("111111111111111111", "aaaaaaaaaaa=", "Account World 1");
    env.create_world("222222222222222222", "bbbbbbbbbbb=", "Account World 2");
    env.create_world("333333333333333333", "ccccccccccc=", "Account World 3");
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 3);
    let folders: Vec<_> = worlds.iter().map(|w| w.folder_name().as_str()).collect();
    assert!(folders.contains(&"aaaaaaaaaaa="));
    assert!(folders.contains(&"bbbbbbbbbbb="));
    assert!(folders.contains(&"ccccccccccc="));
    assert!(worlds.iter().all(|w| !w.is_shared()));
}

#[test]
fn given_shared_world_when_list_all_then_returns_shared_world() {
    // Given
    let env = MinecraftTestEnv::new();
    env.create_world("Shared", "ddddddddddd=", "Shared World");
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    let world = &worlds[0];
    assert!(world.is_shared());
    assert_eq!(world.location().as_path_segment(), "Shared");
    assert_eq!(world.account_id(), None);
}

#[test]
fn given_mixed_accounts_and_shared_when_list_all_then_returns_all() {
    // Given
    let env = MinecraftTestEnv::new();
    env.create_world("111111111111111111", "aaaaaaaaaaa=", "Account World");
    env.create_world(
        "222222222222222222",
        "bbbbbbbbbbb=",
        "Another Account World",
    );
    env.create_world("Shared", "ccccccccccc=", "Shared World");
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 3);
    let shared_count = worlds.iter().filter(|w| w.is_shared()).count();
    let account_count = worlds.iter().filter(|w| !w.is_shared()).count();
    assert_eq!(shared_count, 1);
    assert_eq!(account_count, 2);
}

#[test]
fn given_files_instead_of_dirs_when_list_all_then_ignores_them() {
    // Given
    let env = MinecraftTestEnv::new();
    // Creates valid structure
    env.create_world("111111111111111111", "aaaaaaaaaaa=", "Valid World");
    // Creates junk file at Users/ root
    std::fs::write(env.users_dir.join("junk.txt"), b"junk").unwrap();
    // Creates empty folder (not a valid account_id)
    std::fs::create_dir_all(env.users_dir.join("not_an_account")).unwrap();
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    assert_eq!(worlds[0].folder_name().as_str(), "aaaaaaaaaaa=");
}

#[test]
fn given_missing_level_dat_when_list_all_then_uses_default_version() {
    // Given
    let env = MinecraftTestEnv::new();
    let world_dir = env.create_world("111111111111111111", "aaaaaaaaaaa=", "No Level.dat");
    // Remove level.dat to test fallback
    std::fs::remove_file(world_dir.join("level.dat")).unwrap();
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    // WorldVersion::default() = [0, 0, 0, 0, 0]
    assert_eq!(worlds[0].version().as_array(), &[0, 0, 0, 0, 0]);
}

#[test]
fn given_missing_levelname_txt_when_list_all_then_uses_unknown() {
    // Given
    let env = MinecraftTestEnv::new();
    let world_dir = env.create_world("111111111111111111", "aaaaaaaaaaa=", "Nome Original");
    std::fs::remove_file(world_dir.join("levelname.txt")).unwrap();
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    // LevelName::new("Unknown World").unwrap()
    assert_eq!(worlds[0].level_name().as_str(), "Unknown World");
}

#[test]
fn given_missing_icon_when_list_all_then_icon_path_is_none() {
    // Given
    let env = MinecraftTestEnv::new();
    let world_dir = env.create_world("111111111111111111", "aaaaaaaaaaa=", "No Icon");
    std::fs::remove_file(world_dir.join("world_icon.jpeg")).unwrap();
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    assert!(worlds[0].icon_path().is_none());
}

#[test]
fn given_existing_world_when_find_by_folder_name_then_returns_world() {
    // Given
    let env = MinecraftTestEnv::new();
    env.create_world("111111111111111111", "aaaaaaaaaaa=", "Found World");
    let repo = FileWorldRepository::with_path(env.worlds_root());
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();

    // When
    let result = repo.find_by_folder_name(&folder);

    // Then
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    let world = found.unwrap();
    assert_eq!(world.folder_name().as_str(), "aaaaaaaaaaa=");
    assert_eq!(world.level_name().as_str(), "Found World");
}

#[test]
fn given_shared_world_when_find_by_folder_name_then_returns_shared() {
    // Given
    let env = MinecraftTestEnv::new();
    env.create_world("Shared", "bbbbbbbbbbb=", "Shared World");
    let repo = FileWorldRepository::with_path(env.worlds_root());
    let folder = WorldFolderName::new("bbbbbbbbbbb=").unwrap();

    // When
    let result = repo.find_by_folder_name(&folder);

    // Then
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    let world = found.unwrap();
    assert!(world.is_shared());
    assert_eq!(world.location().as_path_segment(), "Shared");
}

#[test]
fn given_nonexistent_world_when_find_by_folder_name_then_returns_none() {
    // Given
    let env = MinecraftTestEnv::new();
    let repo = FileWorldRepository::with_path(env.worlds_root());
    let folder = WorldFolderName::new("nonexistent=").unwrap();

    // When
    let result = repo.find_by_folder_name(&folder);

    // Then
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

/// ===================== FILE BACKUP REPOSITORY TESTS =====================

#[test]
fn given_empty_backup_root_when_list_by_world_then_returns_empty() {
    // Given
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());
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
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());

    // Create backup in the expected directory structure
    let backup_root = env.backup_root();
    let backup_dir = backup_root
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-01T00-00-00Z");
    std::fs::create_dir_all(&backup_dir).unwrap();

    let backup = Backup::new(
        folder.clone(),
        AccountId::new("111111111111111111").unwrap(),
        WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
        BackupTimestamp::now(),
        BackupPath::new(&backup_dir).unwrap(),
    );

    // When
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
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());

    // Creates real backups with folder structure (format: YYYY-MM-DDTHH-MM-SSZ)
    let backup_root = env.backup_root();
    let b1_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-01T00-00-00Z");
    let b2_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-02T00-00-00Z");
    let b3_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("bbbbbbbbbbb_")
        .join("2024-01-01T00-00-00Z");
    let b4_dir = backup_root
        .as_path()
        .join("222222222222222222")
        .join("aaaaaaaaaaa_")
        .join("2024-01-01T00-00-00Z");

    std::fs::create_dir_all(&b1_dir).unwrap();
    std::fs::create_dir_all(&b2_dir).unwrap();
    std::fs::create_dir_all(&b3_dir).unwrap();
    std::fs::create_dir_all(&b4_dir).unwrap();

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
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());

    // Creates real backup
    let backup_root = env.backup_root();
    let b1_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-01T00-00-00Z");
    std::fs::create_dir_all(&b1_dir).unwrap();

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
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());

    let backup_root = env.backup_root();
    let backup_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-01T00-00-00Z");
    std::fs::create_dir_all(&backup_dir).unwrap();
    std::fs::write(backup_dir.join("level.dat"), b"backup").unwrap();

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
        "Backup directory must have been removed"
    );
}

#[test]
fn given_nonexistent_backup_when_delete_then_succeeds_idempotent() {
    // Given
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());

    let fake_path = env.temp_dir.path().join("nonexistent");

    // When
    let result = repo.delete(&fake_path);

    // Then
    assert!(result.is_ok(), "Delete must be idempotent");
}

/// ===================== EDGE CASES =====================

#[test]
fn given_invalid_folder_name_format_when_list_all_then_skips_it() {
    // Given
    let env = MinecraftTestEnv::new();
    // Folder with invalid format (not base64 + '=')
    let invalid_dir = env
        .users_dir
        .join("111111111111111111")
        .join("games")
        .join("com.mojang")
        .join("minecraftWorlds")
        .join("invalid_folder_name");
    std::fs::create_dir_all(&invalid_dir).unwrap();
    std::fs::write(invalid_dir.join("level.dat"), b"test").unwrap();
    std::fs::write(invalid_dir.join("levelname.txt"), b"Invalid").unwrap();

    // Valid folder
    env.create_world("111111111111111111", "aaaaaaaaaaa=", "Valid");
    let repo = FileWorldRepository::with_path(env.worlds_root());

    // When
    let result = repo.list_all();

    // Then
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1); // Only the valid one
    assert_eq!(worlds[0].folder_name().as_str(), "aaaaaaaaaaa=");
}

#[test]
fn given_backup_timestamp_ordering_when_list_then_sorted_descending() {
    // Given
    let env = MinecraftTestEnv::new();
    let repo = FileBackupRepository::new(env.backup_root());

    let backup_root = env.backup_root();
    // Creates backups with different timestamps (format: YYYY-MM-DDTHH-MM-SSZ)
    let b1_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-01T00-00-00Z");
    let b2_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-03T00-00-00Z"); // newest
    let b3_dir = backup_root
        .as_path()
        .join("111111111111111111")
        .join("aaaaaaaaaaa_")
        .join("2024-01-02T00-00-00Z");

    std::fs::create_dir_all(&b1_dir).unwrap();
    std::fs::create_dir_all(&b2_dir).unwrap();
    std::fs::create_dir_all(&b3_dir).unwrap();

    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());

    // When
    let result = repo.list_by_world(&folder, &location);

    // Then: should come sorted descending (newest first)
    assert!(result.is_ok());
    let backups = result.unwrap();
    assert_eq!(backups.len(), 3);
    let timestamps: Vec<_> = backups
        .iter()
        .map(|b| b.created_at().to_filename_safe())
        .collect();
    assert!(timestamps[0] > timestamps[1]); // 20240103 > 20240102
    assert!(timestamps[1] > timestamps[2]); // 20240102 > 20240101
}
