use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenv::dotenv;
use r2d2::Pool;

use std::env;

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
// }
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub fn get_pool() -> PgPool {
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let migr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}
