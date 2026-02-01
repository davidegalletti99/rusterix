use proc_macro2::TokenStream;
use quote::quote;

use crate::transform::ir::IR;
use super::{item_gen::generate_item, record_gen::generate_record};

/// Main code generation orchestrator.
/// 
/// Produces a complete Rust module containing:
/// - All necessary imports
/// - Category record struct (Cat{N}Record)
/// - All item structs (Item{N})
/// - All enum definitions
/// - All decode/encode implementations
/// 
/// # Arguments
/// 
/// * `ir` - The validated intermediate representation
/// 
/// # Returns
/// 
/// A TokenStream containing the complete generated module.
pub fn generate(ir: &IR) -> TokenStream {
    let category = &ir.category;
    
    // Generate the record struct and its implementations
    let record = generate_record(category);
    
    // Generate all item structs and their implementations
    let items: Vec<_> = category.items.iter()
        .map(generate_item)
        .collect();
    
    // Combine everything into a complete module
    quote! {
        // AUTO-GENERATED CODE — DO NOT EDIT
        //
        // This file was automatically generated from ASTERIX XML definitions.
        // Manual modifications will be lost on regeneration.
        
        #![allow(unused_imports)]
        #![allow(dead_code)]

        use rusterix::framework::{BitReader, BitWriter, DecodeError, Fspec, Decode, Encode};
        use std::io::{Read, Write};
        
        // Category record
        #record
        
        // Data items
        #(#items)*
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform::ir::*;
    
    #[test]
    fn test_generate_complete_module() {
        let ir = IR {
            category: IRCategory {
                id: 48,
                items: vec![
                    IRItem {
                        id: 10,
                        frn: 0,
                        layout: IRLayout::Fixed {
                            bytes: 2,
                            elements: vec![
                                IRElement::Field {
                                    name: "sac".to_string(),
                                    bits: 8,
                                },
                                IRElement::Field {
                                    name: "sic".to_string(),
                                    bits: 8,
                                },
                            ],
                        },
                    },
                ],
            },
        };
        
        let result = generate(&ir);
        let code = result.to_string();
        
        // Check for imports (quote! adds spaces around :: and braces)
        assert!(code.contains("use rusterix :: framework"));
        assert!(code.contains("Decode"));
        assert!(code.contains("Encode"));

        // Check for record
        assert!(code.contains("pub struct Cat048Record"));

        // Check for item
        assert!(code.contains("pub struct Item010"));
        assert!(code.contains("pub sac : u8"));
        assert!(code.contains("pub sic : u8"));
    }
}