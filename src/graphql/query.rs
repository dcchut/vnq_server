use std::convert::TryFrom;

use diesel::prelude::*;

use crate::db::visible_quotes;
use crate::graphql::Context;

pub struct Query;

#[juniper::object(context = Context)]
impl Query {
    fn top_quotes(ctx: &Context) -> Vec<crate::graphql::models::Quote> {
        use crate::schema::quotes::dsl::*;

        // TODO factor this query out to top_quotes() or something like that
        if let Ok(_quotes) = visible_quotes()
            .order((votes.desc(), id.asc()))
            .limit(30)
            .load::<crate::db::models::Quote>(&ctx.conn)
        {
            _quotes
                .into_iter()
                .map(|q| crate::graphql::models::Quote::try_from((q, &ctx.conn)))
                .collect::<Result<Vec<crate::graphql::models::Quote>, diesel::result::Error>>()
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    fn quotes(ctx: &Context) -> Vec<crate::graphql::models::Quote> {
        use crate::schema::quotes::dsl::*;

        if let Ok(_quotes) = visible_quotes()
            .order(id.desc())
            .load::<crate::db::models::Quote>(&ctx.conn)
        {
            _quotes
                .into_iter()
                .map(|q| crate::graphql::models::Quote::try_from((q, &ctx.conn)))
                .collect::<Result<Vec<crate::graphql::models::Quote>, diesel::result::Error>>()
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }
}
