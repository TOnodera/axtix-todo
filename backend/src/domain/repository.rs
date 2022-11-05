use std::future::Future;

use crate::config::types::Result;
use async_trait::async_trait;

pub trait ICrudRepository<T> {
    fn create(
        self: Self,
        create_data: T,
    ) -> dyn Future<Output = Result<i64>>;
    // async fn read(id: i64) -> Result<T>;
    // async fn update(create_data: T) -> Result<bool>;
    // async fn delete(id: i64) -> Result<bool>;
}
