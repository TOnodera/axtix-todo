use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub content: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub done: bool,
}
#[derive(Deserialize)]
pub struct TodoQuery {
    pub id: i64,
}
