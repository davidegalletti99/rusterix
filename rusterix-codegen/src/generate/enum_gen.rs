use proc_macro2::TokenStream;
use quote::{quote};

use super::utils::to_pascal_case;

/// Generates a Rust enum from ASTERIX enum definition.
/// 
/// Creates an enum with:
/// - Named variants for all defined values
/// - An Unknown(u8) variant for undefined values
/// - TryFrom<u8> implementation for decoding
/// - Into<u8> implementation for encoding
/// 
/// # Arguments
/// 
/// * `name` - The enum type name
/// * `bits` - Number of bits used to represent the enum
/// * `values` - List of (variant_name, numeric_value) pairs
/// 
/// # Returns
/// 
/// TokenStream containing the complete enum definition and implementations.
/// 
/// # Example Generated Code
/// 
/// ```rust
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// #[repr(u8)]
/// pub enum TargetType {
///     Psr = 1,
///     Ssr = 2,
///     Unknown(u8),
/// }
/// 
/// impl TryFrom<u8> for TargetType {
///     type Error = ();
///     fn try_from(value: u8) -> Result<Self, Self::Error> {
///         match value {
///             1 => Ok(Self::Psr),
///             2 => Ok(Self::Ssr),
///             _ => Ok(Self::Unknown(value)),
///         }
///     }
/// }
/// 
/// impl From<TargetType> for u8 {
///     fn from(val: TargetType) -> u8 {
///         match val {
///             TargetType::Psr => 1,
///             TargetType::Ssr => 2,
///             TargetType::Unknown(v) => v,
///         }
///     }
/// }
/// ```
pub fn generate_enum(
    name: &str,
    bits: usize,
    values: &[(String, u8)],
) -> TokenStream {
    let _ = bits;
    let enum_name = to_pascal_case(name);
    
    // Generate variants for all defined values
    let variants: Vec<_> = values
        .iter()
        .map(|(variant_name, value)| {
            let variant_ident = to_pascal_case(variant_name);
            quote! { #variant_ident = #value }
        })
        .collect();
    
    // Generate TryFrom match arms
    let try_from_arms: Vec<_> = values
        .iter()
        .map(|(variant_name, value)| {
            let variant_ident = to_pascal_case(variant_name);
            quote! { #value => Ok(Self::#variant_ident) }
        })
        .collect();
    
    // Generate From match arms
    let from_arms: Vec<_> = values
        .iter()
        .map(|(variant_name, value)| {
            let variant_ident = to_pascal_case(variant_name);
            quote! { #enum_name::#variant_ident => #value }
        })
        .collect();
    
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u8)]
        pub enum #enum_name {
            #(#variants,)*
            Unknown(u8),
        }
        
        impl TryFrom<u8> for #enum_name {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, ()> {
                match value {
                    #(#try_from_arms,)*
                    _ => Ok(Self::Unknown(value)),
                }
            }
        }
        
        impl From<#enum_name> for u8 {
            fn from(val: #enum_name) -> u8 {
                match val {
                    #(#from_arms,)*
                    #enum_name::Unknown(v) => v,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_enum() {
        let values = vec![
            ("PSR".to_string(), 1),
            ("SSR".to_string(), 2),
            ("COMBINED".to_string(), 3),
        ];

        let result = generate_enum("target_type", 2, &values);
        let code = result.to_string();

        // Check that enum is generated
        assert!(code.contains("pub enum TargetType"));

        // Check that variants are present (quote! renders u8 literals with suffix)
        assert!(code.contains("Psr = 1u8"));
        assert!(code.contains("Ssr = 2u8"));
        assert!(code.contains("Combined = 3u8"));
        assert!(code.contains("Unknown (u8)")); // quote! adds space

        // Check that implementations exist
        assert!(code.contains("impl TryFrom < u8 > for TargetType"));
        assert!(code.contains("impl From < TargetType > for u8"));
    }
}