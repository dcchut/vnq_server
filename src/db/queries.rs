use std::num::NonZeroU32;

use chrono::NaiveDateTime;
use diesel::helper_types::{Asc, Desc, FindBy, LeftJoin, Limit, Order};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;

use crate::db::models as dbm;
use crate::schema::*;

fn now() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

type QuotesQuery = FindBy<LeftJoin<quotes::table, users::table>, quotes::visible, bool>;

fn quotes_query() -> QuotesQuery {
    quotes::table
        .left_join(users::table)
        .filter(quotes::visible.eq(true))
}

type TopQuotesQuery = Limit<Order<QuotesQuery, (Desc<quotes::votes>, Asc<quotes::id>)>>;

fn top_quotes_query() -> TopQuotesQuery {
    quotes_query()
        .order((quotes::votes.desc(), quotes::id.asc()))
        .limit(30)
}

type RecentQuotesQuery = Order<QuotesQuery, Desc<quotes::id>>;

fn recent_quotes_query() -> RecentQuotesQuery {
    quotes_query().order(quotes::id.desc())
}

type UserQuery<'a> = FindBy<users::table, users::username, &'a str>;

fn user_query(_username: &str) -> UserQuery<'_> {
    users::table.filter(users::username.eq(_username))
}

/// Attempts to return the recent quotes
pub fn recent_quotes(
    conn: &SqliteConnection,
) -> Result<Vec<(dbm::Quote, Option<dbm::User>)>, Error> {
    recent_quotes_query().load::<(dbm::Quote, Option<dbm::User>)>(conn)
}

/// Attempts to return the top quotes
pub fn top_quotes(conn: &SqliteConnection) -> Result<Vec<(dbm::Quote, Option<dbm::User>)>, Error> {
    top_quotes_query().load::<(dbm::Quote, Option<dbm::User>)>(conn)
}

fn generate_salt() -> [u8; 64] {
    use ring::rand::SecureRandom;
    use ring::{digest, rand};

    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).expect("Salt generation failed");

    salt
}

fn hash(salt: &[u8; 64], password: &str) -> [u8; 64] {
    use ring::{digest, pbkdf2};

    let mut pw_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        &digest::SHA512,
        NonZeroU32::new(100_000).unwrap(),
        salt,
        password.as_bytes(),
        &mut pw_hash,
    );

    pw_hash
}

fn verify(salt: &[u8], password: &str, hash: &[u8]) -> bool {
    use ring::{digest, pbkdf2};

    pbkdf2::verify(
        &digest::SHA512,
        NonZeroU32::new(100_000).unwrap(),
        &salt,
        password.as_bytes(),
        &hash,
    )
    .is_ok()
}

/// Attempts to return the user with the given username & password combination
pub fn verify_login(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<dbm::User, Error> {
    let user = user_query(username).get_result::<dbm::User>(conn)?;

    let salt = data_encoding::HEXLOWER.decode(user.salt.as_bytes());
    let hash = data_encoding::HEXLOWER.decode(user.password.as_bytes());

    if salt.is_err()
        || hash.is_err()
        || !verify(salt.as_ref().unwrap(), password, hash.as_ref().unwrap())
    {
        Err(Error::NotFound)
    } else {
        Ok(user)
    }
}

/// Attempts to create a user with the given username & password combination
pub fn create_user(
    conn: &SqliteConnection,
    username: &str,
    password: &str,
) -> Result<crate::db::models::User, Error> {
    let current_time = now();

    let salt = generate_salt();
    let pw_hash = hash(&salt, password);

    let new_user = crate::db::models::NewUser {
        username,
        password: &data_encoding::HEXLOWER.encode(&pw_hash),
        salt: &data_encoding::HEXLOWER.encode(&salt),
        is_admin: false,
        created_at: &current_time,
        updated_at: &current_time,
    };

    conn.transaction(|| {
        let inserted_count = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        Ok(users::table
            .order(users::id.desc())
            .limit(inserted_count as i64)
            .get_result::<dbm::User>(conn)?)
    })
}

/// Attempts to create a new quote
pub fn create_quote(
    conn: &SqliteConnection,
    content: &str,
    ip: &str,
    user_id: Option<i32>,
) -> Result<(dbm::Quote, Option<dbm::User>), Error> {
    let current_time = now();

    let new_quote = dbm::NewQuote {
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
        let inserted_count = diesel::insert_into(quotes::table)
            .values(&new_quote)
            .execute(conn)?;

        Ok(quotes::table
            .order(quotes::id.desc())
            .limit(inserted_count as i64)
            .left_join(users::table)
            .get_result::<(dbm::Quote, Option<dbm::User>)>(conn)?)
    })
}
