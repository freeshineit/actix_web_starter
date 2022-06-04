use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, Result};

/// not found
pub async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}
