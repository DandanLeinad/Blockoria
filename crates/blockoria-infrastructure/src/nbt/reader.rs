// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 DandanLeinad

//! Little-endian binary reader with safety limits for LE-NBT parsing.

use crate::NbtError;
use std::io::{self, BufReader, Read};

/// A little-endian binary reader with offset tracking and safety limits.
///
/// Wraps a `BufReader` to provide typed reading methods for LE-NBT values.
/// Tracks byte offset for precise error reporting.
/// All numeric reads use little-endian byte order.
pub struct LeReader<R: Read> {
    reader: BufReader<R>,
    offset: u64,
}

impl<R: Read> LeReader<R> {
    /// Creates a new `LeReader` wrapping the given reader.
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            offset: 0,
        }
    }

    /// Returns the current byte offset from the start of the stream.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// Returns a reference to the inner reader.
    pub fn inner(&self) -> &BufReader<R> {
        &self.reader
    }

    /// Returns a mutable reference to the inner reader.
    pub fn inner_mut(&mut self) -> &mut BufReader<R> {
        &mut self.reader
    }

    /// Reads exactly `buf.len()` bytes, updating offset.
    /// Returns `NbtError::UnexpectedEof` if EOF is reached early.
    /// Returns `NbtError::Io` for other I/O errors.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), NbtError> {
        self.reader.read_exact(buf).map_err(|e| {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                NbtError::unexpected_eof(self.offset)
            } else {
                NbtError::io(self.offset, e)
            }
        })?;
        self.offset += buf.len() as u64;
        Ok(())
    }

    /// Reads a single signed byte (TAG_Byte).
    pub fn read_i8(&mut self) -> Result<i8, NbtError> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(i8::from_le_bytes(buf))
    }

    /// Reads a single unsigned byte (used for tag IDs and list subtypes).
    pub fn read_u8(&mut self) -> Result<u8, NbtError> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }

    /// Reads a signed 16-bit integer (TAG_Short, little-endian).
    pub fn read_i16(&mut self) -> Result<i16, NbtError> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(i16::from_le_bytes(buf))
    }

    /// Reads an unsigned 16-bit integer (string length, little-endian).
    pub fn read_u16(&mut self) -> Result<u16, NbtError> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Reads a signed 32-bit integer (TAG_Int, array/list lengths, little-endian).
    pub fn read_i32(&mut self) -> Result<i32, NbtError> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }

    /// Reads a signed 64-bit integer (TAG_Long, little-endian).
    pub fn read_i64(&mut self) -> Result<i64, NbtError> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }

    /// Reads a 32-bit float (TAG_Float, little-endian).
    pub fn read_f32(&mut self) -> Result<f32, NbtError> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(f32::from_le_bytes(buf))
    }

    /// Reads a 64-bit double (TAG_Double, little-endian).
    pub fn read_f64(&mut self) -> Result<f64, NbtError> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }

    /// Reads a UTF-8 string with u16 length prefix (TAG_String).
    ///
    /// Validates length against MAX_STRING_LENGTH to prevent OOM.
    pub fn read_string(&mut self) -> Result<String, NbtError> {
        const MAX_STRING_LENGTH: usize = 1_000_000;

        let len = self.read_u16()? as usize;
        if len > MAX_STRING_LENGTH {
            return Err(NbtError::excessive_length(
                len as i32,
                "String",
                MAX_STRING_LENGTH,
                self.offset,
            ));
        }

        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;

        String::from_utf8(buf).map_err(|e| NbtError::invalid_utf8(self.offset, e.utf8_error()))
    }

    /// Reads a byte array with i32 length prefix (TAG_Byte_Array).
    ///
    /// Validates length against MAX_ARRAY_LENGTH to prevent OOM.
    pub fn read_byte_array(&mut self) -> Result<Vec<u8>, NbtError> {
        const MAX_ARRAY_LENGTH: usize = 10_000_000;

        let len = self.read_i32()?;
        if len < 0 {
            return Err(NbtError::negative_length(len, "ByteArray", self.offset));
        }
        let len = len as usize;
        if len > MAX_ARRAY_LENGTH {
            return Err(NbtError::excessive_length(
                len as i32,
                "ByteArray",
                MAX_ARRAY_LENGTH,
                self.offset,
            ));
        }

        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads an int array with i32 length prefix (TAG_Int_Array).
    ///
    /// Validates length against MAX_ARRAY_LENGTH to prevent OOM.
    pub fn read_int_array(&mut self) -> Result<Vec<i32>, NbtError> {
        const MAX_ARRAY_LENGTH: usize = 10_000_000;

        let len = self.read_i32()?;
        if len < 0 {
            return Err(NbtError::negative_length(len, "IntArray", self.offset));
        }
        let len = len as usize;
        if len > MAX_ARRAY_LENGTH {
            return Err(NbtError::excessive_length(
                len as i32,
                "IntArray",
                MAX_ARRAY_LENGTH,
                self.offset,
            ));
        }

        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.read_i32()?);
        }
        Ok(result)
    }

    /// Reads a long array with i32 length prefix (TAG_Long_Array).
    ///
    /// Validates length against MAX_ARRAY_LENGTH to prevent OOM.
    pub fn read_long_array(&mut self) -> Result<Vec<i64>, NbtError> {
        const MAX_ARRAY_LENGTH: usize = 10_000_000;

        let len = self.read_i32()?;
        if len < 0 {
            return Err(NbtError::negative_length(len, "LongArray", self.offset));
        }
        let len = len as usize;
        if len > MAX_ARRAY_LENGTH {
            return Err(NbtError::excessive_length(
                len as i32,
                "LongArray",
                MAX_ARRAY_LENGTH,
                self.offset,
            ));
        }

        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(self.read_i64()?);
        }
        Ok(result)
    }

    /// Reads exactly `count` bytes into a new vector.
    pub fn read_bytes(&mut self, count: usize) -> Result<Vec<u8>, NbtError> {
        let mut buf = vec![0u8; count];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Skips exactly `count` bytes (for seeking past unknown data).
    pub fn skip_bytes(&mut self, count: usize) -> Result<(), NbtError> {
        let mut buf = vec![0u8; count];
        self.read_exact(&mut buf)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Given / When / Then

    #[test]
    fn given_valid_i32_le_when_read_i32_then_returns_correct_value() {
        // Given: 0x12345678 in little-endian = [0x78, 0x56, 0x34, 0x12]
        let data = vec![0x78, 0x56, 0x34, 0x12];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_i32();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x12345678);
    }

    #[test]
    fn given_valid_i16_le_when_read_i16_then_returns_correct_value() {
        // Given
        let data = vec![0x34, 0x12]; // 0x1234
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_i16();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x1234);
    }

    #[test]
    fn given_valid_i64_le_when_read_i64_then_returns_correct_value() {
        // Given
        let data = vec![0x78, 0x56, 0x34, 0x12, 0x00, 0x00, 0x00, 0x00];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_i64();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x12345678);
    }

    #[test]
    fn given_valid_u16_le_when_read_u16_then_returns_correct_value() {
        // Given
        let data = vec![0x34, 0x12];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_u16();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x1234);
    }

    #[test]
    fn given_valid_f32_le_when_read_f32_then_returns_correct_value() {
        // Given: 1.0 in little-endian
        let data = 1.0f32.to_le_bytes().to_vec();
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_f32();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1.0);
    }

    #[test]
    fn given_valid_f64_le_when_read_f64_then_returns_correct_value() {
        // Given: 2.0 in little-endian
        let data = 2.0f64.to_le_bytes().to_vec();
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_f64();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn given_string_with_length_when_read_string_then_returns_string() {
        // Given: length=5 (0x05, 0x00) + "Hello"
        let mut data = vec![0x05, 0x00];
        data.extend_from_slice(b"Hello");
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_string();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn given_empty_string_when_read_string_then_returns_empty() {
        // Given: length=0
        let data = vec![0x00, 0x00];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_string();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn given_max_u16_string_when_read_string_then_works() {
        // Given: length = u16::MAX (65535) - maximum possible in NBT format
        let len = u16::MAX;
        let mut data = len.to_le_bytes().to_vec();
        data.extend(vec![b'A'; len as usize]);
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_string();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), len as usize);
    }

    #[test]
    fn given_invalid_utf8_when_read_string_then_error() {
        // Given: length=2 + invalid UTF-8
        let mut data = vec![0x02, 0x00];
        data.extend_from_slice(&[0xFF, 0xFE]);
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_string();

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::InvalidUtf8 { .. })));
    }

    #[test]
    fn given_byte_array_when_read_byte_array_then_returns_vec() {
        // Given: length=3 + [1, 2, 3]
        let mut data = vec![0x03, 0x00, 0x00, 0x00];
        data.extend_from_slice(&[1, 2, 3]);
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_byte_array();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn given_negative_byte_array_length_when_read_byte_array_then_error() {
        // Given: length=-1
        let data = (-1i32).to_le_bytes().to_vec();
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_byte_array();

        // Then
        assert!(result.is_err());
        assert!(
            matches!(result, Err(NbtError::NegativeLength { context, .. }) if context == "ByteArray")
        );
    }

    #[test]
    fn given_byte_array_exceeding_max_length_when_read_byte_array_then_error() {
        // Given: length > MAX_ARRAY_LENGTH
        let len = 10_000_001i32;
        let mut data = len.to_le_bytes().to_vec();
        data.extend(vec![0u8; 10_000_001]);
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_byte_array();

        // Then
        assert!(result.is_err());
        assert!(
            matches!(result, Err(NbtError::ExcessiveLength { context, .. }) if context == "ByteArray")
        );
    }

    #[test]
    fn given_int_array_when_read_int_array_then_returns_vec() {
        // Given: length=3 + [1, 2, 3]
        let mut data = vec![0x03, 0x00, 0x00, 0x00];
        data.extend_from_slice(&1i32.to_le_bytes());
        data.extend_from_slice(&2i32.to_le_bytes());
        data.extend_from_slice(&3i32.to_le_bytes());
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_int_array();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn given_long_array_when_read_long_array_then_returns_vec() {
        // Given: length=2 + [100, 200]
        let mut data = vec![0x02, 0x00, 0x00, 0x00];
        data.extend_from_slice(&100i64.to_le_bytes());
        data.extend_from_slice(&200i64.to_le_bytes());
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_long_array();

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![100, 200]);
    }

    #[test]
    fn given_eof_when_read_i32_then_unexpected_eof_error() {
        // Given: only 2 bytes
        let data = vec![0x78, 0x56];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_i32();

        // Then
        assert!(result.is_err());
        assert!(matches!(result, Err(NbtError::UnexpectedEof { offset }) if offset == 0));
    }

    #[test]
    fn given_offset_tracking_when_reads_then_offset_updated() {
        // Given
        let data = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let _ = reader.read_i32();
        let _ = reader.read_i32();

        // Then
        assert_eq!(reader.offset(), 8);
    }

    #[test]
    fn given_bytes_read_when_read_bytes_then_returns_vec() {
        // Given
        let data = vec![1, 2, 3, 4, 5];
        let mut reader = LeReader::new(Cursor::new(data));

        // When
        let result = reader.read_bytes(3);

        // Then
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }
}
