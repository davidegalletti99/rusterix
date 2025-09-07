pub mod framework;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test: u128 = 0xFF34_5678_ABCD_EF12_3456_789A_BCDE_F012;
        let test_8 = test as u8;
        let expt_8 = (test & 0xFF) as u8;
        assert_eq!(test_8, expt_8);
        println!("test_8: {:02X}, expt_8: {:02X}", test_8, expt_8);
    }
}
