use actix_web::{post, web, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MyObj {
    email: String,
    password: String,
}

/// /api/register
///
/// curl -i -H 'Content-Type: application/json' -d '{"email": "xiaoshaoqq@gmail.com", "password": "123456"}' -X POST http://localhost:8080/api/register
pub async fn register(user: web::Json<MyObj>) -> web::Json<MyObj> {
    log::info!("Register1");
    format!("{:?}", user);
    web::Json(MyObj {
        email: user.email.to_string(),
        password: user.password.to_string(),
    })
}

/// /api/login
///
/// curl -i -H 'Content-Type: application/json' -d '{"email": "xiaoshaoqq@gmail.com", "password": "123456"}' -X POST http://localhost:8080/api/login
pub async fn login(user: web::Json<MyObj>) -> web::Json<MyObj> {
    log::info!("Register1");
    format!("login {:?}", user);
    user
    // web::Json(MyObj {
    //     email: user.email.to_string(),
    //     password: user.password.to_string(),
    // })
}

/// /api/login_form
///
/// curl -i -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8' -d 'email=xiaoshaoqq@gmail.com&password=123456' -X POST http://localhost:8080/api/login_form
pub async fn login_form(user: web::Form<MyObj>) -> web::Json<MyObj> {
    log::info!("Register1");

    // format!("login {:?}", user.email)
    web::Json(MyObj {
        email: user.email.to_string(),
        password: user.password.to_string(),
    })
}
