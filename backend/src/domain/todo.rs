use chrono::NaiveTime;

use crate::config::types::Result;

use super::repository::ICrudRepository;

pub struct Todo {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub done: bool,
    pub created_at: Option<NaiveTime>,
    pub updated_at: Option<NaiveTime>,
}

impl Todo {
    pub async fn create(
        todo: Self,
        repository: &impl ICrudRepository<Todo>,
    ) -> Result<i64> {
        Ok(repository.create(todo).await?)
    }
}
