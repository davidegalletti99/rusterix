use std::io::{Cursor, Read, Write};

pub struct MemoryBuffer {
    inner: Cursor<Vec<u8>>,
}

impl MemoryBuffer {
    pub fn new() -> Self {
        Self {
            inner: Cursor::new(Vec::new()),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.inner.into_inner()
    }
}

impl Read for MemoryBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for MemoryBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl Default for MemoryBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn new_creates_empty_buffer() {
        let buffer = MemoryBuffer::new();
        assert!(buffer.into_inner().is_empty());
    }

    #[test]
    fn default_creates_empty_buffer() {
        let buffer = MemoryBuffer::default();
        assert!(buffer.into_inner().is_empty());
    }

    #[test]
    fn write_bytes() {
        let mut buffer = MemoryBuffer::new();

        let bytes_written = buffer.write(&[0x01, 0x02, 0x03]).unwrap();

        assert_eq!(bytes_written, 3);
        assert_eq!(buffer.into_inner(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn write_all_bytes() {
        let mut buffer = MemoryBuffer::new();

        buffer.write_all(&[0xAB, 0xCD, 0xEF]).unwrap();

        assert_eq!(buffer.into_inner(), vec![0xAB, 0xCD, 0xEF]);
    }

    #[test]
    fn flush_succeeds() {
        let mut buffer = MemoryBuffer::new();

        buffer.write_all(&[0x01]).unwrap();
        buffer.flush().unwrap(); // Should not error

        assert_eq!(buffer.into_inner(), vec![0x01]);
    }

    #[test]
    fn multiple_writes() {
        let mut buffer = MemoryBuffer::new();

        buffer.write_all(&[0x01, 0x02]).unwrap();
        buffer.write_all(&[0x03, 0x04]).unwrap();
        buffer.write_all(&[0x05]).unwrap();

        assert_eq!(buffer.into_inner(), vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    }

    #[test]
    fn write_empty() {
        let mut buffer = MemoryBuffer::new();

        let bytes_written = buffer.write(&[]).unwrap();

        assert_eq!(bytes_written, 0);
        assert!(buffer.into_inner().is_empty());
    }

    #[test]
    fn into_inner_consumes_buffer() {
        let mut buffer = MemoryBuffer::new();
        buffer.write_all(&[0x42]).unwrap();

        let data = buffer.into_inner();
        assert_eq!(data, vec![0x42]);

        // buffer is now consumed, can't use it anymore (compile-time check)
    }

    #[test]
    fn use_with_bit_writer() {
        use crate::bit_writer::BitWriter;

        let buffer = MemoryBuffer::new();
        let mut writer = BitWriter::new(buffer);

        writer.write_bits(0xABCD, 16).unwrap();
        writer.flush().unwrap();

        // Get the inner buffer from BitWriter (we need to expose this or use into_inner pattern)
        // For now, this test demonstrates the type compatibility
    }
}
