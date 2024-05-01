mod db;
mod handlers;
mod models;
mod utils;

use actix_web::{get, web, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db_pool = db::init_pool().await.expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .configure(handlers::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
