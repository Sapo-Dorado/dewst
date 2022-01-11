use actix_web::{web, HttpResponse};
use crate::{db, errors::DbError};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateDeckParams {
    author: Option<String>,
    token: Option<String>,
}
pub fn deck_list() -> HttpResponse {
    HttpResponse::Ok().body("Deck List")
}

pub async fn create_deck(
    deck: web::Query<CreateDeckParams>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, DbError> {
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let deck_info = db::add_deck(&client, &deck.author, &deck.token).await?;
    Ok(HttpResponse::Ok().json(deck_info))
}

pub fn show_deck() -> HttpResponse {
    HttpResponse::Ok().body("Show Deck")
}

pub fn edit_deck() -> HttpResponse {
    HttpResponse::Ok().body("Edit Deck")
}