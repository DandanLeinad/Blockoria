// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Error handling for the application layer.
//!
//! Provides the `Result<T>` type alias which uses `DomainError` from the
// domain layer as the error type. All use cases return this result type.

use blockoria_domain::DomainError;

/// Application result type — `std::result::Result<T, DomainError>`.
///
/// All use cases and repository operations return this result type.
/// The error type is always `DomainError` from the domain layer,
/// ensuring consistent error handling across the application layer.
pub type Result<T> = std::result::Result<T, DomainError>;
