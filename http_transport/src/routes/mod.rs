use actix_web::web;

pub mod auth;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    auth::register_routes(cfg);
}
