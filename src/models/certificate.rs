use actix_web::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Certificate {
    id: i32,
    name: String,
    description: String,
    created_by_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewCertificate {
    name: String,
    description: String,
    created_by_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Question {
    id: i32,
    certificate_id: i32,
    question: String,
    options: Option<Vec<String>>,
    question_type: String,
    checkbox: Option<Vec<String>>,
    drop_down: Option<Vec<String>>,
    is_required: bool,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}

struct NewQuestion {
    certificate_id: i32,
    question: String,
    options: Option<Vec<std::string::String>>,
    question_type: String,
    checkbox: Option<Vec<std::string::String>>,
    drop_down: Option<Vec<std::string::String>>,
    is_required: bool,
    // created_at: Option<chrono::NaiveDateTime>,
    // updated_at: Option<chrono::NaiveDateTime>,
}

impl NewCertificate {
    pub async fn create_certificate(
        certificate: NewCertificate,
        pool: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Certificate, Error> {
        match sqlx::query_as!(
            Certificate,
            "INSERT INTO certificates (name, description, created_by_id) VALUES ($1, $2, $3) RETURNING *",
            certificate.name,
            certificate.description,
            certificate.created_by_id
        ).fetch_one(&pool).await {
            Ok(cert)=> Ok(cert),
            Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string()))
        }
    }
}

impl NewQuestion {
    pub async fn create_question(
        questions: NewQuestion,
        pool: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Question, Error> {
        match  sqlx::query_as!(
            Question,
            "INSERT INTO questions (certificate_id, question, options, question_type, checkbox, drop_down, is_required) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            questions.certificate_id,
            questions.question,
            // IMPORTANT: here we are converting Option<Vec<String>> to Option<&[String]>
            // this take a lot of time to figure out
            questions.options.as_ref().map(|x| x.as_slice()),
            questions.question_type,
            questions.checkbox.as_ref().map(|x| x.as_slice()),
            questions.drop_down.as_ref().map(|x| x.as_slice()),
            questions.is_required
        )
        .fetch_one(&pool)
        .await {
            Ok(questions) => Ok(questions),
            Err(e) => Err(actix_web::error::ErrorBadRequest(e.to_string())),
        }
    }
}
