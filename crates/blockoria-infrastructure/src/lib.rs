// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! # blockoria-infrastructure
//!
//! Concrete implementations of the ports defined in `blockoria-application`.
//! This crate provides filesystem-based implementations for world and backup repositories.

pub mod config;
pub mod repositories;
pub mod test_contract;

pub use config::Config;
pub use repositories::{FileBackupRepository, FileWorldRepository};
