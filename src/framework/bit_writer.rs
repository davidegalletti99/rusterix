use std::io::{self, Write};

#[derive(Debug)]
pub struct BitWriter<W: Write> {
    writer: W,
    buffer: u8,
    bits_filled: u8,
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            buffer: 0,
            bits_filled: 0,
        }
    }

    pub fn write_bits(&mut self, mut value: u64, count: usize) -> io::Result<()> {
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

    pub fn flush(mut self) -> io::Result<()> {
        if self.bits_filled > 0 {
            self.buffer <<= 8 - self.bits_filled;
            self.writer.write_all(&[self.buffer])?;
        }
        Ok(())
    }
}
