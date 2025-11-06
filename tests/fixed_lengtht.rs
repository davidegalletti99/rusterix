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
    let result = 
        test.set_value(value, 16, 1);
    assert!(result.is_err(), "set_value should have failed but succeeded");
}

fn set_value_test(value_to_set: MaskType, bit_length: u16, offset_bits: u16, test: &mut FixedLength) {
    let res = test.set_value(value_to_set, bit_length, offset_bits);
    assert!(res.is_ok(), "set_value failed: {:?}", res.err());
    
    let end_idx = ((offset_bits + bit_length + 7) as usize) / BYTE_SIZE;
    let start_idx = (offset_bits as usize) / BYTE_SIZE;
    let byte_length = end_idx - start_idx;
    let correction = (byte_length * BYTE_SIZE) as u16 - (offset_bits + bit_length);
    
    // mask used to build the value from the FixedLength under test
    let build_mask = utils::create_bitmask(correction, bit_length);
    let test_value = utils::build_value_from_bytes_masked(test, start_idx, byte_length, build_mask);

    let expected_value = (value_to_set << correction) & build_mask;
    assert_eq!(test_value, expected_value,
        "Data mismatch from byte {} to {}: expected {:X}, got {:X}",
        start_idx, end_idx - 1, expected_value, test_value);
    
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


set_value_tests! {
    simple_set_test: (2, 0xFF, 8, 0),
    simple_cross_byte_set_test: (2, 0x0FFF, 12, 0),
    single_bit_set_test: (1, 0x1, 1, 0),
    half_byte_set_test: (2, 0x0F, 4, 0),
}

#[test]
fn random_set_value_test() {
    let length = 16;
    let bit_length = length * BYTE_SIZE;
    let expected: FixedLength = utils::create_random_fixed_length(length);
    
    let subdivisions = [1, 2, 4, 8, 16, 32, 64];
    for &subdivision in subdivisions.iter() {
        let mut test = FixedLength::new(length);
        let is_less_then_or_equal_a_byte = subdivision <= 8;
        let current_max_steps = bit_length / subdivision;
        for step in 0..current_max_steps {
            let offset = step * subdivision;
            let idx = offset / BYTE_SIZE;
            let mut value_to_set = expected[idx] as u128;
            let mask_offset = (offset % 8) as u16;
            if !is_less_then_or_equal_a_byte {
                value_to_set = utils::build_value_from_bytes(&expected, idx, subdivision / 8);
            }
            let mask: MaskType = utils::create_bitmask(mask_offset, subdivision as u16);

            let res = test.set_value(value_to_set, subdivision as u16, offset as u16);
            assert!(res.is_ok(), "set_value failed at step {}: {:?}", step, res.err());
            let mut value = test[idx] as u128;
            let compare_mask = utils::create_bitmask(0, mask_offset + subdivision as u16);
            let expected_value = value_to_set & compare_mask;
            if !is_less_then_or_equal_a_byte {
                value = utils::build_value_from_bytes(&test, idx, subdivision / 8);
            }
            // inter-byte check
            assert_eq!(value, expected_value,
                "Data mismatch at byte {}: expected {:02X}, got {:02X}",
                idx, expected_value, value);
        }

        // byte correction check 
        for i in 0..length {
            assert_eq!(test[i], expected[i],
                "Data mismatch at byte {}: expected {:02X}, got {:02X}",
                i, expected[i], test[i]);
        }
    }
}



// #[test]
// fn set_value_test() {
//     let mut test = FixedLength::new(2);
//     let result = 
//         test.set_value(0xFFu16, 8, 0, 0xFF);
//     print_bits(&test);
//     assert!(result.is_ok(), "set_value failed: {:?}", result.err());
//     assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
//     assert_eq!(test.size_bytes(), 2, "Expected size 2, got {}", test.size_bytes());
// }

// #[test]
// fn set_multiple_half_byte_fixed_value_test() {
//     let mut test = FixedLength::new(1);
//     let res1 = test.set_value(0xF0u16, 4, 0, 0xF0);
//     assert!(res1.is_ok(), "set_value failed: {:?}", res1.err());
//     let res2 = test.set_value(0x0Fu16, 4, 4, 0x0F);
//     assert!(res2.is_ok(), "set_value failed: {:?}", res2.err());
//     print_bits(&test);

//     assert_eq!(test.size_bytes(), 1, "Expected size 1, got {}", test.size_bytes());
//     assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
// }

// #[test]
// fn set_multiple_quarter_byte_fixed_value_test() {
//     let mut test = FixedLength::new(1);
//     assert_eq!(test.size_bytes(), 1, "Expected size 1, got {}", test.size_bytes());
    
//     let res1 = test.set_value(0xF0u16, 4, 0, 0xC0);
//     assert!(res1.is_ok(), "set_value failed: {:?}", res1.err());

//     assert_eq!(test[0], 0xC0, "Expected 0xC0, got {:02X}", test[0]);

//     let res2 = test.set_value(0xF0u16, 4, 4, 0x30);
//     assert!(res2.is_ok(), "set_value failed: {:?}", res2.err());

//     print_bits(&test);
//     assert_eq!(test[0], 0xF0, "Expected 0xF0, got {:02X}", test[0]);
// }


    
// #[test]
// fn set_multiple_fixed_value_test() {
//     let mut test = FixedLength::new(2);
//     let res1 = 
//         test.set_value(0xFFFFu16, 8, 0, 0xFF);
//     assert!(res1.is_ok(), "set_value failed: {:?}", res1.err());
    
//     assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
//     assert_eq!(test[1], 0x00, "Expected 0x00, got {:02X}", test[1]);

//     let res2 = 
//         test.set_value(0xFFFFu16, 8, 8, 0xFF);
        
//     assert!(res2.is_ok(), "set_value failed: {:?}", res2.err());

//     print_bits(&test);
//     assert_eq!(test.size_bytes(), 2, "Expected size 2, got {}", test.size_bytes());
//     assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
//     assert_eq!(test[1], 0xFF, "Expected 0xFF, got {:02X}", test[1]);
// }