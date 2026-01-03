use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use pulldown_cmark::{html, Event, Parser, TagEnd};
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

  /// Returns a rendered HTML preview of the post, truncated at ~120 chars of text.
  /// Smart truncation ensures we don't cut off in the middle of a markdown element.
  pub fn rendered_description(&self) -> String {
    let parser = Parser::new(&self.plaintext_body);
    let mut events: Vec<Event> = Vec::new();
    let mut text_len = 0;
    let mut open_tags = 0;

    for event in parser {
      match &event {
        Event::Text(text) => {
          if text_len >= 120 && open_tags == 0 {
            break;
          }
          text_len += text.len();
          events.push(event);
        }
        Event::Start(_) => {
          open_tags += 1;
          events.push(event);
        }
        Event::End(TagEnd::Paragraph) => {
          open_tags -= 1;
          events.push(event);
          // Safe to stop after closing a paragraph if we've hit the limit
          if text_len >= 120 && open_tags == 0 {
            break;
          }
        }
        Event::End(_) => {
          open_tags -= 1;
          events.push(event);
        }
        _ => events.push(event),
      }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    format!("{}...", html_output)
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
      pub_date: Some(self.posted.and_local_timezone(Utc).unwrap().to_rfc2822()),
      ..Default::default()
    }
  }
}