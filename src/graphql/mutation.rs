use std::env;

use jsonwebtoken::{Algorithm, encode, Header};
use juniper::{FieldError, FieldResult, graphql_value};

use crate::db::{create_quote, create_user, verify_login};
use crate::graphql::{Claims, Context};

pub struct Mutation;

fn generate_token(user: crate::db::models::User) -> FieldResult<String> {
    // Construct the user's claim
    let claim = Claims::new(user.is_admin, Some(user.id), Some(user.username));
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    Ok(encode(
        &Header::new(Algorithm::HS256),
        &claim,
        secret_key.as_bytes(),
    )?)
}

#[juniper::object(context = Context)]
impl Mutation {
    fn new_quote(ctx: &Context, quote: String) -> FieldResult<crate::graphql::models::Quote> {
        let (quote, user) = create_quote(
            &ctx.conn,
            &quote,
            match &ctx.ip {
                Some(ip) => ip.as_ref(),
                None => "",
            },
            ctx.claims.user_id,
        )
        .map_err(|e| FieldError::new(e, graphql_value!({ "internal_error": "Database error" })))?;

        Ok((quote, user).into())
    }

    fn login(ctx: &Context, username: String, password: String) -> FieldResult<String> {
        let user = verify_login(&ctx.conn, &username, &password).map_err(|e| {
            FieldError::new(
                "Invalid username/password combination",
                graphql_value!({"user_error": "Invalid authentication" }),
            )
        })?;

        generate_token(user)
    }

    fn signup(ctx: &Context, username: String, password: String) -> FieldResult<String> {
        let user = create_user(&ctx.conn, &username, &password).map_err(|e| {
            FieldError::new(
                "Invalid username",
                graphql_value!({"user_error": "Invalid username" }),
            )
        })?;

        generate_token(user)
    }
}
