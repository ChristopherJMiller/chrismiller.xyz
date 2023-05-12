use std::net::SocketAddr;

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::controllers::build_router;
use crate::models::{establish_connection, get_db_url, run_migrations};

mod controllers;
pub mod models;
pub mod schema;
mod views;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "chrismiller_xyz=debug,tower_http=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let db_url = get_db_url();

  run_migrations(&db_url);

  let config =
    AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new_with_setup(db_url, establish_connection);
  let pool = bb8::Pool::builder()
    .min_idle(Some(1))
    .build(config)
    .await
    .expect("Failed to build database pool");

  {
    info!("Testing pool connection");
    let _ = pool.get().await;
  }

  let app = build_router(pool);

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  tracing::debug!("listening on {}", addr);

  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
