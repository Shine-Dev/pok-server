use super::*;
use crate::schema::comments::dsl::*;
use crate::models::{Comment, NewComment};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentData {
    pub content: String,
}

fn db_get_post_comments(
    db: &web::Data<Pool>,
    post_id_path: &web::Path<i64>
) -> Result<Vec<Comment>, diesel::result::Error>
{
    let conn = db.get().unwrap();
    Ok(
        comments
            .filter(post_id.eq(**post_id_path))
            .load::<Comment>(&conn)?
    )
}

pub async fn get_comments(
    db: web::Data<Pool>,
    post_id_path: web::Path<i64>
) -> Result<HttpResponse, Error>
{
    Ok(
        web::block(move || db_get_post_comments(&db, &post_id_path))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

pub async fn create_comment(
    db: web::Data<Pool>,
    comment_data: web::Json<CommentData>,
    post_id_path: web::Path<i64>
) -> Result<HttpResponse, Error>
{
    Ok(
        web::block(move || db_create_comment(&db, &comment_data, &post_id_path))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_create_comment(
    db: &web::Data<Pool>, 
    commentdata: &web::Json<CommentData>,
    post_id_path: &web::Path<i64>
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