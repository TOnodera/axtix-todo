use crate::config::types::{self, Todo};
use crate::repository::todo::TodoRepository;
use crate::request::{
    CreateTodoRequest, TodoQuery, UpdateTodoRequest,
};
use crate::response::TodoResponse;
use crate::{error, repository};
use actix_web::web;
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

pub async fn post(
    todo_request: web::Json<CreateTodoRequest>,
    pool: web::Data<PgPool>,
) -> types::Result<impl Responder> {
    if todo_request.title.is_empty() {
        return Err(Box::new(error::ValidationError {
            message: "タスク名を入力してください。".to_string(),
        }));
    }

    let repository = TodoRepository { pool: &pool };
    let id = repository.create(&todo_request).await?;
    Ok(HttpResponse::Created().json(id))
}

pub async fn get(
    pool: web::Data<PgPool>,
    query: web::Query<TodoQuery>,
) -> types::Result<impl Responder> {
    let repository = TodoRepository { pool: &pool };
    let todo = repository.read(query.id).await?;
    Ok(HttpResponse::Ok().json(todo))
}

pub async fn patch(
    pool: web::Data<PgPool>,
    todo_request: web::Json<UpdateTodoRequest>,
) -> types::Result<impl Responder> {
    let repository = TodoRepository { pool: &pool };
    let affected = repository
        .update(UpdateTodoRequest {
            id: todo_request.id,
            title: todo_request.title.clone(),
            content: todo_request.content.clone(),
            done: todo_request.done,
        })
        .await?;
    Ok(HttpResponse::Ok().json(affected))
}

pub async fn delete(
    pool: web::Data<PgPool>,
    query: web::Query<TodoQuery>,
) -> types::Result<impl Responder> {
    let repository = TodoRepository { pool: &pool };
    let affected = repository.delete(query.id).await?;
    Ok(HttpResponse::Ok().json(affected))
}
