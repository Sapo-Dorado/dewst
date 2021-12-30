use actix_web::{web, HttpResponse, Error};
use crate::{db, db::models::User, errors::DbError};
use deadpool_postgres::{Client, Pool};

pub fn login_page() -> HttpResponse {
    HttpResponse::Ok().body("Login Page")
}

pub fn login() -> HttpResponse {
    HttpResponse::Ok().body("Login")
}

pub async fn create(
    user: web::Json<User>,
    db_pool: web::Data<Pool>
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}
