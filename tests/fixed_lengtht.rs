use rusterix::framework::{commons::{Field, MaskType, BYTE_SIZE}, fixed_length::FixedLength};

fn print_bits(fixed_length: &FixedLength) {
    for byte in 0..fixed_length.size_bytes() {
        print!("{:08b} ", fixed_length[byte]);
    }
    println!();
}

#[test]
fn creation_test() {
    let test = FixedLength::new(8);
    assert_eq!(test.size_bytes(), 8, "Expected size 8, got {}", test.size_bytes());
}

#[test]
fn set_value_test() {
    let mut test = FixedLength::new(2);
    let result = 
        test.set_value(0xFFu16, 8, 0, 0xFF);
    print_bits(&test);
    assert!(result.is_ok(), "set_value failed: {:?}", result.err());
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
    assert_eq!(test.size_bytes(), 2, "Expected size 2, got {}", test.size_bytes());
}

#[test]
fn set_multiple_half_byte_fixed_value_test() {
    let mut test = FixedLength::new(1);
    let res1 = test.set_value(0xF0u16, 4, 0, 0xF0);
    assert!(res1.is_ok(), "set_value failed: {:?}", res1.err());
    let res2 = test.set_value(0x0Fu16, 4, 4, 0x0F);
    assert!(res2.is_ok(), "set_value failed: {:?}", res2.err());
    print_bits(&test);

    assert_eq!(test.size_bytes(), 1, "Expected size 1, got {}", test.size_bytes());
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
}

#[test]
fn set_multiple_quarter_byte_fixed_value_test() {
    let mut test = FixedLength::new(1);
    assert_eq!(test.size_bytes(), 1, "Expected size 1, got {}", test.size_bytes());
    
    let res1 = test.set_value(0xF0u16, 4, 0, 0xC0);
    assert!(res1.is_ok(), "set_value failed: {:?}", res1.err());

    assert_eq!(test[0], 0xC0, "Expected 0xC0, got {:02X}", test[0]);

    let res2 = test.set_value(0xF0u16, 4, 4, 0x30);
    assert!(res2.is_ok(), "set_value failed: {:?}", res2.err());

    print_bits(&test);
    assert_eq!(test[0], 0xF0, "Expected 0xF0, got {:02X}", test[0]);
}
    
#[test]
fn set_multiple_fixed_value_test() {
    let mut test = FixedLength::new(2);
    let res1 = 
        test.set_value(0xFFFFu16, 8, 0, 0xFF);
    assert!(res1.is_ok(), "set_value failed: {:?}", res1.err());
    
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
    assert_eq!(test[1], 0x00, "Expected 0x00, got {:02X}", test[1]);

    let res2 = 
        test.set_value(0xFFFFu16, 8, 8, 0xFF);
        
    assert!(res2.is_ok(), "set_value failed: {:?}", res2.err());

    print_bits(&test);
    assert_eq!(test.size_bytes(), 2, "Expected size 2, got {}", test.size_bytes());
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
    assert_eq!(test[1], 0xFF, "Expected 0xFF, got {:02X}", test[1]);
}

#[test]
fn set_value_fail_test() {
    let mut test = FixedLength::new(2);
    let result = 
        test.set_value(0xFFFFu16, 16, 1, 0xFFFF);
    assert!(result.is_err(), "set_value should have failed but succeeded");
}

fn create_random_fixed_length(size: usize) -> FixedLength {
    let mut fl = FixedLength::new(size);
    for i in 0..size {
        fl[i] = rand::random::<u8>();
    }
    fl
}

fn create_mask_from_offset_and_length(offset: u16, bit_length: u16) -> MaskType {
    if bit_length == 0 {
        return 0;
    }
    let mut mask: MaskType = (1 << bit_length) - 1;
    mask <<= offset;
    mask
}

fn build_value_from_bytes(data: &FixedLength, start_idx: usize, byte_length: usize) -> MaskType {
    let mut value: MaskType = 0;
    for i in 0..byte_length {
        value = (value << 8) | data[start_idx + i] as MaskType;
    }
    value
}

#[test]
fn random_set_value_test() {
    let length = 16;
    let expected: FixedLength = create_random_fixed_length(length);
    
    let bit_length = length * BYTE_SIZE;
    for &subdivision in [1, 2, 4, 8, 16, 32, 64].iter() {
        let mut test = FixedLength::new(length);
        assert_eq!(test.size_bytes(), length, "Expected size {}, got {}", length, test.size_bytes());
        let is_less_then_or_equal_a_byte = subdivision <= 8;
        for step in 0..(bit_length / subdivision) {
            let offset = step * subdivision;
            let idx = offset / BYTE_SIZE;
            let mut value_to_set = expected[idx] as u128;
            let mask_offset = (offset % 8) as u16;
            if !is_less_then_or_equal_a_byte {
                value_to_set = build_value_from_bytes(&expected, idx, subdivision / 8);
            }
            
            let mask: MaskType = create_mask_from_offset_and_length(mask_offset, subdivision as u16);
            // println!("Step {}: Setting value {:X} at offset {} (byte index {}, mask {:X})", 
            //     step, value_to_set, offset, idx, mask);

            let res = test.set_value(value_to_set, subdivision as u16, offset as u16, mask);
            assert!(res.is_ok(), "set_value failed at step {}: {:?}", step, res.err());
            let mut value = test[idx] as u128;
            let compare_mask = create_mask_from_offset_and_length(0, mask_offset + subdivision as u16);
            let expected_value = value_to_set & compare_mask;
            if !is_less_then_or_equal_a_byte {
                value = build_value_from_bytes(&test, idx, subdivision / 8);
            }

            assert_eq!(value, expected_value,
                "Data mismatch at byte {}: expected {:02X}, got {:02X}",
                idx, expected_value, value);
        }

        // Final verification
        for i in 0..length {
            assert_eq!(test[i], expected[i],
                "Data mismatch at byte {}: expected {:02X}, got {:02X}",
                i, expected[i], test[i]);
        }
    }
}