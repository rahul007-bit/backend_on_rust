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
