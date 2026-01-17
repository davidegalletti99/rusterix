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
