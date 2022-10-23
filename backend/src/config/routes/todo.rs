use crate::error::user_error::UserError;
use actix_web::{error, HttpResponse, Responder, Result};
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("hey there")
}

pub async fn post() -> Result<&'static str> {
    let result = Err(UserError::ValidationError {
        field: String::from("test"),
    });

    Ok(result.map_err(|e| error::ErrorBadRequest(e))?)
}
