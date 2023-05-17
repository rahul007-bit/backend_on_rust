use crate::controllers::user_controller;
use crate::middleware::auth;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(user_controller::register_user)), // .route("/login", web::post().to(user_controller::login_user))
            )
            .service(
                web::scope("/user")
                    .wrap(auth::Auth)
                    .route("/get_user", web::get().to(user_controller::get_user)),
            ),
    );
}

#[cfg(test)]
mod tests {
    use crate::{db, models, AppState};

    use super::*;
    use actix_web::{
        http::{
            self,
            header::{self, HeaderValue},
        },
        test,
        web::Data,
        App,
    };

    #[actix_web::test]
    async fn test_get_user() {
        let pool = match db::connection::get_pool().await {
            Ok(pool) => {
                log::info!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                log::error!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState { db: pool.clone() }))
                .service(web::scope("/api").configure(config)),
        )
        .await;

        // add auth header

        let req = test::TestRequest::get()
            .uri("/api/user/get_user")
            .insert_header((header::AUTHORIZATION, HeaderValue::from_static("token")))
            .to_request();

        let resp = test::call_service(&app, req.into()).await;
        assert_eq!(resp.status(), http::StatusCode::OK)
    }

    #[actix_web::test]
    async fn test_register_user() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    db: db::connection::get_pool().await.unwrap(),
                }))
                .service(web::scope("/api").configure(config)),
        )
        .await;
        let random_email = format!("{}@gmail.com", ulid::Ulid::new().to_string());
        let user = models::user::NewUser {
            name: "example".to_string(),
            email: random_email,
            password: "password".to_string(),
            role: "student".to_string(),
            department: "IT".to_string(),
            academic_year: "2020-21".to_string(),
            profile_image: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(&user)
            .to_request();

        let resp = test::call_service(&app, req.into()).await;
        assert_eq!(resp.status(), http::StatusCode::OK)
    }

    #[actix_web::test]
    async fn test_register_user_with_existing_email() {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    db: db::connection::get_pool().await.unwrap(),
                }))
                .service(web::scope("/api").configure(config)),
        )
        .await;
        let user = models::user::NewUser {
            name: "example".to_string(),
            email: "example@gmail.com".to_string(),
            password: "password".to_string(),
            role: "student".to_string(),
            department: "IT".to_string(),
            academic_year: "2020-21".to_string(),
            profile_image: None,
        };

        let req = test::TestRequest::post()
            .uri("/api/auth/register")
            .set_json(&user)
            .to_request();

        let resp = test::call_service(&app, req.into()).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST)
    }
}
