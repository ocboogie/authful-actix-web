use crate::errors::Error;
use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use core::{auth::authenticate_session, Context, Pool};
use uuid::Uuid;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

pub struct LoggedUser {
    pub id: Uuid,
    pub session_id: String,
}

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Result<LoggedUser, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cookie = req.cookie("session_id").ok_or(Error::InvalidSession)?;
        let session_id = cookie.value();

        let pool_data = req.get_app_data::<Pool>().unwrap();
        let pool = pool_data.get().unwrap();
        let ctx = Context::new(&pool, SECRET_KEY.as_str());
        let id = authenticate_session(ctx, session_id)?;

        Ok(LoggedUser {
            id,
            session_id: session_id.to_string(),
        })
    }
}
