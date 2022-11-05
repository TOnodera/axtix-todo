use std::{error::Error as StdError, future, pin::Pin};

use crate::{
    config::types::AsyncTraitReturn,
    domain::repository::ICrudRepository,
};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct TodoRepository {
    pub pool: PgPool,
}

#[async_trait]
impl<Todo> ICrudRepository<Todo> for TodoRepository {
    fn create(
        self: Self,
        data: Todo,
    ) -> AsyncTraitReturn<'async_trait, i64> {
        todo!()
    }
}
