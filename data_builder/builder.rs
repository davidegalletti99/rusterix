use std::{fs, path::PathBuf, vec};
use crate::data_builder::{
    generate::rust::generate,
    parse::parser::parse_category,
    transform::transformer::to_ir
};

pub trait Builder {
    fn build(&self, file_path: &str) -> Result<String, std::io::Error>;
}

pub struct RustBuilder {
    // Nothing for now
}

impl Builder for RustBuilder {
    fn build(&self, file_path: &str) -> Result<String, std::io::Error> {
        let xml = fs::read_to_string(file_path).expect("Failed to read {file_path}");

        let data = parse_category(&xml)
            .expect("Failed to parse layout XML");

        let out_path = PathBuf::from("./generated")
            .join("generated.rs");
        // Create directory if it doesn't exist
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create output directory");
        }
        let root = data; // Wrap single category into a vector
        let ir = to_ir(root);

        let result = generate(&ir);
        Ok(result.to_string())
    }
}

