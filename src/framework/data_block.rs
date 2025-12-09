use crate::framework::record::Record;

/// The size of the Data Block header in bytes 
/// (1 byte for category + 2 bytes for length)
pub const DATA_BLOCK_HEADER_SIZE: usize = 1 + 2; 

/// Represents an ASTERIX Data Block
pub struct DataBlock {
    category: u8,
    length: u16,
    records: Vec<Record>,
}

impl SizedData for DataBlock {
    fn size_bytes(&self) -> usize {
        DATA_BLOCK_HEADER_SIZE + self.records.calculate_size()
    }
}