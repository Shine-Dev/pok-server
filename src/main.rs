use dotenv;
use std::env;

#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};


mod routes;
mod errors;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/posts", web::get().to(routes::posts::get_posts))
            .route("/posts", web::post().to(routes::posts::create_post))
            .route("/posts/{id}/comments", web::get().to(routes::posts::comments::get_comments))
            .route("/posts/{id}/comments", web::post().to(routes::posts::comments::create_comment))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
