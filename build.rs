use std::{fs, path::PathBuf};
mod data_builder;
use data_builder::{parse::parser::{parse_category}, generate::rust::generate_category};

fn main() {
    let xml = fs::read_to_string("test.xml").expect("Failed to read test.xml");

    let data = parse_category(&xml)
        .expect("Failed to parse layout XML");

    // 4. Write to OUT_DIR
    let out_path = PathBuf::from("./")
        .join("generated.rs");

    let generated_code = generate_category(data);

    fs::write(&out_path, generated_code).expect("Failed to write generated.rs");

}


// fn generate_category(c: &Category) -> String {
//     let mut items_code = String::new();
//     for item in &c.item {
//         items_code.push_str(&generate_item(item));
//     }
// }
// fn generate_item(i: &Item) -> String {
//     let mut fields_code = String::new();
//     for field in &i.field {
//         fields_code.push_str(&generate_field(field));
//     }
// }

// fn generate_struct(s: &Category) -> String {
//     // repr attribute
//     let repr = s.repr.as_deref().unwrap_or("C");

//     let mut fields = String::new();
//     for f in &s.field {
//         fields.push_str(&format!("    pub {}: {},\n", f.name, f.ty));
//     }

//     format!(
//         r#"
// #[repr({repr})]
// pub struct {name} {{
// {fields}
// }}

// pub const _: () = {{
//     // Optional: dump size
//     const _SIZE: usize = core::mem::size_of::<{name}>();
// }};
// "#,
//         repr = repr,
//         name = s.name,
//         fields = fields
//     )
// }
