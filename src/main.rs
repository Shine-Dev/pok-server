#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, HttpServer, Error};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_web_httpauth::extractors::{AuthenticationError, bearer::BearerAuth, bearer::Config};
use actix_web_httpauth::middleware::HttpAuthentication;

mod routes;
mod errors;
mod models;
mod schema;
mod auth;
mod environment;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(_) => Ok(req),
        _ => Err(AuthenticationError::from(config).into())
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = environment::variables::expect_variable("DATABASE_URL");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(HttpAuthentication::bearer(validator))
            .data(pool.clone())
            .route("/posts", web::get().to(routes::posts::get_posts))
            .route("/posts/{id}/comments", web::get().to(routes::posts::comments::get_comments))
            .route("/posts", web::post().to(routes::posts::create_post))
            .route("/posts/{id}/comments", web::post().to(routes::posts::comments::create_comment))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
