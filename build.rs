use std::{fs, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Field {
    name: String,
    offset: usize,
}
#[derive(Debug, Deserialize)]
struct Value {
    value: String,
    enumerated: String,
}
#[derive(Debug, Deserialize)]
enum ItemLayout {
    Fixed(FixedLength),
    Extended(ExtendedLength),
    Repetitive(Repetitive),
    Compound(Compound),
}

#[derive(Debug, Deserialize)]
struct Category {
    id: u8,
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    id: u8,
    layout: Vec<ItemLayout>,
}

#[derive(Debug, Deserialize)]
struct FixedLength {
    length: usize,
    fields: Vec<Field>,
}

struct Repetitive {
    length: usize,
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct ExtendedLength {
    primary_part: FixedLength,
    secondary_parts: FixedLength,
}

#[derive(Debug, Deserialize)]
struct PrimaryPart {
    length: usize,
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
struct SecondaryPart {
    length: usize,
    fields: Vec<Field>,
}


#[derive(Debug, Deserialize)]
struct Compound {
    parts: Vec<FixedLength>,
}

fn main() {
    // 1. Read XML
    let xml = fs::read_to_string("layout.xml").expect("Failed to read layout.xml");

    // 2. Parse XML
    let data: Struct = serde_xml_rs::from_str(&xml)
        .expect("Failed to parse layout XML");

    // 3. Generate Rust code
    let code = generate_struct(&data);

    // 4. Write to OUT_DIR
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("generated.rs");

    fs::write(&out_path, code).expect("Failed to write generated.rs");
}
fn generate_category(c: &Category) -> String {
    let mut items_code = String::new();
    for item in &c.item {
        items_code.push_str(&generate_item(item));
    }
}
fn generate_item(i: &Item) -> String {
    let mut fields_code = String::new();
    for field in &i.field {
        fields_code.push_str(&generate_field(field));
    }
}

fn generate_struct(s: &Category) -> String {
    // repr attribute
    let repr = s.repr.as_deref().unwrap_or("C");

    let mut fields = String::new();
    for f in &s.field {
        fields.push_str(&format!("    pub {}: {},\n", f.name, f.ty));
    }

    format!(
        r#"
#[repr({repr})]
pub struct {name} {{
{fields}
}}

pub const _: () = {{
    // Optional: dump size
    const _SIZE: usize = core::mem::size_of::<{name}>();
}};
"#,
        repr = repr,
        name = s.name,
        fields = fields
    )
}
