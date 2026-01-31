use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::transform::ir::*;
use super::utils::to_snake_case;

/// Generates the encode implementation for a Fixed or Explicit item.
/// 
/// # Arguments
/// 
/// * `name` - The struct name
/// * `bytes` - Total size in bytes
/// * `elements` - List of elements to encode
/// * `is_explicit` - Whether this is an Explicit item (has length byte)
/// 
/// # Returns
/// 
/// TokenStream for the encode implementation.
pub fn generate_simple_encode(
    name: &Ident,
    bytes: usize,
    elements: &[IRElement],
    is_explicit: bool,
) -> TokenStream {
    let encode_fields = elements.iter().map(generate_element_encode);
    
    let len_write = if is_explicit {
        let len = bytes + 1; // Length includes itself
        quote! {
            writer.write_bits(#len as u64, 8)?;
        }
    } else {
        quote! {}
    };
    
    quote! {
        impl #name {
            pub fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
            ) -> Result<(), DecodeError> {
                #len_write
                #(#encode_fields)*
                Ok(())
            }
        }
    }
}

/// Generates encode code for a single element.
fn generate_element_encode(element: &IRElement) -> TokenStream {
    match element {
        IRElement::Field { name, bits } => {
            let field_name = to_snake_case(name);
            quote! {
                writer.write_bits(self.#field_name as u64, #bits)?;
            }
        }
        
        IRElement::EPB { content } => {
            let (field_name, encode_expr) = match content.as_ref() {
                IRElement::Field { name, bits } => {
                    let field_name = to_snake_case(name);
                    let expr = quote! {
                        if let Some(value) = self.#field_name {
                            writer.write_bits(1, 1)?; // Valid bit
                            writer.write_bits(value as u64, #bits)?;
                        } else {
                            writer.write_bits(0, 1)?; // Invalid bit
                            writer.write_bits(0, #bits)?; // Zero value
                        }
                    };
                    (field_name, expr)
                }
                
                IRElement::Enum { name, bits, .. } => {
                    let field_name = to_snake_case(name);
                    let expr = quote! {
                        if let Some(value) = self.#field_name {
                            writer.write_bits(1, 1)?; // Valid bit
                            writer.write_bits(u8::from(value) as u64, #bits)?;
                        } else {
                            writer.write_bits(0, 1)?; // Invalid bit
                            writer.write_bits(0, #bits)?; // Zero value
                        }
                    };
                    (field_name, expr)
                }
                
                _ => panic!("EPB can only contain Field or Enum"),
            };
            
            encode_expr
        }
        
        IRElement::Enum { name, bits, .. } => {
            let field_name = to_snake_case(name);
            quote! {
                writer.write_bits(u8::from(self.#field_name) as u64, #bits)?;
            }
        }
        
        IRElement::Spare { bits } => {
            quote! {
                writer.write_bits(0, #bits)?; // Write spare bits as zero
            }
        }
    }
}

/// Generates encode implementation for an Extended item.
pub fn generate_extended_encode(
    name: &Ident,
    part_groups: &[IRPartGroup],
) -> TokenStream {
    let mut part_encodes = Vec::new();
    
    for (i, group) in part_groups.iter().enumerate() {
        let field_name = format_ident!("part{}", group.index);
        let element_encodes: Vec<_> = group.elements.iter()
            .map(generate_element_encode)
            .collect();
        
        if i == 0 {
            // First part is always present
            let has_next = part_groups.len() > 1;
            let fx_value = if has_next { 1 } else { 0 };
            
            part_encodes.push(quote! {
                #(#element_encodes)*
                writer.write_bits(#fx_value, 1)?; // FX bit
            });
        } else {
            // Subsequent parts are optional
            let has_next = i < part_groups.len() - 1;
            let fx_value = if has_next { 1 } else { 0 };
            
            part_encodes.push(quote! {
                if let Some(ref part_data) = self.#field_name {
                    // TODO: Access part_data fields for encoding
                    #(#element_encodes)*
                    writer.write_bits(#fx_value, 1)?; // FX bit
                }
            });
        }
    }
    
    quote! {
        impl #name {
            pub fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
            ) -> Result<(), DecodeError> {
                #(#part_encodes)*
                Ok(())
            }
        }
    }
}

/// Generates encode implementation for a Repetitive item.
pub fn generate_repetitive_encode(
    name: &Ident,
    elements: &[IRElement],
    element_type_name: &Ident,
) -> TokenStream {
    let element_encodes = elements.iter().map(generate_element_encode);
    
    quote! {
        impl #element_type_name {
            fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
            ) -> Result<(), DecodeError> {
                #(#element_encodes)*
                Ok(())
            }
        }
        
        impl #name {
            pub fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
            ) -> Result<(), DecodeError> {
                for item in &self.items {
                    item.encode(writer)?;
                }
                Ok(())
            }
        }
    }
}

/// Generates encode implementation for a Compound item.
pub fn generate_compound_encode(
    name: &Ident,
    sub_items: &[IRSubItem],
) -> TokenStream {
    let mut fspec_setup = Vec::new();
    let mut sub_encodes = Vec::new();
    
    for sub_item in sub_items {
        let sub_name = format_ident!("{}_sub{}", name, sub_item.index);
        let field_name = format_ident!("sub{}", sub_item.index);
        
        let byte = sub_item.index / 8;
        let bit = 7 - (sub_item.index % 8);
        
        fspec_setup.push(quote! {
            if self.#field_name.is_some() {
                fspec.set(#byte, #bit);
            }
        });
        
        sub_encodes.push(quote! {
            if let Some(ref sub_data) = self.#field_name {
                sub_data.encode(&mut writer)?;
            }
        });
    }
    
    quote! {
        impl #name {
            pub fn encode<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> Result<(), DecodeError> {
                let mut fspec = Fspec::new();
                #(#fspec_setup)*
                fspec.write(writer)?;
                
                let mut writer = BitWriter::new(writer);
                #(#sub_encodes)*
                
                writer.flush()?;
                Ok(())
            }
        }
    }
}