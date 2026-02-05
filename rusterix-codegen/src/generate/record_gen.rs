use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::transform::ir::*;
use super::utils::frn_to_fspec_position;

/// Generates the Cat{N}Record struct and its implementations.
/// 
/// The record struct contains all items as Option fields, with an FSPEC
/// that is automatically managed during decode/encode.
/// 
/// # Arguments
/// 
/// * `category` - The category IR to generate the record for
/// 
/// # Returns
/// 
/// TokenStream containing the record struct and implementations.
pub fn generate_record(category: &IRCategory) -> TokenStream {
    let record_name = format_ident!("Cat{:03}Record", category.id);
    
    // Generate fields: all items as Option<ItemXXX>
    let fields: Vec<_> = category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        let item_type = format_ident!("Item{:03}", item.id);
        quote! {
            pub #field_name: Option<#item_type>
        }
    }).collect();
    
    // Generate decode implementation
    let decode_impl = generate_record_decode(category, &record_name);
    
    // Generate encode implementation
    let encode_impl = generate_record_encode(category, &record_name);
    
    quote! {
        /// ASTERIX Category record.
        /// 
        /// Contains optional data items, each controlled by a bit in the FSPEC.
        #[derive(Debug, Clone, PartialEq)]
        pub struct #record_name {
            #(#fields),*
        }
        
        #decode_impl
        
        #encode_impl
    }
}

/// Generates the decode implementation for a record.
fn generate_record_decode(category: &IRCategory, record_name: &proc_macro2::Ident) -> TokenStream {
    let decode_fields: Vec<_> = category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        let item_type = format_ident!("Item{:03}", item.id);
        
        let (byte, bit) = frn_to_fspec_position(item.frn as usize);
        
        quote! {
            #field_name: if fspec.is_set(#byte, #bit) {
                Some(#item_type::decode(&mut bit_reader)?)
            } else {
                None
            }
        }
    }).collect();
    
    quote! {
        impl #record_name {
            /// Decodes a record from a binary stream.
            /// 
            /// Reads the FSPEC to determine which items are present, then
            /// decodes only the present items.
            /// 
            /// # Arguments
            /// 
            /// * `reader` - The input stream to read from
            /// 
            /// # Errors
            /// 
            /// Returns an error if reading or parsing fails.
            pub fn decode<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError> {
                let fspec = Fspec::read(reader)?;
                let mut bit_reader = BitReader::new(reader);
                
                Ok(Self {
                    #(#decode_fields),*
                })
            }
        }
    }
}

/// Generates the encode implementation for a record.
fn generate_record_encode(category: &IRCategory, record_name: &proc_macro2::Ident) -> TokenStream {
    // Generate FSPEC setup - set bits for present items
    let fspec_setup: Vec<_> = category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        let (byte, bit) = frn_to_fspec_position(item.frn as usize);
        
        quote! {
            if self.#field_name.is_some() {
                fspec.set(#byte, #bit);
            }
        }
    }).collect();
    
    // Generate item encoding - encode present items
    let encode_items: Vec<_> = category.items.iter().map(|item| {
        let field_name = format_ident!("item{:03}", item.id);
        
        quote! {
            if let Some(ref item) = self.#field_name {
                item.encode(&mut bit_writer)?;
            }
        }
    }).collect();
    
    quote! {
        impl #record_name {
            /// Encodes a record to a binary stream.
            /// 
            /// Automatically constructs the FSPEC based on which items are present,
            /// then encodes all present items.
            /// 
            /// # Arguments
            /// 
            /// * `writer` - The output stream to write to
            /// 
            /// # Errors
            /// 
            /// Returns an error if writing fails.
            pub fn encode<W: std::io::Write>(&self, writer: &mut W) -> Result<(), DecodeError> {
                // Build FSPEC based on present items
                let mut fspec = Fspec::new();
                #(#fspec_setup)*
                
                // Write FSPEC
                fspec.write(writer)?;
                
                // Write items
                let mut bit_writer = BitWriter::new(writer);
                #(#encode_items)*
                
                bit_writer.flush()?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_record() {
        let category = IRCategory {
            id: 48,
            items: vec![
                IRItem {
                    id: 10,
                    frn: 0,
                    layout: IRLayout::Fixed {
                        bytes: 2,
                        elements: vec![],
                    },
                },
                IRItem {
                    id: 20,
                    frn: 1,
                    layout: IRLayout::Fixed {
                        bytes: 1,
                        elements: vec![],
                    },
                },
            ],
        };
        
        let result = generate_record(&category);
        let code = result.to_string();
        
        assert!(code.contains("pub struct Cat048Record"));
        assert!(code.contains("pub item010 : Option < Item010 >"));
        assert!(code.contains("pub item020 : Option < Item020 >"));
        assert!(code.contains("pub fn decode"));
        assert!(code.contains("pub fn encode"));
    }
}