#![allow(dead_code)]
#![allow(unused)]
use std::io::{self, Read, Write};

#[derive(Debug, Clone)]
pub struct Fspec {
    bytes: Vec<u8>,
}

impl Fspec {
    pub fn new() -> Self {
        Fspec { bytes: vec![] }
    }
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let mut bytes = Vec::new();

        loop {
            let mut b = [0u8];
            reader.read_exact(&mut b)?;
            bytes.push(b[0]);

            // FX bit (LSB)
            if b[0] & 0x01 == 0 {
                break;
            }
        }

        Ok(Self { bytes })
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.bytes)
    }

    pub fn is_set(&self, byte: usize, bit: u8) -> bool {
        self.bytes
            .get(byte)
            .map(|b| (b & (1 << (7 - bit))) != 0)
            .unwrap_or(false)
    }

    pub fn set(&mut self, byte: usize, bit: u8) {
        while self.bytes.len() <= byte {
            self.bytes.push(0);
        }
        self.bytes[byte] |= 1 << (7 - bit);
    }
}
