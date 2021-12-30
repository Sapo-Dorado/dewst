use actix_web::{web, HttpResponse, Error};
use crate::{db, errors::DbError};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

pub fn login_page() -> HttpResponse {
    HttpResponse::Ok().body("Login Page")
}

pub fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login")
}

#[derive(Deserialize)]
pub struct CreateParams {
    username: String,
    password: String,
}

pub async fn create(
    user: web::Query<CreateParams>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;

    let new_user = db::add_user(&client, &user.username, &user.password).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
