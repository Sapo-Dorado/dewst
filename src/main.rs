use actix_web::{web, App, HttpResponse, HttpServer};

mod decks;
mod users;

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Home Page")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/decks")
                    .route("/", web::get().to(decks::deck_list))
                    .route("/", web::post().to(decks::create_deck))
                    .route("/{id}/", web::get().to(decks::show_deck))
                    .route("/{id}/", web::patch().to(decks::edit_deck))
            )
            .service(
                web::scope("/user")
                    .route("/", web::get().to(users::login_page))
                    .route("/", web::post().to(users::login))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}