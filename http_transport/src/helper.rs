use crate::{errors::Error, SECRET_KEY};
use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use core::{auth::authenticate_session, Context, Pool};
use uuid::Uuid;

pub struct LoggedUser {
    pub id: Uuid,
    pub session_id: String,
}

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Result<LoggedUser, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let ctx = ContextProvider::from_request(req, payload)?;

        let cookie = req.cookie("session_id").ok_or(Error::InvalidSession)?;
        let session_id = cookie.value();

        let id = authenticate_session(&ctx.into(), session_id)?;

        Ok(LoggedUser {
            id,
            session_id: session_id.to_string(),
        })
    }
}

pub struct ContextProvider(Pool);

impl FromRequest for ContextProvider {
    type Error = Error;
    type Future = Result<ContextProvider, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let pool = (*req.get_app_data::<Pool>().unwrap()).clone();

        Ok(ContextProvider(pool))
    }
}

impl<'a> From<ContextProvider> for Context<'a> {
    fn from(context_provider: ContextProvider) -> Self {
        Self::new(context_provider.0.get().unwrap(), SECRET_KEY.as_str())
    }
}
