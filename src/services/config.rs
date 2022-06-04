use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::web::{self, scope, ServiceConfig};
use actix_web::{middleware::Logger, App, HttpResponse, Responder};

use super::handler;

/// service config
///
/// router config
pub fn config(cfg: &mut ServiceConfig) {
    // /api/xxxx
    cfg.service(
        scope("/api")
            .route("/register", web::post().to(handler::user::register))
            .route("/login", web::post().to(handler::user::login))
            .route("/login_form", web::post().to(handler::user::login_form))
            .service(
                scope("/v1")
                    .service(handler::v1::greet)
                    .service(handler::v1::greet),
            )
            .service(
                scope("/v2")
                    .service(handler::v2::greet)
                    .service(handler::v2::greet),
            ), // /app/index.html
               // .service(web::scope("/app").route("/index.html", web::get().to(hey)))
               // .route("/hey", web::get().to(hey))
               // .route("/", web::to(index)) // 重定向
               // static file
    )
    .service(
        Files::new("/static", "static")
            .show_files_listing()
            .use_last_modified(true),
    )
    .service(
        Files::new("/", "pages")
            .show_files_listing()
            .use_last_modified(true),
    );
}
