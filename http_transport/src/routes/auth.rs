use crate::{errors::Error, ContextProvider, LoggedUser};
use actix_web::{http::Cookie, web, HttpResponse};
use core::{auth, Context};
use futures::Future;
use time::now;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserForm {
    email: String,
    password: String,
}

// TODO: Literally the same function as `login` but with `auth::signup` instand
// of `auth::login`. Fix that.
pub fn signup(
    form: web::Json<UserForm>,
    ctx: ContextProvider,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let form = form.into_inner();

    web::block(move || {
        let ctx: Context = ctx.into();
        let user_id = auth::signup(&ctx, &form.email, &form.password)?;
        auth::create_session(&ctx, &user_id).map_err(Error::from)
    })
    .from_err()
    .and_then(|session_id| {
        let mut res = HttpResponse::Ok().finish();
        res.add_cookie(&Cookie::new("session_id", session_id))?;
        Ok(res)
    })
}

pub fn login(
    form: web::Json<UserForm>,
    ctx: ContextProvider,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let form = form.into_inner();

    web::block(move || {
        let ctx: Context = ctx.into();
        let user_id = auth::login(&ctx, &form.email, &form.password)?;
        auth::create_session(&ctx, &user_id).map_err(Error::from)
    })
    .from_err()
    .and_then(move |session_id| {
        let mut res = HttpResponse::Ok().finish();
        res.add_cookie(&Cookie::new("session_id", session_id))?;
        Ok(res)
    })
}

pub fn authenticate(_: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn logout(
    logged_user: LoggedUser,
    ctx: ContextProvider,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        auth::delete_session(&ctx.into(), &logged_user.session_id).map_err(Error::from)
    })
    .from_err()
    .and_then(move |_| {
        let mut res = HttpResponse::Ok().finish();
        res.add_cookie(
            &Cookie::build("session_id", "")
                .expires(now() - time::Duration::weeks(1))
                .finish(),
        )
        .unwrap();
        Ok(res)
    })
}

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth")
            .route(web::post().to_async(login))
            .route(web::get().to_async(authenticate))
            .route(web::delete().to_async(logout)),
    )
    .service(web::resource("/auth/signup").route(web::post().to_async(signup)));
}
