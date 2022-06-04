use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    user_id: u32,
    friend: String,
}

/// get ("/api/v1/greet")
#[get("/greet")]
pub async fn greet(path: web::Path<(String, u32)>) -> impl Responder {
    let (name, id) = path.into_inner();
    HttpResponse::Ok().body(format!("hello {} {}", name, id))
}
