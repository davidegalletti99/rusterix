use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::data_builder::transform::ir::{IrNode, IRLayout, IRCondition, IRCounter};
use super::utils::rust_type_for_bits;

// Renamed from generate() to generate_item() for clarity
pub fn generate_item(name: &Ident, node: &IrNode) -> TokenStream {
    let decode_impl = generate_decode_impl(node);

    quote! {
        impl #name {
            pub fn decode<R: std::io::Read>(
                reader: &mut BitReader<R>,
            ) -> Result<Self, DecodeError> {
                #decode_impl
            }
        }
    }
}

fn generate_decode_impl(node: &IrNode) -> TokenStream {
    match &node.layout {
        IRLayout::Sequence { elements } => {
            let has_optional = elements.iter().any(|e| matches!(e.layout, IRLayout::Optional { .. }));
            
            if has_optional {
                let field_decodes: Vec<_> = elements.iter()
                    .map(|e| decode_field_with_fspec(e))
                    .collect();
                
                let field_names: Vec<_> = elements.iter()
                    .filter_map(|e| match &e.layout {
                        IRLayout::Primitive { .. } | IRLayout::Optional { .. } | IRLayout::Repetition { .. } => {
                            Some(format_ident!("{}", e.name))
                        }
                        _ => None
                    })
                    .collect();
                
                quote! {
                    let item_fspec = Fspec::read(reader)?;
                    #(#field_decodes)*
                    
                    Ok(Self {
                        #(#field_names),*
                    })
                }
            } else {
                let field_decodes: Vec<_> = elements.iter()
                    .map(decode_field_simple)
                    .collect();
                
                let field_names: Vec<_> = elements.iter()
                    .map(|e| format_ident!("{}", e.name))
                    .collect();
                
                quote! {
                    #(#field_decodes)*
                    
                    Ok(Self {
                        #(#field_names),*
                    })
                }
            }
        }
        _ => {
            quote! { 
                compile_error!("Non-sequence item layouts not yet supported") 
            }
        }
    }
}

fn decode_field_simple(node: &IrNode) -> TokenStream {
    let name = format_ident!("{}", node.name);
    
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            quote! {
                let #name = reader.read_bits(#bits)? as #ty;
            }
        }
        
        IRLayout::Repetition { counter, node: inner } => {
            let count_expr = counter_expr(counter);
            let inner_decode = decode_expr(inner);
            
            quote! {
                let #name = {
                    let count = #count_expr;
                    let mut v = Vec::new();
                    for _ in 0..count {
                        v.push(#inner_decode);
                    }
                    v
                };
            }
        }
        
        _ => quote! {},
    }
}

fn decode_field_with_fspec(node: &IrNode) -> TokenStream {
    let name = format_ident!("{}", node.name);
    
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            quote! {
                let #name = reader.read_bits(#bits)? as #ty;
            }
        }
        
        IRLayout::Optional { condition, node: inner } => {
            let cond_expr = condition_expr(condition);
            let inner_decode = decode_expr(inner);
            
            quote! {
                let #name = if #cond_expr {
                    Some(#inner_decode)
                } else {
                    None
                };
            }
        }
        
        IRLayout::Repetition { counter, node: inner } => {
            let count_expr = counter_expr(counter);
            let inner_decode = decode_expr(inner);
            
            quote! {
                let #name = {
                    let count = #count_expr;
                    let mut v = Vec::new();
                    for _ in 0..count {
                        v.push(#inner_decode);
                    }
                    v
                };
            }
        }
        
        _ => quote! {},
    }
}

fn decode_expr(node: &IrNode) -> TokenStream {
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            quote! { reader.read_bits(#bits)? as #ty }
        }
        _ => {
            quote! { 
                compile_error!("Nested complex types not yet supported") 
            }
        }
    }
}

fn condition_expr(cond: &IRCondition) -> TokenStream {
    match cond {
        IRCondition::BitSet { byte, bit } => {
            quote! { item_fspec.is_set(#byte, #bit) }
        }
    }
}

fn counter_expr(counter: &IRCounter) -> TokenStream {
    match counter {
        IRCounter::Fixed(n) => quote! { #n },
        IRCounter::FromField { bits } => quote! { reader.read_bits(#bits)? as usize },
    }
}