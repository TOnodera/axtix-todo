use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoRequest {
    pub id: u32,
    pub name: String,
    pub content: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct TodoQuery {
    pub id: u32,
}
