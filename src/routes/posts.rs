use bigdecimal::BigDecimal;
use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::*;
use crate::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use std::str::FromStr;
use diesel::sql_types;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::insert_into;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub mod comments;

const MAX_DISTANCE : &str = "10";

sql_function!(
    fn haversine(
        lat: sql_types::Numeric, 
        lng: sql_types::Numeric, 
        other_lat: sql_types::Numeric, 
        other_lng: sql_types::Numeric
    ) -> sql_types::Numeric
);

#[derive(Debug, Serialize, Deserialize)]
pub struct PostData {
    pub title: String,
    pub content: String,
    pub location: LocationData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: BigDecimal,
    pub longitude: BigDecimal,
}

fn db_get_near_posts(
    pool: &web::Data<Pool>, 
    location: &web::Query<LocationData>
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
            .le(BigDecimal::from_str(MAX_DISTANCE).unwrap())
        )
        .load::<Post>(&conn)?)
}

pub async fn get_posts(
    db: web::Data<Pool>,
    location: web::Query<LocationData>
) -> Result<HttpResponse, Error>
{
        Ok(
            web::block(move || db_get_near_posts(&db, &location))
                .await
                .map(|post| HttpResponse::Ok().json(post))
                .map_err(|_| HttpResponse::InternalServerError())?
        )
}

pub async fn create_post(
    db: web::Data<Pool>,
    postdata: web::Json<PostData>
) -> Result<HttpResponse, Error>
{
    Ok(
        web::block(move || db_create_post(&db, &postdata))
            .await
            .map(|post| HttpResponse::Ok().json(post))
            .map_err(|_| HttpResponse::InternalServerError())?
    )
}

fn db_create_post(
    db: &web::Data<Pool>, 
    postdata: &web::Json<PostData>
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