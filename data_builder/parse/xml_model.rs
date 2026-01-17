use serde::Deserialize;

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

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "$value")]
    pub body: Sequence,
}

//
// ─────────────────────────────
// Core structural elements
// ─────────────────────────────
//

#[derive(Debug, Deserialize)]
pub struct Sequence {
    #[serde(rename = "fspec")]
    pub fspec: Option<Fspec>,

    #[serde(rename = "$value")]
    #[serde(default)]
    pub elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Element {
    Primitive(Primitive),
    Optional(Optional),
    Repeat(Repeat),
    Sequence(Sequence),

    // Ignore text / whitespace nodes
    Text(String),
}

//
// ─────────────────────────────
// Leaf / structural nodes
// ─────────────────────────────
//

#[derive(Debug, Deserialize)]
pub struct Primitive {
    #[serde(rename = "bits")]
    pub bits: usize,

    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Optional {
    #[serde(rename = "condition")]
    pub condition: String,

    #[serde(rename = "$value")]
    pub element: Box<Element>,
}

#[derive(Debug, Deserialize)]
pub struct Repeat {
    #[serde(rename = "counter")]
    pub counter: String,

    #[serde(rename = "$value")]
    pub element: Box<Element>,
}

#[derive(Debug, Deserialize)]
pub struct Fspec {
    #[serde(rename = "bytes")]
    pub bytes: usize,
}