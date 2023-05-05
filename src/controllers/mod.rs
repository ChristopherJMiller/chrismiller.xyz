use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;

use crate::models::Pool;

mod posts;
mod index;

pub fn build_router(pool: Pool) -> Router {
  Router::new()
    // Routes
    .route("/", get(index::index))
    .route("/posts", get(posts::list))

    // Asset Serving
    .nest_service("/assets", ServeDir::new("dist"))
    .nest_service("/public", ServeDir::new("public"))
    .with_state(pool)
}
