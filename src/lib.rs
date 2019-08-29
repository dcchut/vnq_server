#[macro_use]
extern crate diesel;

use std::env;

use diesel::{Connection, SqliteConnection};

pub mod db;
pub mod graphql;
pub mod schema;

/// Establishes a connection to our backend database
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
