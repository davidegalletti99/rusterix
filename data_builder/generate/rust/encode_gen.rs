use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};

use crate::data_builder::transform::ir::{IrNode, IRLayout, IRCondition};

pub fn generate(name: &str, node: &IrNode) -> TokenStream {
    let ident = format_ident!("{}", name);
    let body = encode_body(node);

    quote! {
        impl #ident {
            pub fn encode<W: std::io::Write>(
                &self,
                writer: &mut BitWriter<W>,
                fspec: &mut Fspec,
            ) -> Result<(), DecodeError> {
                #(#body)*
                Ok(())
            }
        }
    }
}

fn encode_body(node: &IrNode) -> Vec<TokenStream> {
    match &node.layout {
        IRLayout::Primitive { bits } => {
            let name = format_ident!("{}", node.name);
            vec![quote! {
                writer.write_bits(self.#name as u64, #bits)?;
            }]
        }

        IRLayout::Sequence { elements } => {
            elements.iter().flat_map(encode_body).collect()
        }

        IRLayout::Optional { condition, node } => {
            let name = format_ident!("{}", node.name);
            let set = set_condition(condition);
            let inner = encode_body(node);

            vec![quote! {
                if let Some(v) = &self.#name {
                    #set
                    #(#inner)*
                }
            }]
        }

        IRLayout::Repetition { node, .. } => {
            let name = format_ident!("{}", node.name);
            let inner = encode_body(node);

            vec![quote! {
                for v in &self.#name {
                    #(#inner)*
                }
            }]
        }
    }
}

fn set_condition(cond: &IRCondition) -> TokenStream {
    match cond {
        IRCondition::BitSet { byte, bit } => {
            quote! { fspec.set(#byte, #bit); }
        }
    }
}
