// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! JSON serialization for NBT values.
//!
//! Provides two serialization modes:
//! - **Simple**: Clean JSON like `{"key": "value"}` for human-readable output
//! - **Typed**: Preserves NBT type info like `{"type": "Int", "value": 42}` for round-trip

use super::{NbtTagType, NbtValue};

/// Serializes NbtValue to JSON in simple mode (clean, human-readable).
///
/// Example: `{"name": "World", "version": [1, 21, 0, 0, 0]}`
pub fn to_json_simple(value: &NbtValue) -> serde_json::Value {
    to_json_simple_inner(value)
}

fn to_json_simple_inner(value: &NbtValue) -> serde_json::Value {
    match value {
        super::NbtValue::Byte(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Short(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Int(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Long(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Float(v) => serde_json::Number::from_f64(*v as f64)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        super::NbtValue::Double(v) => serde_json::Number::from_f64(*v)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        super::NbtValue::ByteArray(arr) => {
            serde_json::Value::Array(arr.iter().map(|b| (*b).into()).collect())
        }
        super::NbtValue::String(s) => serde_json::Value::String(s.clone()),
        super::NbtValue::List(list) => {
            serde_json::Value::Array(list.values.iter().map(to_json_simple_inner).collect())
        }
        super::NbtValue::Compound(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                json_map.insert(k.clone(), to_json_simple_inner(v));
            }
            serde_json::Value::Object(json_map)
        }
        super::NbtValue::IntArray(arr) => {
            serde_json::Value::Array(arr.iter().map(|i| (*i).into()).collect())
        }
        super::NbtValue::LongArray(arr) => {
            serde_json::Value::Array(arr.iter().map(|i| (*i).into()).collect())
        }
    }
}

/// Serializes NbtValue to JSON in typed mode (preserves NBT type info).
///
/// Example: `{"type": "Compound", "value": {"name": {"type": "String", "value": "World"}}}`
pub fn to_json_typed(value: &NbtValue) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert(
        "type".to_string(),
        serde_json::Value::String(value.tag_type_name().to_string()),
    );
    obj.insert("value".to_string(), to_json_typed_inner(value));
    serde_json::Value::Object(obj)
}

fn to_json_typed_inner(value: &NbtValue) -> serde_json::Value {
    match value {
        super::NbtValue::Byte(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Short(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Int(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Long(v) => serde_json::Value::Number((*v).into()),
        super::NbtValue::Float(v) => serde_json::Number::from_f64(*v as f64)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        super::NbtValue::Double(v) => serde_json::Number::from_f64(*v)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        super::NbtValue::ByteArray(arr) => {
            serde_json::Value::Array(arr.iter().map(|b| (*b).into()).collect())
        }
        super::NbtValue::String(s) => serde_json::Value::String(s.clone()),
        super::NbtValue::List(list) => {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "element_type".to_string(),
                serde_json::Value::String(list.element_type.tag_type_name().to_string()),
            );
            obj.insert(
                "value".to_string(),
                serde_json::Value::Array(list.values.iter().map(to_json_typed_inner).collect()),
            );
            serde_json::Value::Object(obj)
        }
        super::NbtValue::Compound(map) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in map {
                json_map.insert(k.clone(), to_json_typed(v));
            }
            serde_json::Value::Object(json_map)
        }
        super::NbtValue::IntArray(arr) => {
            serde_json::Value::Array(arr.iter().map(|i| (*i).into()).collect())
        }
        super::NbtValue::LongArray(arr) => {
            serde_json::Value::Array(arr.iter().map(|i| (*i).into()).collect())
        }
    }
}

impl NbtTagType {
    fn tag_type_name(&self) -> &'static str {
        match self {
            NbtTagType::End => "End",
            NbtTagType::Byte => "Byte",
            NbtTagType::Short => "Short",
            NbtTagType::Int => "Int",
            NbtTagType::Long => "Long",
            NbtTagType::Float => "Float",
            NbtTagType::Double => "Double",
            NbtTagType::ByteArray => "ByteArray",
            NbtTagType::String => "String",
            NbtTagType::List => "List",
            NbtTagType::Compound => "Compound",
            NbtTagType::IntArray => "IntArray",
            NbtTagType::LongArray => "LongArray",
        }
    }
}

impl NbtValue {
    pub fn tag_type_name(&self) -> &'static str {
        match self {
            super::NbtValue::Byte(_) => "Byte",
            super::NbtValue::Short(_) => "Short",
            super::NbtValue::Int(_) => "Int",
            super::NbtValue::Long(_) => "Long",
            super::NbtValue::Float(_) => "Float",
            super::NbtValue::Double(_) => "Double",
            super::NbtValue::ByteArray(_) => "ByteArray",
            super::NbtValue::String(_) => "String",
            super::NbtValue::List(_) => "List",
            super::NbtValue::Compound(_) => "Compound",
            super::NbtValue::IntArray(_) => "IntArray",
            super::NbtValue::LongArray(_) => "LongArray",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{NbtList, NbtValue};
    use super::*;
    use std::collections::BTreeMap;

    // Given / When / Then

    #[test]
    fn given_int_when_to_json_simple_then_returns_number() {
        // Given
        let value = NbtValue::Int(42);

        // When
        let result = to_json_simple(&value);

        // Then
        assert_eq!(result, serde_json::json!(42));
    }

    #[test]
    fn given_string_when_to_json_simple_then_returns_string() {
        // Given
        let value = NbtValue::String("Hello".to_string());

        // When
        let result = to_json_simple(&value);

        // Then
        assert_eq!(result, serde_json::json!("Hello"));
    }

    #[test]
    fn given_compound_when_to_json_simple_then_returns_object() {
        // Given
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), NbtValue::String("World".to_string()));
        map.insert("version".to_string(), NbtValue::Int(1));
        let value = NbtValue::Compound(map);

        // When
        let result = to_json_simple(&value);

        // Then
        assert_eq!(result, serde_json::json!({"name": "World", "version": 1}));
    }

    #[test]
    fn given_list_when_to_json_simple_then_returns_array() {
        // Given
        let list = NbtList::new(
            super::NbtTagType::Int,
            vec![NbtValue::Int(1), NbtValue::Int(2), NbtValue::Int(3)],
        );
        let value = NbtValue::List(list);

        // When
        let result = to_json_simple(&value);

        // Then
        assert_eq!(result, serde_json::json!([1, 2, 3]));
    }

    #[test]
    fn given_int_array_when_to_json_simple_then_returns_array() {
        // Given
        let value = NbtValue::IntArray(vec![1, 21, 0, 0, 0]);

        // When
        let result = to_json_simple(&value);

        // Then
        assert_eq!(result, serde_json::json!([1, 21, 0, 0, 0]));
    }

    #[test]
    fn given_int_when_to_json_typed_then_returns_typed_object() {
        // Given
        let value = NbtValue::Int(42);

        // When
        let result = to_json_typed(&value);

        // Then
        assert_eq!(result, serde_json::json!({"type": "Int", "value": 42}));
    }

    #[test]
    fn given_string_when_to_json_typed_then_returns_typed_object() {
        // Given
        let value = NbtValue::String("Hello".to_string());

        // When
        let result = to_json_typed(&value);

        // Then
        assert_eq!(
            result,
            serde_json::json!({"type": "String", "value": "Hello"})
        );
    }

    #[test]
    fn given_compound_when_to_json_typed_then_returns_typed_object() {
        // Given
        let mut map = BTreeMap::new();
        map.insert("name".to_string(), NbtValue::String("World".to_string()));
        map.insert("version".to_string(), NbtValue::Int(1));
        let value = NbtValue::Compound(map);

        // When
        let result = to_json_typed(&value);

        // Then
        let expected = serde_json::json!({
            "type": "Compound",
            "value": {
                "name": {"type": "String", "value": "World"},
                "version": {"type": "Int", "value": 1}
            }
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn given_list_when_to_json_typed_then_preserves_element_type() {
        // Given
        let list = NbtList::new(
            super::NbtTagType::Int,
            vec![NbtValue::Int(1), NbtValue::Int(2)],
        );
        let value = NbtValue::List(list);

        // When
        let result = to_json_typed(&value);

        // Then
        let expected = serde_json::json!({
            "type": "List",
            "value": {
                "element_type": "Int",
                "value": [1, 2]
            }
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn given_empty_list_when_to_json_typed_then_preserves_element_type() {
        // Given
        let list = NbtList::empty(super::NbtTagType::Long);
        let value = NbtValue::List(list);

        // When
        let result = to_json_typed(&value);

        // Then
        let expected = serde_json::json!({
            "type": "List",
            "value": {
                "element_type": "Long",
                "value": []
            }
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn given_nested_compound_when_to_json_both_then_works() {
        // Given
        let mut inner = BTreeMap::new();
        inner.insert("value".to_string(), NbtValue::Int(42));
        let mut outer = BTreeMap::new();
        outer.insert("inner".to_string(), NbtValue::Compound(inner));
        let value = NbtValue::Compound(outer);

        // When
        let simple = to_json_simple(&value);
        let typed = to_json_typed(&value);

        // Then
        assert_eq!(simple, serde_json::json!({"inner": {"value": 42}}));
        assert!(typed.get("type").unwrap().as_str().unwrap() == "Compound");
    }
}
