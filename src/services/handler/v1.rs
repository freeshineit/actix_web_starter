use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
/// get("/api/v1/users/{user_id}/{friend}")
#[get("/users/{user_id}/{friend}")] // <- define path parameters
pub async fn greet2(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

/// get ("/api/v1/hello/{name}/{id})
#[get("/hello/{name}/{id}")]
pub async fn greet(path: web::Path<(String, u32)>) -> impl Responder {
    let (name, id) = path.into_inner();
    HttpResponse::Ok().body(format!("hello {} {}", name, id))
}
