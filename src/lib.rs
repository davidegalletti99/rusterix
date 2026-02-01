pub mod framework;
pub mod generate;
pub mod parse;
pub mod transform;
pub mod builder;

#[cfg(test)]
mod tests {
    use crate::builder::{RustBuilder};


    #[test]
    fn test_build() {
        let buidler = RustBuilder::new();
        let result = RustBuilder::build_file(
            &buidler, 
            "tests/test.xml".into(),
            "generated".into());
        assert!(result.is_ok(), "An error occours during the build {:?}", result)
    }
}
