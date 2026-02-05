//! Utility to regenerate expected output files.
//!
//! Run with: cargo test -p rusterix-codegen --test regenerate_expected -- --ignored

use rusterix_codegen::generate::generate;
use rusterix_codegen::parse::parser::parse_category;
use rusterix_codegen::transform::transformer::to_ir;
use std::fs;
use test_utils::{load_fixture, testdata_dir};

fn generate_from_fixture(category: &str, filename: &str) -> String {
    let xml = load_fixture(category, filename);
    let parsed = parse_category(&xml).expect("Failed to parse XML fixture");
    let ir = to_ir(parsed);
    let tokens = generate(&ir);
    tokens.to_string()
}

#[test]
#[ignore]
fn regenerate_all_expected_outputs() {
    let fixtures = [
        "simple_fixed",
        "extended_multi_part",
        "enum_basic",
        "epb_field",
        "compound_simple",
        "repetitive_basic",
        "spare_bits",
        "explicit_item",
        "multi_item_record",
    ];

    let expected_dir = testdata_dir().join("expected");
    fs::create_dir_all(&expected_dir).unwrap();

    for name in fixtures {
        let filename = format!("{}.xml", name);
        let output_path = expected_dir.join(format!("{}.rs", name));

        match std::panic::catch_unwind(|| generate_from_fixture("valid", &filename)) {
            Ok(code) => {
                fs::write(&output_path, &code).unwrap();
                println!("Generated: {}", output_path.display());
            }
            Err(_) => {
                println!("Skipped {} (generation failed)", name);
            }
        }
    }
}
