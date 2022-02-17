pub mod models {
  use crate::decks::Card;
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
  #[derive(Serialize)]
    pub struct Deck {
    pub uuid: String,
    pub author: Option<String>,
    pub title: String,
    pub cards: Vec<Card>,
  }
}

use crate::errors::{DbError,AuthError};
use crate::decks;
use crate::decks::Card;
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

pub async fn add_deck(client: &Client, deck_info: &decks::CreateDeckParams) -> Result<Deck, DbError> {
  let mut author = None;
  if let Some(username) = &deck_info.author {
    if let Some(token) = &deck_info.token {
      auth_user(client, &username, &token).await?;
      author = Some(username);
    } else {
      return Err(DbError::AuthError(AuthError::TokenError));
    }
  }

  let _stmt = include_str!("../sql/add_deck.sql");
  let stmt = client.prepare(&_stmt).await.unwrap();

  let uuid = Uuid::new_v4().to_string();
  
  client
    .query(
      &stmt,
      &[
        &uuid,
        &author,
        &deck_info.title,
      ]
    )
    .await?;
  add_cards(client, &uuid, &deck_info.cards).await?;
  Ok(Deck { uuid, author: deck_info.author.clone(), title: deck_info.title.clone(), cards: deck_info.cards.clone()})
}

async fn add_cards(client: &Client, deck_uuid: &String, cards: &Vec<Card>) -> Result<(), DbError>{
  let _stmt = insert_card_string(deck_uuid, &cards);
  let stmt = client.prepare(_stmt.as_str()).await.unwrap();
  client
    .query(
      &stmt,
      &[]
    )
    .await?;
  Ok(())
}

fn insert_card_string(deck_uuid: &String, cards: &Vec<Card>) -> String {
  let mut result = String::from("");
  let stmt = include_str!("../sql/card_insert.sql");
  result.push_str(stmt);
  for (i,card) in cards.iter().enumerate() {
    result.push_str(format!("('{}','{}','{}')",deck_uuid,card.front,card.back).as_str());
    if i < cards.len()-1 {
      result.push_str(",\n");
    }
  }
  result
}