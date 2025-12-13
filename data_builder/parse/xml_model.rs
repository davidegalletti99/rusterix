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
    #[serde(rename = "$value")]
    pub layout: ItemLayout,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemLayout {
    #[serde(rename = "fixed_length")]
    Fixed(FixedLength),
    #[serde(rename = "extended_length")]
    Extended(ExtendedLength),
    #[serde(rename = "repetitive")]
    Repetitive(Repetitive),
    #[serde(rename = "compound")]
    Compound(Compound),
}
#[derive(Debug, Deserialize)]
pub struct FixedLength {
    #[serde(rename = "length")]
    pub length: usize,
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct Repetitive {
    #[serde(rename = "length")]
    pub length: usize,
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedLength {
    #[serde(rename = "primary_part")]
    pub primary_part: FixedLength,
    #[serde(rename = "secondary_parts")]
    pub secondary_parts: FixedLength,
}

#[derive(Debug, Deserialize)]
pub struct Compound {
    #[serde(rename = "part")]
    pub parts: Vec<FixedLength>,
}


#[derive(Debug, Deserialize)]
pub struct PrimaryPart {
    #[serde(rename = "length")]
    pub length: usize,
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct SecondaryPart {
    #[serde(rename = "length")]
    pub length: usize,
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "size")]
    pub size: usize,
    #[serde(rename = "offset")]
    pub offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "enumerated")]
    pub enumerated: u16,
}
