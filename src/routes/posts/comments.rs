use super::*;
use crate::schema::comments::dsl::*;
use crate::models::{Comment, NewComment};
use uuid::Uuid;
use actix_web::error::BlockingError;
use diesel::result::{Error::DatabaseError, DatabaseErrorKind::ForeignKeyViolation};
use crate::errors::ApiError;


#[derive(Debug, Serialize, Deserialize)]
pub struct CommentData {
    pub content: String,
}

fn map_db_error(error: BlockingError<diesel::result::Error>) -> ApiError {
    match error {
        BlockingError::Error(DatabaseError(ForeignKeyViolation, _)) => {
            ApiError::BadRequest(String::from("The post is inexistent."))
        },
        _ => ApiError::InternalServerError,
    }
}

fn db_get_post_comments(
    db: web::Data<Pool>,
    post_id_path: web::Path<Uuid>
) -> Result<Vec<Comment>, diesel::result::Error>
{
    let conn = db.get().unwrap();
    Ok(
        comments
            .filter(post_id.eq(*post_id_path))
            .load::<Comment>(&conn)?
    )
}

pub async fn get_comments(
    db: web::Data<Pool>,
    post_id_path: web::Path<Uuid>
) -> Result<HttpResponse, ApiError>
{
    Ok(
        web::block(move || db_get_post_comments(db, post_id_path))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|error| map_db_error(error))?
    )
}

pub async fn create_comment(
    db: web::Data<Pool>,
    comment_data: web::Json<CommentData>,
    post_id_path: web::Path<Uuid>
) -> Result<HttpResponse, Error>
{
    Ok(
        web::block(move || db_create_comment(db, comment_data, post_id_path))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|error| map_db_error(error))?
    )
}

fn db_create_comment(
    db: web::Data<Pool>, 
    commentdata: web::Json<CommentData>,
    post_id_path: web::Path<Uuid>
) -> Result<Comment, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let new_comment = NewComment {
        post_id: &post_id_path,
        content: &commentdata.content,
        created_at: chrono::Local::now().naive_local(),
    };
    Ok(insert_into(comments).values(&new_comment).get_result(&conn)?)
}