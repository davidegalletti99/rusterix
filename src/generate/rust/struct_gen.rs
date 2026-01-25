use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::{generate::rust::field_extractor::extract_fields, transform::ir::{IRLayout, IrNode}};
use super::utils::rust_type_for_bits;

pub fn generate(name: &Ident, node: &IrNode) -> TokenStream {
    let fields = collect_fields(node);

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #name {
            #(#fields),*
        }
    }
}

fn collect_fields(node: &IrNode) -> Vec<TokenStream> {
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let name = format_ident!("{}", node.name);
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            vec![quote! { pub #name: #ty }]
        }

        IRLayout::Sequence { elements } => {
            elements.iter().flat_map(collect_fields).collect()
        }

        IRLayout::Optional { node, .. } => {
            let name = format_ident!("{}", node.name);
            let ty = type_for_node(node);
            vec![quote! { pub #name: Option<#ty> }]
        }

        IRLayout::Repetition { node, .. } => {
            let name = format_ident!("{}", node.name);
            let ty = type_for_node(node);
            vec![quote! { pub #name: Vec<#ty> }]
        }

        IRLayout::Spare { bits } => {
            let name = format_ident!("{}", node.name);
            let ty = type_for_node(node);
            vec![quote! { pub #name: Vec<#ty> }]
        }
    }
}

fn type_for_node(node: &IrNode) -> TokenStream {
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let ty = format_ident!("{}", rust_type_for_bits(*bits));
            quote! { #ty }
        }
        _ => {
            let ident = format_ident!("{}", node.name);
            quote! { #ident }
        }
    }
}
