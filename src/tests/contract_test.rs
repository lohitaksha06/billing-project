#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;
 // cargo test --features=std use this code to run test
    #[ink::test]
    fn new_contract_has_owner() {
        let contract = Billing::new();
        let default_accounts = test::default_accounts::<Environment>();
        assert_eq!(contract.owner, default_accounts.alice);
    }

    #[ink::test]
    fn storing_receipt_works() {
        let mut contract = Billing::new();
        let receipt_id = [1u8; 32];
        let items = vec![Item {
            name: "Blockchain Book".to_string(),
            price: 30_000_000_000_000_000, // 0.03 ETH in wei
            quantity: 1,
        }];

        assert!(contract.store_receipt(receipt_id, items, 30_000_000_000_000_000, 123456).is_ok());
    }
}