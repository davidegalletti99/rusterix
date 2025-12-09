pub const BYTE_SIZE: usize = 8;
pub type MaskType = u128;

/// Trait for serializing and deserializing ASTERIX fields
pub trait SerializeDeserialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self where Self: Sized;
}

/// Represents an ASTERIX field
pub trait Field { 
    fn data_existing(&self) -> bool;
}

pub trait SizedData {
    fn size_bytes(&self) -> usize;
}

pub fn make_bitmask(offset: u16, bit_length: u16) -> MaskType {
    if bit_length == 0 {
        return 0;
    }
    let mut mask: MaskType = (1 << bit_length) - 1;
    mask <<= offset;
    mask
}