use std::net::SocketAddr;

use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::Json;
use axum::routing::{get, post};
use axum::{async_trait, Router};
use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "chrismiller_xyz=debug".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

  let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not defined");

  let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
  let pool = bb8::Pool::builder()
    .build(config)
    .await
    .expect("Failed to build database pool");

  let app = Router::new().with_state(pool);

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::debug!("listening on {}", addr);

  axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
