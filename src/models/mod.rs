use serde::{Serialize, Deserialize};
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Receipt {
    pub id: String,           // Unique identifier (will be hash)
    pub items: Vec<Item>,     // Purchased items
    pub total: f64,          // Calculated total
    pub store_id: String,     // Store identifier
    pub timestamp: i64,       // Unix timestamp
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Item {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}