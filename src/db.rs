pub mod models {
  use serde::{Deserialize, Serialize};
  use tokio_pg_mapper_derive::PostgresMapper;

  #[derive(Deserialize, PostgresMapper, Serialize)]
  #[pg_mapper(table = "users")]
  pub struct User {
      pub username: String,
      pub password_hash: String,
      pub token_hash: String,
  }
  #[derive(Deserialize, PostgresMapper, Serialize)]
  #[pg_mapper(table = "users")]
  pub struct UserReturn {
      pub username: String,
      pub token: String,
  }
}

use crate::errors::DbError;
use models::{User, UserReturn};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;
use argon2::{self, Config};
use rand::{Rng, distributions::Uniform};



pub async fn add_user(client: &Client, username: &String, password: &String) -> Result<UserReturn, DbError> {
  let _stmt = include_str!("../sql/add_user.sql");
  let stmt = client.prepare(&_stmt).await.unwrap();

  let config = Config::default();
  
  let range = Uniform::new(0, 255);
  let salt: Vec<u8> = rand::thread_rng()
    .sample_iter(&range)
    .take(16)
    .collect();
  
  let token = Uuid::new_v4().to_string();

  client
    .query(
      &stmt,
      &[
        username,
        &argon2::hash_encoded(password.as_bytes(), salt.as_slice(), &config)?,
        &argon2::hash_encoded(token.as_bytes(), salt.as_slice(), &config)?,
      ]
    )
    .await?;
  Ok(UserReturn { username: username.clone(), token })
  
}

pub async fn get_user(client: &Client, username: &String, password: &String) -> Result<UserReturn, DbError> {
  let _stmt = include_str!("../sql/get_user.sql");
  let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  let config = Config::default();
  
  let range = Uniform::new(0, 255);
  let salt: Vec<u8> = rand::thread_rng()
    .sample_iter(&range)
    .take(16)
    .collect();
  
  let token = Uuid::new_v4().to_string();

  let user = client
    .query(
      &stmt,
      &[
        &argon2::hash_encoded(token.as_bytes(), salt.as_slice(), &config)?,
        username,
      ],
    )
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    .ok_or(DbError::NotFound)?;
  
  if argon2::verify_encoded(&user.password_hash, &password.as_bytes())? {
    Ok(UserReturn { username: user.username, token })
  } else {
    Err(DbError::PWError)
  }
}

pub async fn auth_user(client: &Client, username: &String, password: &String) -> Result<(), DbError> {
  let _stmt = include_str!("../sql/auth_user.sql");
  let stmt = client.prepare(&_stmt).await.unwrap();

  let user = client
    .query(
      &stmt,
      &[
        username,
      ],
    )
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    .ok_or(DbError::NotFound)?;

  if argon2::verify_encoded(&user.token_hash, &password.as_bytes())? {
    Ok(())
  } else {
    Err(DbError::PWError)
  }
}
