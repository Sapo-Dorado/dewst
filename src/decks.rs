use actix_web::HttpResponse;

pub fn deck_list() -> HttpResponse {
    HttpResponse::Ok().body("Deck List")
}

pub fn create_deck() -> HttpResponse {
    HttpResponse::Ok().body("create_deck")
}

pub fn show_deck() -> HttpResponse {
    HttpResponse::Ok().body("Show Deck")
}

pub fn edit_deck() -> HttpResponse {
    HttpResponse::Ok().body("Edit Deck")
}