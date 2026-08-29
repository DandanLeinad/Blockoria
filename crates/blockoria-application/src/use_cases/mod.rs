// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Business logic orchestration — use cases.
//!
//! This module contains all use cases (business logic orchestrations) of the
// application layer. Each use case is a pure function that orchestrates
// domain logic and repository interactions.
//!
//! Use cases are pure functions that:
//! - Take repositories (ports) as dependencies
//! - Orchestrate domain logic
//! - Return `Result<T, DomainError>`
//!
//! # Use Cases
//!
//! | Use Case | Description |
//! |----------|-------------|
//! | `create_backup` | Create a new backup of a world |
//! | `list_worlds` | List all known worlds |
//! | `list_backups` | List all backups for a specific world/account |
//! | `restore_backup` | Restore a world from a backup |
//! | `delete_backup` | Delete a backup by its path |

pub mod create_backup;
pub mod delete_backup;
pub mod list_backups;
pub mod list_worlds;
pub mod restore_backup;
