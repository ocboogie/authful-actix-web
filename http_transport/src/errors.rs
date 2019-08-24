use actix_web::{
    error::{BlockingError, ResponseError},
    http::{Error as AWError, StatusCode},
    HttpResponse,
};
use core::auth::Error as AuthError;
use std::fmt;

/// These are front-facing errors, therefore, should not contain any information
/// the user shouldn't be able to see (e.g. a database error).
#[derive(Debug, Clone, Copy, Serialize)]
pub enum Error {
    InternalServerError,
    EmailInUse,
    IncorrectCredentials,
    InvalidSession,
    InvalidPassword,
}

impl Error {
    pub fn status(&self) -> StatusCode {
        use Error::*;

        match self {
            InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            EmailInUse => StatusCode::BAD_REQUEST,
            IncorrectCredentials => StatusCode::UNAUTHORIZED,
            InvalidSession => StatusCode::UNAUTHORIZED,
            InvalidPassword => StatusCode::BAD_REQUEST,
        }
    }
}

/// Since `ResponseError` requires `Display` for the default implemention,
/// we must implement `Display`. This just displays `""`.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl ResponseError for Error {
    fn render_response(&self) -> HttpResponse {
        HttpResponse::build(self.status()).json::<ErrorResponseWrapper>((*self).into())
    }
}

#[derive(Serialize)]
pub struct ErrorResponseWrapper {
    error: ErrorResponse,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    #[serde(rename = "type")]
    ty: Error,
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        Self {
            status: error.status().as_u16(),
            ty: error,
        }
    }
}

impl From<Error> for ErrorResponseWrapper {
    fn from(error: Error) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl From<BlockingError<Error>> for Error {
    fn from(err: BlockingError<Error>) -> Error {
        match err {
            BlockingError::Error(service_error) => service_error,
            BlockingError::Canceled => Error::InternalServerError,
        }
    }
}

impl From<AWError> for Error {
    fn from(_: AWError) -> Error {
        Error::InternalServerError
    }
}

impl From<AuthError> for Error {
    fn from(error: AuthError) -> Error {
        use Error::*;

        match error {
            AuthError::DatabaseError(_) => InternalServerError,
            AuthError::EmailInUse => EmailInUse,
            AuthError::IncorrectCredentials => IncorrectCredentials,
            AuthError::InvalidSession => InvalidSession,
            AuthError::SessionExpired => InvalidSession,
            AuthError::InvalidPassword => InvalidPassword,
        }
    }
}
