#[macro_use]
extern crate diesel;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};

mod controllers;
mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setting up logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
    log::info!("Starting server on http://localhost:8080 ...");
    // get the connection pool
    let pool = db::connection::get_pool();
    // starting server

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .service(web::scope("/api").configure(routes::user_routes::config))
            .default_service(web::route().to(routes::not_found::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
