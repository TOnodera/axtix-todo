use actix_web::{web, App, HttpServer};
use repository::get_pool;
use tokio;

mod config;
mod domain;
mod error;
mod repository;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // DB接続プール
    dotenvy::dotenv().ok();
    let pool = get_pool().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config::config)
    })
    .bind(domain::consts::SERVER)?
    .run()
    .await
}
