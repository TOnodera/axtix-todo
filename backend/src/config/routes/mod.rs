use actix_web::{web, Resource};

mod todo;

pub fn routes() -> Resource {
    web::resource("/")
        .route(web::get().to(todo::index))
        .route(web::post().to(todo::post))
}
