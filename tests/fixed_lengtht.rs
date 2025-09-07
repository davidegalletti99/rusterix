use std::collections::btree_map::Range;

use rand::{self, Rng};
use rusterix::framework::{commons::{MaskType, BYTE_SIZE}, fixed_length::FixedLength};

fn print_bits(fixed_length: &FixedLength) {
    for byte in &fixed_length.data {
        print!("{:08b} ", byte);
    }
    println!();
}

#[test]
fn creation_test() {
    let test = FixedLength::new(8);
    assert_eq!(test.size_bytes(), 8, "Expected size 8, got {}", test.size_bytes());
}

#[test]
fn set_fixed_value_test() {
    let mut test = FixedLength::new(2);
    let result = 
        test.set_fixed_value(0xFF00u16, 8, 0, 0xFF00);
    print_bits(&test);
    assert!(result.is_ok(), "set_fixed_value failed: {:?}", result.err());
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
    assert_eq!(test.size_bytes(), 2, "Expected size 2, got {}", test.size_bytes());
}

#[test]
fn set_multiple_half_byte_fixed_value_test() {
    let mut test = FixedLength::new(1);
    let res1 = test.set_fixed_value(0xF0u16, 4, 0, 0xF0);
    assert!(res1.is_ok(), "set_fixed_value failed: {:?}", res1.err());
    let res2 = test.set_fixed_value(0x0Fu16, 4, 4, 0x0F);
    assert!(res2.is_ok(), "set_fixed_value failed: {:?}", res2.err());
    print_bits(&test);

    assert_eq!(test.size_bytes(), 1, "Expected size 1, got {}", test.size_bytes());
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
}

#[test]
fn set_multiple_quarter_byte_fixed_value_test() {
    let mut test = FixedLength::new(1);
    assert_eq!(test.size_bytes(), 1, "Expected size 1, got {}", test.size_bytes());
    
    let res1 = test.set_fixed_value(0xF0u16, 4, 0, 0xC0);
    assert!(res1.is_ok(), "set_fixed_value failed: {:?}", res1.err());

    assert_eq!(test[0], 0xC0, "Expected 0xC0, got {:02X}", test[0]);

    let res2 = test.set_fixed_value(0xF0u16, 4, 4, 0x30);
    assert!(res2.is_ok(), "set_fixed_value failed: {:?}", res2.err());

    print_bits(&test);
    assert_eq!(test[0], 0xF0, "Expected 0xF0, got {:02X}", test[0]);
}
    
#[test]
fn set_multiple_fixed_value_test() {
    let mut test = FixedLength::new(2);
    let res1 = 
        test.set_fixed_value(0xFFFFu16, 8, 0, 0xFF00);
    assert!(res1.is_ok(), "set_fixed_value failed: {:?}", res1.err());
    
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
    assert_eq!(test[1], 0x00, "Expected 0x00, got {:02X}", test[1]);

    let res2 = 
        test.set_fixed_value(0xFFFFu16, 8, 8, 0x00FF);
        
    assert!(res2.is_ok(), "set_fixed_value failed: {:?}", res2.err());

    print_bits(&test);
    assert_eq!(test.size_bytes(), 2, "Expected size 2, got {}", test.size_bytes());
    assert_eq!(test[0], 0xFF, "Expected 0xFF, got {:02X}", test[0]);
    assert_eq!(test[1], 0xFF, "Expected 0xFF, got {:02X}", test[1]);
}

#[test]
fn set_fixed_value_fail_test() {
    let mut test = FixedLength::new(2);
    let result = 
        test.set_fixed_value(0xFFFFu16, 16, 1, 0xFFFF);
    assert!(result.is_err(), "set_fixed_value should have failed but succeeded");
}

fn create_random_fixed_length(size: usize) -> FixedLength {
    let mut fl = FixedLength::new(size);
    for i in 0..size {
        fl.data[i] = rand::random::<u8>();
    }
    fl
}

fn create_random_mask(bit_length: u16) -> MaskType {
    if bit_length == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    let mut mask: MaskType;
    loop {
        mask = 0;
        for i in 0..bit_length {
            if rng.gen_bool(0.5) {
                mask |= 1 << i;
            }
        }
        let all_ones = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
        if mask != 0 && mask != all_ones {
            break;
        }
    }
    mask
}

fn create_mask_from_offset_and_length(offset: u16, bit_length: u16) -> MaskType {
    if bit_length == 0 {
        return 0;
    }
    let mut mask: MaskType = (1 << bit_length) - 1;
    mask <<= offset;
    mask
}

#[test]
fn random_set_fixed_value_test() {
    let length = 16;
    let expected: FixedLength = create_random_fixed_length(length);
    
    let bit_length = length * BYTE_SIZE;
    for &subdivision in [1, 2, 4, 8].iter() {
        let mut test = FixedLength::new(length);
        assert_eq!(test.size_bytes(), length, "Expected size {}, got {}", length, test.size_bytes());
        for step in 0..(bit_length / subdivision) {
            let offset = ((step * subdivision) % 8) as u16;
            let mask = create_mask_from_offset_and_length(offset, subdivision as u16);
            let idx = (step * subdivision) / BYTE_SIZE;
            let value = expected.data[idx];

            let res = test.set_fixed_value(value, subdivision as u16, (step * subdivision) as u16, mask);
            assert!(res.is_ok(), "set_fixed_value failed at step {}: {:?}", step, res.err());
        }
        for i in 0..length {
            assert_eq!(test.data[i], expected.data[i], 
                "Data mismatch at byte {}: expected {:02X}, got {:02X}", 
                i, expected.data[i], test.data[i]);
        }
    }
}