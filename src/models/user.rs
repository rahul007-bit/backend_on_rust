use actix_web::{Error, HttpResponse};
use bcrypt::hash;
use serde::{Deserialize, Serialize};

use crate::middleware::auth::{JwtMiddleware, TokenClaim};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub department: String,
    pub profile_image: Option<String>,
    pub academic_year: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    // pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub department: String,
    pub profile_image: Option<String>,
    pub academic_year: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

impl User {
    pub async fn find_all(
        pool: sqlx::Pool<sqlx::Postgres>,
        page_request: Page,
    ) -> Result<HttpResponse, Error> {
        let page = page_request.page.unwrap_or(1);
        let per_page = page_request.per_page.unwrap_or(10);

        let users = match sqlx::query_as!(
            User,
            "SELECT * FROM users LIMIT $1 OFFSET $2",
            per_page,
            (page - 1) * per_page
        )
        .fetch_all(&pool)
        .await
        {
            Ok(users) => users,
            Err(err) => {
                return Err(actix_web::error::ErrorBadRequest(err.to_string()));
            }
        };
        let json_response = serde_json::json!({
            "status": "200",
            "message": "Users fetched",
            "data": users
        });
        // return success
        Ok(HttpResponse::Ok().json(json_response))
    }

    pub async fn find_by_email(conn: sqlx::Pool<sqlx::Postgres>, email: &str) -> bool {
        let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&conn)
            .await
        {
            Ok(user) => user,
            Err(_) => return false,
        };
        user.is_some()
    }
}

impl LoginUser {
    pub fn new() -> LoginUser {
        LoginUser {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
    pub async fn login(
        clone: sqlx::Pool<sqlx::Postgres>,
        data: actix_web::web::Json<LoginUser>,
    ) -> Result<HttpResponse, Error> {
        let user = match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", data.email)
            .fetch_optional(&clone)
            .await
        {
            Ok(user) => user,
            Err(err) => return Err(actix_web::error::ErrorBadRequest(err.to_string())),
        };
        if user.is_none() {
            return Err(actix_web::error::ErrorBadRequest("User not found"));
        }
        let user = user.unwrap();

        let is_valid = match bcrypt::verify(&data.password, &user.password) {
            Ok(is_valid) => is_valid,
            Err(err) => return Err(actix_web::error::ErrorBadRequest(err.to_string())),
        };
        if !is_valid {
            return Err(actix_web::error::ErrorBadRequest("Invalid password"));
        }
        let token = JwtMiddleware::generate_token(user.id.to_string(), user.role.to_string());
        let json_response = serde_json::json!({
            "status": "200",
            "message": "User logged in",
            "data": {
                "user": user,
                "token": token
            }
        });
        // return success
        Ok(HttpResponse::Ok().json(json_response))
    }
}

impl NewUser {
    pub fn new_test_user() -> NewUser {
        let random_email = format!("{}@gmail.com", ulid::Ulid::new().to_string());
        NewUser {
            name: "example".to_string(),
            email: random_email,
            password: "password".to_string(),
            role: "student".to_string(),
            department: "IT".to_string(),
            academic_year: "2020-21".to_string(),
            profile_image: None,
        }
    }
    pub fn register_test_user() -> NewUser {
        NewUser {
            name: "example".to_string(),
            email: "example@gmail.com".to_string(),
            password: "password".to_string(),
            role: "student".to_string(),
            department: "IT".to_string(),
            academic_year: "2020-21".to_string(),
            profile_image: None,
        }
    }
    pub async fn register(
        db: sqlx::Pool<sqlx::Postgres>,
        data: actix_web::web::Json<NewUser>,
    ) -> Result<HttpResponse, Error> {
        if User::find_by_email(db.clone(), &data.email).await {
            // return error
            return Err(actix_web::error::ErrorBadRequest("User already exists"));
        }
        let hashed_password = hash(&data.password, 10).unwrap();

        match sqlx::query_as!(
            User,
            "INSERT INTO users (name, email, password, role, department, profile_image, academic_year) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            data.name,
            data.email,
            hashed_password,
            data.role,
            data.department,
            data.profile_image,
            data.academic_year
        )
        .fetch_one(&db)
        .await
        {
            Ok(user) => {
                let json_response = serde_json::json!({
                    "status": "200",
                    "message": "User registered",
                    "data": user
                });
                // return success
                Ok(HttpResponse::Ok().json(json_response))
            }
            Err(err) => {
                // return error
                Err(actix_web::error::ErrorBadRequest(err.to_string()))
            }
        }
    }
}
