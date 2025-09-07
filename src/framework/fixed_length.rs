use std::ops::Index;

use crate::framework::commons::{self, MaskType};


pub trait SerializeDeserialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self where Self: Sized;
}

pub struct FixedLength {
    pub data: Vec<u8>,
}

impl PartialEq for FixedLength {
    fn eq(&self, other: &Self) -> bool {

        if self.data.len() != other.data.len() {
            return false;
        }
        for bit in  0..self.data.len() {
            if self.data[bit] != other.data[bit] {
                return false;
            }
        }
        true
    }
}

impl Index<u8> for FixedLength {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl FixedLength {
    pub fn new(length: usize) -> Self {
        FixedLength { data: vec![0; length] }
    }
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }
    pub fn data_existing(&self) -> bool {
        for byte in &self.data {
            if *byte != 0 {
                return true;
            }
        }
        false
    }

    pub fn set_fixed_value<T>(&mut self, value: T, bit_length: u16, offset_bits: u16, mask: MaskType) 
        -> Result<(), Box<dyn std::error::Error>>
    where T: Into<MaskType> 
    {
            
        let size_bits = self.size_bytes() * commons::BYTE_SIZE;
        if offset_bits + bit_length > size_bits as u16 {
            let error_message = format!("Value does not fit into FixedLength of size {} bits since {} (offset_bits) + {} (bit_length) > {} (size_bits)", 
               size_bits, offset_bits, bit_length, size_bits);
            return Err(error_message.into());
        }
        
        let mut buf: MaskType = value.into();
        let mut bitmask: MaskType = mask.clone();

        // left shift until mask indicates byte boundary
        while !(bitmask & 0x01) == 1 {
            bitmask >>= 1;
            buf <<= 1;
        }
        // let len = (bit_length / 8) as usize;
        // let start = (offset_bits / 8) as usize;
        let involved_bytes = self.size_bytes();
        for i in (0..=involved_bytes - 1).rev() {
            let d = (buf as u8) ^ self.data[i];
            let dm = d & bitmask as u8;
            self.data[i] ^= dm;

            buf >>= 8;
            bitmask >>= 8;
        }
        Ok(())
    }

}