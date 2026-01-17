pub mod generator;
pub mod struct_gen;
pub mod encode_gen;
pub mod decode_gen;
pub mod field_extractor;
pub mod utils;

use proc_macro2::TokenStream;

use crate::data_builder::transform::ir::IR;

pub fn generate(ir: &IR) -> TokenStream {
    generator::generate(ir)
}
