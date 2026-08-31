// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! # blockoria-domain
//!
//! Pure domain layer for the Blockoria Minecraft Bedrock Backup Manager.
//! This crate contains the core domain logic with zero external dependencies.
//!
//! ## Architecture
//!
//! This crate implements the **Domain Layer** in a Clean Architecture / Hexagonal Architecture.
//! It contains only business logic and domain models with no external dependencies
//! (no filesystem, no network, no serialization frameworks).
//!
//! ## Domain Model
//!
//! ### Value Objects (8)
//! - `WorldFolderName` — Minecraft Bedrock world folder name (12 chars, base64, ends with `=`)
//! - `WorldPath` — Validated filesystem path to a world directory
//! - `LevelName` — Display name of the world (non-empty, trimmed)
//! - `AccountId` — Microsoft account identifier (non-empty, trimmed)
//! - `WorldVersion` — World version as `[u16; 5]` with semantic validation
//! - `WorldIconPath` — Path to world icon (`world_icon.jpeg`)
//! - `BackupTimestamp` — UTC timestamp of backup creation
//! - `BackupPath` — Validated filesystem path to a backup directory
//!
//! ### Aggregates (2)
//! - `World` — Aggregate root representing a Minecraft Bedrock world
//! - `Backup` — Aggregate root representing a world backup at a point in time
//!
//! ### Errors
//! - `DomainError` — Enum with 8 variants covering all validation failures
//!
//! ## Design Principles
//!
//! - **Zero external dependencies** — Only uses `std`, `chrono` (optional serde feature)
//! - **Validation at construction** — All invariants enforced in `::new()` constructors
//! - **Immutability** — All types are `Clone` + `Debug` + `PartialEq` + `Eq`
//! - **Type safety** — Newtype pattern for all domain primitives
//! - **Serialization** — Optional `serde` feature for Tauri IPC

//! ## DomainError
//!
//! All domain validation errors are represented as `DomainError` variants:
//! - `InvalidWorldFolderName`
//! - `InvalidWorldPath`
//! - `InvalidWorldIconPath`
//! - `InvalidLevelName`
//! - `InvalidAccountId`
//! - `InvalidWorldVersion`
//! - `InvalidBackupTimestamp`
//! - `InvalidBackupPath`

pub mod error;
pub use error::DomainError;

/// World folder name value object (12 chars base64 + `=` suffix).
pub mod world_folder_name;
pub use world_folder_name::WorldFolderName;

/// Validated filesystem path to a world directory.
pub mod world_path;
pub use world_path::WorldPath;

/// World display name (levelname).
pub mod level_name;
pub use level_name::LevelName;

/// Microsoft account identifier.
pub mod account_id;
pub use account_id::AccountId;

/// World location: Account-specific or Shared storage.
pub mod world_location;
pub use world_location::WorldLocation;

/// World version as `[u16; 5]` with validation.
pub mod world_version;
pub use world_version::WorldVersion;

/// World icon path (`world_icon.jpeg`).
pub mod world_icon_path;
pub use world_icon_path::WorldIconPath;

/// UTC timestamp of backup creation.
pub mod backup_timestamp;
pub use backup_timestamp::BackupTimestamp;

/// Validated filesystem path to a backup directory.
pub mod backup_path;
pub use backup_path::BackupPath;

/// Domain aggregates: `World` and `Backup`.
pub mod entities;
pub use entities::{Backup, World};
