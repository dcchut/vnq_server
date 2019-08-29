use chrono::NaiveDateTime;
use diesel::helper_types::{And, Eq, Filter, FindBy};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::schema;

fn now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

/// The type of the query used to search the db for all visible quotes
pub type VisibleQuotesQuery =
    FindBy<schema::quotes::dsl::quotes, schema::quotes::dsl::visible, bool>;

/// Returns a query that gets all visible quotes
pub fn visible_quotes() -> VisibleQuotesQuery {
    use schema::quotes::dsl::*;

    quotes.filter(visible.eq(true))
}

/// The type of the query used to search the db for a given username/password combination
pub type LoginQuery<'a> = Filter<
    schema::users::dsl::users,
    And<Eq<schema::users::dsl::username, &'a str>, Eq<schema::users::dsl::password, &'a str>>,
>;

/// Returns a query that determines whether a given username/password combination
/// exists in our database.  If the combination does exist, the query
/// returns the user in question.
pub fn login<'a>(_username: &'a str, _password: &'a str) -> LoginQuery<'a> {
    use schema::users::dsl::*;

    users.filter(username.eq(_username).and(password.eq(_password)))
}

pub type UserQuery = FindBy<schema::users::dsl::users, schema::users::dsl::id, i32>;

pub fn get_user(user_id: i32) -> UserQuery {
    use schema::users::dsl::*;

    users.filter(id.eq(user_id))
}

/// Attempts to create a basic user with the given username and password
/// TODO: maybe refactor this as a query function?  the type sounds difficult...
pub fn create_user<'a>(
    conn: &SqliteConnection,
    username: &'a str,
    password: &'a str,
) -> Result<crate::db::models::User, Error> {
    let current_time = now();

    let new_user = crate::db::models::NewUser {
        username,
        password,
        is_admin: false,
        created_at: &current_time,
        updated_at: &current_time,
    };

    conn.transaction(|| {
        use schema::users::dsl::*;

        let inserted_count = diesel::insert_into(schema::users::table)
            .values(&new_user)
            .execute(conn)?;

        Ok(users
            .order(id.desc())
            .limit(inserted_count as i64)
            .get_result::<crate::db::models::User>(conn)?)
    })
}

/// Creates a new quote
/// TODO: maybe refactor as a query function?
pub fn create_quote<'a>(
    conn: &SqliteConnection,
    content: &'a str,
    ip: &'a str,
    user_id: Option<i32>,
) -> Result<crate::db::models::Quote, Error> {
    let current_time = now();

    let new_quote = crate::db::models::NewQuote {
        content,
        votes: 0,
        visible: false,
        moderated_by: None,
        ip,
        created_at: &current_time,
        updated_at: &current_time,
        user_id,
    };

    conn.transaction(|| {
        use schema::quotes::dsl::*;

        let inserted_count = diesel::insert_into(schema::quotes::table)
            .values(&new_quote)
            .execute(conn)?;

        Ok(quotes
            .order(id.desc())
            .limit(inserted_count as i64)
            .get_result::<crate::db::models::Quote>(conn)?)
    })
}
