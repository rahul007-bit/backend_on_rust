use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            log::info!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            log::error!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}
