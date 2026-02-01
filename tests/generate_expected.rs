//! Helper binary to generate expected output files.
//!
//! Run with: cargo test --test generate_expected -- --nocapture
//!
//! This will regenerate all expected output files in tests/fixtures/expected/

mod common;

use std::fs;
use std::path::PathBuf;

/// List of fixtures to generate expected output for.
const FIXTURES: &[&str] = &[
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

fn expected_output_path(fixture_name: &str) -> PathBuf {
    PathBuf::from("tests")
        .join("fixtures")
        .join("expected")
        .join(format!("{}.rs", fixture_name))
}

#[test]
fn generate_all_expected_outputs() {
    let expected_dir = PathBuf::from("tests/fixtures/expected");
    fs::create_dir_all(&expected_dir).expect("Failed to create expected dir");

    for fixture_name in FIXTURES {
        let filename = format!("{}.xml", fixture_name);
        let code = common::generate_from_fixture("valid", &filename);

        // Write raw output - normalization during comparison handles formatting differences
        let output_path = expected_output_path(fixture_name);
        fs::write(&output_path, &code)
            .unwrap_or_else(|e| panic!("Failed to write {}: {}", output_path.display(), e));

        println!("Generated: {}", output_path.display());
    }

    println!("\nAll expected output files generated successfully!");
}
