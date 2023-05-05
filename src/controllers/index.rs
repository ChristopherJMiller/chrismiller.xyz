use axum::response::Html;

use crate::views::index;

pub async fn index() -> Html<String> {
  Html(index::render().to_string())
}
