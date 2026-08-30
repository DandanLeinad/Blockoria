// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Filesystem implementation of `WorldRepository` for Minecraft Bedrock worlds.
//!
//! Scans the Minecraft Bedrock worlds directory structure:
//! `%APPDATA%\Minecraft Bedrock\Users\<account_id|Shared>\games\com.mojang\minecraftWorlds\`

use blockoria_application::ports::WorldRepository;
use blockoria_domain::{
    AccountId, DomainError, LevelName, World, WorldFolderName, WorldIconPath, WorldLocation,
    WorldPath, WorldVersion,
};
use std::fs;
use std::path::{Path, PathBuf};

/// Filesystem-based world repository for Minecraft Bedrock Edition.
///
/// Reads worlds from the standard Windows Bedrock location:
/// `%APPDATA%\Minecraft Bedrock\Users\<account_id|Shared>\games\com.mojang\minecraftWorlds\`
pub struct FileWorldRepository {
    /// Root path to the `Users` directory containing account folders + Shared
    bedrock_users_path: PathBuf,
}

impl FileWorldRepository {
    /// Creates a new repository using the default Minecraft Bedrock path.
    ///
    /// Default path: `%APPDATA%\Minecraft Bedrock\Users\`
    pub fn new() -> Result<Self, DomainError> {
        let path = dirs::data_dir()
            .ok_or_else(|| {
                DomainError::InvalidWorldPath("Could not find APPDATA directory".into())
            })?
            .join("Minecraft Bedrock")
            .join("Users");

        if !path.exists() {
            return Err(DomainError::InvalidWorldPath(
                "Minecraft Bedrock Users directory not found".into(),
            ));
        }

        Ok(Self {
            bedrock_users_path: path,
        })
    }

    /// Creates a repository with a custom path (primarily for testing).
    pub fn with_path(path: impl Into<PathBuf>) -> Self {
        Self {
            bedrock_users_path: path.into(),
        }
    }

    /// Returns the path being used for world discovery.
    pub fn path(&self) -> &Path {
        &self.bedrock_users_path
    }
}

impl WorldRepository for FileWorldRepository {
    fn list_all(&self) -> Result<Vec<World>, DomainError> {
        let mut worlds = Vec::new();

        // Iterate over entries in Users/ (account_ids + "Shared")
        let entries = fs::read_dir(&self.bedrock_users_path)?;
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue; // Ignore files at Users/ level
            }

            let location_name = entry.file_name().to_string_lossy().to_string();
            let location = if location_name == "Shared" {
                WorldLocation::Shared
            } else {
                WorldLocation::Account(AccountId::new(&location_name)?)
            };

            // Navigate to: <account>/games/com.mojang/minecraftWorlds/
            let worlds_dir = entry
                .path()
                .join("games")
                .join("com.mojang")
                .join("minecraftWorlds");

            if !worlds_dir.exists() {
                continue; // Skip if structure doesn't exist
            }

            // Scan world folders inside minecraftWorlds/
            let world_entries = fs::read_dir(&worlds_dir)?;
            for world_entry in world_entries {
                let world_entry = world_entry?;
                if !world_entry.file_type()?.is_dir() {
                    continue; // Only directories are worlds
                }

                let folder_name_str = world_entry.file_name().to_string_lossy().to_string();
                let folder_name = match WorldFolderName::new(&folder_name_str) {
                    Ok(n) => n,
                    Err(_) => continue, // Skip invalid folder names
                };

                let world = self.read_world(world_entry.path(), folder_name, location.clone())?;
                worlds.push(world);
            }
        }

        Ok(worlds)
    }

    fn find_by_folder_name(
        &self,
        folder_name: &WorldFolderName,
    ) -> Result<Option<World>, DomainError> {
        // Search across all account directories + Shared
        let entries = fs::read_dir(&self.bedrock_users_path)?;
        for entry in entries {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }

            let location = if entry.file_name() == "Shared" {
                WorldLocation::Shared
            } else {
                let name = entry.file_name().to_string_lossy().to_string();
                WorldLocation::Account(AccountId::new(&name)?)
            };

            let world_path = entry
                .path()
                .join("games")
                .join("com.mojang")
                .join("minecraftWorlds")
                .join(folder_name.as_str());

            if world_path.exists() {
                return Ok(Some(self.read_world(
                    world_path,
                    folder_name.clone(),
                    location,
                )?));
            }
        }

        Ok(None)
    }
}

impl FileWorldRepository {
    /// Reads a single world from its directory.
    fn read_world(
        &self,
        world_dir: PathBuf,
        folder_name: WorldFolderName,
        location: WorldLocation,
    ) -> Result<World, DomainError> {
        // Parse level.dat for version (simplified - real impl would parse NBT)
        let level_dat = world_dir.join("level.dat");
        let version = Self::parse_level_dat_version(&level_dat).unwrap_or_default();

        // Read levelname.txt
        let level_name = world_dir.join("levelname.txt");
        let level_name = fs::read_to_string(&level_name)
            .ok()
            .and_then(|s| LevelName::new(s.trim()).ok())
            .unwrap_or_else(|| LevelName::new("Unknown World").unwrap());

        // Check for world_icon.jpeg
        let icon_file = world_dir.join("world_icon.jpeg");
        let icon = if icon_file.exists() {
            // WorldIconPath expects just the filename, not full path
            WorldIconPath::new(Some("world_icon.jpeg"))?
        } else {
            WorldIconPath::new(None::<PathBuf>)?
        };

        let path = WorldPath::new(world_dir)?;

        Ok(World::new(
            folder_name,
            level_name,
            path,
            location,
            version,
            icon,
        ))
    }

    /// Parses world version from level.dat (NBT format).
    /// Returns None if parsing fails - caller should use default.
    fn parse_level_dat_version(path: &Path) -> Option<WorldVersion> {
        // TODO: Implement proper NBT parsing
        // For now, return None to use default version
        let _ = path; // suppress unused warning
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, PathBuf) {
        let temp = TempDir::new().unwrap();
        let users_dir = temp.path().join("Users");
        fs::create_dir_all(&users_dir).unwrap();
        (temp, users_dir)
    }

    #[test]
    fn given_empty_users_dir_when_list_all_then_returns_empty() {
        // Given
        let (_temp, users_dir) = setup_test_env();
        let repo = FileWorldRepository::with_path(users_dir);

        // When
        let result = repo.list_all();

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn given_single_account_with_world_when_list_all_then_returns_world() {
        // Given
        let (_temp, users_dir) = setup_test_env();
        let account_dir = users_dir.join("123456789012345678");
        let world_dir = account_dir
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("level.dat"), b"test").unwrap();
        fs::write(world_dir.join("levelname.txt"), b"My World").unwrap();
        fs::write(world_dir.join("world_icon.jpeg"), b"icon").unwrap();

        let repo = FileWorldRepository::with_path(users_dir);

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
        let (_temp, users_dir) = setup_test_env();
        for (acc, folder, name) in [
            ("111111111111111111", "aaaaaaaaaaa=", "World 1"),
            ("222222222222222222", "bbbbbbbbbbb=", "World 2"),
            ("333333333333333333", "ccccccccccc=", "World 3"),
        ] {
            let world_dir = users_dir
                .join(acc)
                .join("games")
                .join("com.mojang")
                .join("minecraftWorlds")
                .join(folder);
            fs::create_dir_all(&world_dir).unwrap();
            fs::write(world_dir.join("level.dat"), b"test").unwrap();
            fs::write(world_dir.join("levelname.txt"), name.as_bytes()).unwrap();
        }
        let repo = FileWorldRepository::with_path(users_dir);

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
        let (_temp, users_dir) = setup_test_env();
        let world_dir = users_dir
            .join("Shared")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("ddddddddddd=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("level.dat"), b"test").unwrap();
        fs::write(world_dir.join("levelname.txt"), b"Shared World").unwrap();
        let repo = FileWorldRepository::with_path(users_dir);

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
        let (_temp, users_dir) = setup_test_env();
        // Account world
        let w1 = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&w1).unwrap();
        fs::write(w1.join("level.dat"), b"test").unwrap();
        fs::write(w1.join("levelname.txt"), b"Account").unwrap();
        // Another account
        let w2 = users_dir
            .join("222222222222222222")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("bbbbbbbbbbb=");
        fs::create_dir_all(&w2).unwrap();
        fs::write(w2.join("level.dat"), b"test").unwrap();
        fs::write(w2.join("levelname.txt"), b"Another Account").unwrap();
        // Shared
        let w3 = users_dir
            .join("Shared")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("ccccccccccc=");
        fs::create_dir_all(&w3).unwrap();
        fs::write(w3.join("level.dat"), b"test").unwrap();
        fs::write(w3.join("levelname.txt"), b"Shared").unwrap();

        let repo = FileWorldRepository::with_path(users_dir);

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
        let (_temp, users_dir) = setup_test_env();
        // Valid world
        let w1 = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&w1).unwrap();
        fs::write(w1.join("level.dat"), b"test").unwrap();
        fs::write(w1.join("levelname.txt"), b"Valid").unwrap();
        // Junk file at Users/ level
        fs::write(users_dir.join("junk.txt"), b"junk").unwrap();
        // Empty dir (not an account)
        fs::create_dir_all(users_dir.join("not_an_account")).unwrap();

        let repo = FileWorldRepository::with_path(users_dir);

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
        let (_temp, users_dir) = setup_test_env();
        let world_dir = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("levelname.txt"), b"No Level.dat").unwrap();
        let repo = FileWorldRepository::with_path(users_dir);

        // When
        let result = repo.list_all();

        // Then
        assert!(result.is_ok());
        let worlds = result.unwrap();
        assert_eq!(worlds.len(), 1);
        assert_eq!(worlds[0].version().as_array(), &[0, 0, 0, 0, 0]);
    }

    #[test]
    fn given_missing_levelname_txt_when_list_all_then_uses_unknown() {
        // Given
        let (_temp, users_dir) = setup_test_env();
        let world_dir = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("level.dat"), b"test").unwrap();
        let repo = FileWorldRepository::with_path(users_dir);

        // When
        let result = repo.list_all();

        // Then
        assert!(result.is_ok());
        let worlds = result.unwrap();
        assert_eq!(worlds.len(), 1);
        assert_eq!(worlds[0].level_name().as_str(), "Unknown World");
    }

    #[test]
    fn given_missing_icon_when_list_all_then_icon_path_is_none() {
        // Given
        let (_temp, users_dir) = setup_test_env();
        let world_dir = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("level.dat"), b"test").unwrap();
        fs::write(world_dir.join("levelname.txt"), b"No Icon").unwrap();
        let repo = FileWorldRepository::with_path(users_dir);

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
        let (_temp, users_dir) = setup_test_env();
        let world_dir = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("level.dat"), b"test").unwrap();
        fs::write(world_dir.join("levelname.txt"), b"Found World").unwrap();
        let repo = FileWorldRepository::with_path(users_dir);
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
        let (_temp, users_dir) = setup_test_env();
        let world_dir = users_dir
            .join("Shared")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("bbbbbbbbbbb=");
        fs::create_dir_all(&world_dir).unwrap();
        fs::write(world_dir.join("level.dat"), b"test").unwrap();
        fs::write(world_dir.join("levelname.txt"), b"Shared World").unwrap();
        let repo = FileWorldRepository::with_path(users_dir);
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
        let (_temp, users_dir) = setup_test_env();
        let repo = FileWorldRepository::with_path(users_dir);
        let folder = WorldFolderName::new("nonexistent=").unwrap();

        // When
        let result = repo.find_by_folder_name(&folder);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn given_invalid_folder_name_format_when_list_all_then_skips_it() {
        // Given
        let (_temp, users_dir) = setup_test_env();
        // Invalid folder name
        let invalid_dir = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("invalid_folder_name");
        fs::create_dir_all(&invalid_dir).unwrap();
        fs::write(invalid_dir.join("level.dat"), b"test").unwrap();
        fs::write(invalid_dir.join("levelname.txt"), b"Invalid").unwrap();
        // Valid world
        let valid_dir = users_dir
            .join("111111111111111111")
            .join("games")
            .join("com.mojang")
            .join("minecraftWorlds")
            .join("aaaaaaaaaaa=");
        fs::create_dir_all(&valid_dir).unwrap();
        fs::write(valid_dir.join("level.dat"), b"test").unwrap();
        fs::write(valid_dir.join("levelname.txt"), b"Valid").unwrap();

        let repo = FileWorldRepository::with_path(users_dir);

        // When
        let result = repo.list_all();

        // Then
        assert!(result.is_ok());
        let worlds = result.unwrap();
        assert_eq!(worlds.len(), 1);
        assert_eq!(worlds[0].folder_name().as_str(), "aaaaaaaaaaa=");
    }
}
