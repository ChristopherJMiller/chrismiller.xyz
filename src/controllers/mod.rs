use axum::routing::get;
use axum::Router;

use crate::models::Pool;

mod posts;

pub fn build_router(pool: Pool) -> Router {
  Router::new().route("/posts", get(posts::list)).with_state(pool)
}
