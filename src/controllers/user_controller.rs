use actix_web::{web, HttpResponse, Responder};

use serde::Serialize;
use serde_json::json;

use crate::AppState;

use crate::models::user::{NewUser, User};

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

pub async fn get_user(pool: web::Data<AppState>) -> impl Responder {
    println!("get_user");
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool.db)
        .await
        .unwrap();
    println!("{:?}", users);
    HttpResponse::Ok().json({
        json!({
            "status": "200",
            "message": "User fetched successfully",
            "data": users
        })
    })
}

pub async fn register_user(pool: web::Data<AppState>, data: web::Json<NewUser>) -> impl Responder {
    match NewUser::register(pool.db.clone(), data).await {
        Ok(response) => response,
        Err(err) => {
            let json_response = json!({
                "status": "400",
                "message": err.to_string()
            });
            HttpResponse::BadRequest().json(json_response)
        }
    }
}
