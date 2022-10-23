use actix_web::{App, HttpServer};
use tokio;

mod config;
mod error;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config::config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
