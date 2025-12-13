use crate::data_builder::parse::xml_model::Category;


pub fn parse_category(xml: &str) -> Result<Category, serde_xml_rs::Error> {
    serde_xml_rs::from_str(xml)
}