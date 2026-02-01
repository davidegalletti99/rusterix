/// Shared test utilities for integration tests.

use std::fs;
use std::path::PathBuf;

/// Loads an XML fixture file from the fixtures directory.
///
/// # Arguments
///
/// * `category` - "valid" or "invalid"
/// * `filename` - Name of the XML file (without path)
///
/// # Returns
///
/// The contents of the file as a string
pub fn load_fixture(category: &str, filename: &str) -> String {
    let path = fixture_path(category, filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", path.display(), e))
}

/// Returns the path to a fixture file.
pub fn fixture_path(category: &str, filename: &str) -> PathBuf {
    PathBuf::from("tests")
        .join("fixtures")
        .join(category)
        .join(filename)
}

/// Loads expected Rust code output for a test case.
pub fn load_expected_output(test_name: &str) -> String {
    let path = PathBuf::from("tests")
        .join("fixtures")
        .join("expected")
        .join(format!("{}.rs", test_name));

    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read expected output {}: {}", path.display(), e))
}

/// Asserts that generated code contains all expected fragments.
pub fn assert_code_contains(generated: &str, expected_fragments: &[&str]) {
    for fragment in expected_fragments {
        assert!(
            generated.contains(fragment),
            "Generated code missing fragment: {}",
            fragment
        );
    }
}

/// Asserts that generated code does NOT contain any of the given fragments.
pub fn assert_code_not_contains(generated: &str, forbidden_fragments: &[&str]) {
    for fragment in forbidden_fragments {
        assert!(
            !generated.contains(fragment),
            "Generated code contains forbidden fragment: {}",
            fragment
        );
    }
}

/// Normalizes whitespace in code for comparison.
pub fn normalize_whitespace(code: &str) -> String {
    code.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Asserts that two code strings are equal after normalizing whitespace.
/// This handles differences from quote! formatting vs stored expected output.
pub fn assert_normalized_eq(generated: &str, expected: &str, fixture_name: &str) {
    let gen_normalized = normalize_whitespace(generated);
    let exp_normalized = normalize_whitespace(expected);

    assert_eq!(
        gen_normalized,
        exp_normalized,
        "Generated code for '{}' doesn't match expected output.\n\
         --- Generated (first 500 chars) ---\n{}\n\
         --- Expected (first 500 chars) ---\n{}",
        fixture_name,
        &generated.chars().take(500).collect::<String>(),
        &expected.chars().take(500).collect::<String>()
    );
}

/// Builds IR from an XML fixture.
pub fn build_ir_from_fixture(category: &str, filename: &str) -> rusterix::transform::ir::IR {
    let xml = load_fixture(category, filename);
    let parsed = rusterix::parse::parser::parse_category(&xml)
        .expect("Failed to parse XML fixture");
    rusterix::transform::transformer::to_ir(parsed)
}

/// Generates code from an XML fixture.
pub fn generate_from_fixture(category: &str, filename: &str) -> String {
    let ir = build_ir_from_fixture(category, filename);
    let tokens = rusterix::generate::generate(&ir);
    tokens.to_string()
}

/// Creates a temporary test file and returns its path.
pub fn create_temp_file(content: &str, extension: &str) -> PathBuf {
    use std::io::Write;

    let temp_dir = PathBuf::from("target").join("test_temp");
    fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    // Use a simple counter-based name instead of uuid
    let filename = format!("test_{}.{}", std::process::id(), extension);
    let path = temp_dir.join(filename);

    let mut file = fs::File::create(&path).expect("Failed to create temp file");
    file.write_all(content.as_bytes()).expect("Failed to write temp file");

    path
}

/// Cleans up temporary test files.
pub fn cleanup_temp_files() {
    let temp_dir = PathBuf::from("target").join("test_temp");
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir).ok();
    }
}
