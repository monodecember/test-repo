pub mod biguint;
pub mod decimal;

#[cfg(test)]
mod tests {
    use super::*;

    use decimal::Decimal;

    #[test]
    fn it_works() {
        // let result = 2 + 2;
        // assert_eq!(result, 4);

        let d1 = Decimal::from("-23.32897589237589378592789357893275892375823758961723858093419742570923749172322", 100);
        d1.debug_display();
    }
}
