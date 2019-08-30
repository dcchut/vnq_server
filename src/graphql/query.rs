use juniper::{FieldError, FieldResult, graphql_value};

use crate::db::{models as dbm, recent_quotes, top_quotes};
use crate::graphql::{Context, Quote};

pub struct Query;

/// Transforms a vector of (db::Quote,Option<db::User>) to a vector of Quote's
/// (i.e. db -> graphql)
fn transform_quotes(v: Vec<(dbm::Quote, Option<dbm::User>)>) -> Vec<Quote> {
    v.into_iter().map(|x| x.into()).collect()
}

#[juniper::object(context = Context)]
impl Query {
    fn top_quotes(ctx: &Context) -> FieldResult<Vec<Quote>> {
        top_quotes(&ctx.conn)
            .map(transform_quotes)
            .map_err(|e| FieldError::new(e, graphql_value!({ "internal_error": "Database error" })))
    }

    fn recent_quotes(ctx: &Context) -> FieldResult<Vec<Quote>> {
        recent_quotes(&ctx.conn)
            .map(transform_quotes)
            .map_err(|e| FieldError::new(e, graphql_value!({ "internal_error": "Database error" })))
    }
}
