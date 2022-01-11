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
  #[derive(Deserialize, PostgresMapper, Serialize)]
  #[pg_mapper(table = "decks")]
  pub struct Deck {
      pub uuid: String,
      pub author: Option<String>,
  }
}

use crate::errors::{DbError,AuthError};
use models::{User, UserReturn, Deck};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;
use argon2::{self, Config};
use rand::{Rng, distributions::Uniform};

fn hash(config: &argon2::Config, value: &String) -> Result<String, DbError> {
  let range = Uniform::new(0, 255);
  let salt: Vec<u8> = rand::thread_rng()
    .sample_iter(&range)
    .take(16)
    .collect();
  Ok(argon2::hash_encoded(value.as_bytes(), salt.as_slice(), config)?)
}

fn verify(hash: &String, value: &String) -> Result<bool,DbError> {
  Ok(argon2::verify_encoded(hash, value.as_bytes())?)
}

pub async fn add_user(client: &Client, username: &String, password: &String) -> Result<UserReturn, DbError> {
  let _stmt = include_str!("../sql/add_user.sql");
  let stmt = client.prepare(&_stmt).await.unwrap();

  let config = Config::default();
  
  let token = Uuid::new_v4().to_string();

  client
    .query(
      &stmt,
      &[
        username,
        &hash(&config, password)?,
        &hash(&config, &token)?,
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
  
  let token = Uuid::new_v4().to_string();

  let user = client
    .query(
      &stmt,
      &[
        &hash(&config, &token)?,
        username,
      ],
    )
    .await?
    .iter()
    .map(|row| User::from_row_ref(row).unwrap())
    .collect::<Vec<User>>()
    .pop()
    .ok_or(DbError::NotFound)?;
  
  if verify(&user.password_hash, &password)? {
    Ok(UserReturn { username: user.username, token })
  } else {
    Err(DbError::AuthError(AuthError::PWError))
  }
}

pub async fn auth_user(client: &Client, username: &String, token: &String) -> Result<(), DbError> {
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

  if verify(&user.token_hash, &token)? {
    Ok(())
  } else {
    Err(DbError::AuthError(AuthError::TokenError))
  }
}

pub async fn add_deck(client: &Client, optional_author: &Option<String>, optional_token: &Option<String>) -> Result<Deck, DbError> {
  let mut author = None;
  if let Some(username) = optional_author {
    if let Some(token) = optional_token {
      auth_user(client, &username, &token).await?;
      author = Some(username);
    } else {
      return Err(DbError::AuthError(AuthError::TokenError));
    }
  }
  let _stmt = include_str!("../sql/add_deck.sql");
  let _stmt = _stmt.replace("$table_fields", &Deck::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client
    .query(
      &stmt,
      &[
        &Uuid::new_v4().to_string(),
        &author,
      ]
    )
    .await?
    .iter()
    .map(|row| Deck::from_row_ref(row).unwrap())
    .collect::<Vec<Deck>>()
    .pop()
    .ok_or(DbError::NotFound)
}
