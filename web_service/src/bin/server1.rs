use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("hello {}", name);
    HttpResponse::Ok().body("1234123")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let  port = 8080;

    println!("Http serve listen: http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
