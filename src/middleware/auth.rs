use std::fmt;

use actix_web::body::EitherBody;
use actix_web::dev::{Service, Transform};
use actix_web::http::header::HeaderValue;
use actix_web::{
    dev::{forward_ready, ServiceRequest, ServiceResponse},
    Error,
};
use actix_web::{HttpMessage, HttpResponse};

use futures_util::future::{ready, LocalBoxFuture, Ready};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaim {
    // subject this can be anything such as auth, refresh, etc
    sub: String,
    // expiry time is 24 hours
    exp: usize,
    // user id
    user_id: String,
    // user role
    role: String,
}

pub struct JwtMiddleware {
    pub user_id: String,
    pub role: String,
}

impl JwtMiddleware {
    pub fn new() -> Self {
        JwtMiddleware {
            user_id: "".to_string(),
            role: "".to_string(),
        }
    }

    pub fn generate_token(user_id: String, role: String) -> String {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let claim = TokenClaim {
            sub: "auth".to_string(),
            // expiry time is 24 hours
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
            user_id,
            role,
        };
        match jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(token) => token,
            Err(_) => "".to_string(),
        }
    }

    pub fn decode_token(token: String) -> Result<TokenClaim, jsonwebtoken::errors::Error> {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = jsonwebtoken::Validation::default();
        let claim = match jsonwebtoken::decode::<TokenClaim>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(claim) => claim.claims,
            Err(err) => {
                return Err(err);
            }
        };
        Ok(claim)
    }
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let token = request
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .unwrap_or(&HeaderValue::from_static(""))
            .clone();

        // if token is not present then return error

        if token.is_empty() {
            let json_response = json!(
                {
                    "status": 401,
                    "message": "Unauthorized"
                }
            );
            // return error
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Unauthorized()
                .json(json_response)
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        // if token is present then decode it
        let token: String = token.to_str().unwrap().to_string();

        let claim: TokenClaim = match JwtMiddleware::decode_token(token) {
            Ok(claim) => claim,
            Err(err) => {
                let json_response = json!(
                    {
                        "status": 401,
                        "message": err.to_string()
                    }
                );
                // return error
                let (request, _pl) = request.into_parts();

                let response = HttpResponse::Unauthorized()
                    .json(json_response)
                    .map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
            }
        };
        request.extensions_mut().insert(claim);

        let res = self.service.call(request);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
