use actix_web::Responder;
use actix_web::web;

pub async fn get_comments(post_id: web::Path<i32>) -> impl Responder {
    format!("GET comments on post {}", post_id.into_inner())
}

pub async fn create_comment(post_id: web::Path<i32>) -> impl Responder {
    format!("CREATE comment on POST {}", post_id.into_inner())
}
