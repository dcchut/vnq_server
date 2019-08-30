use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

pub use self::models::*;
pub use self::mutation::Mutation;
pub use self::query::Query;

pub mod models;
pub mod mutation;
pub mod query;

pub struct Context {
    conn: SqliteConnection,
    claims: Claims,
    ip: Option<String>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // If this user is logged in, are they an admin?
    is_admin: bool,

    // If this user is logged in, what is their user id
    user_id: Option<i32>,

    // The user's name
    username: Option<String>,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            is_admin: false,
            user_id: None,
            username: None,
        }
    }
}

impl Claims {
    pub fn new(is_admin: bool, user_id: Option<i32>, username: Option<String>) -> Self {
        Self {
            is_admin,
            user_id,
            username,
        }
    }
}

impl Context {
    pub fn new(conn: SqliteConnection) -> Self {
        Self {
            conn,
            claims: Claims::default(),
            ip: None,
        }
    }

    pub fn with(&mut self, claims: Claims) {
        self.claims = claims;
    }

    pub fn ip(&mut self, ip: &str) {
        self.ip = Some(String::from(ip));
    }
}

impl juniper::Context for Context {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
