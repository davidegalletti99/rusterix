mod utils;
use rusterix::framework::{commons::{Field, MaskType, BYTE_SIZE}, fixed_length::FixedLength};

#[test]
fn creation_test() {
    let test = FixedLength::new(8);
    assert_eq!(test.size_bytes(), 8, "Expected size 8, got {}", test.size_bytes());
}

#[test]
fn set_value_fail_test() {
    let mut test = FixedLength::new(2);
    let value = utils::get_random!(MaskType);
    let result = test.set_value(value, 16, 1);
    assert!(result.is_err(), "set_value should have failed but succeeded");
}

fn set_value_test(value_to_set: MaskType, bit_length: u16, offset_bits: u16, test: &mut FixedLength) {
    let res = test.set_value(value_to_set, bit_length, offset_bits);
    assert!(res.is_ok(), "set_value failed: {:?}", res.err());
    
    let end_idx = ((offset_bits + bit_length + 7) as usize) / BYTE_SIZE;
    let start_idx = (offset_bits as usize) / BYTE_SIZE;
    let byte_length = end_idx - start_idx;
    let correction = (end_idx * BYTE_SIZE) as u16 - (offset_bits + bit_length);
    
    // mask used to build the value from the FixedLength under test
    let build_mask = utils::create_bitmask(correction, bit_length);
    let test_value = utils::build_value_from_bytes_masked(test, start_idx, byte_length, build_mask);
    let expected_value = (value_to_set << correction) & build_mask;
    assert_eq!(test_value, expected_value,
        "Data mismatch from byte {} to {}: expected {:X}, got {:X}",
        start_idx, end_idx - 1, expected_value, test_value);
    
}

fn set_multiple_values_test(length_step: usize, test: &mut FixedLength) {
    let size = test.size_bytes() * BYTE_SIZE;

    for i in (0..size).into_iter().step_by(length_step) {
        let value = utils::get_random!(MaskType);
        set_value_test(value, length_step as u16, i as u16, test);
    }
}


macro_rules! set_value_tests {
    ($($name:ident: $values:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (size, value_to_set, bit_length, offset_bits) = $values;
                let mut test = FixedLength::new(size);
                set_value_test(value_to_set, bit_length, offset_bits, &mut test);
            }
        )*
    }
}

macro_rules! set_multiple_values_tests {
    ($($name:ident: $values:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (step, repeat_count) = $values;
                let size = (step * repeat_count) / BYTE_SIZE;
                let mut test = FixedLength::new(size);
                set_multiple_values_test(step, &mut test);
            }
        )*
    }
}


set_value_tests! {
    simple_set_test: (2, 0xFF, 8, 0),
    two_byte_set_test: (2, 0xFFFF, 16, 0),
    half_byte_set_test: (2, 0x0F, 4, 0),
    single_bit_set_test: (1, 0x1, 1, 0),
    simple_cross_byte_set_test: (2, 0x0FF0, 8, 4),
}

set_multiple_values_tests!(
    multiple_set_step_1_test: (1, 16),
    multiple_set_step_4_test: (4, 8),
    multiple_set_step_8_test: (8, 4),
    multiple_set_step_16_test: (16, 2),
);

