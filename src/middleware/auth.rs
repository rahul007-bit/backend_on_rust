use std::fmt;
use std::future::{ready, Ready};

use actix_web::error::{Error as ActixWebError, ErrorUnauthorized};
use actix_web::http::header::HeaderValue;
use actix_web::{FromRequest, HttpRequest};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey};
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
struct TokenClaim {
    sub: String,
    exp: usize,
    user_id: String,
    role: String,
}

pub struct JwtMiddleware {
    pub user_id: String,
}

impl JwtMiddleware {
    pub fn new() -> Self {
        JwtMiddleware {
            user_id: "".to_string(),
        }
    }
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .map(|header_value| header_value.to_str().unwrap().split_at(7).1.to_string());

        if token.is_none() {
            let json_response = ErrorResponse {
                status: "401".to_string(),
                message: "Unauthorized".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_response)));
        }

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let validation = jsonwebtoken::Validation::default();
        let claim = match decode::<TokenClaim>(
            &token.unwrap(),
            &DecodingKey::from_secret(&secret.as_ref()),
            &validation,
        ) {
            Ok(claim) => claim.claims,
            Err(_) => {
                let json_response = ErrorResponse {
                    status: "401".to_string(),
                    message: "Unauthorized".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_response)));
            }
        };
        ready(Ok(JwtMiddleware {
            user_id: claim.user_id,
        }))
    }
}
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            // get the token from header
            let token = res
                .headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .unwrap_or(&HeaderValue::from_static(""))
                .clone();
            let header = res.headers_mut();
            // if token is not present then return error
            if token.is_empty() {
                // set header content type to json
                header.insert(
                    actix_web::http::header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                println!("{:?}", res.headers());
                // set the response body
                let json_response = json!(
                    {
                        "status": "401",
                        "message": "Unauthorized"
                    }
                );
                // return error
                return Err(ErrorUnauthorized(json_response));
            } else {
                print!("token is present");
                // bypass the middleware if the request has token
                Ok(res)
            }
        })
    }
}
