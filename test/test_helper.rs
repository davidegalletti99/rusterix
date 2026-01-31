/// Test helper utilities for loading fixtures and common assertions.

use std::fs;
use std::path::{Path, PathBuf};

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
/// 
/// # Panics
/// 
/// Panics if the file cannot be read
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
    
    let filename = format!("test_{}.{}", uuid::Uuid::new_v4(), extension);
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fixture_path() {
        let path = fixture_path("valid", "test.xml");
        assert!(path.to_str().unwrap().contains("tests/fixtures/valid/test.xml"));
    }
    
    #[test]
    fn test_normalize_whitespace() {
        let code = "pub   struct\n  Test {\n    field: u8\n  }";
        let normalized = normalize_whitespace(code);
        assert_eq!(normalized, "pub struct Test { field: u8 }");
    }
}