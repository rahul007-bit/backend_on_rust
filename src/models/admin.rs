use actix_web::web;
use sqlx::PgPool;

use super::user::NewUser;

pub async fn init_admin(pool: PgPool) {
    let name = std::env::var("ADMIN_NAME").expect("ADMIN_NAME must be set");
    let email = std::env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set");
    let password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    let role = std::env::var("ADMIN_ROLE").expect("ADMIN_ROLE must be set");
    let department = std::env::var("ADMIN_DEPARTMENT").unwrap_or("IT".to_string());
    let academic_year = std::env::var("ADMIN_ACADEMIC_YEAR").unwrap_or("2020-21".to_string());
    let profile_image = std::env::var("ADMIN_PROFILE_IMAGE").unwrap_or("".to_string());

    // check if admin exists
    let admin = sqlx::query!("SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if admin.is_some() {
        log::debug!("Admin already exists");
        return;
    }

    let user = NewUser {
        name,
        email,
        password,
        role,
        department,
        academic_year,
        profile_image: Some(profile_image),
    };

    match NewUser::register(pool, web::Json(user)).await {
        Ok(_) => log::debug!("Admin created successfully"),
        Err(err) => log::error!("Error creating admin: {}", err.to_string()),
    }
}
