use crate::controllers::user_controller;

use actix_web::web::{self};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/register", web::post().to(user_controller::register_user))
            .route("/get_user", web::get().to(user_controller::get_user)),
    );
}
