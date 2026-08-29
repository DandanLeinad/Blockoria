// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! List all known worlds.
//!
//! This use case retrieves all worlds known to the repository.
//! It simply delegates to the `WorldRepository::list_all` method.

use crate::ports::WorldRepository;
use blockoria_domain::{DomainError, World};

/// Returns all worlds known to the repository.
pub fn list_worlds(repo: &dyn WorldRepository) -> Result<Vec<World>, DomainError> {
    repo.list_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockoria_domain::{
        AccountId, LevelName, World, WorldFolderName, WorldIconPath, WorldPath, WorldVersion,
    };
    use std::path::PathBuf;
    use tempfile::TempDir;

    struct MockWorldRepo {
        worlds: Vec<World>,
    }

    impl MockWorldRepo {
        fn new(worlds: Vec<World>) -> Self {
            Self { worlds }
        }
    }

    impl WorldRepository for MockWorldRepo {
        fn list_all(&self) -> Result<Vec<World>, DomainError> {
            Ok(self.worlds.clone())
        }

        fn find_by_folder_name(
            &self,
            folder_name: &WorldFolderName,
        ) -> Result<Option<World>, DomainError> {
            Ok(self
                .worlds
                .iter()
                .find(|w| w.folder_name() == folder_name)
                .cloned())
        }
    }

    fn make_world(folder: &str, name: &str) -> World {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("level.dat"), b"test").unwrap();
        World::new(
            WorldFolderName::new(folder).unwrap(),
            LevelName::new(name).unwrap(),
            WorldPath::new(temp.path()).unwrap(),
            AccountId::new("123456789012345678").unwrap(),
            WorldVersion::new([1, 21, 0, 0, 0]).unwrap(),
            WorldIconPath::new(None::<PathBuf>).unwrap(),
        )
    }

    #[test]
    fn given_empty_repo_when_list_worlds_then_returns_empty_vec() {
        // Given
        let repo = MockWorldRepo::new(vec![]);

        // When
        let result = list_worlds(&repo);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn given_repo_with_worlds_when_list_worlds_then_returns_all() {
        // Given
        let w1 = make_world("aaaaaaaaaaa=", "Mundo 1");
        let w2 = make_world("bbbbbbbbbbb=", "Mundo 2");
        let repo = MockWorldRepo::new(vec![w1, w2]);

        // When
        let result = list_worlds(&repo);

        // Then
        assert!(result.is_ok());
        let worlds = result.unwrap();
        assert_eq!(worlds.len(), 2);
    }
}
