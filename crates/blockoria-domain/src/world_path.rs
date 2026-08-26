// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use std::path::{Path, PathBuf};
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Value Object for Minecraft Bedrock world path.
///
/// Validates that the path:
/// - Is not empty
/// - Exists in the filesystem
/// - Is a directory (not a file)
///
/// # Examples
///
/// ```
/// use blockoria_domain::WorldPath;
/// use tempfile::tempdir;
/// let dir = tempdir().unwrap();
/// let path = WorldPath::new(dir.path()).unwrap();
/// assert!(path.as_path().exists());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorldPath(PathBuf);

impl WorldPath {
    /// Creates a new WorldPath validating the filesystem.
    ///
    /// # Errors
    ///
    /// Returns `DomainError::InvalidWorldPath` if:
    /// - Path is empty
    /// - Path is "." (current directory)
    /// - Path does not exist (after resolving symlinks/junctions)
    /// - Path is not a directory (after resolving symlinks/junctions)
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, DomainError> {
        let path = path.into();
        // Explicitly reject "." (like Python)
        if path == Path::new(".") {
            return Err(DomainError::InvalidWorldPath(
                "World path cannot be current directory".into(),
            ));
        }
        // Resolve symlinks/junctions to prevent path traversal attacks.
        // This must happen before validation to ensure we validate the real path.
        let path = path
            .canonicalize()
            .map_err(|e| DomainError::InvalidWorldPath(format!("Failed to resolve path: {e}")))?;
        if !path.exists() {
            return Err(DomainError::InvalidWorldPath(
                "World path does not exist".into(),
            ));
        }
        if !path.is_dir() {
            return Err(DomainError::InvalidWorldPath(
                "World path must be a directory".into(),
            ));
        }
        Ok(WorldPath(path))
    }

    /// Returns the path as a `Path` reference.
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tempfile::tempdir; // para criar dirs temporários nos testes

    #[test]
    fn given_existing_dir_when_new_then_ok() {
        // Given
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path();

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_empty_when_new_then_err() {
        // Given
        let path = PathBuf::from("");

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_not_exists_when_new_then_err() {
        // Given
        let path = PathBuf::from("/path/that/does/not/exist");

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_file_not_dir_when_new_then_err() {
        // Given
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_dot_when_new_then_err() {
        // Given
        let path = PathBuf::from(".");

        // When
        let result = super::WorldPath::new(path);

        // Then
        assert!(result.is_err());
    }
}
