use serde::Deserialize;

//
// Top-level structures
//
#[derive(Debug, Deserialize)]
pub struct Category {
    #[serde(rename = "id")]
    pub id: u8,

    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "id")]
    pub id: u8,

    #[serde(rename = "frn")]
    pub frn: u8,

    #[serde(rename = "$value")]
    pub data: ItemStructure,
}

//
// Reusable compoundable abstraction
//
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Compoundable<T> {
    Fixed(T),
    Explicit(T),
    Extended(T),
    Repetitive(RepetitiveItem),
}

//
// Core structural elements
//
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemStructure {
    Fixed(SimpleItem),
    Explicit(SimpleItem),
    Extended(SimpleItem),
    Repetitive(RepetitiveItem),
    Compound(CompoundItem),
}

#[derive(Debug, Deserialize)]
pub struct SimpleItem {
    #[serde(rename = "bytes")]
    pub bytes: usize,

    #[serde(rename = "$value")]
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
pub struct RepetitiveItem {
    #[serde(rename = "bytes")]
    pub bytes: usize,

    #[serde(rename = "counter")]
    pub counter: String,

    #[serde(rename = "$value")]
    pub elements: Vec<Element>,
}

//
// Compound wrapper (syntax matters)
//
#[derive(Debug, Deserialize)]
pub struct CompoundItem {
    #[serde(rename = "$value")]
    pub data: Box<Compoundable<SimpleItem>>,
}

//
// Leaf / structural nodes
//
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Element {
    Field(Field),
    EPBField(EPBField),
    Enum(Enum),
    Spare(Spare),
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "bits")]
    pub bits: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "epb-field")]
pub struct EPBField {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "bits")]
    pub bits: usize,
    
}

#[derive(Debug, Deserialize)]
pub struct Spare {
    #[serde(rename = "bits")]
    pub bits: usize,
}

#[derive(Debug, Deserialize)]
pub struct Enum {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "bits")]
    pub bits: usize,

    #[serde(rename = "value")]
    pub values: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "value")]
    pub value: String,
}
