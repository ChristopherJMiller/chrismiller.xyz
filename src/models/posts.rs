use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Serialize;

use super::{ConnectionFromPool, DatabaseError};

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
  pub id: i32,
  pub post_url: String,
  pub posted: NaiveDateTime,
  pub image_url: Option<String>,
  pub title: String,
  pub body: String,
}

impl Post {
  pub async fn get_posts<'a>(conn: &mut ConnectionFromPool, top: u32, skip: u32) -> Result<Vec<Post>, DatabaseError> {
    use crate::schema::posts::dsl::*;

    Ok(
      posts
        .limit(top.into())
        .offset(skip.into())
        .load(conn)
        .await
        .map_err(DatabaseError::DieselError)?,
    )
  }
}
