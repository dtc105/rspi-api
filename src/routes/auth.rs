use actix_web::{HttpResponse, Responder, web};

pub fn router() -> actix_web::Scope {
    web::scope("/auth")
        .route("/token", web::get().to(read_token))
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/password", web::patch().to(change_password))
        .route("/username", web::patch().to(change_username))
}

async fn read_token() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn login() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn register() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn change_password() -> impl Responder {
    HttpResponse::NotImplemented()
}

async fn change_username() -> impl Responder {
    HttpResponse::NotImplemented()
}
