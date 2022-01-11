use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use argon2::Error as ARError;

#[derive(Display, Debug)]
pub enum AuthError {
  PWError,
  TokenError,
}

#[derive(Display, From, Debug)]
pub enum DbError {
  NotFound,
  AuthError(AuthError),
  PGError(PGError),
  PGMError(PGMError),
  PoolError(PoolError),
  ARError(ARError),
}
impl ResponseError for DbError {
  fn error_response(&self) -> HttpResponse {
      match *self {
          DbError::NotFound => HttpResponse::NotFound().body(&self.to_string()),
          DbError::PoolError(ref err) => {
              HttpResponse::InternalServerError().body(err.to_string())
          }
          DbError::AuthError(AuthError::PWError) => HttpResponse::Unauthorized().body(format!("Invalid password")),
          DbError::AuthError(AuthError::TokenError) => HttpResponse::Unauthorized().body(format!("Invalid token")),
          _ => HttpResponse::InternalServerError().body(&self.to_string()),
      }
  }
}

impl std::error::Error for DbError {}