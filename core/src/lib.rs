#[macro_use]
extern crate diesel;

mod schema;
mod services;
pub mod errors;
pub mod models;
pub mod utils;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub use errors::Error;
pub use services::*;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Context<'a> {
    pub conn: &'a PgConnection,
    // TODO: Might want to make this `SecretKey`
    pub secret: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(conn: &'a PgConnection, secret: &'a str) -> Self {
        Self {
            conn,
            secret,
        }
    }
}

pub fn connect_db_url(database_url: &str) -> Option<Pool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).ok()
}

/// A convenience function that gets the database url from the `DATABASE_URL`
/// environment variables, and panics if it fails to connect.
pub fn connect_db() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    connect_db_url(&database_url).expect("Failed to create pool.")
}
