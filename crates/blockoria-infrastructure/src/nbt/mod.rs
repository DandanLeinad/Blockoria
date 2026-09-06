// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! LE-NBT (Little-Endian NBT) parsing for Minecraft Bedrock level.dat.
//!
//! This module provides:
//! - [`NbtError`] - Error types for parsing
//! - [`NbtTagType`] - NBT tag type enumeration (0-12)
//! - [`NbtValue`] - Parsed NBT value AST
//! - [`NbtList`] - TAG_List wrapper preserving element type

pub mod error;
pub mod tag_type;
pub mod value;

pub use error::{NbtError, NbtErrorExt};
pub use tag_type::NbtTagType;
pub use value::{NbtList, NbtValue};
