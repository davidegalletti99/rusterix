use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::{generate::utils::frn_to_fspec_position, transform::ir::*};
use super::utils::{rust_type_for_bits, to_pascal_case, to_snake_case};

/// Generates the decode implementation for a Fixed or Explicit item.
///
/// # Arguments
///
/// * `name` - The struct name
/// * `elements` - List of elements to decode
/// * `is_explicit` - Whether this is an Explicit item (has length byte)
///
/// # Returns
///
/// TokenStream for the decode implementation.
pub fn generate_simple_decode(
    name: &Ident,
    elements: &[IRElement],
    is_explicit: bool,
) -> TokenStream {
    let decode_fields = elements.iter().map(generate_element_decode);

    let field_names: Vec<_> = elements
        .iter()
        .filter(|e| e.is_visible())
        .filter_map(|e| match e {
            IRElement::Field { name, .. } => Some(to_snake_case(name)),
            IRElement::Enum { name, .. } => Some(to_snake_case(name)),
            IRElement::EPB { content } => match content.as_ref() {
                IRElement::Field { name, .. } => Some(to_snake_case(name)),
                IRElement::Enum { name, .. } => Some(to_snake_case(name)),
                _ => None,
            },
            _ => None,
        })
        .collect();

    let len_check = if is_explicit {
        quote! {
            let _len = reader.read_bits(8)? as usize;
            // Length includes itself, so actual data is len - 1 bytes
        }
    } else {
        quote! {}
    };

    quote! {
        impl Decode for #name {
            fn decode<R: std::io::Read>(
                reader: &mut BitReader<R>,
            ) -> Result<Self, DecodeError> {
                #len_check
                #(#decode_fields)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    }
}

/// Generates decode code for a single element.
fn generate_element_decode(element: &IRElement) -> TokenStream {
    match element {
        IRElement::Field { name, bits } => {
            let field_name = to_snake_case(name);
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            quote! {
                let #field_name = reader.read_bits(#bits)? as #ty;
            }
        }
        
        IRElement::EPB { content } => {
            let (_field_name, decode_expr) = match content.as_ref() {
                IRElement::Field { name, bits } => {
                    let field_name = to_snake_case(name);
                    let ty = format_ident!("{}", rust_type_for_bits(*bits));
                    let expr = quote! {
                        let #field_name = {
                            let valid = reader.read_bits(1)? != 0;
                            if valid {
                                Some(reader.read_bits(#bits)? as #ty)
                            } else {
                                reader.read_bits(#bits)?; // Skip the value
                                None
                            }
                        };
                    };
                    (field_name, expr)
                }
                
                IRElement::Enum { name, bits, .. } => {
                    let field_name = to_snake_case(name);
                    let enum_type = to_pascal_case(name);
                    let expr = quote! {
                        let #field_name = {
                            let valid = reader.read_bits(1)? != 0;
                            if valid {
                                let value = reader.read_bits(#bits)? as u8;
                                Some(#enum_type::try_from(value).unwrap())
                            } else {
                                reader.read_bits(#bits)?; // Skip the value
                                None
                            }
                        };
                    };
                    (field_name, expr)
                }
                
                _ => panic!("EPB can only contain Field or Enum"),
            };
            
            decode_expr
        }
        
        IRElement::Enum { name, bits, .. } => {
            let field_name = to_snake_case(name);
            let enum_type = to_pascal_case(name);
            quote! {
                let #field_name = {
                    let value = reader.read_bits(#bits)? as u8;
                    #enum_type::try_from(value).unwrap()
                };
            }
        }
        
        IRElement::Spare { bits } => {
            quote! {
                reader.read_bits(#bits)?; // Skip spare bits
            }
        }
    }
}

/// Generates decode implementation for an Extended item.
///
/// Creates:
/// - A decode method for each PartX struct (decodes only the 7 data bits)
/// - A decode method for the main struct (calls part decodes and handles FX bits)
pub fn generate_extended_decode(
    name: &Ident,
    part_groups: &[IRPartGroup],
) -> TokenStream {
    let mut part_impl_tokens = Vec::new();
    let mut main_decode_body = Vec::new();
    let mut field_names = Vec::new();

    for (i, group) in part_groups.iter().enumerate() {
        let part_name = format_ident!("{}Part{}", name, group.index);
        let field_name = format_ident!("part{}", group.index);
        field_names.push(field_name.clone());

        // Generate element decode statements for this part
        let element_decodes: Vec<_> = group.elements.iter()
            .map(generate_element_decode)
            .collect();

        // Collect visible field names for struct construction
        let element_names: Vec<_> = group.elements.iter()
            .filter(|e| e.is_visible())
            .filter_map(|e| match e {
                IRElement::Field { name, .. } => Some(to_snake_case(name)),
                IRElement::Enum { name, .. } => Some(to_snake_case(name)),
                IRElement::EPB { content } => match content.as_ref() {
                    IRElement::Field { name, .. } => Some(to_snake_case(name)),
                    IRElement::Enum { name, .. } => Some(to_snake_case(name)),
                    _ => None,
                },
                _ => None,
            })
            .collect();

        // Generate decode impl for this part struct (7 data bits only, no FX)
        part_impl_tokens.push(quote! {
            impl #part_name {
                pub fn decode<R: std::io::Read>(
                    reader: &mut BitReader<R>,
                ) -> Result<Self, DecodeError> {
                    #(#element_decodes)*
                    Ok(Self { #(#element_names),* })
                }
            }
        });

        // Generate main struct decode logic for this part
        if i == 0 {
            // First part is always present
            main_decode_body.push(quote! {
                let #field_name = #part_name::decode(reader)?;
                let mut fx = reader.read_bits(1)? != 0;
            });
        } else {
            // Subsequent parts depend on previous FX bit
            main_decode_body.push(quote! {
                let #field_name = if fx {
                    let part = #part_name::decode(reader)?;
                    fx = reader.read_bits(1)? != 0;
                    Some(part)
                } else {
                    None
                };
            });
        }
    }

    quote! {
        #(#part_impl_tokens)*

        impl Decode for #name {
            fn decode<R: std::io::Read>(
                reader: &mut BitReader<R>,
            ) -> Result<Self, DecodeError> {
                #(#main_decode_body)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    }
}

/// Generates decode implementation for a Repetitive item.
pub fn generate_repetitive_decode(
    name: &Ident,
    count: usize,
    elements: &[IRElement],
    element_type_name: &Ident,
) -> TokenStream {
    let element_decodes = elements.iter().map(generate_element_decode);
    
    let field_names: Vec<_> = elements
        .iter()
        .filter(|e| e.is_visible())
        .filter_map(|e| match e {
            IRElement::Field { name, .. } => Some(to_snake_case(name)),
            IRElement::Enum { name, .. } => Some(to_snake_case(name)),
            IRElement::EPB { content } => match content.as_ref() {
                IRElement::Field { name, .. } => Some(to_snake_case(name)),
                IRElement::Enum { name, .. } => Some(to_snake_case(name)),
                _ => None,
            },
            _ => None,
        })
        .collect();
    
    quote! {
        impl #element_type_name {
            fn decode<R: std::io::Read>(
                reader: &mut BitReader<R>,
            ) -> Result<Self, DecodeError> {
                #(#element_decodes)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }

        impl Decode for #name {
            fn decode<R: std::io::Read>(
                reader: &mut BitReader<R>,
            ) -> Result<Self, DecodeError> {
                let mut items = Vec::with_capacity(#count);
                for _ in 0..#count {
                    items.push(#element_type_name::decode(reader)?);
                }

                Ok(Self { items })
            }
        }
    }
}

/// Generates decode implementation for a Compound item.
pub fn generate_compound_decode(
    name: &Ident,
    sub_items: &[IRSubItem],
) -> TokenStream {
    let mut sub_decodes = Vec::new();
    let mut field_names = Vec::new();

    for sub_item in sub_items {
        let sub_name = format_ident!("{}Sub{}", name, sub_item.index);
        let field_name = format_ident!("sub{}", sub_item.index);
        field_names.push(field_name.clone());

        let (byte, bit) = frn_to_fspec_position(sub_item.index);
        sub_decodes.push(quote! {
            let #field_name = if fspec.is_set(#byte, #bit) {
                Some(#sub_name::decode(&mut reader)?)
            } else {
                None
            };
        });
    }

    quote! {
        impl #name {
            pub fn decode<R: std::io::Read>(
                reader: &mut R,
            ) -> Result<Self, DecodeError> {
                let fspec = Fspec::read(reader)?;
                let mut reader = BitReader::new(reader);

                #(#sub_decodes)*

                Ok(Self {
                    #(#field_names),*
                })
            }
        }
    }
}

/// Generates decode implementations for all sub-items in a compound.
pub fn generate_compound_sub_decodes(
    parent_name: &Ident,
    sub_items: &[IRSubItem],
) -> TokenStream {
    let mut all_impls = Vec::new();

    for sub_item in sub_items {
        let sub_name = format_ident!("{}Sub{}", parent_name, sub_item.index);

        let impl_tokens = match &sub_item.layout {
            IRLayout::Fixed { elements, .. } => {
                generate_simple_decode(&sub_name, elements, false)
            }

            IRLayout::Explicit { elements, .. } => {
                generate_simple_decode(&sub_name, elements, true)
            }

            IRLayout::Extended { part_groups, .. } => {
                generate_extended_decode(&sub_name, part_groups)
            }

            IRLayout::Repetitive { count, elements, .. } => {
                let element_type = format_ident!("{}Element", sub_name);
                generate_repetitive_decode(&sub_name, *count, elements, &element_type)
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