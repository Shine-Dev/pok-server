use actix_web::Responder;

pub mod comments;

pub async fn get_posts() -> impl Responder {
    format!("GET posts")
}

pub async fn create_post() -> impl Responder {
    format!("Create post")
}