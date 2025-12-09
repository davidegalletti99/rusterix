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
    fn data_existing(&self) -> bool {
        for byte in &self.data {
            if *byte != 0 {
                return true;
            }
        }
        false
    }
}

impl SizedData for FixedLength {
    fn size_bytes(&self) -> usize {
        self.data.len()
    }
}

impl FixedLength {
    pub fn new(length: usize) -> Self {
        FixedLength { data: vec![0; length] }
    }

    /// Sets a value into the FixedLength field at the specified bit offset and length,
    /// using the extraction mask to ensure only relevant bits are modified.
    /// The extraction_mask is relative to the bit_length bytes 
    pub fn set_value<T>(&mut self, value: T, bit_length: u16, offset_bits: u16) 
        -> Result<(), Box<dyn std::error::Error>>
    where T: Into<MaskType> 
    {
        let size_bits = self.size_bytes() * commons::BYTE_SIZE;
        let end_bit = (offset_bits + bit_length) as usize;
        if end_bit > size_bits {
            let error_message = format!("Value does not fit into FixedLength of size {} bits since {} (offset_bits) + {} (bit_length) > {} (size_bits)", 
               size_bits, offset_bits, bit_length, size_bits);
            return Err(error_message.into());
        }
        
        let end_byte = (end_bit + commons::BYTE_SIZE - 1) / commons::BYTE_SIZE;
        let start_byte = offset_bits as usize / commons::BYTE_SIZE;
        
        let correction = (end_byte * commons::BYTE_SIZE - end_bit) as u16;
        let extraction_mask = commons::make_bitmask(correction, bit_length);
        
        let mut buf: MaskType = value.into() << correction;
        let mut bitmask: MaskType = extraction_mask.clone();

        for i in (start_byte..end_byte).rev() {
            let d = (buf as u8) ^ self.data[i];
            let dm = d & bitmask as u8;
            self.data[i] ^= dm;

            buf >>= 8;
            bitmask >>= 8;
        }
        Ok(())
    }

}