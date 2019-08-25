#[macro_use]
extern crate serde_derive;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use core::connect_db;

mod errors;
mod helpers;
mod routes;

pub use helpers::{ContextProvider, LoggedUser};

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = connect_db();

    // Start http server
    HttpServer::new(move || {
        (App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .supports_credentials()
                    .allowed_methods(vec!["GET", "POST", "DELETE"]),
            )
            .data(web::JsonConfig::default().limit(4096))
            .configure(routes::register_routes))
    })
    .bind("127.0.0.1:3000")?
    .run()?;

    Ok(())
}
