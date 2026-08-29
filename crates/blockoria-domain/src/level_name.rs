// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! World display name (levelname).
//!
//! Provides the `LevelName` value object representing the world's display name
//! as read from `levelname.txt`. Validates that the name is not empty or whitespace.

use crate::DomainError;
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Value Object for Minecraft Bedrock world display name.
///
/// This name is read from the `levelname.txt` file inside the world folder.
/// It is the name the player sees in the world selection menu.
///
/// Validation rule:
/// - Cannot be empty or just whitespace
///
/// # Examples
///
/// ```
/// use blockoria_domain::LevelName;
/// let name = LevelName::new("My World").unwrap();
/// assert_eq!(name.as_str(), "My World");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LevelName(String);

impl LevelName {
    /// Creates a new LevelName validating the format.
    ///
    /// # Errors
    ///
    /// Returns `DomainError::InvalidLevelName` if:
    /// - Empty string
    /// - Just whitespace (spaces, tabs, newlines)
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::InvalidLevelName(
                "Level name cannot be empty or whitespace".into(),
            ));
        }
        Ok(LevelName(value))
    }

    /// Returns the inner string as a slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_name_when_new_then_ok() {
        // Given
        let input = "My World";

        // When
        let result = LevelName::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "My World");
    }

    #[test]
    fn given_empty_when_new_then_err() {
        // Given
        let input = "";

        // When
        let result = LevelName::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidLevelName(_))));
    }

    #[test]
    fn given_whitespace_only_when_new_then_err() {
        // Given
        let input = "   ";

        // When
        let result = LevelName::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidLevelName(_))));
    }

    #[test]
    fn given_tabs_newlines_when_new_then_err() {
        // Given
        let input = "\t\n\r";

        // When
        let result = LevelName::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidLevelName(_))));
    }
}
