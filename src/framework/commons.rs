pub const BYTE_SIZE: usize = 8;
pub type MaskType = u128;

pub trait SerializeDeserialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self where Self: Sized;
}

pub trait Field { 
    fn size_bytes(&self) -> usize;
    fn data_existing(&self) -> bool;
}

pub fn make_bitmask(offset: u16, bit_length: u16) -> MaskType {
    if bit_length == 0 {
        return 0;
    }
    let mut mask: MaskType = (1 << bit_length) - 1;
    mask <<= offset;
    mask
}