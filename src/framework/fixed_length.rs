use std::ops::{Index, IndexMut};

use crate::framework::commons::{self, Field, MaskType};



/// Represents a fixed length field
pub struct FixedLength {
    data: Vec<u8>,
}

// Implement PartialEq to compare FixedLength instances
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

impl Index<usize> for FixedLength {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for FixedLength {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }

}

impl Field for FixedLength {
    fn size_bytes(&self) -> usize {
        self.data.len()
    }
    fn data_existing(&self) -> bool {
        for byte in &self.data {
            if *byte != 0 {
                return true;
            }
        }
        false
    }
}

impl FixedLength {
    pub fn new(length: usize) -> Self {
        FixedLength { data: vec![0; length] }
    }

    pub fn set_value<T>(&mut self, value: T, bit_length: u16, offset_bits: u16, mask: MaskType) 
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

        let end_byte = ((offset_bits + bit_length + 7) / 8) as usize;
        let start_byte = (offset_bits / 8) as usize;
        for i in (start_byte..=end_byte - 1).rev() {
            let d = (buf as u8) ^ self.data[i];
            let dm = d & bitmask as u8;
            self.data[i] ^= dm;

            buf >>= 8;
            bitmask >>= 8;
        }
        Ok(())
    }

}