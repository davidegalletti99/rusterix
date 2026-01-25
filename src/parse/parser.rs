use crate::parse::xml_model::{Category};

/// Parses the given XML string into a Category struct.
/// # Arguments
/// * `xml` - A string slice that holds the XML data.
/// # Returns
/// * `Result<Category, serde_xml_rs::Error>` - The parsed Category or an error if parsing fails.
pub fn parse_category(xml: &str) -> Result<Category, serde_xml_rs::Error> {
    serde_xml_rs::from_str(xml)
}