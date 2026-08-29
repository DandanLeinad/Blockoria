// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! # blockoria-application
//!
//! Application layer for the Blockoria Minecraft Bedrock Backup Manager.
//! Contains use cases (business logic orchestration) and ports (traits) that
//! define the interfaces required by the application.
//!
//! ## Architecture
//!
//! This crate implements the **Application Layer** in Clean Architecture.
//! It contains only business logic and port definitions — no external
//! dependencies like filesystem, network, or framework code.
//!
//! The application layer depends **only** on `blockoria-domain`.
//! It defines **ports** (traits) that the infrastructure layer must implement.
//!
//! ## Structure
//!
//! - `ports` — Repository traits (`WorldRepository`, `BackupRepository`)
//! - `use_cases` — Business logic orchestrations (5 use cases)
//! - `util` — Shared utilities (filesystem operations)
//! - `error` — Error type alias (`Result<T>` = `std::result::Result<T, DomainError>`)
//!
//! ## Use Cases
//!
//! | Use Case | Description |
//! |----------|-------------|
//! | `create_backup` | Create a new backup of a world |
//! | `list_worlds` | List all known worlds |
//! | `list_backups` | List all backups for a specific world/account |
//! | `restore_backup` | Restore a world from a backup |
//! | `delete_backup` | Delete a backup by its path |
//!
//! ## Dependency Rule
//!
//! ```text
//! Application -> Domain (only)
//! ```
//!
//! The application layer **never** depends on infrastructure, filesystem,
//! network, or framework code. All external interactions go through ports.

/// Error handling — public result type alias.
pub mod error;
/// Repository ports (traits) — interfaces for persistence.
pub mod ports;
/// Business logic orchestration — use cases.
pub mod use_cases;
/// Shared utilities — filesystem operations.
pub mod util;

pub use error::Result;
pub use ports::{BackupRepository, WorldRepository};
pub use use_cases::{create_backup, delete_backup, list_backups, list_worlds, restore_backup};
