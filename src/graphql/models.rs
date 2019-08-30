use juniper::{GraphQLInputObject, GraphQLObject, ID};

use crate::db::models as dbm;

/// This struct represents the publicly available information about a user
#[derive(GraphQLObject)]
#[graphql(description = "A user of our website")]
pub struct User {
    pub username: String,
    pub is_admin: bool,
}

/// The information required by the client to create a new user
#[derive(GraphQLInputObject)]
#[graphql(description = "A user of our website")]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl From<dbm::User> for User {
    fn from(user: dbm::User) -> Self {
        Self {
            username: user.username,
            is_admin: user.is_admin,
        }
    }
}

/// Represents a user submitted quote
#[derive(GraphQLObject)]
#[graphql(description = "A user submitted quote")]
pub struct Quote {
    pub id: ID,
    pub content: String,
    pub votes: i32,
    pub visible: bool,
    pub user: Option<crate::graphql::models::User>,
}

impl From<(dbm::Quote, Option<dbm::User>)> for Quote {
    fn from((_quote, _user): (dbm::Quote, Option<dbm::User>)) -> Self {
        Self {
            id: ID::from(_quote.id.to_string()),
            content: _quote.content,
            votes: _quote.votes,
            visible: _quote.visible,
            user: _user.map(dbm::User::into),
        }
    }
}
