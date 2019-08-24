pub use crate::auth::Error as AuthError;

pub enum Error {
    Auth(AuthError)
}