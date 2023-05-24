use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};

use serde::Serialize;
use serde_json::json;

use crate::AppState;

use crate::middleware::auth::TokenClaim;
use crate::models::user::{LoginUser, NewUser, User};

pub async fn get_user(pool: web::Data<AppState>, request: HttpRequest) -> HttpResponse {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool.db)
        .await
        .unwrap();
    let exts = request.extensions();
    let token = exts.get::<TokenClaim>().unwrap();
    println!("token: {:?}", token);
    HttpResponse::Ok().json({
        json!({
            "status": "200",
            "message": "User fetched successfully",
            "data": users
        })
    })
}

pub async fn register_user(pool: web::Data<AppState>, data: web::Json<NewUser>) -> HttpResponse {
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

pub async fn login_user(pool: web::Data<AppState>, data: web::Json<LoginUser>) -> HttpResponse {
    let result = LoginUser::login(pool.db.clone(), data).await;
    match result {
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

#[cfg(test)]
mod tests {
    use crate::db;

    use super::*;
    use actix_web::http;
    use bcrypt::hash;

    #[actix_web::test]
    async fn register_user_test() {
        let pool = db::connection::get_pool().await;
        let user = NewUser::new_test_user();

        let resp = register_user(web::Data::new(AppState { db: pool }), web::Json(user)).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn login_user_test() {
        let pool = db::connection::get_pool().await;
        let user = NewUser::register_test_user();

        let resp = login_user(
            web::Data::new(AppState { db: pool }),
            web::Json(LoginUser {
                email: user.email,
                password: user.password,
            }),
        )
        .await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn login_incorrect_user() {
        let pool = db::connection::get_pool().await;
        let user = NewUser::register_test_user();
        let res = login_user(
            web::Data::new(AppState { db: pool }),
            web::Json(LoginUser {
                email: user.email,
                password: "incorrect_password".to_string(),
            }),
        )
        .await;
        assert_eq!(res.status(), http::StatusCode::BAD_REQUEST);
    }
}
