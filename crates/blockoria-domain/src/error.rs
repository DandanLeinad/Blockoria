// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use std::fmt;

/// Blockoria domain validation errors.
///
/// Each variant carries a descriptive message about the validation problem.
/// Use `DomainError::Variant("detail")` to create instances.
///
/// # Examples
///
/// ```
/// use blockoria_domain::DomainError;
/// let err = DomainError::InvalidWorldFolderName("empty".into());
/// assert_eq!(err.to_string(), "InvalidWorldFolderName: empty");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// Invalid world folder name.
    ///
    /// Rules: exactly 12 characters, ends with '=', not just whitespace.
    /// Ex: `InvalidWorldFolderName("empty")`, `InvalidWorldFolderName("11 chars")`
    InvalidWorldFolderName(String),

    /// Invalid world path.
    ///
    /// Rules:
    /// - Not empty
    /// - Not "." (current directory)
    /// - Must exist in filesystem (after resolving symlinks/junctions)
    /// - Must be a directory (after resolving symlinks/junctions)
    InvalidWorldPath(String),

    /// Invalid world icon path.
    ///
    /// Rules:
    /// - Not empty
    /// - File name must be exactly "world_icon.jpeg"
    /// - No path traversal (".." not allowed)
    /// - No parent directories (icon must be directly in world folder)
    InvalidWorldIconPath(String),

    /// Invalid world name (levelname).
    ///
    /// Rule: cannot be empty or just whitespace.
    InvalidLevelName(String),

    /// Invalid Microsoft account ID.
    ///
    /// Rule: cannot be empty or just whitespace.
    InvalidAccountId(String),

    /// Invalid world version.
    ///
    /// Rules:
    /// - Exactly 5 elements
    /// - All non-negative integers (u16)
    /// - Each component must not exceed 32767 (i16::MAX)
    InvalidWorldVersion(String),

    /// Invalid backup timestamp.
    ///
    /// Rule: cannot be before 1970-01-01 (Unix epoch).
    InvalidBackupTimestamp(String),

    /// Invalid backup path.
    ///
    /// Rules:
    /// - Not empty
    /// - Not "." (current directory)
    /// - Must exist in filesystem (after resolving symlinks/junctions)
    /// - Must be a directory (after resolving symlinks/junctions)
    InvalidBackupPath(String),
}

/// Display implementation for readable messages in logs/UX.
impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::InvalidWorldFolderName(msg) => {
                write!(f, "InvalidWorldFolderName: {}", msg)
            }
            DomainError::InvalidWorldPath(msg) => write!(f, "InvalidWorldPath: {}", msg),
            DomainError::InvalidWorldIconPath(msg) => {
                write!(f, "InvalidWorldIconPath: {}", msg)
            }
            DomainError::InvalidAccountId(msg) => write!(f, "InvalidAccountId: {}", msg),
            DomainError::InvalidWorldVersion(msg) => write!(f, "InvalidWorldVersion: {}", msg),
            DomainError::InvalidLevelName(msg) => write!(f, "InvalidLevelName: {}", msg),
            DomainError::InvalidBackupTimestamp(msg) => {
                write!(f, "InvalidBackupTimestamp: {}", msg)
            }
            DomainError::InvalidBackupPath(msg) => {
                write!(f, "InvalidBackupPath: {}", msg)
            }
        }
    }
}

impl std::error::Error for DomainError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats_variant_message() {
        // Given
        let err = DomainError::InvalidWorldFolderName("empty".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidWorldFolderName: empty");
    }

    #[test]
    fn error_trait_implemented() {
        // Given
        let err = DomainError::InvalidWorldPath("not found".into());

        // When
        let _: &dyn std::error::Error = &err;

        // Then (compiles if Error trait implemented)
    }

    #[test]
    fn partial_eq_works_for_assertions() {
        // Given
        let a = DomainError::InvalidLevelName("x".into());
        let b = DomainError::InvalidLevelName("x".into());

        // When
        let equal = a == b;

        // Then
        assert!(equal);
    }

    #[test]
    fn clone_works() {
        // Given
        let err = DomainError::InvalidWorldVersion("bad".into());

        // When
        let cloned = err.clone();

        // Then
        assert_eq!(err, cloned);
    }

    #[test]
    fn world_icon_path_error_works() {
        // Given
        let err = DomainError::InvalidWorldIconPath("not a file".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidWorldIconPath: not a file");
    }

    #[test]
    fn account_id_error_works() {
        // Given
        let err = DomainError::InvalidAccountId("empty".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidAccountId: empty");
    }

    #[test]
    fn world_version_error_works() {
        // Given
        let err = DomainError::InvalidWorldVersion("not 5 ints".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidWorldVersion: not 5 ints");
    }

    #[test]
    fn backup_timestamp_error_works() {
        // Given
        let err = DomainError::InvalidBackupTimestamp("before epoch".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidBackupTimestamp: before epoch");
    }

    #[test]
    fn backup_path_error_works() {
        // Given
        let err = DomainError::InvalidBackupPath("not a directory".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "InvalidBackupPath: not a directory");
    }
}
