// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! World location identifier: Account-specific or Shared storage.
//!
//! Provides the `WorldLocation` enum representing where a world is stored:
//! - `Account(AccountId)` — World belongs to a specific Microsoft account
//! - `Shared` — World in the shared storage accessible by all accounts

use crate::AccountId;
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the location where a Minecraft Bedrock world is stored.
///
/// Worlds can be in two locations:
/// - **Account-specific**: Under `%APPDATA%\Minecraft Bedrock\Users\<account_id>\games\com.mojang\minecraftWorlds\`
/// - **Shared**: Under `%APPDATA%\Minecraft Bedrock\Users\Shared\games\com.mojang\minecraftWorlds\`
///
/// # Examples
///
/// ```
/// use blockoria_domain::{WorldLocation, AccountId};
///
/// // World in account-specific storage
/// let account = WorldLocation::Account(AccountId::new("3603359306789601710").unwrap());
/// assert_eq!(account.as_path_segment(), "3603359306789601710");
/// assert!(!account.is_shared());
///
/// // World in shared storage
/// let shared = WorldLocation::Shared;
/// assert_eq!(shared.as_path_segment(), "Shared");
/// assert!(shared.is_shared());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WorldLocation {
    /// World belongs to a specific Microsoft account.
    Account(AccountId),
    /// World is in the shared storage (accessible by all accounts).
    Shared,
}

impl WorldLocation {
    /// Returns the path segment used in filesystem paths.
    ///
    /// For `Account(id)`, returns the account ID string.
    /// For `Shared`, returns "Shared".
    pub fn as_path_segment(&self) -> &str {
        match self {
            WorldLocation::Account(id) => id.as_str(),
            WorldLocation::Shared => "Shared",
        }
    }

    /// Returns `true` if the world is in shared storage.
    pub fn is_shared(&self) -> bool {
        matches!(self, WorldLocation::Shared)
    }

    /// Returns the account ID if this is an account-specific location.
    pub fn account_id(&self) -> Option<&AccountId> {
        match self {
            WorldLocation::Account(id) => Some(id),
            WorldLocation::Shared => None,
        }
    }
}

impl std::fmt::Display for WorldLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_path_segment())
    }
}

impl From<AccountId> for WorldLocation {
    fn from(id: AccountId) -> Self {
        WorldLocation::Account(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AccountId;

    #[test]
    fn given_account_id_when_account_variant_then_returns_path_segment() {
        // Given
        let account_id = AccountId::new("1234567890098765432").unwrap();

        // When
        let location = WorldLocation::Account(account_id);

        // Then
        assert_eq!(location.as_path_segment(), "1234567890098765432");
    }

    #[test]
    fn given_shared_when_shared_variant_then_returns_shared_path_segment() {
        // Given / When
        let location = WorldLocation::Shared;

        // Then
        assert_eq!(location.as_path_segment(), "Shared");
    }

    #[test]
    fn given_account_when_is_shared_then_false() {
        // Given
        let location = WorldLocation::Account(AccountId::new("1234567890098765432").unwrap());

        // When / Then
        assert!(!location.is_shared());
    }

    #[test]
    fn given_shared_when_is_shared_then_true() {
        // Given
        let location = WorldLocation::Shared;

        // When / Then
        assert!(location.is_shared());
    }

    #[test]
    fn given_account_when_account_id_then_some() {
        // Given
        let account_id = AccountId::new("1234567890098765432").unwrap();
        let location = WorldLocation::Account(account_id.clone());

        // When
        let result = location.account_id();

        // Then
        assert_eq!(result, Some(&account_id));
    }

    #[test]
    fn given_shared_when_account_id_then_none() {
        // Given
        let location = WorldLocation::Shared;

        // When
        let result = location.account_id();

        // Then
        assert_eq!(result, None);
    }

    #[test]
    fn given_account_id_when_from_then_account_variant() {
        // Given
        let account_id = AccountId::new("1234567890098765432").unwrap();

        // When
        let location: WorldLocation = account_id.into();

        // Then
        assert_eq!(
            location,
            WorldLocation::Account(AccountId::new("1234567890098765432").unwrap())
        );
    }

    #[test]
    fn given_display_when_account_then_shows_id() {
        // Given
        let location = WorldLocation::Account(AccountId::new("1234567890098765432").unwrap());

        // When
        let displayed = location.to_string();

        // Then
        assert_eq!(displayed, "1234567890098765432");
    }

    #[test]
    fn given_display_when_shared_then_shows_shared() {
        // Given
        let location = WorldLocation::Shared;

        // When
        let displayed = location.to_string();

        // Then
        assert_eq!(displayed, "Shared");
    }

    #[test]
    fn given_account_when_clone_then_equal() {
        // Given
        let location = WorldLocation::Account(AccountId::new("1234567890098765432").unwrap());

        // When
        let cloned = location.clone();

        // Then
        assert_eq!(location, cloned);
    }

    #[test]
    fn given_shared_when_clone_then_equal() {
        // Given
        let location = WorldLocation::Shared;

        // When
        let cloned = location.clone();

        // Then
        assert_eq!(location, cloned);
    }
}
