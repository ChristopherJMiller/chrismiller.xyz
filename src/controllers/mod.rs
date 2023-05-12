use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::models::Pool;

mod index;
mod posts;

pub fn build_router(pool: Pool) -> Router {
  Router::new()
    // Routes
    .route("/", get(index::index))
    .route("/posts", get(posts::list))
    .route("/blog/:blog_url", get(posts::get))
    .route("/rss.xml", get(posts::rss))

    // Asset Serving
    .nest_service("/assets", ServeDir::new("dist"))
    .nest_service("/public", ServeDir::new("public"))
    .layer(TraceLayer::new_for_http())
    .with_state(pool)
}
