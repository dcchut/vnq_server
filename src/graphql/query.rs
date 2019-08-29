use crate::graphql::Context;
use crate::visible_quotes;
use diesel::prelude::*;
use std::convert::TryFrom;

pub struct Query;

#[juniper::object(context = Context)]
impl Query {
    fn quotes(ctx: &Context) -> Vec<crate::graphql::models::Quote> {
        if let Ok(quotes) = visible_quotes().load::<crate::db::models::Quote>(&ctx.conn) {
            quotes
                .into_iter()
                .map(|q| crate::graphql::models::Quote::try_from((q, &ctx.conn)))
                .collect::<Result<Vec<crate::graphql::models::Quote>, diesel::result::Error>>()
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }
}
