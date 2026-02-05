//! Round-trip test infrastructure.
//!
//! This module provides utilities for testing encode/decode round-trips
//! of generated ASTERIX data structures.
//!
//! The round-trip tests verify that:
//! 1. decode(encode(value)) == value (struct round-trip)
//! 2. encode(decode(bytes)) == bytes (bytes round-trip)

#![allow(dead_code)]

use rusterix_core::{BitReader, BitWriter, Decode, Encode};
use std::fmt::Debug;
use std::io::Cursor;

/// Performs a struct round-trip test: encode a value, then decode it back.
///
/// Returns the decoded value for further assertions.
pub fn roundtrip_struct<T>(original: &T) -> T
where
    T: Encode + Decode + Clone + Debug + PartialEq,
{
    // Encode
    let mut buffer = Vec::new();
    {
        let mut writer = BitWriter::new(&mut buffer);
        original.encode(&mut writer).expect("Encode failed");
        writer.flush().expect("Flush failed");
    }

    // Decode
    let mut reader = BitReader::new(Cursor::new(&buffer));
    let decoded = T::decode(&mut reader).expect("Decode failed");

    decoded
}

/// Performs a struct round-trip test and asserts equality.
pub fn assert_roundtrip<T>(original: &T)
where
    T: Encode + Decode + Clone + Debug + PartialEq,
{
    let decoded = roundtrip_struct(original);
    assert_eq!(
        original, &decoded,
        "Round-trip failed: original != decoded"
    );
}

/// Performs a bytes round-trip test: decode bytes, then encode back.
///
/// Returns the re-encoded bytes for comparison.
pub fn roundtrip_bytes<T>(bytes: &[u8]) -> Vec<u8>
where
    T: Encode + Decode,
{
    // Decode
    let mut reader = BitReader::new(Cursor::new(bytes));
    let value = T::decode(&mut reader).expect("Decode failed");

    // Re-encode
    let mut buffer = Vec::new();
    {
        let mut writer = BitWriter::new(&mut buffer);
        value.encode(&mut writer).expect("Encode failed");
        writer.flush().expect("Flush failed");
    }

    buffer
}

/// Performs a bytes round-trip test and asserts equality.
pub fn assert_bytes_roundtrip<T>(bytes: &[u8])
where
    T: Encode + Decode,
{
    let reencoded = roundtrip_bytes::<T>(bytes);
    assert_eq!(
        bytes, &reencoded[..],
        "Bytes round-trip failed: original bytes != re-encoded bytes"
    );
}

/// Test helper for generating random-ish test values.
pub mod generators {
    /// Generates test u8 values covering edge cases.
    pub fn u8_test_values() -> Vec<u8> {
        vec![0, 1, 127, 128, 254, 255]
    }

    /// Generates test u16 values covering edge cases.
    pub fn u16_test_values() -> Vec<u16> {
        vec![0, 1, 255, 256, 32767, 32768, 65534, 65535]
    }

    /// Generates test u32 values covering edge cases.
    pub fn u32_test_values() -> Vec<u32> {
        vec![0, 1, 255, 65535, 0x7FFFFFFF, 0x80000000, 0xFFFFFFFE, 0xFFFFFFFF]
    }
}
