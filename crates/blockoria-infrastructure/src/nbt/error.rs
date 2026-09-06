// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Error types for LE-NBT parsing.

use std::io;
use thiserror::Error;

/// Errors that can occur during LE-NBT parsing.
#[derive(Debug, Error)]
pub enum NbtError {
    /// Unexpected end of file reached while reading.
    #[error("Unexpected EOF at offset {offset}")]
    UnexpectedEof {
        /// Byte offset where EOF was encountered.
        offset: u64,
    },

    /// Unknown or unsupported tag ID encountered.
    #[error("Unknown tag ID {id} at offset {offset}")]
    UnknownTag {
        /// The unknown tag ID.
        id: u8,
        /// Byte offset where the unknown tag was found.
        offset: u64,
    },

    /// Invalid UTF-8 sequence in a TAG_String.
    #[error("Invalid UTF-8 in string at offset {offset}: {source}")]
    InvalidUtf8 {
        /// Byte offset where the invalid UTF-8 was found.
        offset: u64,
        /// The underlying UTF-8 error.
        #[source]
        source: std::str::Utf8Error,
    },

    /// Negative length value for array, list, or string.
    #[error("Negative length {len} for {context} at offset {offset}")]
    NegativeLength {
        /// The negative length value.
        len: i32,
        /// Context description (e.g., "ByteArray", "List", "String").
        context: &'static str,
        /// Byte offset where the negative length was found.
        offset: u64,
    },

    /// Length exceeds maximum allowed for the given context.
    #[error("Excessive length {len} for {context} at offset {offset} (max {max})")]
    ExcessiveLength {
        /// The excessive length value.
        len: i32,
        /// Context description (e.g., "ByteArray", "List", "Compound", "String").
        context: &'static str,
        /// Maximum allowed length for this context.
        max: usize,
        /// Byte offset where the excessive length was found.
        offset: u64,
    },

    /// Maximum nesting depth exceeded during recursive parsing.
    #[error("Maximum nesting depth ({max}) exceeded at offset {offset}")]
    MaxDepthExceeded {
        /// The maximum allowed nesting depth.
        max: usize,
        /// Byte offset where the limit was exceeded.
        offset: u64,
    },

    /// I/O error during reading.
    #[error("IO error at offset {offset}: {source}")]
    Io {
        /// Byte offset where the I/O error occurred.
        offset: u64,
        /// The underlying I/O error.
        #[source]
        source: io::Error,
    },

    /// Invalid level.dat header.
    #[error("Invalid level.dat header: {reason}")]
    InvalidHeader {
        /// Description of why the header is invalid.
        reason: String,
    },
}

impl NbtError {
    /// Creates an `UnexpectedEof` error with the current offset.
    pub fn unexpected_eof(offset: u64) -> Self {
        Self::UnexpectedEof { offset }
    }

    /// Creates an `UnknownTag` error with the current offset.
    pub fn unknown_tag(id: u8, offset: u64) -> Self {
        Self::UnknownTag { id, offset }
    }

    /// Creates an `InvalidUtf8` error with the current offset.
    pub fn invalid_utf8(offset: u64, source: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8 { offset, source }
    }

    /// Creates a `NegativeLength` error with the current offset.
    pub fn negative_length(len: i32, context: &'static str, offset: u64) -> Self {
        Self::NegativeLength {
            len,
            context,
            offset,
        }
    }

    /// Creates an `ExcessiveLength` error with the current offset.
    pub fn excessive_length(len: i32, context: &'static str, max: usize, offset: u64) -> Self {
        Self::ExcessiveLength {
            len,
            context,
            max,
            offset,
        }
    }

    /// Creates a `MaxDepthExceeded` error with the current offset.
    pub fn max_depth_exceeded(max: usize, offset: u64) -> Self {
        Self::MaxDepthExceeded { max, offset }
    }

    /// Creates an `Io` error with the current offset.
    pub fn io(offset: u64, source: io::Error) -> Self {
        Self::Io { offset, source }
    }

    /// Creates an `InvalidHeader` error.
    pub fn invalid_header(reason: impl Into<String>) -> Self {
        Self::InvalidHeader {
            reason: reason.into(),
        }
    }
}

/// Extension trait providing constructor methods for `NbtError`.
///
/// This trait is implemented for `NbtError` to provide ergonomic
/// construction of error variants with offset tracking.
pub trait NbtErrorExt {
    fn unexpected_eof(offset: u64) -> NbtError;
    fn unknown_tag(id: u8, offset: u64) -> NbtError;
    fn invalid_utf8(offset: u64, source: std::str::Utf8Error) -> NbtError;
    fn negative_length(len: i32, context: &'static str, offset: u64) -> NbtError;
    fn excessive_length(len: i32, context: &'static str, max: usize, offset: u64) -> NbtError;
    fn max_depth_exceeded(max: usize, offset: u64) -> NbtError;
    fn io(offset: u64, source: std::io::Error) -> NbtError;
    fn invalid_header(reason: impl Into<String>) -> NbtError;
}

impl NbtErrorExt for NbtError {
    fn unexpected_eof(offset: u64) -> NbtError {
        NbtError::UnexpectedEof { offset }
    }

    fn unknown_tag(id: u8, offset: u64) -> NbtError {
        NbtError::UnknownTag { id, offset }
    }

    fn invalid_utf8(offset: u64, source: std::str::Utf8Error) -> NbtError {
        NbtError::InvalidUtf8 { offset, source }
    }

    fn negative_length(len: i32, context: &'static str, offset: u64) -> NbtError {
        NbtError::NegativeLength {
            len,
            context,
            offset,
        }
    }

    fn excessive_length(len: i32, context: &'static str, max: usize, offset: u64) -> NbtError {
        NbtError::ExcessiveLength {
            len,
            context,
            max,
            offset,
        }
    }

    fn max_depth_exceeded(max: usize, offset: u64) -> NbtError {
        NbtError::MaxDepthExceeded { max, offset }
    }

    fn io(offset: u64, source: std::io::Error) -> NbtError {
        NbtError::Io { offset, source }
    }

    fn invalid_header(reason: impl Into<String>) -> NbtError {
        NbtError::InvalidHeader {
            reason: reason.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Given / When / Then

    #[test]
    fn given_offset_when_unexpected_eof_then_error_contains_offset() {
        // Given
        let offset = 42u64;

        // When
        let err = NbtError::unexpected_eof(offset);

        // Then
        assert!(matches!(err, NbtError::UnexpectedEof { offset: o } if o == 42));
        assert_eq!(err.to_string(), "Unexpected EOF at offset 42");
    }

    #[test]
    fn given_tag_id_and_offset_when_unknown_tag_then_error_contains_both() {
        // Given
        let id = 99u8;
        let offset = 100u64;

        // When
        let err = NbtError::unknown_tag(id, offset);

        // Then
        assert!(matches!(err, NbtError::UnknownTag { id: i, offset: o } if i == 99 && o == 100));
        assert_eq!(err.to_string(), "Unknown tag ID 99 at offset 100");
    }

    #[test]
    fn given_utf8_error_and_offset_when_invalid_utf8_then_error_contains_both() {
        // Given
        let offset = 200u64;
        // Create invalid UTF-8 dynamically to avoid compile-time warning
        let invalid_utf8 = vec![0xFF, 0xFE];
        let source = std::str::from_utf8(&invalid_utf8).unwrap_err();

        // When
        let err = NbtError::invalid_utf8(offset, source);

        // Then
        assert!(matches!(err, NbtError::InvalidUtf8 { offset: o, .. } if o == 200));
        assert!(err.to_string().contains("Invalid UTF-8"));
        assert!(err.to_string().contains("200"));
    }

    #[test]
    fn given_negative_len_context_and_offset_when_negative_length_then_error_contains_all() {
        // Given
        let len = -10i32;
        let context = "List";
        let offset = 300u64;

        // When
        let err = NbtError::negative_length(len, context, offset);

        // Then
        assert!(
            matches!(err, NbtError::NegativeLength { len: l, context: c, offset: o } if l == -10 && c == "List" && o == 300)
        );
        assert_eq!(
            err.to_string(),
            "Negative length -10 for List at offset 300"
        );
    }

    #[test]
    fn given_excessive_len_context_max_and_offset_when_excessive_length_then_error_contains_all() {
        // Given
        let len = 2_000_000i32;
        let context = "String";
        let max = 1_000_000usize;
        let offset = 400u64;

        // When
        let err = NbtError::excessive_length(len, context, max, offset);

        // Then
        assert!(
            matches!(err, NbtError::ExcessiveLength { len: l, context: c, max: m, offset: o } if l == 2_000_000 && c == "String" && m == 1_000_000 && o == 400)
        );
        assert_eq!(
            err.to_string(),
            "Excessive length 2000000 for String at offset 400 (max 1000000)"
        );
    }

    #[test]
    fn given_max_depth_and_offset_when_max_depth_exceeded_then_error_contains_both() {
        // Given
        let max = 128usize;
        let offset = 500u64;

        // When
        let err = NbtError::max_depth_exceeded(max, offset);

        // Then
        assert!(
            matches!(err, NbtError::MaxDepthExceeded { max: m, offset: o } if m == 128 && o == 500)
        );
        assert_eq!(
            err.to_string(),
            "Maximum nesting depth (128) exceeded at offset 500"
        );
    }

    #[test]
    fn given_io_error_and_offset_when_io_then_error_contains_both() {
        // Given
        let offset = 600u64;
        let source = io::Error::new(io::ErrorKind::UnexpectedEof, "test error");

        // When
        let err = NbtError::io(offset, source);

        // Then
        assert!(matches!(err, NbtError::Io { offset: o, .. } if o == 600));
        assert!(err.to_string().contains("IO error at offset 600"));
    }

    #[test]
    fn given_reason_when_invalid_header_then_error_contains_reason() {
        // Given
        let reason = "nbt_size negative";

        // When
        let err = NbtError::invalid_header(reason);
        let err_str = err.to_string();

        // Then
        assert!(matches!(err, NbtError::InvalidHeader { reason: r } if r == "nbt_size negative"));
        assert_eq!(err_str, "Invalid level.dat header: nbt_size negative");
    }

    #[test]
    fn given_all_variants_when_debug_then_all_printable() {
        // Given
        let invalid_utf8 = vec![0xFF];
        let errors = vec![
            NbtError::unexpected_eof(0),
            NbtError::unknown_tag(0, 0),
            NbtError::invalid_utf8(0, std::str::from_utf8(&invalid_utf8).unwrap_err()),
            NbtError::negative_length(-1, "test", 0),
            NbtError::excessive_length(100, "test", 50, 0),
            NbtError::max_depth_exceeded(10, 0),
            NbtError::io(0, io::Error::other("test")),
            NbtError::invalid_header("test"),
        ];

        // When / Then - all should be printable via Debug
        for err in errors {
            let _ = format!("{:?}", err);
        }
    }
}
