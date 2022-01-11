use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

extern crate argon2;
mod config;
mod decks;
mod users;
mod errors;
mod db;

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Home Page")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/decks")
                    .route("/", web::get().to(decks::deck_list))
                    .route("/create/", web::post().to(decks::create_deck))
                    .route("/{id}/", web::get().to(decks::show_deck))
                    .route("/{id}/", web::patch().to(decks::edit_deck))
            )
            .service(
                web::scope("/user")
                    .route("/", web::get().to(users::login_page))
                    .route("/login/", web::get().to(users::login))
                    .route("/create/", web::post().to(users::create))
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server running at http://{}/", config.server_addr);
    server.await
}