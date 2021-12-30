pub mod models {
  use serde::{Deserialize, Serialize};
  use tokio_pg_mapper_derive::PostgresMapper;

  #[derive(Deserialize, PostgresMapper, Serialize)]
  #[pg_mapper(table = "users")]
  pub struct User {
      pub username: String,
      pub password_hash: String,
      pub token: String,
  }
}

use crate::errors::DbError;
use models::User;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

pub async fn add_user(client: &Client, user_info: User) -> Result<User, DbError> {
  let _stmt = include_str!("../sql/add_user.sql");
  let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client
    .query(
      &stmt,
      &[
        &user_info.username,
        &user_info.password_hash,
        &Uuid::new_v4().to_string(),
      ]
    )
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    .ok_or(DbError::NotFound)
}
