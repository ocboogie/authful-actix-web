use crate::schema::{sessions, users};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[table_name = "sessions"]
pub struct Session {
    pub id: String,
    pub user_id: Uuid,
    pub expires: NaiveDateTime,
}
