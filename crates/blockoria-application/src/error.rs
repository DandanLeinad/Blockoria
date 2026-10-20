// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Error handling for the application layer.
//!
//! Provides the `Result<T>` type alias currently used by ports and use cases
//! (`DomainError`), and — per `docs/specs/07-error-handling.md` (RF-01) — the
//! `ApplicationError` taxonomy that will replace it: honest categories
//! (NotFound, Validation, Storage, Config) instead of validation variants
//! reused for everything.

use blockoria_domain::DomainError;
use thiserror::Error;

/// Application result type — `std::result::Result<T, DomainError>`.
///
/// All use cases and repository operations return this result type.
/// The error type is always `DomainError` from the domain layer,
/// ensuring consistent error handling across the application layer.
pub type Result<T> = std::result::Result<T, DomainError>;

/// Which aggregate a `NotFound` refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum What {
    /// A `World` aggregate was not found.
    World,
    /// A `Backup` aggregate was not found.
    Backup,
}

impl std::fmt::Display for What {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            What::World => write!(f, "world"),
            What::Backup => write!(f, "backup"),
        }
    }
}

/// Application-layer error taxonomy (`docs/specs/07-error-handling.md`, RF-01).
///
/// Honest categories instead of validation variants reused for everything:
/// `NotFound` for missing aggregates, `Validation` wrapping domain rule
/// violations, `Storage` for real I/O failures, `Config` for configuration
/// problems.
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// The requested aggregate does not exist.
    #[error("not found: {what}")]
    NotFound {
        /// Which aggregate was not found.
        what: What,
    },

    /// A domain validation rule was violated.
    #[error("validation: {0}")]
    Validation(#[from] DomainError),

    /// A real I/O failure occurred (permissions, disk, missing path).
    #[error("storage: {0}")]
    Storage(#[from] std::io::Error),

    /// Reading, writing, or parsing the configuration failed.
    #[error("config: {0}")]
    Config(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn not_found_reports_missing_world() {
        // Given
        let err = ApplicationError::NotFound { what: What::World };

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "not found: world");
    }

    #[test]
    fn not_found_distinguishes_backup_from_world() {
        // Given
        let err = ApplicationError::NotFound { what: What::Backup };

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "not found: backup");
    }

    #[test]
    fn domain_validation_wraps_into_validation_category() {
        // Given
        let domain_err = DomainError::InvalidWorldFolderName("11 chars".into());

        // When
        let err: ApplicationError = domain_err.into();

        // Then
        assert!(matches!(
            err,
            ApplicationError::Validation(DomainError::InvalidWorldFolderName(_))
        ));
        assert_eq!(
            err.to_string(),
            "validation: InvalidWorldFolderName: 11 chars"
        );
    }

    #[test]
    fn io_failure_wraps_into_storage_category() {
        // Given
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");

        // When
        let err: ApplicationError = io_err.into();

        // Then
        assert!(matches!(err, ApplicationError::Storage(_)));
        assert!(err.to_string().starts_with("storage: "));
    }

    #[test]
    fn config_error_carries_detail() {
        // Given
        let err = ApplicationError::Config("failed to parse config.toml".into());

        // When
        let display = err.to_string();

        // Then
        assert_eq!(display, "config: failed to parse config.toml");
    }

    #[test]
    fn application_error_implements_std_error() {
        // Given
        let err = ApplicationError::NotFound { what: What::Backup };

        // When
        let _: &dyn std::error::Error = &err;

        // Then (compiles if the Error trait is implemented)
    }
}
