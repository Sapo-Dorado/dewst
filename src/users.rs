use actix_web::{web, HttpResponse, Error};
use crate::{db, errors::DbError};
use deadpool_postgres::{Client, Pool};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserParams {
    username: String,
    password: String,
}

pub fn login_page() -> HttpResponse {
    HttpResponse::Ok().body("Login Page")
}

pub async fn login(
    user: web::Query<UserParams>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let user_info = db::get_user(&client, &user.username, &user.password).await?;
    Ok(HttpResponse::Ok().json(user_info))
}

pub async fn create(
    user: web::Query<UserParams>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;

    let new_user = db::add_user(&client, &user.username, &user.password).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
