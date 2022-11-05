use actix_web::{App, HttpServer};
use tokio;

mod config;
mod domain;
mod error;
mod repository;
mod request;
mod response;
mod routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(config::config))
        .bind(config::consts::SERVER)?
        .run()
        .await
}
