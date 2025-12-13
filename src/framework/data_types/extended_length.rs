use crate::framework::{commons::{self, Field, SizedData}, data_types::{fixed_length::FixedLength}};

struct ExtendedField {
    data: FixedLength,
    field_extension: bool
}

pub struct ExtendedLength {
    primary: FixedLength,
    secondary: Vec<FixedLength>,
}

impl Field for ExtendedLength {
    fn data_existing(&self) -> bool {
        for part in &self.secondary {
            if part.data_existing() {
                return true;
            }
        }
        false
    }
}

impl SizedData for ExtendedLength {
    fn size_bytes(&self) -> usize {
        self.primary.size_bytes() + self.secondary.iter().map(|part| part.size_bytes()).sum::<usize>()
    }
}

impl ExtendedLength {
    pub fn new(primary_length: usize) -> Self {
        ExtendedLength { primary: FixedLength::new(primary_length), secondary: Vec::new() }
    }

    /// Updates the FX bit with the given value (true = 1, false = 0)
    fn update_fx(part: &mut FixedLength, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        let bit_length = part.size_bytes() * commons::BYTE_SIZE;
        let fx_bit_position = bit_length - 1;
        part.set_value(value, 1, fx_bit_position as u16)
    }

    /// Adds a new FixedLength part to the ExtendedLength item
    pub fn add_part(&mut self, part: FixedLength) {
        let intial_size = self.secondary.len();
        let last_idx = intial_size - 1;
        self.secondary.push(part);
        if intial_size > 1 {
            let is_valued= self.secondary[last_idx].data_existing();
            let result = Self::update_fx(&mut self.secondary[last_idx], is_valued);
            if let Err(error) = result {
                eprintln!("Error updating FX bit: {}", error);
            }
        }
    }
}