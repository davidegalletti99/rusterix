use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::transform::ir::*;
use super::utils::{to_snake_case, frn_to_fspec_position};

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
        impl Encode for #name {
            fn encode<W: std::io::Write>(
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
            let (_field_name, encode_expr) = match content.as_ref() {
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
///
/// Creates:
/// - An encode method for each PartX struct (encodes only the 7 data bits)
/// - An encode method for the main struct (calls part encodes and handles FX bits)
pub fn generate_extended_encode(
    name: &Ident,
    part_groups: &[IRPartGroup],
) -> TokenStream {
    let mut part_impl_tokens = Vec::new();
    let mut main_encode_body = Vec::new();

    let total_parts = part_groups.len();

    for (i, group) in part_groups.iter().enumerate() {
        let part_name = format_ident!("{}Part{}", name, group.index);
        let field_name = format_ident!("part{}", group.index);

        // Generate element encode statements for this part
        let element_encodes: Vec<_> = group.elements.iter()
            .map(generate_element_encode)
            .collect();

        // Generate encode impl for this part struct (7 data bits only, no FX)
        part_impl_tokens.push(quote! {
            impl #part_name {
                pub fn encode<W: std::io::Write>(
                    &self,
                    writer: &mut BitWriter<W>,
                ) -> Result<(), DecodeError> {
                    #(#element_encodes)*
                    Ok(())
                }
            }
        });

        // Generate main struct encode logic for this part
        if i == 0 {
            // First part is always present
            if total_parts > 1 {
                // Check if next part exists to determine FX
                let next_field = format_ident!("part{}", i + 1);
                main_encode_body.push(quote! {
                    self.#field_name.encode(writer)?;
                    writer.write_bits(self.#next_field.is_some() as u64, 1)?; // FX bit
                });
            } else {
                // No more parts possible, FX = 0
                main_encode_body.push(quote! {
                    self.#field_name.encode(writer)?;
                    writer.write_bits(0, 1)?; // FX bit = 0, no extension
                });
            }
        } else {
            // Subsequent parts are optional
            if i < total_parts - 1 {
                // There could be more parts after this one
                let next_field = format_ident!("part{}", i + 1);
                main_encode_body.push(quote! {
                    if let Some(ref part_data) = self.#field_name {
                        part_data.encode(writer)?;
                        writer.write_bits(self.#next_field.is_some() as u64, 1)?; // FX bit
                    }
                });
            } else {
                // This is the last possible part, FX = 0
                main_encode_body.push(quote! {
                    if let Some(ref part_data) = self.#field_name {
                        part_data.encode(writer)?;
                        writer.write_bits(0, 1)?; // FX bit = 0, no more extension
                    }
                });
            }
        }
    }

    quote! {
        #(#part_impl_tokens)*

        impl Encode for #name {
            fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
            ) -> Result<(), DecodeError> {
                #(#main_encode_body)*
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

        impl Encode for #name {
            fn encode<W: std::io::Write>(
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
        let _sub_name = format_ident!("{}Sub{}", name, sub_item.index);
        let field_name = format_ident!("sub{}", sub_item.index);

        let (byte, bit) = frn_to_fspec_position(sub_item.index);
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

/// Generates encode implementations for all sub-items in a compound.
pub fn generate_compound_sub_encodes(
    parent_name: &Ident,
    sub_items: &[IRSubItem],
) -> TokenStream {
    let mut all_impls = Vec::new();

    for sub_item in sub_items {
        let sub_name = format_ident!("{}Sub{}", parent_name, sub_item.index);

        let impl_tokens = match &sub_item.layout {
            IRLayout::Fixed { bytes, elements } => {
                generate_simple_encode(&sub_name, *bytes, elements, false)
            }

            IRLayout::Explicit { bytes, elements } => {
                generate_simple_encode(&sub_name, *bytes, elements, true)
            }

            IRLayout::Extended { part_groups, .. } => {
                generate_extended_encode(&sub_name, part_groups)
            }

            IRLayout::Repetitive { elements, .. } => {
                let element_type = format_ident!("{}Element", sub_name);
                generate_repetitive_encode(&sub_name, elements, &element_type)
            }

            IRLayout::Compound { .. } => {
                panic!("Nested compounds not supported")
            }
        };

        all_impls.push(impl_tokens);
    }

    quote! {
        #(#all_impls)*
    }
}