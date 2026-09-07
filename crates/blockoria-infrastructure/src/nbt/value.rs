// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! NBT Value AST (Abstract Syntax Tree) for parsed LE-NBT data.
//!
//! Represents the complete parsed structure of an NBT document.

use crate::NbtError;
use crate::nbt::NbtTagType;
use std::collections::BTreeMap;

/// Wrapper for TAG_List that preserves element_type even for empty lists.
/// This is required by the NBT specification where TAG_List always has a subtype.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NbtList {
    /// The type of elements in this list (all elements have the same type).
    pub element_type: NbtTagType,
    /// The list elements.
    pub values: Vec<NbtValue>,
}

impl NbtList {
    /// Creates a new NbtList with the given element type and values.
    pub fn new(element_type: NbtTagType, values: Vec<NbtValue>) -> Self {
        Self {
            element_type,
            values,
        }
    }

    /// Creates an empty NbtList with the given element type.
    pub fn empty(element_type: NbtTagType) -> Self {
        Self {
            element_type,
            values: Vec::new(),
        }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Parsed NBT value - represents any valid NBT tag payload.
#[derive(Debug, Clone)]
pub enum NbtValue {
    /// TAG_Byte - Signed 8-bit integer
    Byte(i8),
    /// TAG_Short - Signed 16-bit integer (little-endian)
    Short(i16),
    /// TAG_Int - Signed 32-bit integer (little-endian)
    Int(i32),
    /// TAG_Long - Signed 64-bit integer (little-endian)
    Long(i64),
    /// TAG_Float - 32-bit floating point (little-endian)
    Float(f32),
    /// TAG_Double - 64-bit floating point (little-endian)
    Double(f64),
    /// TAG_Byte_Array - Array of bytes
    ByteArray(Vec<u8>),
    /// TAG_String - UTF-8 string
    String(String),
    /// TAG_List - Homogeneous list with preserved element type
    List(NbtList),
    /// TAG_Compound - Map of named tags (ordered for deterministic output)
    Compound(BTreeMap<String, NbtValue>),
    /// TAG_Int_Array - Array of 32-bit integers
    IntArray(Vec<i32>),
    /// TAG_Long_Array - Array of 64-bit integers
    LongArray(Vec<i64>),
}

impl PartialEq for NbtValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NbtValue::Byte(a), NbtValue::Byte(b)) => a == b,
            (NbtValue::Short(a), NbtValue::Short(b)) => a == b,
            (NbtValue::Int(a), NbtValue::Int(b)) => a == b,
            (NbtValue::Long(a), NbtValue::Long(b)) => a == b,
            (NbtValue::Float(a), NbtValue::Float(b)) => a.to_bits() == b.to_bits(),
            (NbtValue::Double(a), NbtValue::Double(b)) => a.to_bits() == b.to_bits(),
            (NbtValue::ByteArray(a), NbtValue::ByteArray(b)) => a == b,
            (NbtValue::String(a), NbtValue::String(b)) => a == b,
            (NbtValue::List(a), NbtValue::List(b)) => a == b,
            (NbtValue::Compound(a), NbtValue::Compound(b)) => a == b,
            (NbtValue::IntArray(a), NbtValue::IntArray(b)) => a == b,
            (NbtValue::LongArray(a), NbtValue::LongArray(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for NbtValue {}

impl NbtValue {
    /// Returns the tag type of this value.
    pub fn tag_type(&self) -> NbtTagType {
        match self {
            NbtValue::Byte(_) => NbtTagType::Byte,
            NbtValue::Short(_) => NbtTagType::Short,
            NbtValue::Int(_) => NbtTagType::Int,
            NbtValue::Long(_) => NbtTagType::Long,
            NbtValue::Float(_) => NbtTagType::Float,
            NbtValue::Double(_) => NbtTagType::Double,
            NbtValue::ByteArray(_) => NbtTagType::ByteArray,
            NbtValue::String(_) => NbtTagType::String,
            NbtValue::List(_) => NbtTagType::List,
            NbtValue::Compound(_) => NbtTagType::Compound,
            NbtValue::IntArray(_) => NbtTagType::IntArray,
            NbtValue::LongArray(_) => NbtTagType::LongArray,
        }
    }

    /// Returns true if this value is a container (Compound or List).
    pub fn is_container(&self) -> bool {
        matches!(self, NbtValue::Compound(_) | NbtValue::List(_))
    }

    /// Attempts to get this value as a Compound.
    pub fn as_compound(&self) -> Option<&BTreeMap<String, NbtValue>> {
        match self {
            NbtValue::Compound(map) => Some(map),
            _ => None,
        }
    }

    /// Attempts to get this value as a Compound mutably.
    pub fn as_compound_mut(&mut self) -> Option<&mut BTreeMap<String, NbtValue>> {
        match self {
            NbtValue::Compound(map) => Some(map),
            _ => None,
        }
    }

    /// Attempts to get this value as a List.
    pub fn as_list(&self) -> Option<&NbtList> {
        match self {
            NbtValue::List(list) => Some(list),
            _ => None,
        }
    }

    /// Attempts to get a named value from a Compound.
    pub fn get(&self, key: &str) -> Option<&NbtValue> {
        self.as_compound()?.get(key)
    }

    /// Attempts to get a named value mutably from a Compound.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtValue> {
        self.as_compound_mut()?.get_mut(key)
    }

    /// Inserts a named value into a Compound.
    /// Returns an error if this value is not a Compound.
    pub fn insert(&mut self, key: String, value: NbtValue) -> Result<Option<NbtValue>, NbtError> {
        match self {
            NbtValue::Compound(map) => Ok(map.insert(key, value)),
            _ => Err(NbtError::invalid_header(
                "Cannot insert into non-Compound value",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Given / When / Then

    #[test]
    fn given_various_values_when_constructed_then_all_variants_work() {
        // Given / When / Then
        let _ = NbtValue::Byte(42);
        let _ = NbtValue::Short(300);
        let _ = NbtValue::Int(123456);
        let _ = NbtValue::Long(9999999999);
        let _ = NbtValue::Float(std::f32::consts::PI);
        let _ = NbtValue::Double(std::f64::consts::E);
        let _ = NbtValue::ByteArray(vec![1, 2, 3]);
        let _ = NbtValue::String("hello".to_string());
        let _ = NbtValue::List(NbtList::new(
            NbtTagType::Int,
            vec![NbtValue::Int(1), NbtValue::Int(2)],
        ));
        let _ = NbtValue::Compound(BTreeMap::new());
        let _ = NbtValue::IntArray(vec![1, 2, 3]);
        let _ = NbtValue::LongArray(vec![100, 200]);
    }

    #[test]
    fn given_nbt_list_when_created_then_preserves_element_type() {
        // Given
        let list = NbtList::new(NbtTagType::Int, vec![]);

        // When / Then
        assert_eq!(list.element_type, NbtTagType::Int);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn given_empty_nbt_list_when_created_via_empty_then_preserves_type() {
        // Given / When
        let list = NbtList::empty(NbtTagType::Long);

        // Then
        assert_eq!(list.element_type, NbtTagType::Long);
        assert!(list.is_empty());
    }

    #[test]
    fn given_nbt_value_when_tag_type_then_returns_correct_type() {
        // Given / When / Then
        assert_eq!(NbtValue::Byte(1).tag_type(), NbtTagType::Byte);
        assert_eq!(NbtValue::Int(1).tag_type(), NbtTagType::Int);
        assert_eq!(NbtValue::String("x".into()).tag_type(), NbtTagType::String);
        assert_eq!(
            NbtValue::List(NbtList::new(NbtTagType::Int, vec![])).tag_type(),
            NbtTagType::List
        );
        assert_eq!(
            NbtValue::Compound(BTreeMap::new()).tag_type(),
            NbtTagType::Compound
        );
        assert_eq!(NbtValue::IntArray(vec![]).tag_type(), NbtTagType::IntArray);
        assert_eq!(
            NbtValue::LongArray(vec![]).tag_type(),
            NbtTagType::LongArray
        );
    }

    #[test]
    fn given_compound_value_when_as_compound_then_returns_map() {
        // Given
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), NbtValue::Int(42));
        let value = NbtValue::Compound(map);

        // When
        let compound = value.as_compound();

        // Then
        assert!(compound.is_some());
        assert_eq!(compound.unwrap().get("key"), Some(&NbtValue::Int(42)));
    }

    #[test]
    fn given_non_compound_when_as_compound_then_returns_none() {
        // Given
        let value = NbtValue::Int(42);

        // When
        let compound = value.as_compound();

        // Then
        assert!(compound.is_none());
    }

    #[test]
    fn given_list_value_when_as_list_then_returns_list() {
        // Given
        let list = NbtList::new(NbtTagType::Int, vec![NbtValue::Int(1), NbtValue::Int(2)]);
        let value = NbtValue::List(list);

        // When
        let result = value.as_list();

        // Then
        assert!(result.is_some());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn given_compound_when_get_then_returns_value() {
        // Given
        let mut map = BTreeMap::new();
        map.insert("version".to_string(), NbtValue::Int(123));
        let compound = NbtValue::Compound(map);

        // When
        let result = compound.get("version");

        // Then
        assert_eq!(result, Some(&NbtValue::Int(123)));
    }

    #[test]
    fn given_compound_when_get_missing_key_then_returns_none() {
        // Given
        let compound = NbtValue::Compound(BTreeMap::new());

        // When
        let result = compound.get("missing");

        // Then
        assert!(result.is_none());
    }

    #[test]
    fn given_compound_when_insert_then_returns_old_value() {
        // Given
        let mut compound = NbtValue::Compound(BTreeMap::new());

        // When
        let old = compound
            .insert("key".to_string(), NbtValue::Int(1))
            .unwrap();
        let new = compound
            .insert("key".to_string(), NbtValue::Int(2))
            .unwrap();

        // Then
        assert!(old.is_none());
        assert_eq!(new, Some(NbtValue::Int(1)));
        assert_eq!(compound.get("key"), Some(&NbtValue::Int(2)));
    }

    #[test]
    fn given_non_compound_when_insert_then_returns_error() {
        // Given
        let mut value = NbtValue::Int(42);

        // When
        let result = value.insert("key".to_string(), NbtValue::Int(1));

        // Then
        assert!(result.is_err());
    }

    #[test]
    fn given_all_variants_when_debug_then_all_printable() {
        // Given
        let values = vec![
            NbtValue::Byte(1),
            NbtValue::Short(2),
            NbtValue::Int(3),
            NbtValue::Long(4),
            NbtValue::Float(1.0),
            NbtValue::Double(2.0),
            NbtValue::ByteArray(vec![1, 2]),
            NbtValue::String("test".to_string()),
            NbtValue::List(NbtList::empty(NbtTagType::Int)),
            NbtValue::Compound(BTreeMap::new()),
            NbtValue::IntArray(vec![1, 2]),
            NbtValue::LongArray(vec![3, 4]),
        ];

        // When / Then
        for value in values {
            let _ = format!("{:?}", value);
        }
    }

    #[test]
    fn given_nbt_list_when_clone_then_works() {
        // Given
        let list = NbtList::new(NbtTagType::Int, vec![NbtValue::Int(1), NbtValue::Int(2)]);

        // When
        let cloned = list.clone();

        // Then
        assert_eq!(cloned.element_type, NbtTagType::Int);
        assert_eq!(cloned.values.len(), 2);
    }

    #[test]
    fn given_nbt_value_when_partial_eq_then_works() {
        // Given / When / Then
        assert_eq!(NbtValue::Int(42), NbtValue::Int(42));
        assert_ne!(NbtValue::Int(42), NbtValue::Int(43));
        assert_eq!(
            NbtValue::List(NbtList::new(NbtTagType::Int, vec![NbtValue::Int(1)])),
            NbtValue::List(NbtList::new(NbtTagType::Int, vec![NbtValue::Int(1)]))
        );
    }
}
