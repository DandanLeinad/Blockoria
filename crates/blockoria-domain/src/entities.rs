// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::{
    AccountId, BackupPath, BackupTimestamp, LevelName, WorldFolderName, WorldIconPath, WorldPath,
    WorldVersion,
};
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Aggregate Root: Minecraft Bedrock World.
///
/// Represents a complete world with all its identity and location data.
///
/// # Invariants
/// - All fields are required and validated on creation
/// - `icon_path` can be `None` (world without icon)
///
/// # Examples
///
/// ```
/// use blockoria_domain::{World, WorldFolderName, LevelName, WorldPath, AccountId, WorldVersion, WorldIconPath};
/// use std::path::PathBuf;
/// use tempfile::tempdir;
/// use chrono::{TimeZone, Utc};
///
/// let dir = tempdir().unwrap();
/// let world = World::new(
///     WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
///     LevelName::new("My World").unwrap(),
///     WorldPath::new(dir.path()).unwrap(),
///     AccountId::new("123456789012345678").unwrap(),
///     WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
///     WorldIconPath::new(None::<PathBuf>).unwrap(),
/// );
/// assert_eq!(world.level_name().as_str(), "My World");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct World {
    folder_name: WorldFolderName,
    level_name: LevelName,
    path: WorldPath,
    account_id: AccountId,
    version: WorldVersion,
    icon_path: WorldIconPath,
}

impl World {
    /// Creates a new world with all validated data.
    pub fn new(
        folder_name: WorldFolderName,
        level_name: LevelName,
        path: WorldPath,
        account_id: AccountId,
        version: WorldVersion,
        icon_path: WorldIconPath,
    ) -> Self {
        Self {
            folder_name,
            level_name,
            path,
            account_id,
            version,
            icon_path,
        }
    }

    /// Returns the world folder name (base64 format, 11 chars + '=').
    pub fn folder_name(&self) -> &WorldFolderName {
        &self.folder_name
    }

    /// Returns the world display name (levelname).
    pub fn level_name(&self) -> &LevelName {
        &self.level_name
    }

    /// Returns the world path in the filesystem.
    pub fn path(&self) -> &WorldPath {
        &self.path
    }

    /// Returns the associated Microsoft account ID.
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// Returns the world version (lastOpenedWithVersion).
    pub fn version(&self) -> &WorldVersion {
        &self.version
    }

    /// Returns the world icon path (can be None).
    pub fn icon_path(&self) -> &WorldIconPath {
        &self.icon_path
    }
}

/// Aggregate Root: Minecraft Bedrock World Backup.
///
/// Represents a complete backup of a world at a specific point in time.
/// Contains the world version at backup time to ensure correct restore.
///
/// # Invariants
/// - `world_version` ensures restore knows which version to restore
/// - `created_at` is the exact timestamp of backup creation
/// - `backup_path` points to the directory where files were copied
///
/// # Examples
///
/// ```
/// use blockoria_domain::{Backup, WorldFolderName, AccountId, WorldVersion, BackupTimestamp, BackupPath};
/// use std::path::PathBuf;
/// use tempfile::tempdir;
/// use chrono::{TimeZone, Utc};
///
/// let dir = tempdir().unwrap();
/// let backup = Backup::new(
///     WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
///     AccountId::new("123456789012345678").unwrap(),
///     WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
///     BackupTimestamp::now(),
///     BackupPath::new(dir.path()).unwrap(),
/// );
/// assert_eq!(backup.world_folder_name().as_str(), "6LknJ3qXcJo=");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Backup {
    world_folder_name: WorldFolderName,
    world_account_id: AccountId,
    world_version: WorldVersion,
    created_at: BackupTimestamp,
    backup_path: BackupPath,
}

impl Backup {
    /// Creates a new backup with all validated data.
    ///
    /// # Arguments
    /// * `world_folder_name` - World folder name (base64, 11 chars + '=')
    /// * `world_account_id` - Microsoft account ID of the world owner
    /// * `world_version` - World version at backup time (for correct restore)
    /// * `created_at` - Exact timestamp of backup creation
    /// * `backup_path` - Path of the directory where backup was stored
    pub fn new(
        world_folder_name: WorldFolderName,
        world_account_id: AccountId,
        world_version: WorldVersion,
        created_at: BackupTimestamp,
        backup_path: BackupPath,
    ) -> Self {
        Self {
            world_folder_name,
            world_account_id,
            world_version,
            created_at,
            backup_path,
        }
    }

    /// Returns the original world folder name.
    pub fn world_folder_name(&self) -> &WorldFolderName {
        &self.world_folder_name
    }

    /// Returns the world owner's account ID.
    pub fn world_account_id(&self) -> &AccountId {
        &self.world_account_id
    }

    /// Returns the world version at backup time.
    ///
    /// Essential for correct restore - ensures restored version matches
    /// the world version at backup time.
    pub fn world_version(&self) -> &WorldVersion {
        &self.world_version
    }

    /// Returns the exact timestamp of backup creation.
    pub fn created_at(&self) -> &BackupTimestamp {
        &self.created_at
    }

    /// Returns the path of the directory where the backup was stored.
    pub fn backup_path(&self) -> &BackupPath {
        &self.backup_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccountId, LevelName, WorldFolderName, WorldIconPath, WorldPath, WorldVersion};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_test_world() -> World {
        let dir = tempdir().unwrap();
        World::new(
            WorldFolderName::new("6LknJ3qXcJo=").unwrap(),
            LevelName::new("Test World").unwrap(),
            WorldPath::new(dir.path()).unwrap(),
            AccountId::new("123456789012345678").unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            WorldIconPath::new(None::<PathBuf>).unwrap(),
        )
    }

    #[test]
    fn given_valid_data_when_new_world_then_ok() {
        // Given
        let world = make_test_world();

        // Then
        assert_eq!(world.folder_name().as_str(), "6LknJ3qXcJo=");
        assert_eq!(world.level_name().as_str(), "Test World");
        assert_eq!(world.account_id().as_str(), "123456789012345678");
        assert_eq!(world.version().as_array(), &[1, 21, 0, 0, 0]);
        assert!(world.icon_path().is_none());
    }

    #[test]
    fn given_valid_data_when_new_backup_then_ok() {
        // Given
        let world = make_test_world();
        let dir = tempdir().unwrap();

        // When
        let backup = Backup::new(
            world.folder_name().clone(),
            world.account_id().clone(),
            world.version().clone(),
            BackupTimestamp::now(),
            BackupPath::new(dir.path()).unwrap(),
        );

        // Then
        assert_eq!(backup.world_folder_name().as_str(), "6LknJ3qXcJo=");
        assert_eq!(backup.world_account_id().as_str(), "123456789012345678");
        assert_eq!(backup.world_version().as_array(), &[1, 21, 0, 0, 0]);
        assert!(backup.backup_path().as_path().exists());
    }

    #[test]
    fn given_backup_when_created_then_has_timestamp() {
        // Given
        let world = make_test_world();
        let dir = tempdir().unwrap();
        let before = chrono::Utc::now();

        // When
        let backup = Backup::new(
            world.folder_name().clone(),
            world.account_id().clone(),
            world.version().clone(),
            BackupTimestamp::now(),
            BackupPath::new(dir.path()).unwrap(),
        );

        // Then
        let ts = backup.created_at();
        assert!(ts.as_datetime() >= &before);
        assert!(ts.as_datetime() <= &chrono::Utc::now());
    }

    #[test]
    fn given_backup_when_created_then_version_preserved() {
        // Given
        let world = make_test_world();
        let dir = tempdir().unwrap();

        // When
        let backup = Backup::new(
            world.folder_name().clone(),
            world.account_id().clone(),
            world.version().clone(),
            BackupTimestamp::now(),
            BackupPath::new(dir.path()).unwrap(),
        );

        // Then
        assert_eq!(
            backup.world_version().as_array(),
            world.version().as_array()
        );
    }
}
