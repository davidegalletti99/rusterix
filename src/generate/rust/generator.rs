use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::transform::ir::*;
use super::{struct_gen, decode_gen, encode_gen};

pub fn generate(ir: &IR) -> TokenStream {
    let category = &ir.category;
    
    let items = generate_items(category);
    let record = generate_record(category);
    
    quote! {
        // AUTO-GENERATED CODE — DO NOT EDIT
        use rusterix::framework::bit_reader::BitReader;
        use rusterix::framework::bit_writer::BitWriter;
        use rusterix::framework::fspec::Fspec;
        use rusterix::framework::error::DecodeError;
        use std::io::{Read, Write};

        #record
        #(#items)*
    }
}

fn generate_items(category: &IRCategory) -> Vec<TokenStream> {
    category.items.iter().map(|item| {
        let struct_name = format_ident!("Item{:03}", item.id);
        
        let s = struct_gen::generate(&struct_name, &item.node);
        let d = decode_gen::generate_item(&struct_name, &item.node);
        let e = encode_gen::generate_item(&struct_name, &item.node);

        quote! {
            #s
            #d
            #e
        }
    }).collect()
}

fn generate_record(category: &IRCategory) -> TokenStream {
    let record_name = format_ident!("Cat{:03}", category.id);
    
    let fields: Vec<_> = category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        let item_type = format_ident!("Item{:03}", item.id);
        quote! {
            pub #field_name: Option<#item_type>
        }
    }).collect();
    
    let decode_fields = generate_record_decode(category);
    let encode_fspec_setup = generate_encode_fspec_setup(category);
    let encode_items_write = generate_encode_items_write(category);

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #record_name {
            #(#fields),*
        }

        impl #record_name {
            pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
                let fspec = Fspec::read(reader)?;
                let mut bit_reader = BitReader::new(reader);
                
                Ok(Self {
                    #(#decode_fields),*
                })
            }

            pub fn encode<W: Write>(&self, writer: &mut W) -> Result<(), DecodeError> {
                let mut fspec = Fspec::new();
                #(#encode_fspec_setup)*
                
                fspec.write(writer)?;
                let mut bit_writer = BitWriter::new(writer);
                #(#encode_items_write)*
                
                bit_writer.flush()?;
                
                Ok(())
            }
        }
    }
}

fn generate_record_decode(category: &IRCategory) -> Vec<TokenStream> {
    category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        let item_type = format_ident!("Item{:03}", item.id);
        
        let frn = item.frn as usize;
        let byte = frn / 8;
        let bit = 7 - (frn % 8);
        
        quote! {
            #field_name: if fspec.is_set(#byte, #bit) {
                Some(#item_type::decode(&mut bit_reader)?)
            } else {
                None
            }
        }
    }).collect()
}

fn generate_encode_fspec_setup(category: &IRCategory) -> Vec<TokenStream> {
    category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        
        let frn = item.frn as usize;
        let byte = frn / 8;
        let bit = 7 - (frn % 8);
        
        quote! {
            if self.#field_name.is_some() {
                fspec.set(#byte, #bit);
            }
        }
    }).collect()
}

fn generate_encode_items_write(category: &IRCategory) -> Vec<TokenStream> {
    category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        
        quote! {
            if let Some(ref item) = self.#field_name {
                item.encode(&mut bit_writer)?;
            }
        }
    }).collect()
}