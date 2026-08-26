// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use std::path::{Component, Path, PathBuf};
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Value Object for Minecraft world icon path.
///
/// Represents the `world_icon.jpeg` file inside the world folder.
/// Can be `None` if the world has no icon (feature disabled).
///
/// Validation rule (when `Some`):
/// - File must be named exactly `world_icon.jpeg`
/// - Path cannot be empty
/// - Path must be a simple filename (no parent directories, no path traversal)
///
/// # Examples
///
/// ```
/// use blockoria_domain::WorldIconPath;
/// use std::path::PathBuf;
///
/// // With valid icon
/// let icon = WorldIconPath::new(Some(PathBuf::from("world_icon.jpeg"))).unwrap();
/// assert!(icon.is_some());
///
/// // Without icon
/// let no_icon = WorldIconPath::new(None::<PathBuf>).unwrap();
/// assert!(no_icon.is_none());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorldIconPath(Option<PathBuf>);

impl WorldIconPath {
    /// Creates a new WorldIconPath validating the format.
    ///
    /// # Errors
    ///
    /// Returns `DomainError::InvalidWorldIconPath` if:
    /// - `Some(path)` where filename is not `world_icon.jpeg`
    /// - `Some(path)` where path is empty
    /// - `Some(path)` contains path traversal (`..`)
    /// - `Some(path)` contains parent directories (subdirectories not allowed)
    pub fn new(value: Option<impl Into<PathBuf>>) -> Result<Self, DomainError> {
        match value {
            Some(path) => {
                let path = path.into();
                if path.as_os_str().is_empty() {
                    return Err(DomainError::InvalidWorldIconPath(
                        "World icon path cannot be empty".into(),
                    ));
                }

                // Reject path traversal attempts (..)
                if path.components().any(|c| matches!(c, Component::ParentDir)) {
                    return Err(DomainError::InvalidWorldIconPath(
                        "Path traversal not allowed (..)".into(),
                    ));
                }

                // Reject paths with parent directories (subdirectories not allowed)
                // The icon must be directly in the world folder, not in a subdirectory.
                if path.parent().is_some_and(|p| !p.as_os_str().is_empty()) {
                    return Err(DomainError::InvalidWorldIconPath(
                        "Only filename allowed, no subdirectories".into(),
                    ));
                }

                let file_name = path.file_name().and_then(|s| s.to_str()).ok_or_else(|| {
                    DomainError::InvalidWorldIconPath("World icon path has invalid filename".into())
                })?;
                if file_name != "world_icon.jpeg" {
                    return Err(DomainError::InvalidWorldIconPath(
                        "World icon must be named 'world_icon.jpeg'".into(),
                    ));
                }
                Ok(WorldIconPath(Some(path)))
            }
            None => Ok(WorldIconPath(None)),
        }
    }

    /// Returns `true` if has icon.
    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    /// Returns `true` if has no icon.
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the inner path as `Option<&Path>`.
    pub fn as_path(&self) -> Option<&Path> {
        self.0.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn given_none_when_new_then_ok() {
        // Given
        let input = None::<PathBuf>;

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn given_valid_filename_when_new_then_ok() {
        // Given
        let input = Some(PathBuf::from("world_icon.jpeg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn given_parent_dir_traversal_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from("../world_icon.jpeg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }

    #[test]
    fn given_subdirectory_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from("folder/world_icon.jpeg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }

    #[test]
    fn given_wrong_filename_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from("icon.png"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }

    #[test]
    fn given_wrong_extension_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from("world_icon.jpg"));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }

    #[test]
    fn given_empty_path_when_new_then_err() {
        // Given
        let input = Some(PathBuf::from(""));

        // When
        let result = WorldIconPath::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldIconPath(_))));
    }
}
