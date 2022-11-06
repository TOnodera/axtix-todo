use std::error;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub done: bool,
    pub created_at: Option<NaiveTime>,
    pub updated_at: Option<NaiveTime>,
}
