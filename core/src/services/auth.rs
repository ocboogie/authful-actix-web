use crate::{
    models::auth::{Session, User},
    utils::auth::{hash_password, hash_session_id, verify_password},
    Context,
};
use chrono::{Duration, Utc};
use diesel::{
    prelude::*,
    result::{DatabaseErrorKind, Error as DBError},
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use uuid::Uuid;
use log::error;

lazy_static::lazy_static! {
    static ref MOCK_PASSWORD: String = hash_password("never_used", "this shouldn't match anything").unwrap();
}

pub enum Error {
    DatabaseError(DBError),
    EmailInUse,
    IncorrectCredentials,
    InvalidSession,
    SessionExpired,
    InvalidPassword,
}

impl From<DBError> for Error {
    fn from(db_error: DBError) -> Self {
        Error::DatabaseError(db_error)
    }
}

pub fn signup(ctx: &Context, user_email: &str, user_password: &str) -> Result<Uuid, Error> {
    use crate::schema::users::dsl::*;

    let uuid = uuid::Uuid::new_v4();

    let hashed_password =
        hash_password(ctx.secret, &user_password).map_err(|_| Error::InvalidPassword)?;

    diesel::insert_into(users)
        .values((
            id.eq(uuid),
            email.eq(user_email),
            password.eq(hashed_password),
            created_at.eq(diesel::dsl::now),
        ))
        .execute(&ctx.conn)
        .map_err(|err| match err {
            DBError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => Error::EmailInUse,
            error => Error::DatabaseError(error),
        })?;

    Ok(uuid)
}

pub fn login(ctx: &Context, user_email: &str, user_password: &str) -> Result<Uuid, Error> {
    use crate::schema::users::dsl::*;

    match users.filter(email.eq(user_email)).first::<User>(&ctx.conn) {
        Ok(user) => {
            match verify_password(ctx.secret, &user.password, &user_password) {
                Ok(true) => Ok(user.id),
                _ => Err(Error::IncorrectCredentials)
            }
        }
        Err(_) => {
            // This is here to prevent an attacker from getting a list of user's
            // emails by comparing time differences.
            // https://www.owasp.org/index.php/Testing_for_User_Enumeration_and_Guessable_User_Account_(OWASP-AT-002)
            if let Err(err) = verify_password(ctx.secret, &MOCK_PASSWORD, &user_password) {
                error!("There was an error trying to prevent the possibility of an attack. THIS SHOULD BE FIXED AS SOON AS POSSIBLE: {}", err);
            }
            Err(Error::IncorrectCredentials)
        }
    }
}

pub fn create_session(ctx: &Context, user_uuid: &uuid::Uuid) -> Result<String, Error> {
    use crate::schema::sessions::dsl::*;

    let session_id: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    let hashed_session_id = hash_session_id(&session_id);

    // FIXME
    let expiration = (Utc::now() + Duration::days(1)).naive_utc();

    diesel::insert_into(sessions)
        .values((
            id.eq(hashed_session_id),
            user_id.eq(user_uuid),
            expires.eq(expiration),
        ))
        .execute(&ctx.conn)?;

    Ok(session_id)
}

pub fn authenticate_session(ctx: &Context, session_id: &str) -> Result<Uuid, Error> {
    use crate::schema::sessions::dsl::*;

    let hashed_session_id = hash_session_id(session_id);

    let session = sessions
        .filter(id.eq(&hashed_session_id))
        .first::<Session>(&ctx.conn)
        .map_err(|_| Error::InvalidSession)?;

    if Utc::now().naive_utc() > session.expires {
        delete_session_hashed_id(ctx, &hashed_session_id)?;

        return Err(Error::SessionExpired);
    }

    Ok(session.user_id)
}

pub fn delete_session(ctx: &Context, session_id: &str) -> Result<(), Error> {
    let hashed_session_id = hash_session_id(session_id);
    delete_session_hashed_id(ctx, &hashed_session_id)
}

pub fn delete_session_hashed_id(ctx: &Context, hashed_session_id: &str) -> Result<(), Error> {
    use crate::schema::sessions::dsl::*;
    diesel::delete(sessions.filter(id.eq(hashed_session_id))).execute(&ctx.conn)?;

    Ok(())
}
