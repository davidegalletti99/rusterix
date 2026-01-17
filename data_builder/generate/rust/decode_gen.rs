use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::data_builder::transform::ir::{IrNode, IRLayout, IRCondition, IRCounter};
use super::utils::rust_type_for_bits;

pub fn generate(name: &str, node: &IrNode) -> TokenStream {
    let ident = format_ident!("{}", name);
    let body = decode_body(node);

    quote! {
        impl #ident {
            pub fn decode<R: std::io::Read>(
                reader: &mut BitReader<R>,
                fspec: &Fspec,
            ) -> Result<Self, DecodeError> {
                Ok(Self {
                    #(#body),*
                })
            }
        }
    }
}

fn decode_body(node: &IrNode) -> Vec<TokenStream> {
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let name = format_ident!("{}", node.name);
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            vec![quote! {
                #name: reader.read_bits(#bits)? as #ty
            }]
        }

        IRLayout::Sequence { elements } => {
            elements.iter().flat_map(decode_body).collect()
        }

        IRLayout::Optional { condition, node } => {
            let name = format_ident!("{}", node.name);
            let cond = condition_expr(condition);
            let expr = decode_expr(node);

            vec![quote! {
                #name: if #cond {
                    Some(#expr)
                } else {
                    None
                }
            }]
        }

        IRLayout::Repetition { counter, node } => {
            let name = format_ident!("{}", node.name);
            let count = counter_expr(counter);
            let expr = decode_expr(node);

            vec![quote! {
                #name: {
                    let mut v = Vec::new();
                    for _ in 0..#count {
                        v.push(#expr);
                    }
                    v
                }
            }]
        }
    }
}

fn decode_expr(node: &IrNode) -> TokenStream {
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            quote! { reader.read_bits(#bits)? as #ty }
        }
        _ => {
            let ident = format_ident!("{}", node.name);
            quote! { #ident::decode(reader, fspec)? }
        }
    }
}

fn condition_expr(cond: &IRCondition) -> TokenStream {
    match cond {
        IRCondition::BitSet { byte, bit } => {
            quote! { fspec.is_set(#byte, #bit) }
        }
    }
}

fn counter_expr(counter: &IRCounter) -> TokenStream {
    match counter {
        IRCounter::Fixed(n) => quote! { #n },
        IRCounter::FromField { bits } => quote! { reader.read_bits(#bits)? },
    }
}
