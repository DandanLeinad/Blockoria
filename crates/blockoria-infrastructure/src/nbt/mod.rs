// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! LE-NBT (Little-Endian NBT) parsing for Minecraft Bedrock level.dat.
//!
//! This module provides:
//! - [`NbtError`] - Error types for parsing

pub mod error;

pub use error::{NbtError, NbtErrorExt};
