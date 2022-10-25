use crate::routes;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::route());
    cfg.service(routes::todos_route());
}

pub mod consts;
pub mod types;
