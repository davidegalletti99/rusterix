use std::io::{self, Write};

/// Writes individual bits to a byte-oriented [`Write`] sink.
///
/// Bits are accumulated MSB-first into an internal byte buffer and flushed to
/// the underlying writer each time a full byte has been assembled.  Call
/// [`flush`](Self::flush) after the last write to emit any remaining partial
/// byte (padded with zero bits on the right).
///
/// The struct also implements [`Write`] for byte-level access, but only when
/// the internal bit buffer is empty (i.e. [`is_byte_aligned`](Self::is_byte_aligned)
/// returns `true`).
#[derive(Debug)]
pub struct BitWriter<W: Write> {
    writer: W,
    buffer: u8,
    bits_filled: u8,
}

impl<W: Write> BitWriter<W> {
    /// Wraps an existing writer for bit-level access.
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: 0,
            bits_filled: 0,
        }
    }

    /// Writes the lowest `count` bits of `value`, MSB-first.
    ///
    /// Full bytes are emitted to the underlying writer as soon as they are
    /// complete; any remaining bits stay buffered until the next call or
    /// until [`flush`](Self::flush) is called.
    pub fn write_bits(&mut self, value: u64, count: usize) -> io::Result<()> {
        for i in (0..count).rev() {
            let bit = ((value >> i) & 1) as u8;
            self.buffer = (self.buffer << 1) | bit;
            self.bits_filled += 1;

            if self.bits_filled == 8 {
                self.writer.write_all(&[self.buffer])?;
                self.buffer = 0;
                self.bits_filled = 0;
            }
        }
        Ok(())
    }

    /// Flushes any buffered partial byte to the underlying writer, padding the
    /// remaining bits with zeros on the right.  Does nothing when already
    /// byte-aligned.
    pub fn flush(&mut self) -> io::Result<()> {
        if self.bits_filled > 0 {
            self.buffer <<= 8 - self.bits_filled;
            self.writer.write_all(&[self.buffer])?;
            self.buffer = 0;
            self.bits_filled = 0;
        }
        Ok(())
    }

    /// Writes a fixed-length string field to the stream.
    ///
    /// Writes exactly `byte_len` bytes: the bytes of `s` followed by space
    /// padding if `s` is shorter than `byte_len`. If `s` is longer, it is
    /// truncated. This is used for ASTERIX string fields such as callsigns.
    pub fn write_string(&mut self, s: &str, byte_len: usize) -> io::Result<()> {
        let bytes = s.as_bytes();
        for i in 0..byte_len {
            let byte = if i < bytes.len() { bytes[i] } else { b' ' };
            self.write_bits(byte as u64, 8)?;
        }
        Ok(())
    }

    /// Returns true if the writer is at a byte boundary (no partial byte buffered).
    pub fn is_byte_aligned(&self) -> bool {
        self.bits_filled == 0
    }
}

/// Implement Write for BitWriter to allow byte-level operations.
/// Note: This only works correctly when the writer is at a byte boundary.
impl<W: Write> Write for BitWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        debug_assert!(
            self.bits_filled == 0,
            "BitWriter::write called with {} bits buffered",
            self.bits_filled
        );
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        // Flush any partial bits first
        BitWriter::flush(self)?;
        self.writer.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_writer() {
        let buffer = Vec::new();
        let writer = BitWriter::new(buffer);
        assert!(writer.is_byte_aligned());
    }

    #[test]
    fn write_single_bit() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(1, 1).unwrap(); // Write bit 1
        assert!(!writer.is_byte_aligned());

        writer.write_bits(0, 1).unwrap(); // Write bit 0
        writer.write_bits(1, 1).unwrap(); // Write bit 1
        writer.write_bits(0, 1).unwrap(); // Write bit 0
        writer.write_bits(1, 1).unwrap(); // Write bit 1
        writer.write_bits(0, 1).unwrap(); // Write bit 0
        writer.write_bits(1, 1).unwrap(); // Write bit 1
        writer.write_bits(0, 1).unwrap(); // Write bit 0

        assert!(writer.is_byte_aligned());
        assert_eq!(buffer, vec![0xAA]); // 0b10101010
    }

    #[test]
    fn write_full_byte() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(0xAB, 8).unwrap();
        assert!(writer.is_byte_aligned());
        assert_eq!(buffer, vec![0xAB]);
    }

    #[test]
    fn write_multiple_bytes() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(0xAB, 8).unwrap();
        writer.write_bits(0xCD, 8).unwrap();
        assert_eq!(buffer, vec![0xAB, 0xCD]);
    }

    #[test]
    fn write_across_byte_boundary() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        // Write 12 bits: 0xABC = 0b101010111100
        writer.write_bits(0xABC, 12).unwrap();
        assert!(!writer.is_byte_aligned());

        // Flush to complete the partial byte
        writer.flush().unwrap();

        // Should be: 0xAB (first 8 bits) + 0xC0 (last 4 bits + padding)
        assert_eq!(buffer, vec![0xAB, 0xC0]);
    }

    #[test]
    fn write_16_bits() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(0x1234, 16).unwrap();
        assert_eq!(buffer, vec![0x12, 0x34]);
    }

    #[test]
    fn write_32_bits() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(0x12345678, 32).unwrap();
        assert_eq!(buffer, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn write_zero_bits() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(0xFF, 0).unwrap();
        assert!(writer.is_byte_aligned());
        assert!(buffer.is_empty());
    }

    #[test]
    fn flush_partial_byte() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        // Write 3 bits: 0b101
        writer.write_bits(0b101, 3).unwrap();
        assert!(!writer.is_byte_aligned());

        writer.flush().unwrap();
        assert!(writer.is_byte_aligned());

        // Should be 0b10100000 = 0xA0
        assert_eq!(buffer, vec![0xA0]);
    }

    #[test]
    fn flush_empty_does_nothing() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.flush().unwrap();
        assert!(buffer.is_empty());
    }

    #[test]
    fn byte_alignment_tracking() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        assert!(writer.is_byte_aligned());
        writer.write_bits(1, 1).unwrap();
        assert!(!writer.is_byte_aligned());
        writer.write_bits(0, 7).unwrap();
        assert!(writer.is_byte_aligned());
    }

    #[test]
    fn write_trait_at_byte_boundary() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        // Write first byte using bit writer
        writer.write_bits(0xAB, 8).unwrap();

        // Now use Write trait for remaining bytes
        writer.write_all(&[0xCD, 0xEF]).unwrap();

        assert_eq!(buffer, vec![0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn write_multiple_sizes() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_bits(0b111, 3).unwrap();  // 3 bits
        writer.write_bits(0b111, 3).unwrap();  // 3 bits
        writer.write_bits(0b11, 2).unwrap();   // 2 bits

        assert!(writer.is_byte_aligned());
        assert_eq!(buffer, vec![0xFF]); // 0b11111111
    }

    #[test]
    fn round_trip_with_reader() {
        use crate::bit_reader::BitReader;
        use std::io::Cursor;

        // Write some bits
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            writer.write_bits(0xABCD, 16).unwrap();
            writer.write_bits(0b101, 3).unwrap();
            writer.write_bits(0b11111, 5).unwrap();
        }

        // Read them back
        let mut reader = BitReader::new(Cursor::new(&buffer));
        assert_eq!(reader.read_bits(16).unwrap(), 0xABCD);
        assert_eq!(reader.read_bits(3).unwrap(), 0b101);
        assert_eq!(reader.read_bits(5).unwrap(), 0b11111);
    }

    #[test]
    fn write_string_basic() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_string("ABC", 5).unwrap();
        assert_eq!(buffer, vec![0x41, 0x42, 0x43, 0x20, 0x20]);
    }

    #[test]
    fn write_string_exact_length() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_string("AB", 2).unwrap();
        assert_eq!(buffer, vec![0x41, 0x42]);
    }

    #[test]
    fn write_string_truncated() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        writer.write_string("ABCDE", 3).unwrap();
        assert_eq!(buffer, vec![0x41, 0x42, 0x43]);
    }

    #[test]
    fn round_trip_string() {
        use crate::bit_reader::BitReader;
        use std::io::Cursor;

        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            writer.write_string("TEST", 8).unwrap();
        }

        let mut reader = BitReader::new(Cursor::new(&buffer));
        let s = reader.read_string(8).unwrap();
        assert_eq!(s, "TEST");
    }

    #[test]
    fn write_alternating_pattern() {
        let mut buffer = Vec::new();
        let mut writer = BitWriter::new(&mut buffer);

        // Write alternating bits: 01010101 = 0x55
        for i in 0..8 {
            writer.write_bits((i % 2) as u64, 1).unwrap();
        }

        assert_eq!(buffer, vec![0x55]); // 0b01010101
    }
}
