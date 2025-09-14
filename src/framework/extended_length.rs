use crate::framework::{commons::{self, Field}, fixed_length::FixedLength};

pub struct ExtendedLength {
    pub parts: Vec<FixedLength>,
}

impl Field for ExtendedLength {
    fn size_bytes(&self) -> usize {
        self.parts.iter().map(|part| part.size_bytes()).sum()
    }
    fn data_existing(&self) -> bool {
        for part in &self.parts {
            if part.data_existing() {
                return true;
            }
        }
        false
    }
}

impl ExtendedLength {
    pub fn new() -> Self {
        ExtendedLength { parts: Vec::new() }
    }

    /// Updates the FX bit with the given value (true = 1, false = 0)
    fn update_fx(part: &mut FixedLength, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let byte_length = part.size_bytes();
        let bit_length = byte_length * commons::BYTE_SIZE;
        part.set_value(value, byte_length as u16, (bit_length - 1) as u16, 0x01)
    }

    /// Adds a new FixedLength part to the ExtendedLength item
    pub fn add_part(&mut self, part: FixedLength) {
        let size = self.parts.len();
        if size > 0 {
            let result = Self::update_fx(&mut self.parts[size - 1], true);
            if let Err(error) = result {
                eprintln!("Error updating FX bit: {}", error);
            }
        }
        self.parts.push(part);
    }
}