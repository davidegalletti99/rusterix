use std::io::{self, Read};

#[derive(Debug)]
pub struct BitReader<R: Read> {
    reader: R,
    buffer: u8,
    bits_left: u8,
}

impl<R: Read> BitReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: 0,
            bits_left: 0,
        }
    }

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
}
