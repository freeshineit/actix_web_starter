use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
use std::sync::Mutex;

struct AppState {
    pub app_name: String,
    counter: Mutex<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn greet2(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

#[get("/hello/{name}/{id}")]
async fn greet(path: web::Path<(String, u32)>) -> impl Responder {
    let (name, id) = path.into_inner();

    HttpResponse::Ok().body(format!("hello {} {}", name, id))
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name

    let mut counter = data.counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().body(format!("this is home page {app_name} {counter}"))
}

async fn hey() -> impl Responder {
    HttpResponse::Ok().body("this is hey page")
}

async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;

    println!("Http serve listen: http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            // 注册状态类型
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                counter: Mutex::new(0),
            }))
            // static file
            // .service(
            //     Files::new("/static", "static")
            //         .show_files_listing()
            //         .use_last_modified(true),
            // )
            // .service(
            //     Files::new("/", "pages")
            //         .show_files_listing()
            //         .use_last_modified(true),
            // )
            .service(index)
            .service(greet)
            .service(greet2)
            .service(web::scope("/app").route("/index.html", web::get().to(hey)))
            .route("/hey", web::get().to(hey))
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
