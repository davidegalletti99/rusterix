use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::transform::ir::*;
use super::{
    struct_gen::*,
    decode_gen::*,
    encode_gen::*,
    enum_gen::*,
};

/// Generates all code for a single ASTERIX item.
/// 
/// This includes:
/// - Enum definitions for any enum fields
/// - Struct definition(s) for the item
/// - Decode implementation
/// - Encode implementation
/// 
/// # Arguments
/// 
/// * `item` - The IR item to generate code for
/// 
/// # Returns
/// 
/// TokenStream containing all generated code for this item.
pub fn generate_item(item: &IRItem) -> TokenStream {
    let item_name = format_ident!("Item{:03}", item.id);
    
    // Collect all enums that need to be generated
    let enums = collect_enums(&item.layout);
    
    // Generate enum definitions
    let enum_defs: Vec<_> = enums.iter().map(|(name, bits, values)| {
        generate_enum(name, *bits, values)
    }).collect();
    
    // Generate struct and implementations based on layout type
    let (struct_def, decode_impl, encode_impl) = match &item.layout {
        IRLayout::Fixed { bytes, elements } => {
            let struct_def = generate_struct(&item_name, elements);
            let decode_impl = generate_simple_decode(&item_name, elements, false);
            let encode_impl = generate_simple_encode(&item_name, *bytes, elements, false);
            (struct_def, decode_impl, encode_impl)
        }
        
        IRLayout::Explicit { bytes, elements } => {
            let struct_def = generate_struct(&item_name, elements);
            let decode_impl = generate_simple_decode(&item_name, elements, true);
            let encode_impl = generate_simple_encode(&item_name, *bytes, elements, true);
            (struct_def, decode_impl, encode_impl)
        }
        
        IRLayout::Extended { bytes: _, part_groups } => {
            let struct_def = generate_extended_structs(&item_name, part_groups);
            let decode_impl = generate_extended_decode(&item_name, part_groups);
            let encode_impl = generate_extended_encode(&item_name, part_groups);
            (struct_def, decode_impl, encode_impl)
        }
        
        IRLayout::Repetitive { bytes: _, count, elements } => {
            let element_type = format_ident!("{}Element", item_name);
            let struct_def = generate_repetitive_struct(&item_name, elements, &element_type);
            let decode_impl = generate_repetitive_decode(&item_name, *count, elements, &element_type);
            let encode_impl = generate_repetitive_encode(&item_name, elements, &element_type);
            (struct_def, decode_impl, encode_impl)
        }
        
        IRLayout::Compound { sub_items } => {
            let struct_def = generate_compound_structs(&item_name, sub_items);
            let sub_decode_impls = generate_compound_sub_decodes(&item_name, sub_items);
            let sub_encode_impls = generate_compound_sub_encodes(&item_name, sub_items);
            let decode_impl = generate_compound_decode(&item_name, sub_items);
            let encode_impl = generate_compound_encode(&item_name, sub_items);

            let combined_decode = quote! {
                #sub_decode_impls
                #decode_impl
            };
            let combined_encode = quote! {
                #sub_encode_impls
                #encode_impl
            };
            (struct_def, combined_decode, combined_encode)
        }
    };
    
    quote! {
        #(#enum_defs)*
        
        #struct_def
        
        #decode_impl
        
        #encode_impl
    }
}

/// Recursively collects all enum definitions from a layout.
/// 
/// Returns a vector of (name, bits, values) tuples.
fn collect_enums(layout: &IRLayout) -> Vec<(String, usize, Vec<(String, u8)>)> {
    let mut enums = Vec::new();
    
    match layout {
        IRLayout::Fixed { elements, .. } | IRLayout::Explicit { elements, .. } => {
            collect_enums_from_elements(elements, &mut enums);
        }
        
        IRLayout::Extended { bytes: _, part_groups } => {
            for group in part_groups {
                collect_enums_from_elements(&group.elements, &mut enums);
            }
        }
        
        IRLayout::Repetitive { elements, .. } => {
            collect_enums_from_elements(elements, &mut enums);
        }
        
        IRLayout::Compound { sub_items } => {
            for sub_item in sub_items {
                let sub_enums = collect_enums(&sub_item.layout);
                enums.extend(sub_enums);
            }
        }
    }
    
    enums
}

fn collect_enums_from_elements(
    elements: &[IRElement],
    enums: &mut Vec<(String, usize, Vec<(String, u8)>)>,
) {
    for element in elements {
        match element {
            IRElement::Enum { name, bits, values } => {
                enums.push((name.clone(), *bits, values.clone()));
            }
            
            IRElement::EPB { content } => {
                if let IRElement::Enum { name, bits, values } = content.as_ref() {
                    enums.push((name.clone(), *bits, values.clone()));
                }
            }
            
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collect_enums() {
        let elements = vec![
            IRElement::Field {
                name: "sac".to_string(),
                bits: 8,
            },
            IRElement::Enum {
                name: "target_type".to_string(),
                bits: 2,
                values: vec![
                    ("PSR".to_string(), 1),
                    ("SSR".to_string(), 2),
                ],
            },
        ];
        
        let layout = IRLayout::Fixed {
            bytes: 2,
            elements,
        };
        
        let enums = collect_enums(&layout);
        
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].0, "target_type");
        assert_eq!(enums[0].1, 2);
        assert_eq!(enums[0].2.len(), 2);
    }
}