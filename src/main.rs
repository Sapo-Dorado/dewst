use actix_web::{web, App, HttpResponse, HttpServer};

fn index() -> HttpResponse {
    HttpResponse::Ok().body("Home Page")
}

fn deck_list() -> HttpResponse {
    HttpResponse::Ok().body("Deck List")
}

fn create_deck() -> HttpResponse {
    HttpResponse::Ok().body("create_deck")
}

fn show_deck() -> HttpResponse {
    HttpResponse::Ok().body("Show Deck")
}

fn edit_deck() -> HttpResponse {
    HttpResponse::Ok().body("Edit Deck")
}

fn login_page() -> HttpResponse {
    HttpResponse::Ok().body("Login Page")
}

fn login() -> HttpResponse {
    HttpResponse::Ok().body("login")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::scope("/decks")
                    .route("/", web::get().to(deck_list))
                    .route("/", web::post().to(create_deck))
                    .route("/{id}/", web::get().to(show_deck))
                    .route("/{id}/", web::patch().to(edit_deck))
            )
            .service(
                web::scope("/user")
                    .route("/", web::get().to(login_page))
                    .route("/", web::post().to(login))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}