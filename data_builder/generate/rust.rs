use crate::data_builder::parse::xml_model::*;

const INDENT: &str = "    ";
const CATEGORY_PREFIX: &str = "Cat";
const ITEM_PREFIX: &str = "Item";


pub fn generate_category(category: Category) -> String {
    let items = category.items;
    let mut cat_struct: String = format!("pub struct {}{:03} {{\n", CATEGORY_PREFIX, category.id);
    for item in items {
        cat_struct.push_str(&format!("{}pub {}{:03}: Vec<u8>,\n", INDENT, ITEM_PREFIX.to_lowercase(), item.id));
    }

    cat_struct.push_str("}\n");
    cat_struct
    
}

// use proc_macro2::TokenStream;
// use quote::{quote, format_ident};
// use crate::data_builder::transform::ir::*;

// pub fn generate_rust(ir: &IR) -> TokenStream {
//     let mut structs_ts = Vec::new();

//     for cat in &ir.categories {
//         for item in &cat.items {
//             // Creiamo un nome per la struct tipo: Category0_Item1
//             let struct_name = format_ident!("Cat{:03}Item{:03}", cat.id, item.id);

//             let fields_ts = match &item.layout {
//                 IRLayout::Fixed(f) => generate_fields(&f.fields),
//                 IRLayout::Repetitive(r) => generate_fields(&r.fields),
//                 IRLayout::Extended(e) => {
//                     let mut all_fields = e.primary.fields.clone();
//                     all_fields.extend(e.secondary.fields.clone());
//                     generate_fields(&all_fields)
//                 },
//                 IRLayout::Compound(c) => {
//                     let mut all_fields = Vec::new();
//                     for part in &c.parts {
//                         all_fields.extend(part.fields.clone());
//                     }
//                     generate_fields(&all_fields)
//                 },
//             };

//             let ts = quote! {
//                 pub struct #struct_name {
//                     #(#fields_ts),*
//                 }
//             };

//             structs_ts.push(ts);
//         }
//     }

//     // Combiniamo tutti i token
//     quote! {
//         #( #structs_ts )*
//     }
// }

// fn generate_fields(fields: &[IRField]) -> Vec<TokenStream> {
//     fields.iter().map(|f| {
//         let name = format_ident!("{}", f.name);
//         let ty = quote! { u32 }; // esempio: puoi calcolare il tipo da size
//         quote! { pub #name: #ty }
//     }).collect()
// }
