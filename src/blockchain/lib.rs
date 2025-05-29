#[ink::contract]
mod billing {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Billing {
        receipts: Mapping<String, Receipt>,
    }

    #[derive(Debug, scale::Encode, scale::Decode)]
    pub struct Receipt {
        id: String,
        items: Vec<Item>,
        total: u128, // ETH uses integer values
    }

    #[ink(message)]
    pub fn store_receipt(&mut self, receipt: Receipt) {
        self.receipts.insert(&receipt.id, &receipt);
    }
}