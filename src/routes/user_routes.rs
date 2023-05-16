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
    use super::*;
    use actix_web::{
        http::{
            self,
            header::{self, HeaderValue},
        },
        test, App, HttpMessage,
    };

    #[actix_web::test]
    async fn test_get_user() {
        let app =
            test::init_service(App::new().service(web::scope("/api").configure(config))).await;

        // add auth header

        let mut req = test::TestRequest::get()
            .uri("/api/user/get_user")
            .to_request();

        req.headers_mut()
            .insert(header::AUTHORIZATION, HeaderValue::from_static("token"));
        println!("{:?}", req.headers());
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK)
    }

    #[actix_web::test]
    async fn test_register_user() {
        let mut app = test::init_service(App::new().configure(config)).await;
        let req = test::TestRequest::post()
            .uri("/api/user/auth/register")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK)
    }
}
