use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::transform::ir::*;
use super::utils::{rust_type_for_bits, to_pascal_case, to_snake_case};

/// Generates a struct field declaration.
/// 
/// # Arguments
/// 
/// * `element` - The IR element to generate a field for
/// 
/// # Returns
/// 
/// A TokenStream for the field declaration, or None for spare bits.
/// 
/// # Panics
/// 
/// Panics if EPB content is not Field or Enum.
pub fn generate_field(element: &IRElement) -> Option<TokenStream> {
    match element {
        IRElement::Field { name, bits } => {
            let field_name = to_snake_case(name);
            let field_type = format_ident!("{}", rust_type_for_bits(*bits));
            Some(quote! { pub #field_name: #field_type })
        }
        
        IRElement::EPB { content } => {
            let (field_name, inner_type) = match content.as_ref() {
                IRElement::Field { name, bits } => {
                    let field_name = to_snake_case(name);
                    let ty = format_ident!("{}", rust_type_for_bits(*bits));
                    (field_name, quote! { #ty })
                }
                IRElement::Enum { name, .. } => {
                    let field_name = to_snake_case(name);
                    let ty = to_pascal_case(name);
                    (field_name, quote! { #ty })
                }
                _ => panic!("EPB can only contain Field or Enum"),
            };
            
            Some(quote! { pub #field_name: Option<#inner_type> })
        }
        
        IRElement::Enum { name, .. } => {
            let field_name = to_snake_case(name);
            let enum_type = to_pascal_case(name);
            Some(quote! { pub #field_name: #enum_type })
        }
        
        IRElement::Spare { .. } => None,
    }
}

/// Generates a complete struct definition with all visible fields.
/// 
/// # Arguments
/// 
/// * `name` - The struct name
/// * `elements` - List of IR elements to include as fields
/// 
/// # Returns
/// 
/// TokenStream for the struct definition.
/// 
/// # Panics
/// 
/// Panics if duplicate field names are detected within the same scope.
pub fn generate_struct(name: &Ident, elements: &[IRElement]) -> TokenStream {
    // Validate no duplicate field names in this scope
    validate_no_duplicates(name, elements);
    
    let fields: Vec<_> = elements
        .iter()
        .filter_map(generate_field)
        .collect();
    
    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            #(#fields),*
        }
    }
}

/// Validates that there are no duplicate field names in the same scope.
/// 
/// # Panics
/// 
/// Panics with a descriptive error if duplicates are found.
fn validate_no_duplicates(struct_name: &Ident, elements: &[IRElement]) {
    let mut seen_names = std::collections::HashSet::new();
    
    for element in elements {
        if let Some(name) = get_field_name(element) {
            if !seen_names.insert(name.clone()) {
                panic!(
                    "Duplicate field name '{}' detected in struct {}. \
                    Each field, enum, and EPB content must have a unique name within the same scope.",
                    name, struct_name
                );
            }
        }
    }
}

/// Extracts the field name from an IR element.
fn get_field_name(element: &IRElement) -> Option<String> {
    match element {
        IRElement::Field { name, .. } => Some(name.clone()),
        IRElement::Enum { name, .. } => Some(name.clone()),
        IRElement::EPB { content } => {
            match content.as_ref() {
                IRElement::Field { name, .. } => Some(name.clone()),
                IRElement::Enum { name, .. } => Some(name.clone()),
                _ => None,
            }
        }
        IRElement::Spare { .. } => None,
    }
}

/// Generates a struct for a repetitive item.
/// 
/// Creates a struct with a Vec field containing the repeated elements.
/// 
/// # Arguments
/// 
/// * `name` - The struct name
/// * `elements` - Elements of a single repetition
/// * `element_type_name` - Name for the nested element type
/// 
/// # Returns
/// 
/// TokenStream for both the element struct and the container struct.
pub fn generate_repetitive_struct(
    name: &Ident,
    elements: &[IRElement],
    element_type_name: &Ident,
) -> TokenStream {
    let element_struct = generate_struct(element_type_name, elements);
    
    quote! {
        #element_struct
        
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            pub items: Vec<#element_type_name>,
        }
    }
}

/// Generates structs for an extended item with multiple parts.
/// 
/// Creates:
/// - A struct for each part (Part0, Part1, etc.)
/// - A main struct with the first part (always present) and optional subsequent parts
/// 
/// # Arguments
/// 
/// * `name` - The main struct name
/// * `part_groups` - All parts in the extended item
/// 
/// # Returns
/// 
/// TokenStream for all structs.
pub fn generate_extended_structs(
    name: &Ident,
    part_groups: &[IRPartGroup],
) -> TokenStream {
    let mut all_structs = Vec::new();
    let mut main_fields = Vec::new();
    
    for group in part_groups {
        let part_name = format_ident!("{}Part{}", name, group.index);
        let part_struct = generate_struct(&part_name, &group.elements);
        all_structs.push(part_struct);
        
        let field_name = format_ident!("part{}", group.index);
        
        if group.index == 0 {
            // First part is always present
            main_fields.push(quote! { pub #field_name: #part_name });
        } else {
            // Subsequent parts are optional (depend on FX bit)
            main_fields.push(quote! { pub #field_name: Option<#part_name> });
        }
    }
    
    quote! {
        #(#all_structs)*
        
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            #(#main_fields),*
        }
    }
}

/// Generates structs for a compound item with sub-items.
/// 
/// Creates:
/// - A struct for each sub-item
/// - A main struct with all sub-items as Option fields
/// 
/// # Arguments
/// 
/// * `name` - The main struct name
/// * `sub_items` - All sub-items in the compound
/// 
/// # Returns
/// 
/// TokenStream for all structs.
pub fn generate_compound_structs(
    name: &Ident,
    sub_items: &[IRSubItem],
) -> TokenStream {
    let mut all_structs = Vec::new();
    let mut main_fields = Vec::new();
    
    for sub_item in sub_items {
        let sub_name = format_ident!("{}_sub{}", name, sub_item.index);
        
        // Generate struct for this sub-item based on its layout
        let sub_struct = match &sub_item.layout {
            IRLayout::Fixed { elements, .. } | IRLayout::Explicit { elements, .. } => {
                generate_struct(&sub_name, elements)
            }
            
            IRLayout::Extended { part_groups } => {
                generate_extended_structs(&sub_name, part_groups)
            }
            
            IRLayout::Repetitive { elements, .. } => {
                let element_type = format_ident!("{}Element", sub_name);
                generate_repetitive_struct(&sub_name, elements, &element_type)
            }
            
            IRLayout::Compound { .. } => {
                panic!("Nested compounds not supported")
            }
        };
        
        all_structs.push(sub_struct);
        
        let field_name = format_ident!("sub{}", sub_item.index);
        main_fields.push(quote! { pub #field_name: Option<#sub_name> });
    }
    
    quote! {
        #(#all_structs)*
        
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            #(#main_fields),*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_field() {
        let field = IRElement::Field {
            name: "test_field".to_string(),
            bits: 8,
        };
        
        let result = generate_field(&field).unwrap();
        let code = result.to_string();
        
        assert!(code.contains("pub test_field : u8"));
    }
    
    #[test]
    fn test_generate_field_epb() {
        let epb = IRElement::EPB {
            content: Box::new(IRElement::Field {
                name: "optional_field".to_string(),
                bits: 16,
            }),
        };
        
        let result = generate_field(&epb).unwrap();
        let code = result.to_string();
        
        assert!(code.contains("pub optional_field : Option < u16 >"));
    }
    
    #[test]
    fn test_generate_field_spare() {
        let spare = IRElement::Spare { bits: 3 };
        
        let result = generate_field(&spare);
        
        assert!(result.is_none());
    }
}