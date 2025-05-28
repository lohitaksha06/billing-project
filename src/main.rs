use actix_web::{web, App, HttpServer};
mod blockchain;
mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/receipt", web::post().to(blockchain::create_receipt))
            .route("/receipt/{id}", web::get().to(blockchain::get_receipt))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}