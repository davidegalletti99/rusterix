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