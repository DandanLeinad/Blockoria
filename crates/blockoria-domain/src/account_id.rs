// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Microsoft account identifier.
//!
//! Provides the `AccountId` value object representing the player's Microsoft
// account identifier. Read from the world's level.dat or Minecraft Bedrock
// configuration.

use crate::DomainError;
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Value Object for the player's Microsoft account ID.
///
/// Unique identifier of the Microsoft account associated with the world.
/// Read from the `level.dat` file or Minecraft Bedrock configuration.
///
/// Validation rule:
/// - Cannot be empty or just whitespace
///
/// # Examples
///
/// ```
/// use blockoria_domain::AccountId;
/// let id = AccountId::new("123456789012345678").unwrap();
/// assert_eq!(id.as_str(), "123456789012345678");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccountId(String);

impl AccountId {
    /// Creates a new AccountId validating the format.
    ///
    /// # Errors
    ///
    /// Returns `DomainError::InvalidAccountId` if:
    /// - Empty string
    /// - Just whitespace (spaces, tabs, newlines)
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DomainError::InvalidAccountId(
                "Account ID cannot be empty or whitespace".into(),
            ));
        }
        Ok(AccountId(value))
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
    fn given_valid_id_when_new_then_ok() {
        // Given
        let input = "123456789012345678";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "123456789012345678");
    }

    #[test]
    fn given_empty_when_new_then_err() {
        // Given
        let input = "";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidAccountId(_))));
    }

    #[test]
    fn given_whitespace_only_when_new_then_err() {
        // Given
        let input = "   ";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidAccountId(_))));
    }

    #[test]
    fn given_tabs_newlines_when_new_then_err() {
        // Given
        let input = "\t\n\r";

        // When
        let result = AccountId::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidAccountId(_))));
    }
}
