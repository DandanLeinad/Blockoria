// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! LE-NBT (Little-Endian NBT) parsing for Minecraft Bedrock level.dat.
//!
//! This module provides:
//! - [`NbtError`] - Error types for parsing
//! - [`NbtTagType`] - NBT tag type enumeration (0-12)
//! - [`NbtValue`] - Parsed NBT value AST
//! - [`NbtList`] - TAG_List wrapper preserving element type
//! - [`LeReader`] - Little-endian binary reader with safety limits
//! - [`Parser`] - Generic LE-NBT parser with depth tracking
//! - [`LevelDatHeader`] - level.dat file header
//! - [`LevelDatParser`] - level.dat file parser
//! - [`extract_world_version`] - Extract WorldVersion from NBT

pub mod error;
pub mod level_dat;
pub mod parser;
pub mod reader;
pub mod tag_type;
pub mod value;

pub use error::{NbtError, NbtErrorExt};
pub use level_dat::{LevelDatHeader, LevelDatParser, extract_world_version};
pub use parser::Parser;
pub use reader::LeReader;
pub use tag_type::NbtTagType;
pub use value::{NbtList, NbtValue};
