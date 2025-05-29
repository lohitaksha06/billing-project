#![cfg_attr(not(feature = "std"), no_std]

#[ink::contract]
mod billing {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Billing {
        receipts: Mapping<[u8; 32], Receipt>,  // Using hash as key
        owner: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Receipt {
        id: [u8; 32],
        items: Vec<Item>,
        total: u64,  // In smallest units (wei)
        timestamp: u64,
    }

    #[derive(Debug, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Item {
        name: String,
        price: u64,
        quantity: u32,
    }

    impl Billing {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                receipts: Mapping::new(),
                owner: Self::env().caller(),
            }
        }

        #[ink(message)]
        pub fn store_receipt(
            &mut self,
            id: [u8; 32],
            items: Vec<Item>,
            total: u64,
            timestamp: u64,
        ) -> Result<(), Error> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            
            let receipt = Receipt {
                id,
                items,
                total,
                timestamp,
            };
            self.receipts.insert(&id, &receipt);
            
            Ok(())
        }

        #[ink(message)]
        pub fn get_receipt(&self, id: [u8; 32]) -> Option<Receipt> {
            self.receipts.get(&id)
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        InvalidReceipt,
    }
}