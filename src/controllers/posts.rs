use axum::extract::Path;
use axum::response::Html;
use rss::{ChannelBuilder, Item};

use crate::models::posts::Post;
use crate::models::{DatabaseConnection, DatabaseError};
use crate::views::{blog_post, posts};

pub async fn list(DatabaseConnection(mut conn): DatabaseConnection) -> Result<Html<String>, DatabaseError> {
  let posts = Post::get_posts(&mut conn, 10, 0).await?;
  Ok(Html(posts::render(posts).to_string()))
}

pub async fn get(
  Path(blog_url): Path<String>,
  DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Html<String>, DatabaseError> {
  let post = Post::get_post_by_url(&mut conn, blog_url).await?;
  let res = blog_post::render(&post).to_string();
  Ok(Html(res))
}

pub async fn rss(DatabaseConnection(mut conn): DatabaseConnection) -> Result<String, DatabaseError> {
  let post: Vec<Item> = Post::get_all_posts(&mut conn)
    .await?
    .into_iter()
    .map(|x| x.into())
    .collect::<Vec<_>>();

  let channel = ChannelBuilder::default()
    .title("Ramblings form Chris".to_string())
    .link("https://chrismiller.xyz/posts".to_string())
    .description("Tech and Art Ramblings".to_string())
    .items(post)
    .build();

  Ok(channel.to_string())
}
