use crate::app;
use crate::error;
use crate::request::TodoRequest;
use actix_web::web;
use actix_web::{HttpResponse, Responder};
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

pub async fn post(
    todo_request: web::Json<TodoRequest>,
) -> app::Result<impl Responder> {
    if todo_request.name.is_empty() {
        return Err(Box::new(error::ValidationError {
            message: "タスク名を入力してください。".to_string(),
        }));
    }

    Ok(HttpResponse::Created())
}
