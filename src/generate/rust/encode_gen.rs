use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::transform::ir::{IrNode, IRLayout, IRCondition};

// Renamed from generate() to generate_item() for clarity
pub fn generate_item(name: &Ident, node: &IrNode) -> TokenStream {
    let encode_impl = generate_encode_impl(node);

    quote! {
        impl #name {
            pub fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
            ) -> Result<(), DecodeError> {
                #encode_impl
                Ok(())
            }
        }
    }
}

fn generate_encode_impl(node: &IrNode) -> TokenStream {
    match &node.layout {
        IRLayout::Sequence { elements } => {
            let has_optional = elements.iter().any(|e| matches!(e.layout, IRLayout::Optional { .. }));
            
            if has_optional {
                let fspec_setup: Vec<_> = elements.iter()
                    .filter_map(|e| encode_fspec_setup(e))
                    .collect();
                
                let field_writes: Vec<_> = elements.iter()
                    .map(|e| encode_field_with_fspec(e))
                    .collect();
                
                quote! {
                    let mut item_fspec = Fspec::new();
                    #(#fspec_setup)*
                    item_fspec.write(writer)?;
                    
                    #(#field_writes)*
                }
            } else {
                let field_writes: Vec<_> = elements.iter()
                    .map(encode_field_simple)
                    .collect();
                
                quote! {
                    #(#field_writes)*
                }
            }
        }
        _ => quote! { 
            compile_error!("Non-sequence item layouts not yet supported") 
        }
    }
}

fn encode_fspec_setup(node: &IrNode) -> Option<TokenStream> {
    match &node.layout {
        IRLayout::Optional { condition, .. } => {
            let name = format_ident!("{}", node.name);
            let set_expr = set_condition_expr(condition);
            
            Some(quote! {
                if self.#name.is_some() {
                    #set_expr
                }
            })
        }
        _ => None
    }
}

fn encode_field_simple(node: &IrNode) -> TokenStream {
    let name = format_ident!("{}", node.name);
    
    match &node.layout {
        IRLayout::Primitive { bits } => {
            quote! {
                writer.write_bits(self.#name as u64, #bits)?;
            }
        }
        
        IRLayout::Repetition { node: inner, .. } => {
            match &inner.layout {
                IRLayout::Primitive { bits } => {
                    quote! {
                        for item in &self.#name {
                            writer.write_bits(*item as u64, #bits)?;
                        }
                    }
                }
                _ => quote! {
                    compile_error!("Complex repetition not yet supported")
                }
            }
        }
        
        _ => quote! {}
    }
}

fn encode_field_with_fspec(node: &IrNode) -> TokenStream {
    let name = format_ident!("{}", node.name);
    
    match &node.layout {
        IRLayout::Primitive { bits } => {
            quote! {
                writer.write_bits(self.#name as u64, #bits)?;
            }
        }
        
        IRLayout::Optional { node: inner, .. } => {
            match &inner.layout {
                IRLayout::Primitive { bits } => {
                    quote! {
                        if let Some(value) = self.#name {
                            writer.write_bits(value as u64, #bits)?;
                        }
                    }
                }
                _ => quote! {
                    compile_error!("Complex optional not yet supported")
                }
            }
        }
        
        IRLayout::Repetition { node: inner, .. } => {
            match &inner.layout {
                IRLayout::Primitive { bits } => {
                    quote! {
                        for item in &self.#name {
                            writer.write_bits(*item as u64, #bits)?;
                        }
                    }
                }
                _ => quote! {
                    compile_error!("Complex repetition not yet supported")
                }
            }
        }
        
        _ => quote! {}
    }
}

fn set_condition_expr(cond: &IRCondition) -> TokenStream {
    match cond {
        IRCondition::BitSet { byte, bit } => {
            quote! { item_fspec.set(#byte, #bit); }
        }
    }
}