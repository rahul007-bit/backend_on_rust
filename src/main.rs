use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{Pool, Postgres};

mod controllers;
mod db;
mod middleware;
mod models;
mod routes;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setting up logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();
    log::info!("Starting server on http://localhost:8080 ...");
    // get the connection pool
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
    // starting server

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(web::scope("/api").configure(routes::user_routes::config))
            .default_service(web::route().to(routes::not_found::not_found))
            // if there is any error in deserializing the request body then this handler will be called and it will return a json response with the error message
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                // Create custom error response
                let err = format!("Error: {}", err);
                log::error!("{}", err);
                let json = serde_json::json!({
                    "status": "400",
                    "message": err
                });

                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest().json(json),
                )
                .into()
            }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
