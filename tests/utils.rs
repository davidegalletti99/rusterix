use rusterix::framework::{commons::{Field, MaskType}, fixed_length::FixedLength};


macro_rules! get_random {
    ($type:ty) => {
        rand::random::<$type>()
    };
}
macro_rules! get_random_ranged {
    ($type:ty, $min:expr, $max:expr) => {
        rand::Rng::gen_range(&mut rand::thread_rng(), $min..$max) as $type
    };
}

// macro exported for use in other test modules
pub(crate) use get_random;
pub(crate) use get_random_ranged;

/// Creates a FixedLength instance of the given size filled with random data.
pub fn create_random_fixed_length(size: usize) -> FixedLength {
    let mut fl = FixedLength::new(size);
    for i in 0..size {
        fl[i] = rand::random::<u8>();
    }
    fl
}
/// Creates a bitmask from that contains a sequence of 1s of length `bit_length`
/// starting at the bit position `offset`.
pub fn create_bitmask(offset: u16, bit_length: u16) -> MaskType {
    if bit_length == 0 {
        return 0;
    }
    let mut mask: MaskType = (1 << bit_length) - 1;
    mask <<= offset;
    mask
}

/// Builds a value from the bytes in `data` starting at `start_idx`
/// and spanning `byte_length` bytes.
pub fn build_value_from_bytes(data: &FixedLength, start_idx: usize, byte_length: usize) -> MaskType {
    let mut value: MaskType = 0;
    for i in 0..byte_length {
        value = (value << 8) | data[start_idx + i] as MaskType;
    }
    value
}

/// Builds a value from the bytes in `data` starting at `start_idx`
/// and spanning `byte_length` bytes, applying the given `mask`.
pub fn build_value_from_bytes_masked(data: &FixedLength, start_idx: usize, byte_length: usize, mask: MaskType) -> MaskType {
    let mut value: MaskType = 0;
    let mut mask = mask;
    for i in (0..byte_length).rev() {
        value |= ((data[start_idx + i] as MaskType) & mask) << (byte_length - i - 1) * 8;
        mask >>= 8;
    }
    value
}


// utility function to print bits of a FixedLength for debugging
#[allow(dead_code)]
fn print_bits(fixed_length: &FixedLength) {
    for byte in 0..fixed_length.size_bytes() {
        print!("{:08b} ", fixed_length[byte]);
    }
    println!();
}