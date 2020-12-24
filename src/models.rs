use crate::schema::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub latitude: &'a f64,
    pub longitude: &'a f64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, Identifiable)]
#[belongs_to(Post)]
#[table_name = "comments"]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub content: &'a str,
    pub post_id: &'a Uuid,
    pub created_at: chrono::NaiveDateTime,
}