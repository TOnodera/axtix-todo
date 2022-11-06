use serde::Serialize;

#[derive(Serialize)]
pub struct TodoResponse {
    pub id: u32,
    pub name: String,
    pub content: String,
    pub done: bool,
}
