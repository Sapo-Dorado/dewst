use actix_web::HttpResponse;

pub fn login_page() -> HttpResponse {
    HttpResponse::Ok().body("Login Page")
}

pub fn login() -> HttpResponse {
    HttpResponse::Ok().body("login")
}