// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Repository ports (traits) for the application layer.
//!
//! This module defines the repository interfaces (ports) that the
//! infrastructure layer must implement. These traits define the contract
//! between the application layer and the persistence layer.
//!
//! By depending on traits rather than concrete implementations, the
//! application layer remains decoupled from specific storage mechanisms
//! (filesystem, database, cloud, etc.).

/// Backup repository port — read/write operations for backups.
pub mod backup_repository;
/// World repository port — read operations for worlds.
pub mod world_repository;

pub use backup_repository::BackupRepository;
pub use world_repository::WorldRepository;
