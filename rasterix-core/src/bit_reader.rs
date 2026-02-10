use std::io::{self, Read};

/// Reads individual bits from a byte-oriented [`Read`] source.
///
/// Bits are consumed MSB-first within each byte.  New bytes are fetched from
/// the underlying reader on demand, so the reader is never read ahead of what
/// is needed.
///
/// The struct also implements [`Read`] for byte-level access, but only when
/// the internal bit buffer is empty (i.e. [`is_byte_aligned`](Self::is_byte_aligned)
/// returns `true`).
#[derive(Debug)]
pub struct BitReader<R: Read> {
    reader: R,
    buffer: u8,
    bits_left: u8,
}

impl<R: Read> BitReader<R> {
    /// Wraps an existing reader for bit-level access.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: 0,
            bits_left: 0,
        }
    }

    /// Reads up to 64 bits and returns them right-aligned in a `u64`.
    ///
    /// Bits are read MSB-first: the first bit read becomes the most
    /// significant bit of the returned value.
    ///
    /// Returns an I/O error if the underlying reader runs out of data before
    /// `count` bits have been consumed.
    pub fn read_bits(&mut self, count: usize) -> io::Result<u64> {
        let mut value = 0u64;

        for _ in 0..count {
            if self.bits_left == 0 {
                let mut byte = [0u8];
                self.reader.read_exact(&mut byte)?;
                self.buffer = byte[0];
                self.bits_left = 8;
            }

            self.bits_left -= 1;
            let bit = (self.buffer >> self.bits_left) & 1;
            value = (value << 1) | bit as u64;
        }

        Ok(value)
    }

    /// Reads a fixed-length string field from the stream.
    ///
    /// Reads `byte_len` bytes, interprets them as ASCII/UTF-8, and trims
    /// trailing spaces and null bytes. This is used for ASTERIX string fields
    /// such as callsigns and target identifications.
    pub fn read_string(&mut self, byte_len: usize) -> io::Result<String> {
        let mut bytes = vec![0u8; byte_len];
        for byte in bytes.iter_mut() {
            *byte = self.read_bits(8)? as u8;
        }
        let s = String::from_utf8_lossy(&bytes);
        Ok(s.trim_end_matches(|c: char| c == ' ' || c == '\0').to_string())
    }

    /// Returns true if the reader is at a byte boundary (no partial byte buffered).
    pub fn is_byte_aligned(&self) -> bool {
        self.bits_left == 0
    }
}

/// Implement Read for BitReader to allow byte-level operations.
/// Note: This only works correctly when the reader is at a byte boundary.
impl<R: Read> Read for BitReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If we have partial bits buffered, we need to handle them
        // For now, assert byte alignment for simplicity
        debug_assert!(
            self.bits_left == 0,
            "BitReader::read called with {} bits still buffered",
            self.bits_left
        );
        self.reader.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn new_creates_empty_reader() {
        let data = vec![0xAB];
        let reader = BitReader::new(Cursor::new(data));
        assert!(reader.is_byte_aligned());
    }

    #[test]
    fn read_single_bit() {
        // 0b10101010 = 0xAA
        let data = vec![0xAA];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(1).unwrap(), 1); // First bit is 1
        assert_eq!(reader.read_bits(1).unwrap(), 0); // Second bit is 0
        assert_eq!(reader.read_bits(1).unwrap(), 1); // Third bit is 1
        assert_eq!(reader.read_bits(1).unwrap(), 0); // Fourth bit is 0
    }

    #[test]
    fn read_full_byte() {
        let data = vec![0xAB, 0xCD];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(8).unwrap(), 0xAB);
        assert!(reader.is_byte_aligned());
        assert_eq!(reader.read_bits(8).unwrap(), 0xCD);
    }

    #[test]
    fn read_across_byte_boundary() {
        // Read 12 bits from 0xAB 0xCD = 0b10101011 0b11001101
        // First 12 bits: 0b101010111100 = 0xABC
        let data = vec![0xAB, 0xCD];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(12).unwrap(), 0xABC);
        assert!(!reader.is_byte_aligned());
    }

    #[test]
    fn read_multiple_sizes() {
        // 0xFF = 0b11111111
        let data = vec![0xFF];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(3).unwrap(), 0b111); // 7
        assert_eq!(reader.read_bits(3).unwrap(), 0b111); // 7
        assert_eq!(reader.read_bits(2).unwrap(), 0b11);  // 3
        assert!(reader.is_byte_aligned());
    }

    #[test]
    fn read_16_bits() {
        let data = vec![0x12, 0x34];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(16).unwrap(), 0x1234);
    }

    #[test]
    fn read_32_bits() {
        let data = vec![0x12, 0x34, 0x56, 0x78];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(32).unwrap(), 0x12345678);
    }

    #[test]
    fn read_zero_bits() {
        let data = vec![0xAB];
        let mut reader = BitReader::new(Cursor::new(data));

        assert_eq!(reader.read_bits(0).unwrap(), 0);
        assert!(reader.is_byte_aligned()); // No data consumed
    }

    #[test]
    fn byte_alignment_tracking() {
        let data = vec![0xFF, 0xFF];
        let mut reader = BitReader::new(Cursor::new(data));

        assert!(reader.is_byte_aligned());
        reader.read_bits(1).unwrap();
        assert!(!reader.is_byte_aligned());
        reader.read_bits(7).unwrap();
        assert!(reader.is_byte_aligned());
    }

    #[test]
    fn read_trait_at_byte_boundary() {
        let data = vec![0xAB, 0xCD, 0xEF];
        let mut reader = BitReader::new(Cursor::new(data));

        // Read first byte using bit reader
        assert_eq!(reader.read_bits(8).unwrap(), 0xAB);

        // Now use Read trait for remaining bytes
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [0xCD, 0xEF]);
    }

    #[test]
    fn read_string_basic() {
        // "ABC" as bytes, followed by spaces
        let data = vec![0x41, 0x42, 0x43, 0x20, 0x20];
        let mut reader = BitReader::new(Cursor::new(data));

        let s = reader.read_string(5).unwrap();
        assert_eq!(s, "ABC");
    }

    #[test]
    fn read_string_no_padding() {
        let data = vec![0x41, 0x42, 0x43]; // "ABC"
        let mut reader = BitReader::new(Cursor::new(data));

        let s = reader.read_string(3).unwrap();
        assert_eq!(s, "ABC");
    }

    #[test]
    fn read_string_with_null_padding() {
        let data = vec![0x41, 0x42, 0x00, 0x00]; // "AB\0\0"
        let mut reader = BitReader::new(Cursor::new(data));

        let s = reader.read_string(4).unwrap();
        assert_eq!(s, "AB");
    }

    #[test]
    fn read_insufficient_data() {
        let data = vec![0xAB];
        let mut reader = BitReader::new(Cursor::new(data));

        // Try to read more bits than available
        assert_eq!(reader.read_bits(8).unwrap(), 0xAB);
        assert!(reader.read_bits(8).is_err());
    }

    #[test]
    fn read_alternating_pattern() {
        // 0b01010101 = 0x55
        // Reading MSB first: 0, 1, 0, 1, 0, 1, 0, 1
        let data = vec![0x55];
        let mut reader = BitReader::new(Cursor::new(data));

        for i in 0..8 {
            let bit = reader.read_bits(1).unwrap();
            let expected = i % 2;
            assert_eq!(bit, expected as u64, "Bit {} should be {}", i, expected);
        }
    }
}
