use std::env;

use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tokio;

mod config;
mod error;
mod repository;
mod request;
mod response;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // DB接続プール
    dotenvy::dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config::config)
    })
    .bind(config::consts::SERVER)?
    .run()
    .await
}
