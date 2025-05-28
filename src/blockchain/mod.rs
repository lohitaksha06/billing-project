use crate::models::{Receipt, Item};  // Update this line
use actix_web::{web, HttpResponse, Responder};

pub async fn create_receipt() -> impl Responder {
    let sample_receipt = Receipt {
        id: "test_123".to_string(),
        items: vec![
            Item {
                name: "maths textbook".to_string(),
                price: 50.99,
                quantity: 1,
            }
        ],
        total: 99.99,
        store_id: "store_456".to_string(),
        timestamp: 1672531200,
    };
    HttpResponse::Ok().json(sample_receipt)  // Return actual struct
}