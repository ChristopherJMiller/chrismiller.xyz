use axum::response::Html;

use crate::{views::index, models::{posts::Post, DatabaseConnection, DatabaseError}};

pub async fn index(DatabaseConnection(mut conn): DatabaseConnection) -> Result<Html<String>, DatabaseError> {
  let most_recent_post = Post::get_posts(&mut conn, 2, 0).await?;

  let res = Html(index::render(&most_recent_post).to_string());
  Ok(res)
}
