use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}
