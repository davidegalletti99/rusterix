pub mod rust;

use proc_macro2::TokenStream;

use crate::data_builder::transform::ir::IR;

/// Generate source code from IR
pub fn generate(ir: &IR) -> TokenStream {
    rust::generate(ir)
}
