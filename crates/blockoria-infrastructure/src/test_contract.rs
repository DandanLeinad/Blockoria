// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Test utilities and contract tests shared between integration tests.

use blockoria_application::ports::{BackupRepository, WorldRepository};
use blockoria_domain::{
    AccountId, Backup, BackupPath, BackupTimestamp, LevelName, World, WorldFolderName,
    WorldIconPath, WorldLocation, WorldPath, WorldVersion,
};
use std::path::PathBuf;
use tempfile::TempDir;

/// Shared context for contract tests.
pub struct ContractTestContext {
    pub temp_dir: TempDir,
    pub backup_root: BackupPath,
    pub users_dir: PathBuf,
}

impl Default for ContractTestContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractTestContext {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let users_dir = temp_dir.path().join("Users");
        let backup_dir = temp_dir.path().join("backups");
        std::fs::create_dir_all(&users_dir).unwrap();
        std::fs::create_dir_all(&backup_dir).unwrap();
        let backup_root = BackupPath::new(&backup_dir).unwrap();
        Self {
            temp_dir,
            backup_root,
            users_dir,
        }
    }

    /// Returns the worlds root path for FileWorldRepository
    pub fn worlds_root(&self) -> PathBuf {
        self.temp_dir.path().join("Users")
    }

    /// Creates world folder: Users/<account_or_shared>/games/com.mojang/minecraftWorlds/<folder_name>/
    pub fn make_world(&self, folder: &str, name: &str, location: WorldLocation) -> World {
        let location_str = match &location {
            WorldLocation::Account(account_id) => account_id.as_str(),
            WorldLocation::Shared => "Shared",
        };
        let world_dir = self
            .users_dir
            .join(location_str)
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join(folder);
        std::fs::create_dir_all(&world_dir).unwrap();
        std::fs::write(world_dir.join("level.dat"), b"test").unwrap();
        std::fs::write(world_dir.join("levelname.txt"), name.as_bytes()).unwrap();

        World::new(
            WorldFolderName::new(folder).unwrap(),
            LevelName::new(name).unwrap(),
            WorldPath::new(&world_dir).unwrap(),
            location,
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            WorldIconPath::new(None::<PathBuf>).unwrap(),
        )
    }

    pub fn make_backup(&self, folder: &str, account: &str) -> Backup {
        let backup_dir = self
            .temp_dir
            .path()
            .join("backups")
            .join(account)
            .join(format!("{}_", folder))
            .join("2024-01-01T00-00-00Z");
        std::fs::create_dir_all(&backup_dir).unwrap();

        Backup::new(
            WorldFolderName::new(folder).unwrap(),
            AccountId::new(account).unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            BackupTimestamp::now(),
            BackupPath::new(&backup_dir).unwrap(),
        )
    }
}

/// ===================== WORLD REPOSITORY CONTRACT =====================
/// Runs the WorldRepository contract test suite.
pub fn run_world_repository_contract<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    test_list_all_empty(repo, ctx);
    test_list_all_multiple_accounts(repo, ctx);
    test_list_all_shared_worlds(repo, ctx);
    test_list_all_ignores_non_directories(repo, ctx);
    test_find_by_folder_name_found_account(repo, ctx);
    test_find_by_folder_name_found_shared(repo, ctx);
    test_find_by_folder_name_not_found(repo, ctx);
    test_find_by_folder_name_wrong_account(repo, ctx);
}

fn test_list_all_empty<R: WorldRepository>(repo: &R, _ctx: &ContractTestContext) {
    let result = repo.list_all();
    assert!(result.is_ok(), "list_all should return Ok");
    assert!(
        result.unwrap().is_empty(),
        "empty repo should return empty list"
    );
}

fn test_list_all_multiple_accounts<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    let _w1 = ctx.make_world(
        "aaaaaaaaaaa=",
        "World Account 1",
        WorldLocation::Account(AccountId::new("111111111111111111").unwrap()),
    );
    let _w2 = ctx.make_world(
        "bbbbbbbbbbb=",
        "World Account 2",
        WorldLocation::Account(AccountId::new("222222222222222222").unwrap()),
    );
    let result = repo.list_all();
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 2);
    assert!(
        worlds
            .iter()
            .any(|w| w.folder_name().as_str() == "aaaaaaaaaaa=")
    );
    assert!(
        worlds
            .iter()
            .any(|w| w.folder_name().as_str() == "bbbbbbbbbbb=")
    );
    assert!(
        worlds
            .iter()
            .all(|w| matches!(w.location(), WorldLocation::Account(_)))
    );
}

fn test_list_all_shared_worlds<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    let _w1 = ctx.make_world(
        "aaaaaaaaaaa=",
        "World Account",
        WorldLocation::Account(AccountId::new("111111111111111111").unwrap()),
    );
    let _w2 = ctx.make_world("ccccccccccc=", "Shared World", WorldLocation::Shared);
    let result = repo.list_all();
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 2);
    let shared = worlds.iter().find(|w| w.is_shared()).unwrap();
    assert_eq!(shared.folder_name().as_str(), "ccccccccccc=");
    assert_eq!(shared.location().as_path_segment(), "Shared");
}

fn test_list_all_ignores_non_directories<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    let _w1 = ctx.make_world(
        "aaaaaaaaaaa=",
        "Valid World",
        WorldLocation::Account(AccountId::new("111111111111111111").unwrap()),
    );
    std::fs::write(ctx.temp_dir.path().join("junk.txt"), b"junk").unwrap();
    let result = repo.list_all();
    assert!(result.is_ok());
    let worlds = result.unwrap();
    assert_eq!(worlds.len(), 1);
    assert_eq!(worlds[0].folder_name().as_str(), "aaaaaaaaaaa=");
}

fn test_find_by_folder_name_found_account<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    let _w = ctx.make_world(
        "aaaaaaaaaaa=",
        "Test World",
        WorldLocation::Account(AccountId::new("111111111111111111").unwrap()),
    );
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let result = repo.find_by_folder_name(&folder);
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    let world = found.unwrap();
    assert_eq!(world.folder_name().as_str(), "aaaaaaaaaaa=");
    assert_eq!(world.level_name().as_str(), "Test World");
    assert!(matches!(world.location(), WorldLocation::Account(_)));
}

fn test_find_by_folder_name_found_shared<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    let _w = ctx.make_world("bbbbbbbbbbb=", "Shared World", WorldLocation::Shared);
    let folder = WorldFolderName::new("bbbbbbbbbbb=").unwrap();
    let result = repo.find_by_folder_name(&folder);
    assert!(result.is_ok());
    let found = result.unwrap();
    assert!(found.is_some());
    let world = found.unwrap();
    assert!(world.is_shared());
    assert_eq!(world.location().as_path_segment(), "Shared");
}

fn test_find_by_folder_name_not_found<R: WorldRepository>(repo: &R, _ctx: &ContractTestContext) {
    let folder = WorldFolderName::new("nonexistent=").unwrap();
    let result = repo.find_by_folder_name(&folder);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

fn test_find_by_folder_name_wrong_account<R: WorldRepository>(repo: &R, ctx: &ContractTestContext) {
    let _w = ctx.make_world(
        "aaaaaaaaaaa=",
        "World Account 1",
        WorldLocation::Account(AccountId::new("111111111111111111").unwrap()),
    );
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let result = repo.find_by_folder_name(&folder);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

/// ===================== BACKUP REPOSITORY CONTRACT =====================
/// Runs the BackupRepository contract test suite.
pub fn run_backup_repository_contract<R: BackupRepository>(repo: &R, ctx: &ContractTestContext) {
    test_save_and_list(repo, ctx);
    test_list_by_world_filters_correctly(repo, ctx);
    test_list_by_world_shared_returns_empty(repo, ctx);
    test_delete_backup(repo, ctx);
    test_delete_nonexistent_backup(repo, ctx);
}

fn test_save_and_list<R: BackupRepository>(repo: &R, ctx: &ContractTestContext) {
    let backup = ctx.make_backup("aaaaaaaaaaa=", "111111111111111111");
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());
    let save_result = repo.save(&backup);
    let list_result = repo.list_by_world(&folder, &location);
    assert!(save_result.is_ok());
    assert!(list_result.is_ok());
    let backups = list_result.unwrap();
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0].world_folder_name().as_str(), "aaaaaaaaaaa=");
    assert_eq!(backups[0].world_account_id().as_str(), "111111111111111111");
}

fn test_list_by_world_filters_correctly<R: BackupRepository>(repo: &R, ctx: &ContractTestContext) {
    let b1 = ctx.make_backup("aaaaaaaaaaa=", "111111111111111111");
    let b2 = ctx.make_backup("aaaaaaaaaaa=", "111111111111111111");
    let b3 = ctx.make_backup("bbbbbbbbbbb=", "111111111111111111");
    let b4 = ctx.make_backup("aaaaaaaaaaa=", "222222222222222222");
    repo.save(&b1).unwrap();
    repo.save(&b2).unwrap();
    repo.save(&b3).unwrap();
    repo.save(&b4).unwrap();
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());
    let result = repo.list_by_world(&folder, &location);
    assert!(result.is_ok());
    let backups = result.unwrap();
    assert_eq!(backups.len(), 2);
    assert!(
        backups
            .iter()
            .all(|b| b.world_folder_name().as_str() == "aaaaaaaaaaa=")
    );
    assert!(
        backups
            .iter()
            .all(|b| b.world_account_id().as_str() == "111111111111111111")
    );
}

fn test_list_by_world_shared_returns_empty<R: BackupRepository>(
    repo: &R,
    ctx: &ContractTestContext,
) {
    let backup = ctx.make_backup("aaaaaaaaaaa=", "111111111111111111");
    repo.save(&backup).unwrap();
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let location = WorldLocation::Shared;
    let result = repo.list_by_world(&folder, &location);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

fn test_delete_backup<R: BackupRepository>(repo: &R, ctx: &ContractTestContext) {
    let backup = ctx.make_backup("aaaaaaaaaaa=", "111111111111111111");
    repo.save(&backup).unwrap();
    let backup_path = backup.backup_path().clone();
    let result = repo.delete(backup_path.as_path());
    assert!(result.is_ok());
    let folder = WorldFolderName::new("aaaaaaaaaaa=").unwrap();
    let location = WorldLocation::Account(AccountId::new("111111111111111111").unwrap());
    let remaining = repo.list_by_world(&folder, &location).unwrap();
    assert!(remaining.is_empty());
}

fn test_delete_nonexistent_backup<R: BackupRepository>(repo: &R, _ctx: &ContractTestContext) {
    let temp = TempDir::new().unwrap();
    let fake_path = temp.path().join("nonexistent");
    let result = repo.delete(&fake_path);
    assert!(result.is_ok());
}
