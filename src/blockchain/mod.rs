use crate::models::{Receipt, Item};
use actix_web::{web, HttpResponse, Responder};
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

pub async fn create_receipt() -> impl Responder {
    let receipt = Receipt {
        id: "receipt_123".to_string(),
        items: vec![
            Item {
                name: "Sample Item".to_string(),
                price: 29.99,
                quantity: 1,
            }
        ],
        total: 29.99,
        store_id: "store_123".to_string(),
        timestamp: 1672531200,
    };
    
    // SOLANA: Send receipt to blockchain
    let program_id = Pubkey::new_unique();
    let _instruction = Instruction::new_with_borsh(
        program_id,
        &receipt,
        vec![],
    );
    
    
    HttpResponse::Ok().json(receipt)
}

pub async fn get_receipt(path: web::Path<(String,)>) -> impl Responder {
    let (receipt_id,) = path.into_inner();
    // In a real app, you'd look this up from a database
    let sample_receipt = Receipt {
        id: receipt_id,
        items: vec![
            Item {
                name: "maths textbook".to_string(),
                price: 50.99,
                quantity: 1,
            }
        ],
        total: 50.99,
        store_id: "store_456".to_string(),
        timestamp: 1672531200,
    };
    HttpResponse::Ok().json(sample_receipt)
}