use crate::config::types;
use crate::error;
use crate::request::{TodoQuery, TodoRequest};
use crate::response::TodoResponse;
use actix_web::web;
use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

pub async fn post(
    todo_request: web::Json<TodoRequest>,
) -> types::Result<impl Responder> {
    if todo_request.name.is_empty() {
        return Err(Box::new(error::ValidationError {
            message: "タスク名を入力してください。".to_string(),
        }));
    }

    Ok(HttpResponse::Created())
}

pub async fn get(
    query: web::Query<TodoQuery>,
) -> types::Result<impl Responder> {
    Ok(HttpResponse::Ok().json(TodoResponse {
        id: query.id,
        name: "test_name".to_string(),
        content: "test_content".to_string(),
        done: true,
    }))
}
