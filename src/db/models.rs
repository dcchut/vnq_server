use chrono::NaiveDateTime;

use crate::schema::{quotes, users};

#[derive(Identifiable, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub is_admin: bool,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug)]
#[belongs_to(User)]
pub struct Quote {
    pub id: i32,
    pub content: String,
    pub votes: i32,
    pub visible: bool,
    pub moderated_by: Option<i32>,
    pub ip: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "quotes"]
pub struct NewQuote<'a> {
    pub content: &'a str,
    pub votes: i32,
    pub visible: bool,
    pub moderated_by: Option<i32>,
    pub ip: &'a str,
    pub created_at: &'a NaiveDateTime,
    pub updated_at: &'a NaiveDateTime,
    pub user_id: Option<i32>,
}
