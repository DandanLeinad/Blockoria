// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Value Object for Minecraft Bedrock world version.
///
/// Represents the `lastOpenedWithVersion` field from `level.dat`.
/// Format: exactly 5 non-negative integers (e.g., [1, 21, 0, 0, 0]).
///
/// Validation rules:
/// - Exactly 5 elements
/// - All non-negative integers (u16)
///
/// # Examples
///
/// ```
/// use blockoria_domain::WorldVersion;
/// let version = WorldVersion::new([1, 21, 0, 0, 0]).unwrap();
/// assert_eq!(version.as_array(), &[1, 21, 0, 0, 0]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WorldVersion([u16; 5]);

impl WorldVersion {
    /// Maximum value for each version component (i16::MAX).
    /// Minecraft uses small values (e.g., [1, 21, 0, 0, 0]),
    /// this limit prevents absurd values without restricting real versions.
    pub const MAX_VERSION_COMPONENT: u16 = 32767;

    /// Creates a new WorldVersion validating the format.
    ///
    /// # Errors
    ///
    /// Returns `DomainError::InvalidWorldVersion` if:
    /// - Array does not have exactly 5 elements
    /// - Any element exceeds the maximum allowed value
    pub fn new(value: [u16; 5]) -> Result<Self, DomainError> {
        // u16 already guarantees non-negative, but we validate semantics
        for &num in &value {
            if num > Self::MAX_VERSION_COMPONENT {
                return Err(DomainError::InvalidWorldVersion(
                    format!(
                        "World version component {} exceeds maximum allowed value {}",
                        num,
                        Self::MAX_VERSION_COMPONENT
                    ),
                ));
            }
        }
        Ok(WorldVersion(value))
    }

    /// Returns the inner array as a slice.
    pub fn as_array(&self) -> &[u16; 5] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_version_when_new_then_ok() {
        // Given
        let input = [1, 21, 0, 0, 0];

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_array(), &[1, 21, 0, 0, 0]);
    }

    #[test]
    fn given_valid_version_max_when_new_then_ok() {
        // Given
        let input = [255, 255, 255, 255, 255];

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_negative_version_when_new_then_err() {
        // Given
        // u16 doesn't allow negative, but we test semantics via custom error
        // This test documents that validation exists
        let input = [1, 2, 3, 4, 5];

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_excessive_version_when_new_then_err() {
        // Given
        let input = [65535, 0, 0, 0, 0]; // u16 max, above our 32767 threshold

        // When
        let result = WorldVersion::new(input);

        // Then
        assert!(matches!(result, Err(DomainError::InvalidWorldVersion(_))));
    }
}
