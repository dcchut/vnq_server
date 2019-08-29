use std::convert::TryFrom;
use std::env;

use diesel::prelude::*;
use jsonwebtoken::{encode, Algorithm, Header};
use juniper::{graphql_value, FieldError, FieldResult};

use crate::db::{create_quote, create_user, login};
use crate::graphql::{Claims, Context};

pub struct Mutation;

fn generate_token(user: crate::db::models::User) -> FieldResult<String> {
    dbg!(&user);

    // Construct the user's claim
    let claim = Claims::new(user.is_admin, Some(user.id));
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    Ok(encode(
        &Header::new(Algorithm::HS256),
        &claim,
        secret_key.as_bytes(),
    )?)
}

#[juniper::object(context = Context)]
impl Mutation {
    fn new_quote(
        ctx: &Context,
        quote: crate::graphql::models::NewQuote,
    ) -> FieldResult<crate::graphql::models::Quote> {
        let quote = create_quote(&ctx.conn, &quote.content, "127.0.0.1", ctx.claims.user_id)
            .map_err(|e| {
                FieldError::new(e, graphql_value!({ "internal_error": "Database error" }))
            })?;

        // Attempt to convert our database quote object into a graphql quote object
        crate::graphql::models::Quote::try_from((quote, &ctx.conn))
            .map_err(|e| FieldError::new(e, graphql_value!({ "internal_error": "Database error" })))
    }

    fn login(ctx: &Context, username: String, password: String) -> FieldResult<String> {
        let user = login(&username, &password)
            .get_result::<crate::db::models::User>(&ctx.conn)
            .map_err(|e| {
                FieldError::new(
                    "Invalid username/password combination",
                    graphql_value!({"user_error": "Invalid authentication" }),
                )
            })?;

        generate_token(user)
    }

    fn signup(ctx: &Context, username: String, password: String) -> FieldResult<String> {
        // DO SOME PROCESSING TO THE PASSWORD HERE!
        // TODO

        let user = create_user(&ctx.conn, &username, &password).map_err(|e| {
            FieldError::new(
                "Invalid username",
                graphql_value!({"user_error": "Invalid username" }),
            )
        })?;

        generate_token(user)
    }
}
