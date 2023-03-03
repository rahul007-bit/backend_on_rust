use actix_web::{web, HttpResponse, Responder};

use serde::Serialize;
use serde_json::json;

use crate::db::connection::PgPool;
use crate::models::user::User;

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn get_user(pool: web::Data<PgPool>) -> impl Responder {
    // get users from database
    let mut conn = pool.get().unwrap();
    let users = User::find_all(&mut conn).unwrap();
    println!("{:?}", users);
    HttpResponse::Ok().json({
        json!({
            "status": "200",
            "message": "User registered",
            "data": users
        })
    })
}

pub async fn register_user(_pool: web::Data<PgPool>, data: web::Json<User>) -> impl Responder {
    let user = data.into_inner();
    println!("{:?}", user);
    HttpResponse::Ok().json({
        json!({
            "status": "200",
            "message": "User registered",
            "data": user
        })
    })
}
