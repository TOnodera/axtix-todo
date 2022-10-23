use actix_web::{HttpResponse, Responder, Result};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpServer,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

pub async fn post() -> Result<&'static str> {
    let result = Err(UserError::ValidationError {
        field: String::from("test"),
    });

    Ok(result.map_err(|e| error::ErrorBadRequest(e))?)
}
