use std::{fs, path::PathBuf};
mod data_builder;
use data_builder::{builder::RustBuilder};

use crate::data_builder::{builder::Builder};

fn main() {
    let builder = RustBuilder {};
    let data = builder.build("test.xml").expect("Failed to build data");

    let out_path = PathBuf::from("./generated")
        .join("generated.rs");
    // Create directory if it doesn't exist
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    fs::write(&out_path, data).expect("Failed to write generated.rs");

}