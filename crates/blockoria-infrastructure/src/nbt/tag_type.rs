// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! NBT Tag Type enumeration (IDs 0-12) for Little-Endian NBT.
//!
//! Matches Minecraft Bedrock Edition LE-NBT specification.
//! Reference: https://minecraft.fandom.com/wiki/Bedrock_Edition_level_format

use crate::NbtError;
use std::convert::TryFrom;

/// NBT tag types as defined in the LE-NBT specification.
/// Each variant corresponds to a tag ID (0-12).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NbtTagType {
    /// TAG_End - Marks end of compound/list (no payload)
    End = 0,
    /// TAG_Byte - Signed 8-bit integer
    Byte = 1,
    /// TAG_Short - Signed 16-bit integer (little-endian)
    Short = 2,
    /// TAG_Int - Signed 32-bit integer (little-endian)
    Int = 3,
    /// TAG_Long - Signed 64-bit integer (little-endian)
    Long = 4,
    /// TAG_Float - 32-bit floating point (little-endian)
    Float = 5,
    /// TAG_Double - 64-bit floating point (little-endian)
    Double = 6,
    /// TAG_Byte_Array - Array of bytes [i32 length][bytes...]
    ByteArray = 7,
    /// TAG_String - UTF-8 string [u16 length][bytes...]
    String = 8,
    /// TAG_List - Homogeneous list [u8 subtype][i32 length][elements...]
    List = 9,
    /// TAG_Compound - Map of named tags [u8 tag_id][string name][payload]... TAG_End
    Compound = 10,
    /// TAG_Int_Array - Array of ints [i32 length][i32 values...]
    IntArray = 11,
    /// TAG_Long_Array - Array of longs [i32 length][i64 values...]
    LongArray = 12,
}

impl NbtTagType {
    /// Returns the numeric tag ID (0-12).
    pub const fn id(self) -> u8 {
        self as u8
    }

    /// Returns true if this tag type has a payload (not TAG_End).
    pub const fn has_payload(self) -> bool {
        !matches!(self, NbtTagType::End)
    }

    /// Returns true if this tag type is a container (Compound or List).
    pub const fn is_container(self) -> bool {
        matches!(self, NbtTagType::Compound | NbtTagType::List)
    }

    /// Returns true if this tag type is an array type.
    pub const fn is_array(self) -> bool {
        matches!(
            self,
            NbtTagType::ByteArray | NbtTagType::IntArray | NbtTagType::LongArray
        )
    }
}

impl TryFrom<u8> for NbtTagType {
    type Error = NbtError;

    /// Attempts to convert a raw tag ID to `NbtTagType`.
    /// Returns `NbtError::UnknownTag` for IDs outside 0-12.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(NbtTagType::End),
            1 => Ok(NbtTagType::Byte),
            2 => Ok(NbtTagType::Short),
            3 => Ok(NbtTagType::Int),
            4 => Ok(NbtTagType::Long),
            5 => Ok(NbtTagType::Float),
            6 => Ok(NbtTagType::Double),
            7 => Ok(NbtTagType::ByteArray),
            8 => Ok(NbtTagType::String),
            9 => Ok(NbtTagType::List),
            10 => Ok(NbtTagType::Compound),
            11 => Ok(NbtTagType::IntArray),
            12 => Ok(NbtTagType::LongArray),
            id => Err(NbtError::unknown_tag(id, 0)), // offset será preenchido pelo caller
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Given / When / Then

    #[test]
    fn given_valid_tag_id_when_try_from_then_returns_correct_variant() {
        // Given / When / Then
        assert!(matches!(NbtTagType::try_from(0), Ok(NbtTagType::End)));
        assert!(matches!(NbtTagType::try_from(1), Ok(NbtTagType::Byte)));
        assert!(matches!(NbtTagType::try_from(2), Ok(NbtTagType::Short)));
        assert!(matches!(NbtTagType::try_from(3), Ok(NbtTagType::Int)));
        assert!(matches!(NbtTagType::try_from(4), Ok(NbtTagType::Long)));
        assert!(matches!(NbtTagType::try_from(5), Ok(NbtTagType::Float)));
        assert!(matches!(NbtTagType::try_from(6), Ok(NbtTagType::Double)));
        assert!(matches!(NbtTagType::try_from(7), Ok(NbtTagType::ByteArray)));
        assert!(matches!(NbtTagType::try_from(8), Ok(NbtTagType::String)));
        assert!(matches!(NbtTagType::try_from(9), Ok(NbtTagType::List)));
        assert!(matches!(NbtTagType::try_from(10), Ok(NbtTagType::Compound)));
        assert!(matches!(NbtTagType::try_from(11), Ok(NbtTagType::IntArray)));
        assert!(matches!(
            NbtTagType::try_from(12),
            Ok(NbtTagType::LongArray)
        ));
    }

    #[test]
    fn given_invalid_tag_id_when_try_from_then_returns_unknown_tag_error() {
        // Given
        let invalid_ids = [13u8, 99, 255];

        // When / Then
        for id in invalid_ids {
            let result = NbtTagType::try_from(id);
            assert!(result.is_err(), "ID {id} should be invalid");
            assert!(matches!(result, Err(NbtError::UnknownTag { id: i, .. }) if i == id));
        }
    }

    #[test]
    fn given_tag_type_when_id_then_returns_correct_numeric_id() {
        // Given / When / Then
        assert_eq!(NbtTagType::End.id(), 0);
        assert_eq!(NbtTagType::Byte.id(), 1);
        assert_eq!(NbtTagType::Short.id(), 2);
        assert_eq!(NbtTagType::Int.id(), 3);
        assert_eq!(NbtTagType::Long.id(), 4);
        assert_eq!(NbtTagType::Float.id(), 5);
        assert_eq!(NbtTagType::Double.id(), 6);
        assert_eq!(NbtTagType::ByteArray.id(), 7);
        assert_eq!(NbtTagType::String.id(), 8);
        assert_eq!(NbtTagType::List.id(), 9);
        assert_eq!(NbtTagType::Compound.id(), 10);
        assert_eq!(NbtTagType::IntArray.id(), 11);
        assert_eq!(NbtTagType::LongArray.id(), 12);
    }

    #[test]
    fn given_tag_type_when_has_payload_then_returns_correct_value() {
        // Given / When / Then
        assert!(!NbtTagType::End.has_payload());
        assert!(NbtTagType::Byte.has_payload());
        assert!(NbtTagType::Int.has_payload());
        assert!(NbtTagType::String.has_payload());
        assert!(NbtTagType::List.has_payload());
        assert!(NbtTagType::Compound.has_payload());
    }

    #[test]
    fn given_tag_type_when_is_container_then_returns_correct_value() {
        // Given / When / Then
        assert!(!NbtTagType::End.is_container());
        assert!(!NbtTagType::Byte.is_container());
        assert!(!NbtTagType::Int.is_container());
        assert!(NbtTagType::List.is_container());
        assert!(NbtTagType::Compound.is_container());
    }

    #[test]
    fn given_tag_type_when_is_array_then_returns_correct_value() {
        // Given / When / Then
        assert!(!NbtTagType::End.is_array());
        assert!(!NbtTagType::Byte.is_array());
        assert!(!NbtTagType::List.is_array());
        assert!(NbtTagType::ByteArray.is_array());
        assert!(NbtTagType::IntArray.is_array());
        assert!(NbtTagType::LongArray.is_array());
    }

    #[test]
    fn given_all_variants_when_debug_then_all_printable() {
        // Given
        let variants = [
            NbtTagType::End,
            NbtTagType::Byte,
            NbtTagType::Short,
            NbtTagType::Int,
            NbtTagType::Long,
            NbtTagType::Float,
            NbtTagType::Double,
            NbtTagType::ByteArray,
            NbtTagType::String,
            NbtTagType::List,
            NbtTagType::Compound,
            NbtTagType::IntArray,
            NbtTagType::LongArray,
        ];

        // When / Then
        for variant in variants {
            let _ = format!("{:?}", variant);
        }
    }

    #[test]
    fn given_tag_type_when_copy_then_works() {
        // Given
        let original = NbtTagType::Int;

        // When
        let copied = original;

        // Then
        assert_eq!(copied, NbtTagType::Int);
    }

    #[test]
    fn given_tag_type_when_partial_eq_then_works() {
        // Given / When / Then
        assert_eq!(NbtTagType::Int, NbtTagType::Int);
        assert_ne!(NbtTagType::Int, NbtTagType::Long);
    }
}
