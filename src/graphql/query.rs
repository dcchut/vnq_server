use std::convert::TryFrom;

use diesel::prelude::*;

use crate::db::visible_quotes;
use crate::graphql::Context;

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
