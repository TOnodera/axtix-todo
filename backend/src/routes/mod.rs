use actix_web::{web, Resource};

mod todo;

pub fn route() -> Resource {
    web::resource("/").route(web::get().to(todo::index))
}

pub fn todos_route() -> Resource {
    web::resource("/todos")
        .route(web::get().to(todo::get))
        .route(web::post().to(todo::post))
}
