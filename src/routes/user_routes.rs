use crate::controllers::user_controller;
use crate::middleware::auth;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(user_controller::register_user)), // .route("/login", web::post().to(user_controller::login_user))
                                                                                         // .route("/logout", web::post().to(user_controller::logout_user)),
            )
            .service(
                web::scope("")
                    .wrap(auth::Auth)
                    .route("/get_user", web::get().to(user_controller::get_user)),
            ),
    );
}

// write a test for this module
#[cfg(test)]
mod tests {
    use crate::{db, AppState};

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
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(config))).await;
        let mut req = test::TestRequest::get()
            .uri("/api/user/get_user")
            .to_request();
        let header = req.headers_mut();
        header.insert(
            http::header::AUTHORIZATION,
            HeaderValue::from_static("token"),
        );
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK)

        // let req = test::TestRequest::post()
        //     .uri("/api/user/auth/register")
        //     .to_request();
        // let resp = test::call_service(&mut app, req).await;
        // assert_eq!(resp.status(), http::StatusCode::OK)
    }
}
