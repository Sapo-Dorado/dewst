use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use argon2::Error as ARError;

#[derive(Display, From, Debug)]
pub enum DbError {
  NotFound,
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
          _ => HttpResponse::InternalServerError().body(&self.to_string()),
      }
  }
}

impl std::error::Error for DbError {}