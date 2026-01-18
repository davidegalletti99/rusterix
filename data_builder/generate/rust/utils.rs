

pub fn rust_type_for_bits(bits: usize) -> &'static str {
    match bits {
        0..=8 => "u8",
        9..=16 => "u16",
        17..=32 => "u32",
        33..=64 => "u64",
        _ => "u128"
    }
}
pub fn read_bits(buf: &[u8], bit_offset: usize, bit_size: usize) -> u64 {
    let mut value = 0;
    for i in 0..bit_size {
        let bit = (buf[(bit_offset + i) / 8] >> (7 - ((bit_offset + i) % 8))) & 1;
        value = (value << 1) | bit as u64;
    }
    value
}

pub fn write_bits(buf: &mut [u8], bit_offset: usize, bit_size: usize, value: u64) {
    for i in 0..bit_size {
        let bit = (value >> (bit_size - 1 - i)) & 1;
        let idx = (bit_offset + i) / 8;
        let shift = 7 - ((bit_offset + i) % 8);
        buf[idx] = (buf[idx] & !(1 << shift)) | ((bit as u8) << shift);
    }
}
