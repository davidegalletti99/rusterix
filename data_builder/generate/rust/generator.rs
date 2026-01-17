use proc_macro2::TokenStream;
use quote::quote;

use crate::data_builder::transform::ir::IR;
use super::{struct_gen, decode_gen, encode_gen};

pub fn generate(ir: &IR) -> TokenStream {
    let category = &ir.category;
    
    let mut items = Vec::new();
    for item in &category.items {
        let struct_name =
            format!("Item{:03}", item.id);

        let s = struct_gen::generate(&struct_name, &item.node);
        let d = decode_gen::generate(&struct_name, &item.node);
        let e = encode_gen::generate(&struct_name, &item.node);

        items.push(quote! {
            #s
            #d
            #e
        });
    }

    quote! {
        // AUTO-GENERATED CODE — DO NOT EDIT
        use rusterix::framework::bit_reader::BitReader;
        use rusterix::framework::bit_writer::BitWriter;
        use rusterix::framework::fspec::Fspec;
        use rusterix::framework::error::DecodeError;

        #(#items)*
    }
}
