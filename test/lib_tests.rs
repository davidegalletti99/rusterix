/// Integration tests using external XML fixtures.

mod test_helper;
use test_helper::*;

// ============================================================================
// Parsing Tests
// ============================================================================

#[test]
fn test_parse_simple_fixed_item() {
    let xml = load_fixture("valid", "simple_fixed.xml");
    let category = rusterix::parse::parser::parse_category(&xml)
        .expect("Failed to parse XML");
    
    assert_eq!(category.id, 1);
    assert_eq!(category.items.len(), 1);
    assert_eq!(category.items[0].id, 10);
    assert_eq!(category.items[0].frn, 0);
}

#[test]
fn test_parse_extended_item() {
    let xml = load_fixture("valid", "extended_multi_part.xml");
    let category = rusterix::parse::parser::parse_category(&xml)
        .expect("Failed to parse XML");
    
    match &category.items[0].data {
        rusterix::parse::xml_model::ItemStructure::Extended(ext) => {
            assert_eq!(ext.bytes, 2);
            assert_eq!(ext.part_groups.len(), 2);
        }
        _ => panic!("Expected Extended item"),
    }
}

#[test]
fn test_parse_epb_field() {
    let xml = load_fixture("valid", "epb_field.xml");
    let category = rusterix::parse::parser::parse_category(&xml)
        .expect("Failed to parse XML");
    
    match &category.items[0].data {
        rusterix::parse::xml_model::ItemStructure::Fixed(fixed) => {
            match &fixed.elements[0] {
                rusterix::parse::xml_model::Element::EPB(epb) => {
                    match &epb.content {
                        rusterix::parse::xml_model::EPBContent::Field(field) => {
                            assert_eq!(field.name, "optional_value");
                            assert_eq!(field.bits, 15);
                        }
                        _ => panic!("Expected Field in EPB"),
                    }
                }
                _ => panic!("Expected EPB element"),
            }
        }
        _ => panic!("Expected Fixed item"),
    }
}

#[test]
fn test_parse_enum() {
    let xml = load_fixture("valid", "enum_basic.xml");
    let category = rusterix::parse::parser::parse_category(&xml)
        .expect("Failed to parse XML");
    
    match &category.items[0].data {
        rusterix::parse::xml_model::ItemStructure::Fixed(fixed) => {
            match &fixed.elements[0] {
                rusterix::parse::xml_model::Element::Enum(enum_def) => {
                    assert_eq!(enum_def.name, "target_type");
                    assert_eq!(enum_def.bits, 3);
                    assert_eq!(enum_def.values.len(), 2);
                }
                _ => panic!("Expected Enum"),
            }
        }
        _ => panic!("Expected Fixed item"),
    }
}

#[test]
fn test_parse_invalid_xml() {
    let xml = load_fixture("invalid", "invalid_xml.xml");
    let result = rusterix::parse::parser::parse_category(&xml);
    assert!(result.is_err());
}

// ============================================================================
// Transform Tests
// ============================================================================

#[test]
fn test_transform_to_ir() {
    let ir = build_ir_from_fixture("valid", "simple_fixed.xml");
    
    assert_eq!(ir.category.id, 1);
    assert_eq!(ir.category.items.len(), 1);
    assert_eq!(ir.category.items[0].id, 10);
    assert_eq!(ir.category.items[0].frn, 0);
}

#[test]
#[should_panic(expected = "Bit count mismatch")]
fn test_validation_bit_mismatch() {
    let _ = build_ir_from_fixture("invalid", "bit_mismatch.xml");
}

#[test]
#[should_panic(expected = "Part group")]
fn test_validation_extended_bit_mismatch() {
    let _ = build_ir_from_fixture("invalid", "extended_bit_mismatch.xml");
}

// ============================================================================
// Code Generation Tests
// ============================================================================

#[test]
fn test_generate_simple_code() {
    let code = generate_from_fixture("valid", "simple_fixed.xml");
    
    assert_code_contains(&code, &[
        "pub struct Cat001Record",
        "pub struct Item010",
        "pub sac",
        "pub sic",
        "pub fn decode",
        "pub fn encode",
    ]);
}

#[test]
fn test_generate_extended_code() {
    let code = generate_from_fixture("valid", "extended_multi_part.xml");
    
    assert_code_contains(&code, &[
        "pub struct Item020Part0",
        "pub struct Item020Part1",
        "pub part0",
        "pub part1 : Option",
    ]);
}

#[test]
fn test_generate_enum_code() {
    let code = generate_from_fixture("valid", "enum_basic.xml");
    
    assert_code_contains(&code, &[
        "pub enum TargetType",
        "Psr = 1",
        "Ssr = 2",
        "Unknown(u8)",
        "impl TryFrom < u8 > for TargetType",
    ]);
}

#[test]
fn test_generate_epb_code() {
    let code = generate_from_fixture("valid", "epb_field.xml");
    
    // EPB should generate Option<T>
    assert_code_contains(&code, &[
        "pub optional_value : Option",
    ]);
    
    // Should NOT have a separate "name" field
    assert_code_not_contains(&code, &[
        "pub name",
    ]);
}

#[test]
fn test_generate_compound_code() {
    let code = generate_from_fixture("valid", "compound_simple.xml");
    
    assert_code_contains(&code, &[
        "Item100_sub0",
        "Item100_sub1",
        "pub sub0 : Option",
        "pub sub1 : Option",
    ]);
}

#[test]
fn test_generate_repetitive_code() {
    let code = generate_from_fixture("valid", "repetitive_basic.xml");
    
    assert_code_contains(&code, &[
        "Item070Element",
        "pub items : Vec < Item070Element >",
    ]);
}

#[test]
fn test_spare_bits_ignored() {
    let code = generate_from_fixture("valid", "spare_bits.xml");
    
    assert_code_contains(&code, &[
        "pub data",
    ]);
    
    assert_code_not_contains(&code, &[
        "spare",
        "pub spare",
    ]);
}

#[test]
fn test_explicit_item() {
    let code = generate_from_fixture("valid", "explicit_item.xml");
    
    assert_code_contains(&code, &[
        "pub struct Item060",
        "pub altitude",
        "pub speed",
    ]);
}

#[test]
fn test_record_generation() {
    let code = generate_from_fixture("valid", "multi_item_record.xml");
    
    assert_code_contains(&code, &[
        "pub struct Cat048Record",
        "pub item010 : Option < Item010 >",
        "pub item020 : Option < Item020 >",
        "let fspec = Fspec :: read",
        "fspec . is_set",
        "fspec . set",
    ]);
}

// ============================================================================
// Builder Tests
// ============================================================================

#[test]
fn test_builder_from_fixture() {
    use rusterix::builder::{Builder, RustBuilder};
    
    let temp_path = create_temp_file(
        &load_fixture("valid", "simple_fixed.xml"),
        "xml"
    );
    
    let builder = RustBuilder::new();
    let result = builder.build(temp_path.to_str().unwrap());
    
    cleanup_temp_files();
    
    assert!(result.is_ok());
    let code = result.unwrap();
    assert!(code.contains("Cat001Record"));
}

#[test]
fn test_builder_build_file() {
    use rusterix::builder::RustBuilder;
    use std::fs;
    
    let xml_content = load_fixture("valid", "simple_fixed.xml");
    
    fs::create_dir_all("target/test_output").unwrap();
    fs::write("target/test_output/test.xml", xml_content).unwrap();
    
    let builder = RustBuilder::new();
    let result = builder.build_file(
        "target/test_output/test.xml",
        "target/test_output/generated"
    );
    
    assert!(result.is_ok());
    let output_path = result.unwrap();
    assert!(output_path.exists());
    
    let generated = fs::read_to_string(&output_path).unwrap();
    assert!(generated.contains("Cat001Record"));
    
    // Cleanup
    fs::remove_dir_all("target/test_output").ok();
}


// ============================================================================
// Round-trip Tests (if framework runtime is available)
// ============================================================================

#[test]
#[ignore] // Requires generated code to be compiled
fn test_round_trip_simple_fixed() {
    // This test would compile generated code and test encode->decode
    // Requires integration with actual runtime framework
    todo!("Implement once generated code can be compiled in tests");
}