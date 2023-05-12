use axum::response::Html;

use crate::models::posts::Post;
use crate::models::{DatabaseConnection, DatabaseError};
use crate::views::index;

pub async fn index(DatabaseConnection(mut conn): DatabaseConnection) -> Result<Html<String>, DatabaseError> {
  let most_recent_post = Post::get_posts(&mut conn, 1, 0).await?;

  let res = Html(index::render(&most_recent_post[0]).to_string());
  Ok(res)
}
