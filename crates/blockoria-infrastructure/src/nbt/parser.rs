// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Generic LE-NBT parser with recursive parsing and depth tracking.
//!
//! Parses the complete NBT structure from a binary stream using LeReader.
//! Handles all tag types (0-12) with safety limits for nesting depth,
//! compound entries, and list lengths.

use super::{NbtError, NbtList, NbtTagType, NbtValue, reader::LeReader};
use std::collections::BTreeMap;

/// Maximum nesting depth for compounds and lists to prevent stack overflow.
const MAX_NESTING_DEPTH: usize = 128;

/// Maximum number of entries in a TAG_Compound to prevent OOM.
const MAX_COMPOUND_ENTRIES: usize = 100_000;

/// Maximum number of elements in a TAG_List to prevent OOM.
const MAX_LIST_LENGTH: usize = 1_000_000;

/// Generic LE-NBT parser with configurable limits.
///
/// Parses NBT data from any reader implementing `Read`.
/// Tracks nesting depth and enforces safety limits.
pub struct Parser<R: std::io::Read> {
    reader: LeReader<R>,
    depth: usize,
}

impl<R: std::io::Read> Parser<R> {
    /// Creates a new parser wrapping the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: LeReader::new(reader),
            depth: 0,
        }
    }

    /// Returns a reference to the inner LeReader.
    pub fn reader(&self) -> &LeReader<R> {
        &self.reader
    }

    /// Returns a mutable reference to the inner LeReader.
    pub fn reader_mut(&mut self) -> &mut LeReader<R> {
        &mut self.reader
    }

    /// Parses a complete NBT value from the current position.
    ///
    /// Reads a tag ID, then the tag name (for non-End tags), then dispatches
    /// to the appropriate parse method. The root tag is typically TAG_Compound (10).
    pub fn parse_value(&mut self) -> Result<NbtValue, NbtError> {
        let tag_id = self.reader.read_u8()?;
        let tag_type = NbtTagType::try_from(tag_id)
            .map_err(|_| NbtError::unknown_tag(tag_id, self.reader.offset()))?;

        // TAG_End has no name or payload
        if tag_type == NbtTagType::End {
            return Err(NbtError::unknown_tag(0, self.reader.offset()));
        }

        // Read the tag name (all non-End tags have a name)
        let name = self.reader.read_string()?;

        // Parse the payload
        let value = self.parse_payload(tag_type)?;

        // Wrap in a Compound with the name
        let mut compound = BTreeMap::new();
        compound.insert(name, value);
        Ok(NbtValue::Compound(compound))
    }

    /// Parses a TAG_Compound (map of named tags).
    ///
    /// Reads repeated [tag_id][name][payload] until TAG_End (0).
    /// Enforces MAX_COMPOUND_ENTRIES and MAX_NESTING_DEPTH limits.
    pub fn parse_compound(&mut self) -> Result<BTreeMap<String, NbtValue>, NbtError> {
        if self.depth >= MAX_NESTING_DEPTH {
            return Err(NbtError::max_depth_exceeded(
                MAX_NESTING_DEPTH,
                self.reader.offset(),
            ));
        }

        self.depth += 1;
        let mut map = BTreeMap::new();

        loop {
            if map.len() >= MAX_COMPOUND_ENTRIES {
                return Err(NbtError::excessive_length(
                    map.len() as i32,
                    "Compound",
                    MAX_COMPOUND_ENTRIES,
                    self.reader.offset(),
                ));
            }

            let tag_id = self.reader.read_u8()?;
            if tag_id == 0 {
                // TAG_End
                break;
            }

            let tag_type = NbtTagType::try_from(tag_id)
                .map_err(|_| NbtError::unknown_tag(tag_id, self.reader.offset()))?;

            let name = self.reader.read_string()?;
            let value = self.parse_payload(tag_type)?;
            map.insert(name, value);
        }

        self.depth -= 1;
        Ok(map)
    }

    /// Parses the payload for a given tag type (without the tag ID byte).
    ///
    /// Called after reading the tag ID and name (for compounds).
    fn parse_payload(&mut self, tag_type: NbtTagType) -> Result<NbtValue, NbtError> {
        match tag_type {
            NbtTagType::Byte => {
                let value = self.reader.read_i8()?;
                Ok(NbtValue::Byte(value))
            }
            NbtTagType::Short => {
                let value = self.reader.read_i16()?;
                Ok(NbtValue::Short(value))
            }
            NbtTagType::Int => {
                let value = self.reader.read_i32()?;
                Ok(NbtValue::Int(value))
            }
            NbtTagType::Long => {
                let value = self.reader.read_i64()?;
                Ok(NbtValue::Long(value))
            }
            NbtTagType::Float => {
                let value = self.reader.read_f32()?;
                Ok(NbtValue::Float(value))
            }
            NbtTagType::Double => {
                let value = self.reader.read_f64()?;
                Ok(NbtValue::Double(value))
            }
            NbtTagType::ByteArray => {
                let bytes = self.reader.read_byte_array()?;
                Ok(NbtValue::ByteArray(bytes))
            }
            NbtTagType::String => {
                let string = self.reader.read_string()?;
                Ok(NbtValue::String(string))
            }
            NbtTagType::List => {
                let list = self.parse_list()?;
                Ok(NbtValue::List(list))
            }
            NbtTagType::Compound => {
                let compound = self.parse_compound()?;
                Ok(NbtValue::Compound(compound))
            }
            NbtTagType::IntArray => {
                let array = self.reader.read_int_array()?;
                Ok(NbtValue::IntArray(array))
            }
            NbtTagType::LongArray => {
                let array = self.reader.read_long_array()?;
                Ok(NbtValue::LongArray(array))
            }
            NbtTagType::End => {
                // TAG_End should never appear as a payload
                Err(NbtError::unknown_tag(0, self.reader.offset()))
            }
        }
    }

    /// Parses a TAG_List (homogeneous list with subtype and length).
    ///
    /// Format: [u8 subtype][i32 length][elements...]
    /// Enforces MAX_LIST_LENGTH and MAX_NESTING_DEPTH limits.
    fn parse_list(&mut self) -> Result<NbtList, NbtError> {
        let subtype_id = self.reader.read_u8()?;
        let element_type = NbtTagType::try_from(subtype_id)
            .map_err(|_| NbtError::unknown_tag(subtype_id, self.reader.offset()))?;

        let len = self.reader.read_i32()?;
        if len < 0 {
            return Err(NbtError::negative_length(len, "List", self.reader.offset()));
        }
        let len = len as usize;

        if len > MAX_LIST_LENGTH {
            return Err(NbtError::excessive_length(
                len as i32,
                "List",
                MAX_LIST_LENGTH,
                self.reader.offset(),
            ));
        }

        if element_type.is_container() && self.depth >= MAX_NESTING_DEPTH {
            return Err(NbtError::max_depth_exceeded(
                MAX_NESTING_DEPTH,
                self.reader.offset(),
            ));
        }

        let mut values = Vec::with_capacity(len);
        for _ in 0..len {
            let value = self.parse_payload(element_type)?;
            values.push(value);
        }

        Ok(NbtList::new(element_type, values))
    }

    /// Returns the current byte offset.
    pub fn offset(&self) -> u64 {
        self.reader.offset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Given / When / Then

    #[test]
    fn given_byte_payload_when_parse_then_returns_byte_value() {
        // Given: TAG_Byte payload (value=42)
        let data = vec![42u8];
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::Byte);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::Byte(42));
    }

    #[test]
    fn given_short_payload_when_parse_then_returns_short_value() {
        // Given: TAG_Short payload (value=300) little-endian
        let data = 300i16.to_le_bytes().to_vec();
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::Short);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::Short(300));
    }

    #[test]
    fn given_int_payload_when_parse_then_returns_int_value() {
        // Given: TAG_Int payload (value=123456)
        let data = 123456i32.to_le_bytes().to_vec();
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::Int);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::Int(123456));
    }

    #[test]
    fn given_long_payload_when_parse_then_returns_long_value() {
        // Given
        let data = 9999999999i64.to_le_bytes().to_vec();
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::Long);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::Long(9999999999));
    }

    #[test]
    fn given_float_payload_when_parse_then_returns_float_value() {
        // Given
        let data = std::f32::consts::PI.to_le_bytes().to_vec();
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::Float);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::Float(std::f32::consts::PI));
    }

    #[test]
    fn given_double_payload_when_parse_then_returns_double_value() {
        // Given
        let data = std::f64::consts::E.to_le_bytes().to_vec();
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::Double);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::Double(std::f64::consts::E));
    }

    #[test]
    fn given_byte_array_payload_when_parse_then_returns_byte_array() {
        // Given: length=3 + [1, 2, 3]
        let mut data = vec![0x03, 0x00, 0x00, 0x00];
        data.extend_from_slice(&[1, 2, 3]);
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::ByteArray);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::ByteArray(vec![1, 2, 3]));
    }

    #[test]
    fn given_string_payload_when_parse_then_returns_string() {
        // Given: length=5 + "Hello"
        let mut data = vec![0x05, 0x00];
        data.extend_from_slice(b"Hello");
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::String);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::String("Hello".to_string()));
    }

    #[test]
    fn given_list_of_int_when_parse_then_returns_list() {
        // Given: subtype=Int(3), length=3, values=[1,2,3]
        let mut data = vec![3u8]; // subtype = Int
        data.extend_from_slice(&3i32.to_le_bytes()); // length = 3
        data.extend_from_slice(&1i32.to_le_bytes());
        data.extend_from_slice(&2i32.to_le_bytes());
        data.extend_from_slice(&3i32.to_le_bytes());
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::List);

        // Then
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(
            matches!(value, NbtValue::List(list) if list.element_type == NbtTagType::Int && list.values.len() == 3)
        );
    }

    #[test]
    fn given_empty_list_when_parse_then_returns_empty_list_with_type() {
        // Given: subtype=Long(4), length=0
        let data = vec![4u8, 0x00, 0x00, 0x00, 0x00];
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::List);

        // Then
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(
            matches!(value, NbtValue::List(list) if list.element_type == NbtTagType::Long && list.is_empty())
        );
    }

    #[test]
    fn given_compound_when_parse_then_returns_compound() {
        // Given: Compound with "version"=Int(1), "name"=String("Test"), TAG_End
        // TAG_Int(3) + name="version" + value=1
        // TAG_String(8) + name="name" + value="Test"
        // TAG_End(0)
        let mut data = vec![];
        // version entry
        data.push(3); // TAG_Int
        data.extend_from_slice(&7u16.to_le_bytes()); // "version" length
        data.extend_from_slice(b"version");
        data.extend_from_slice(&1i32.to_le_bytes());
        // name entry
        data.push(8); // TAG_String
        data.extend_from_slice(&4u16.to_le_bytes()); // "name" length
        data.extend_from_slice(b"name");
        data.extend_from_slice(&4u16.to_le_bytes()); // "Test" length
        data.extend_from_slice(b"Test");
        // TAG_End
        data.push(0);

        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_compound();

        // Then
        assert!(result.is_ok());
        let compound = result.unwrap();
        assert_eq!(compound.get("version"), Some(&NbtValue::Int(1)));
        assert_eq!(
            compound.get("name"),
            Some(&NbtValue::String("Test".to_string()))
        );
        assert_eq!(compound.len(), 2);
    }

    #[test]
    fn given_nested_compound_when_parse_then_returns_nested() {
        // Given: outer -> inner -> "value"=Int(42)
        // Structure: outer Compound -> inner Compound -> value Int
        // Need 3 TAG_End markers: one for inner, one for middle, one for outer
        let mut data = vec![];
        // outer key
        data.push(10); // TAG_Compound
        data.extend_from_slice(&5u16.to_le_bytes());
        data.extend_from_slice(b"outer");
        // inner key
        data.push(10); // TAG_Compound
        data.extend_from_slice(&5u16.to_le_bytes());
        data.extend_from_slice(b"inner");
        // value entry
        data.push(3); // TAG_Int
        data.extend_from_slice(&5u16.to_le_bytes());
        data.extend_from_slice(b"value");
        data.extend_from_slice(&42i32.to_le_bytes());
        // TAG_End for inner compound
        data.push(0);
        // TAG_End for middle compound
        data.push(0);
        // TAG_End for outer compound
        data.push(0);

        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_compound();

        // Then
        if let Err(e) = &result {
            eprintln!("Error: {:?}", e);
        }
        assert!(result.is_ok());
        let compound = result.unwrap();
        let outer = compound.get("outer").unwrap();
        let inner = outer.as_compound().unwrap().get("inner").unwrap();
        let value = inner.as_compound().unwrap().get("value").unwrap();
        assert_eq!(*value, NbtValue::Int(42));
    }

    #[test]
    fn given_int_array_when_parse_then_returns_int_array() {
        // Given: length=3 + [1,2,3]
        let mut data = vec![0x03, 0x00, 0x00, 0x00];
        data.extend_from_slice(&1i32.to_le_bytes());
        data.extend_from_slice(&2i32.to_le_bytes());
        data.extend_from_slice(&3i32.to_le_bytes());
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::IntArray);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::IntArray(vec![1, 2, 3]));
    }

    #[test]
    fn given_long_array_when_parse_then_returns_long_array() {
        // Given: length=2 + [100, 200]
        let mut data = vec![0x02, 0x00, 0x00, 0x00];
        data.extend_from_slice(&100i64.to_le_bytes());
        data.extend_from_slice(&200i64.to_le_bytes());
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::LongArray);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), NbtValue::LongArray(vec![100, 200]));
    }

    #[test]
    fn given_unknown_tag_when_parse_then_error() {
        // Given: invalid tag ID 99
        let data = vec![99u8];
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_value();

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::UnknownTag { id, .. }) if id == 99));
    }

    #[test]
    fn given_max_depth_exceeded_when_parse_nested_then_error() {
        // Given: nested compounds exceeding MAX_NESTING_DEPTH
        let mut data = vec![];
        for _ in 0..MAX_NESTING_DEPTH + 1 {
            data.push(10); // TAG_Compound
            data.extend_from_slice(&1u16.to_le_bytes());
            data.push(b'a');
        }
        // Add TAG_End for each level
        data.extend(std::iter::repeat_n(0, MAX_NESTING_DEPTH + 1));

        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_compound();

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::MaxDepthExceeded { .. })));
    }

    #[test]
    fn given_excessive_compound_entries_when_parse_then_error() {
        // Given: compound with more entries than MAX_COMPOUND_ENTRIES
        // Use unique keys to ensure map.len() grows
        let mut data = vec![];
        for i in 0..MAX_COMPOUND_ENTRIES + 1 {
            data.push(3); // TAG_Int
            let key = format!("key_{}", i);
            data.extend_from_slice(&(key.len() as u16).to_le_bytes());
            data.extend_from_slice(key.as_bytes());
            data.extend_from_slice(&(i as i32).to_le_bytes());
        }
        data.push(0); // TAG_End

        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_compound();

        // Then
        eprintln!("Result: {:?}", result);
        assert!(result.is_err());
        assert!(
            matches!(result, Err(NbtError::ExcessiveLength { context, .. }) if context == "Compound")
        );
    }

    #[test]
    fn given_negative_list_length_when_parse_list_then_error() {
        // Given: subtype=Int(3), length=-1
        let mut data = vec![3u8];
        data.extend_from_slice(&(-1i32).to_le_bytes());
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::List);

        // Then
        assert!(result.is_err());
        assert!(
            matches!(result, Err(NbtError::NegativeLength { context, .. }) if context == "List")
        );
    }

    #[test]
    fn given_excessive_list_length_when_parse_list_then_error() {
        // Given: length > MAX_LIST_LENGTH
        let len = MAX_LIST_LENGTH + 1;
        let mut data = vec![3u8]; // subtype=Int
        data.extend_from_slice(&(len as i32).to_le_bytes());
        for _ in 0..len {
            data.extend_from_slice(&0i32.to_le_bytes());
        }
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::List);

        // Then
        assert!(result.is_err());
        assert!(
            matches!(result, Err(NbtError::ExcessiveLength { context, .. }) if context == "List")
        );
    }

    #[test]
    fn given_tag_end_as_payload_when_parse_then_error() {
        // Given: TAG_End (0) as payload
        let data = vec![0u8];
        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_payload(NbtTagType::End);

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::UnknownTag { id, .. }) if id == 0));
    }

    #[test]
    fn given_parse_value_with_compound_when_root_tag_then_returns_compound() {
        // Given: TAG_Compound root with name "Data" containing "version"=Int(1)
        // Format: [tag_id=10][name="Data"][payload...]
        let mut data = vec![];
        data.push(10); // TAG_Compound
        data.extend_from_slice(&4u16.to_le_bytes()); // "Data" length
        data.extend_from_slice(b"Data");
        // Payload: TAG_Int with name "version" and value 1
        data.push(3); // TAG_Int
        data.extend_from_slice(&7u16.to_le_bytes()); // "version" length
        data.extend_from_slice(b"version");
        data.extend_from_slice(&1i32.to_le_bytes());
        data.push(0); // TAG_End

        let mut parser = Parser::new(Cursor::new(data));

        // When
        let result = parser.parse_value();

        // Then
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(
            matches!(value, NbtValue::Compound(map) if map.get("Data") == Some(&NbtValue::Compound({
                let mut m = BTreeMap::new();
                m.insert("version".to_string(), NbtValue::Int(1));
                m
            })))
        );
    }
}
