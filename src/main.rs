use std::net::SocketAddr;

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::controllers::build_router;
use crate::models::{run_migrations, get_db_url};

mod controllers;
pub mod models;
pub mod schema;
mod views;

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "chrismiller_xyz=debug".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

  let db_url = get_db_url();

  run_migrations(&db_url);

  let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
  let pool = bb8::Pool::builder()
    .build(config)
    .await
    .expect("Failed to build database pool");

  let app = build_router(pool);

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  tracing::debug!("listening on {}", addr);

  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
