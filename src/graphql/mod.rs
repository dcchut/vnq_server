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
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.claims)
    }
}

// TODO: move this somewhere more appropriate
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // If this user is logged in, are they an admin?
    is_admin: bool,

    // If this user is logged in, what is their user id
    user_id: Option<i32>,
}

impl Default for Claims {
    fn default() -> Self {
        Self {
            is_admin: false,
            user_id: None,
        }
    }
}

impl Claims {
    pub fn new(is_admin: bool, user_id: Option<i32>) -> Self {
        Self { is_admin, user_id }
    }
}

impl Context {
    pub fn new(conn: SqliteConnection) -> Self {
        Self {
            conn,
            claims: Claims::default(),
        }
    }

    pub fn with(&mut self, claims: Claims) {
        self.claims = claims;
    }
}

impl juniper::Context for Context {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
