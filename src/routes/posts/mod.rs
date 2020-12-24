use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::*;
use crate::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use diesel::sql_types;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::insert_into;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::errors::ApiError;

pub mod comments;

const MAX_DISTANCE : f64 = 20f64;

sql_function!(
    fn haversine(
        lat: sql_types::Double, 
        lng: sql_types::Double, 
        other_lat: sql_types::Double, 
        other_lng: sql_types::Double
    ) -> sql_types::Double
);

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    pub title: String,
    pub content: String,
    pub location: LocationData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
}

fn db_get_near_posts(
    pool: web::Data<Pool>, 
    location: web::Query<LocationData>
) -> Result<Vec<Post>, diesel::result::Error> 
{
    let conn = pool.get().unwrap();
    Ok(posts
        .filter(
            haversine(
                latitude, 
                longitude, 
                &location.latitude, 
                &location.longitude
            )
            .le(MAX_DISTANCE)
        )
        .load::<Post>(&conn)?)
}

pub async fn get_posts(
    db: web::Data<Pool>,
    location: web::Query<LocationData>
) -> Result<HttpResponse, ApiError>
{
    Ok(
       web::block(move || db_get_near_posts(db, location))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| ApiError::InternalServerError)?
    )
}

pub async fn create_post(
    db: web::Data<Pool>,
    postdata: web::Json<PostData>
) -> Result<HttpResponse, ApiError>
{
    Ok(
        web::block(move || db_create_post(db, postdata))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| ApiError::InternalServerError)?
    )
}

fn db_create_post(
    db: web::Data<Pool>, 
    postdata: web::Json<PostData>
) -> Result<Post, diesel::result::Error>
{
    let conn = db.get().unwrap();
    let new_post = NewPost {
        title: &postdata.title,
        content: &postdata.content,
        latitude: &postdata.location.latitude,
        longitude: &postdata.location.longitude,
        created_at: chrono::Local::now().naive_local(),
    };
    Ok(insert_into(posts).values(&new_post).get_result(&conn)?)
}