use axum::response::Html;

use crate::models::posts::Post;
use crate::models::{DatabaseConnection, DatabaseError};
use crate::views::posts;

pub async fn list(DatabaseConnection(mut conn): DatabaseConnection) -> Result<Html<String>, DatabaseError> {
  let _posts = Post::get_posts(&mut conn, 10, 0).await?;

  Ok(Html(posts::render().to_string()))
}
