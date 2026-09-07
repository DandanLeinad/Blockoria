// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! level.dat parser for Minecraft Bedrock Edition.
//!
//! Parses the level.dat file format which consists of:
//! - 8-byte header: [i32 version][i32 nbt_size] (little-endian)
//! - NBT payload of exactly nbt_size bytes

use super::{NbtError, parser::Parser};
use blockoria_domain::WorldVersion;
use std::io::{self, Read};

/// level.dat header (8 bytes little-endian).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LevelDatHeader {
    /// File version/type (e.g., 123 for Bedrock).
    pub version: i32,
    /// Size of the NBT payload in bytes.
    pub nbt_size: i32,
}

impl LevelDatHeader {
    /// Maximum allowed level.dat file size (10 MB).
    pub const MAX_SIZE: usize = 10_000_000;

    /// Reads the header from a reader.
    pub fn read_from<R: Read>(reader: &mut R) -> Result<Self, NbtError> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf).map_err(|e| {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                NbtError::unexpected_eof(0)
            } else {
                NbtError::io(0, e)
            }
        })?;

        let version = i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        let nbt_size = i32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);

        if nbt_size < 0 {
            return Err(NbtError::invalid_header("nbt_size negative"));
        }
        if nbt_size as usize > Self::MAX_SIZE {
            return Err(NbtError::invalid_header("nbt_size exceeds maximum"));
        }

        Ok(Self { version, nbt_size })
    }
}

/// Parser for level.dat files.
///
/// Reads the 8-byte header, validates nbt_size, then parses exactly
/// nbt_size bytes of NBT data using the generic Parser.
pub struct LevelDatParser<R: Read> {
    reader: R,
    header: LevelDatHeader,
}

impl<R: Read> LevelDatParser<R> {
    /// Creates a new LevelDatParser by reading and validating the header.
    pub fn new(mut reader: R) -> Result<Self, NbtError> {
        let header = LevelDatHeader::read_from(&mut reader)?;
        Ok(Self { reader, header })
    }

    /// Returns the parsed header.
    pub fn header(&self) -> &LevelDatHeader {
        &self.header
    }

    /// Parses the NBT payload (exactly nbt_size bytes).
    ///
    /// Limits reading to the declared nbt_size bytes, then parses the NBT.
    pub fn parse(mut self) -> Result<super::NbtValue, NbtError> {
        let nbt_size = self.header.nbt_size as usize;

        // Read exactly nbt_size bytes
        let mut payload = vec![0u8; nbt_size];
        self.reader.read_exact(&mut payload).map_err(|e| {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                NbtError::unexpected_eof(self.header.nbt_size as u64)
            } else {
                NbtError::io(self.header.nbt_size as u64, e)
            }
        })?;

        // Parse the NBT payload
        let mut parser = Parser::new(payload.as_slice());
        parser.parse_value()
    }
}

/// Extracts WorldVersion from parsed NBT.
///
/// Searches for `lastOpenedWithVersion` in the NBT structure.
/// Supports both TAG_Int_Array and TAG_List of TAG_Int formats.
/// Returns None if not found or format is invalid.
pub fn extract_world_version(nbt: &super::NbtValue) -> Option<WorldVersion> {
    // Navigate: root Compound -> "Data" (or root or empty-key) -> "lastOpenedWithVersion"
    let root_compound = match nbt {
        super::NbtValue::Compound(map) => map,
        _ => return None,
    };

    // Try "Data" first (common in Bedrock), then empty-key root (common in newer Bedrock), then root
    let data_compound = root_compound
        .get("Data")
        .and_then(|v| v.as_compound())
        .or_else(|| root_compound.get("").and_then(|v| v.as_compound()))
        .unwrap_or(root_compound);

    let version_value = data_compound.get("lastOpenedWithVersion")?;

    // Try TAG_Int_Array (Bedrock format)
    if let Some(arr) = version_value.as_int_array()
        && arr.len() == 5
    {
        let version_arr: [u16; 5] = [
            arr[0] as u16,
            arr[1] as u16,
            arr[2] as u16,
            arr[3] as u16,
            arr[4] as u16,
        ];
        return WorldVersion::new(version_arr).ok();
    }

    // Try TAG_List of TAG_Int
    if let Some(list) = version_value.as_list()
        && list.element_type == super::NbtTagType::Int
        && list.values.len() == 5
    {
        let version_arr: [u16; 5] = [
            list.values[0].as_int()? as u16,
            list.values[1].as_int()? as u16,
            list.values[2].as_int()? as u16,
            list.values[3].as_int()? as u16,
            list.values[4].as_int()? as u16,
        ];
        return WorldVersion::new(version_arr).ok();
    }

    None
}

// Extension trait for NbtValue to access typed getters
#[expect(dead_code)]
trait NbtValueExt {
    fn as_int_array(&self) -> Option<&Vec<i32>>;
    fn as_int(&self) -> Option<i32>;
    fn as_list(&self) -> Option<&super::NbtList>;
    fn as_compound(&self) -> Option<&std::collections::BTreeMap<String, super::NbtValue>>;
}

impl NbtValueExt for super::NbtValue {
    fn as_int_array(&self) -> Option<&Vec<i32>> {
        match self {
            super::NbtValue::IntArray(arr) => Some(arr),
            _ => None,
        }
    }

    fn as_int(&self) -> Option<i32> {
        match self {
            super::NbtValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    fn as_list(&self) -> Option<&super::NbtList> {
        match self {
            super::NbtValue::List(list) => Some(list),
            _ => None,
        }
    }

    fn as_compound(&self) -> Option<&std::collections::BTreeMap<String, super::NbtValue>> {
        match self {
            super::NbtValue::Compound(map) => Some(map),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{NbtList, NbtTagType, NbtValue};
    use super::*;
    use std::io::Cursor;

    // Given / When / Then

    #[test]
    fn given_valid_header_when_read_then_returns_header() {
        // Given: version=123, nbt_size=456
        let mut data = vec![];
        data.extend_from_slice(&123i32.to_le_bytes());
        data.extend_from_slice(&456i32.to_le_bytes());
        let mut cursor = Cursor::new(data);

        // When
        let result = LevelDatHeader::read_from(&mut cursor);

        // Then
        assert!(result.is_ok());
        let header = result.unwrap();
        assert_eq!(header.version, 123);
        assert_eq!(header.nbt_size, 456);
    }

    #[test]
    fn given_negative_nbt_size_when_read_then_error() {
        // Given
        let mut data = vec![];
        data.extend_from_slice(&123i32.to_le_bytes());
        data.extend_from_slice(&(-1i32).to_le_bytes());
        let mut cursor = Cursor::new(data);

        // When
        let result = LevelDatHeader::read_from(&mut cursor);

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::InvalidHeader { .. })));
    }

    #[test]
    fn given_excessive_nbt_size_when_read_then_error() {
        // Given: nbt_size > MAX_SIZE
        let mut data = vec![];
        data.extend_from_slice(&123i32.to_le_bytes());
        data.extend_from_slice(&(LevelDatHeader::MAX_SIZE as i32 + 1).to_le_bytes());
        let mut cursor = Cursor::new(data);

        // When
        let result = LevelDatHeader::read_from(&mut cursor);

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::InvalidHeader { .. })));
    }

    #[test]
    fn given_eof_during_header_when_read_then_error() {
        // Given: only 4 bytes
        let data = vec![0x00, 0x00, 0x00, 0x00];
        let mut cursor = Cursor::new(data);

        // When
        let result = LevelDatHeader::read_from(&mut cursor);

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::UnexpectedEof { .. })));
    }

    #[test]
    fn given_valid_level_dat_when_parse_then_returns_nbt() {
        // Given: valid level.dat with header + simple NBT
        let mut data = vec![];
        // Header
        data.extend_from_slice(&1i32.to_le_bytes()); // version
        data.extend_from_slice(&0x16i32.to_le_bytes()); // nbt_size = 22 bytes
        // NBT: TAG_Compound root with name "Data", containing version=Int(1)
        data.push(10); // TAG_Compound
        data.extend_from_slice(&4u16.to_le_bytes()); // "Data"
        data.extend_from_slice(b"Data");
        data.push(3); // TAG_Int
        data.extend_from_slice(&7u16.to_le_bytes()); // "version"
        data.extend_from_slice(b"version");
        data.extend_from_slice(&1i32.to_le_bytes());
        data.push(0); // TAG_End

        let mut cursor = Cursor::new(data);

        // When
        let parser = LevelDatParser::new(&mut cursor);
        assert!(parser.is_ok());
        let result = parser.unwrap().parse();

        // Then
        assert!(result.is_ok());
    }

    #[test]
    fn given_level_dat_with_wrong_nbt_size_when_parse_then_error() {
        // Given: header says 100 bytes but only 10 bytes follow
        let mut data = vec![];
        data.extend_from_slice(&1i32.to_le_bytes()); // version
        data.extend_from_slice(&100i32.to_le_bytes()); // nbt_size (wrong)
        data.extend_from_slice(&[0u8; 10]); // only 10 bytes
        let mut cursor = Cursor::new(data);

        // When
        let parser = LevelDatParser::new(&mut cursor);
        assert!(parser.is_ok());
        let result = parser.unwrap().parse();

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::UnexpectedEof { .. })));
    }

    #[test]
    fn given_nbt_with_int_array_version_when_extract_then_returns_world_version() {
        // Given: NBT with lastOpenedWithVersion as IntArray [1,21,0,0,0]
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "lastOpenedWithVersion".to_string(),
            NbtValue::IntArray(vec![1, 21, 0, 0, 0]),
        );
        let nbt = NbtValue::Compound(map);

        // When
        let result = extract_world_version(&nbt);

        // Then
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_array(), &[1, 21, 0, 0, 0]);
    }

    #[test]
    fn given_nbt_with_list_version_when_extract_then_returns_world_version() {
        // Given: NBT with lastOpenedWithVersion as List of Int [1,21,0,0,0]
        let mut map = std::collections::BTreeMap::new();
        let list = NbtList::new(
            NbtTagType::Int,
            vec![
                NbtValue::Int(1),
                NbtValue::Int(21),
                NbtValue::Int(0),
                NbtValue::Int(0),
                NbtValue::Int(0),
            ],
        );
        map.insert("lastOpenedWithVersion".to_string(), NbtValue::List(list));
        let nbt = NbtValue::Compound(map);

        // When
        let result = extract_world_version(&nbt);

        // Then
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_array(), &[1, 21, 0, 0, 0]);
    }

    #[test]
    fn given_nbt_with_version_in_data_when_extract_then_returns_world_version() {
        // Given: NBT with "Data" compound containing lastOpenedWithVersion
        let mut data_map = std::collections::BTreeMap::new();
        data_map.insert(
            "lastOpenedWithVersion".to_string(),
            NbtValue::IntArray(vec![2, 0, 0, 0, 0]),
        );
        let mut root_map = std::collections::BTreeMap::new();
        root_map.insert("Data".to_string(), NbtValue::Compound(data_map));
        let nbt = NbtValue::Compound(root_map);

        // When
        let result = extract_world_version(&nbt);

        // Then
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_array(), &[2, 0, 0, 0, 0]);
    }

    #[test]
    fn given_nbt_without_version_when_extract_then_returns_none() {
        // Given: NBT without lastOpenedWithVersion
        let mut map = std::collections::BTreeMap::new();
        map.insert("other".to_string(), NbtValue::Int(42));
        let nbt = NbtValue::Compound(map);

        // When
        let result = extract_world_version(&nbt);

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn given_nbt_with_invalid_version_format_when_extract_then_returns_none() {
        // Given: lastOpenedWithVersion as String (invalid)
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "lastOpenedWithVersion".to_string(),
            NbtValue::String("invalid".to_string()),
        );
        let nbt = NbtValue::Compound(map);

        // When
        let result = extract_world_version(&nbt);

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn given_nbt_with_wrong_array_length_when_extract_then_returns_none() {
        // Given: IntArray with wrong length (4 instead of 5)
        let mut map = std::collections::BTreeMap::new();
        map.insert(
            "lastOpenedWithVersion".to_string(),
            NbtValue::IntArray(vec![1, 21, 0, 0]),
        );
        let nbt = NbtValue::Compound(map);

        // When
        let result = extract_world_version(&nbt);

        // Then
        assert!(result.is_none());
    }
}
