use std::convert::TryFrom;

use diesel::prelude::*;
use diesel::SqliteConnection;
use juniper::{GraphQLInputObject, GraphQLObject, ID};

use crate::db::get_user;

// TODO: perhaps define a proc-macro to generate the From<> implementations below

/// This struct represents the publicly available information about a user
/// TODO: getters/setters instead of public fields
#[derive(GraphQLObject)]
#[graphql(description = "A user of our website")]
pub struct User {
    pub username: String,
    pub is_admin: bool,
}

/// The information required by the client to create a new user
/// TODO: getters/setters instead of public fields
#[derive(GraphQLInputObject)]
#[graphql(description = "A user of our website")]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl From<crate::db::models::User> for User {
    fn from(user: crate::db::models::User) -> Self {
        Self {
            username: user.username,
            is_admin: user.is_admin,
        }
    }
}

/// Represents a user submitted quote
/// TODO: getters/setters instead of public fields
#[derive(GraphQLObject)]
#[graphql(description = "A user submitted quote")]
pub struct Quote {
    pub id: ID,
    pub content: String,
    pub votes: i32,
    pub visible: bool,
    pub user: Option<crate::graphql::models::User>,
}

impl TryFrom<(crate::db::models::Quote, &SqliteConnection)> for Quote {
    type Error = diesel::result::Error;

    fn try_from(v: (crate::db::models::Quote, &SqliteConnection)) -> Result<Self, Self::Error> {
        let (_quote, conn) = v;

        let mut quote = Quote {
            id: ID::new(_quote.id.to_string()),
            content: _quote.content,
            votes: _quote.votes,
            visible: _quote.visible,
            user: None,
        };

        // get the user associated with this quote
        // TODO: investigate whether diesel has a way to access the parent from a child
        if let Some(user_id) = _quote.user_id {
            let user = get_user(user_id).get_result::<crate::db::models::User>(conn)?;

            quote.user = Some(crate::graphql::User::from(user));
        }

        Ok(quote)
    }
}
