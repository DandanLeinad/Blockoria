// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! World repository port — read operations for worlds.
//!
//! Defines the `WorldRepository` trait which provides read access to world
// aggregates. The infrastructure layer must implement this trait to provide
// world data from any storage backend (filesystem, database, etc.).

use blockoria_domain::{DomainError, World, WorldFolderName};

/// Port for reading World aggregates.
pub trait WorldRepository: Send + Sync {
    /// Returns all worlds known to the repository.
    fn list_all(&self) -> Result<Vec<World>, DomainError>;

    /// Finds a world by its folder name.
    fn find_by_folder_name(
        &self,
        folder_name: &WorldFolderName,
    ) -> Result<Option<World>, DomainError>;
}
