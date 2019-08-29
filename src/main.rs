use std::sync::Arc;
use std::{env, io};

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use dotenv::dotenv;
use futures::future::Future;
use jsonwebtoken::{decode, Algorithm, Validation};
use juniper::http::GraphQLRequest;

use server2::establish_connection;
use server2::graphql::Claims;
use server2::graphql::{create_schema, Context, Schema};

fn graphql(
    st: web::Data<Arc<Schema>>,
    credentials: Option<BearerAuth>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // It seems there are two ways we could get a connection here
    // (1) Establish a new connection on each request, or
    // (2) Pass around an Arc<Mutex<SqliteConnection>>
    // For the moment we're using (1), though the performance implications of this
    // choice should be investigated.
    let mut ctx = Context::new(establish_connection());

    // Update the claims in our context
    if let Some(credentials) = credentials {
        let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

        // Our tokens don't have expiry dates, so don't validate them
        // TODO: maybe our tokens should have expiry dates?
        let mut v = Validation::new(Algorithm::HS256);
        v.validate_exp = false;

        if let Ok(token) = decode::<Claims>(credentials.token(), secret_key.as_bytes(), &v) {
            ctx.with(token.claims);
        }
    }

    let res = data.execute(&st, &ctx);

    futures::done(
        serde_json::to_string(&res)
            .map_err(Error::from)
            .and_then(|user| {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(user))
            }),
    )
}

fn main() -> io::Result<()> {
    dotenv().ok();

    // Create Juniper schema
    let schema = Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .send_wildcard()
                    // TODO: figure out correct CORS settings
                    // for now we just send *
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to_async(graphql))
                    .route(web::get().to_async(graphql)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
