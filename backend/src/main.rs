use actix_web::{App, HttpServer};
use tokio;

mod app;
mod config;
mod error;
mod request;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config::config))
        .bind(app::SERVER)?
        .run()
        .await
}
