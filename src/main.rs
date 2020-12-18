use actix_web::{web, App, HttpServer};

mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/posts", web::get().to(routes::posts::get_posts))
            .route("/posts", web::post().to(routes::posts::create_post))
            .route("/posts/{id}/comments", web::get().to(routes::posts::comments::get_comments))
            .route("/posts/{id}/comments", web::post().to(routes::posts::comments::create_comment))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
