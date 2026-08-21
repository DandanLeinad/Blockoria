// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

use crate::DomainError;
use chrono::{DateTime, Utc};
// These imports are required when the "serde" feature is enabled.
// rust-analyzer may mark them as "unused" when the feature is disabled,
// but they are REQUIRED for the derive(Serialize, Deserialize) to work
// when the "serde" feature is enabled (for Tauri serialization).
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Value Object for backup creation timestamp.
///
/// Stores the exact UTC date/time of backup creation.
/// Uses `chrono::DateTime<Utc>` internally for precision and serialization.
///
/// Validation rule:
/// - Timestamp cannot be before 1970 (Unix epoch)
///
/// # Examples
///
/// ```
/// use blockoria_domain::BackupTimestamp;
/// use chrono::{TimeZone, Utc, Datelike};
///
/// let ts = BackupTimestamp::new(Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap()).unwrap();
/// assert_eq!(ts.as_datetime().year(), 2024);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BackupTimestamp(DateTime<Utc>);

impl BackupTimestamp {
    /// Creates a new BackupTimestamp validating the timestamp.
    ///
    /// # Errors
    ///
    /// Returns `DomainError::InvalidBackupTimestamp` if:
    /// - Timestamp before 1970-01-01 00:00:00 UTC
    pub fn new(value: DateTime<Utc>) -> Result<Self, DomainError> {
        if value.timestamp() < 0 {
            return Err(DomainError::InvalidBackupTimestamp(
                "Backup timestamp cannot be before Unix epoch (1970-01-01)".into(),
            ));
        }
        Ok(BackupTimestamp(value))
    }

    /// Creates timestamp for current moment (now).
    pub fn now() -> Self {
        // safe unwrap: now() is always >= epoch
        BackupTimestamp::new(Utc::now()).expect("current time is always valid")
    }

    /// Returns the inner `DateTime<Utc>`.
    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }

    /// Formats as ISO 8601 string (e.g., "2024-01-15T10:30:00Z").
    pub fn to_iso_string(&self) -> String {
        self.0.to_rfc3339()
    }

    /// Formats as filename-safe string.
    ///
    /// Replaces Windows-invalid characters (`:`, etc.) with safe alternatives.
    /// E.g., "2024-01-15T10-30-00Z" (colons replaced with hyphens).
    pub fn to_filename_safe(&self) -> String {
        self.0.format("%Y-%m-%dT%H-%M-%SZ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn given_valid_utc_datetime_when_new_then_ok() {
        // Given
        let input = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();

        // When
        let result = BackupTimestamp::new(input);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_datetime(), &input);
    }

    #[test]
    fn given_epoch_when_new_then_ok() {
        // Given
        let input = Utc.timestamp_opt(0, 0).single().unwrap();

        // When
        let result = BackupTimestamp::new(input);

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_before_epoch_when_new_then_err() {
        // Given
        let input = Utc.timestamp_opt(-1, 0).single().unwrap();

        // When
        let result = BackupTimestamp::new(input);

        // Then
        assert!(matches!(
            result,
            Err(DomainError::InvalidBackupTimestamp(_))
        ));
    }

    #[test]
    fn given_now_when_created_then_valid() {
        // When
        let before = chrono::Utc::now();
        let ts = BackupTimestamp::now();
        let after = chrono::Utc::now();

        // Then - timestamp should be between before and after (or equal)
        assert!(ts.as_datetime() >= &before);
        assert!(ts.as_datetime() <= &after);
    }

    #[test]
    fn given_timestamp_when_formatted_then_iso_string() {
        // Given
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let ts = BackupTimestamp::new(dt).unwrap();

        // When
        let formatted = ts.to_iso_string();

        // Then
        assert_eq!(formatted, "2024-01-15T10:30:00+00:00");
    }
}
