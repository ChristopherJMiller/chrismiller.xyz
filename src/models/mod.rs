pub mod posts;

use std::env;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{async_trait, Json};
use bb8::{PooledConnection, RunError};
use diesel::{Connection, PgConnection};
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolError};
use diesel_async::AsyncPgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde_json::json;
use tracing::{info, debug};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type ConnectionFromPool = PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub fn get_db_url() -> String {
  if let Ok(url) = env::var("DATABASE_URL") {
    info!("Database Connection String Provided");
    url
  } else {
    info!("Building Connection String from Parts");
    let username = env::var("DATABASE_USER").expect("Failed to find DATABASE_USER");
    let password = env::var("DATABASE_PASS").expect("Failed to find DATABASE_PASS");
    let host = env::var("DATABASE_HOST").expect("Failed to find DATABASE_HOST");
    let db = env::var("DATABASE_DB").expect("Failed to find DATABASE_DB");

    format!("postgres://{username}:{password}@{host}/{db}")
  }
}

pub fn run_migrations(db_url: &str) {
  info!("Running any pending Migrations");
  let mut conn = PgConnection::establish(db_url).expect("Failed to establish sync db connection for migrations");
  conn
    .run_pending_migrations(MIGRATIONS)
    .expect("Failed to run migrations");
}

pub struct DatabaseConnection(pub bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
  S: Send + Sync,
  Pool: FromRef<S>,
{
  type Rejection = (StatusCode, String);

  async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    let pool = Pool::from_ref(state);
    debug!("Getting connection");
    let conn = pool.get_owned().await.map_err(internal_error)?;

    Ok(Self(conn))
  }
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
  E: std::error::Error,
{
  (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub enum DatabaseError {
  PoolError(RunError<PoolError>),
  DieselError(diesel::result::Error),
}

impl IntoResponse for DatabaseError {
  fn into_response(self) -> Response {
    let (status, error_message) = match self {
      DatabaseError::PoolError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
      DatabaseError::DieselError(e) => (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()),
    };

    let body = Json(json!({
        "error": error_message,
    }));

    (status, body).into_response()
  }
}
