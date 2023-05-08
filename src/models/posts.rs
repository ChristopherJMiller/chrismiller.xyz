use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use rss::Item;
use serde::Serialize;

use super::{ConnectionFromPool, DatabaseError};

#[derive(Queryable, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
  pub id: i32,
  pub post_url: String,
  pub posted: NaiveDateTime,
  pub image_url: Option<String>,
  pub title: String,
  pub body: String,
  pub plaintext_body: String,
}

impl Post {
  pub fn description(&self) -> String {
    let mut copy = self.plaintext_body.clone().replace("#", "");
    copy.truncate(120);
    format!("{}...", copy)
  }

  pub fn format_date(&self) -> String {
    self.posted.format("%B %d, %Y").to_string()
  }

  pub async fn get_post_by_url<'a>(conn: &mut ConnectionFromPool, url: String) -> Result<Post, DatabaseError> {
    use crate::schema::posts::dsl::*;

    Ok(
      posts
        .filter(post_url.eq(url))
        .first(conn)
        .await
        .map_err(DatabaseError::DieselError)?,
    )
  }

  pub async fn get_posts<'a>(conn: &mut ConnectionFromPool, top: u32, skip: u32) -> Result<Vec<Post>, DatabaseError> {
    use crate::schema::posts::dsl::*;

    Ok(
      posts
        .limit(top.into())
        .offset(skip.into())
        .order_by(posted.desc())
        .load(conn)
        .await
        .map_err(DatabaseError::DieselError)?,
    )
  }

  pub async fn get_all_posts<'a>(conn: &mut ConnectionFromPool) -> Result<Vec<Post>, DatabaseError> {
    use crate::schema::posts::dsl::*;

    Ok(
      posts
        .load(conn)
        .await
        .map_err(DatabaseError::DieselError)?,
    )
  }
}

impl Into<Item> for Post {
  fn into(self) -> Item {
    Item {
      title: Some(self.title.clone()),
      link: Some(format!("https://chrismiller.xyz/blog/{}", self.post_url)),
      description: Some(self.description().clone()),
      pub_date: Some(self.posted.format("%+").to_string()),
      ..Default::default()
    }
  }
}