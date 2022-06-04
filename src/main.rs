use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod services;

struct AppState {
    pub app_name: String,
    counter: Mutex<u32>,
}

// #[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name

    let mut counter = data.counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().body(format!("this is home page {app_name} {counter}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting HTTP server: go to http://localhost:{}", port);

    HttpServer::new(move || {
        // 注册状态类型
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                counter: Mutex::new(0),
            }))
            // cors
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("http://localhost:8080")
            //         .allowed_methods(vec!["GET", "POST", "PUT"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .supports_credentials()
            //         .max_age(3600),
            // )
            .wrap(middleware::Logger::default())
            // // 默认响应
            // .default_service(web::route().to(services::handler::error::not_found))
            .configure(services::config::config)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
