use std::{env, io::ErrorKind};

use sqlx::{postgres::PgPoolOptions, PgPool};

pub mod todo;

pub async fn get_pool() -> Result<PgPool, std::io::Error> {
    let env = env::var("DATABASE_URL");
    let url = match env {
        Ok(url) => url,
        Err(e) => {
            return Err(std::io::Error::new(ErrorKind::NotFound, e))
        }
    };
    let pool_result =
        PgPoolOptions::new().max_connections(10).connect(&url).await;
    match pool_result {
        Ok(pool) => Ok(pool),
        Err(e) => {
            Err(std::io::Error::new(ErrorKind::ConnectionAborted, e))
        }
    }
}
