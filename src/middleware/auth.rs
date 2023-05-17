use std::fmt;

use actix_web::body::EitherBody;
use actix_web::dev::{Service, Transform};
use actix_web::http::header::HeaderValue;
use actix_web::{dev, http, HttpResponse};
use actix_web::{
    dev::{forward_ready, ServiceRequest, ServiceResponse},
    Error,
};

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

// impl FromRequest for JwtMiddleware {
//     type Error = ActixWebError;
//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
//         let token = req
//             .headers()
//             .get(actix_web::http::header::AUTHORIZATION)
//             .map(|header_value| header_value.to_str().unwrap().split_at(7).1.to_string());

//         if token.is_none() {
//             let json_response = ErrorResponse {
//                 status: "401".to_string(),
//                 message: "Unauthorized".to_string(),
//             };
//             return ready(Err(ErrorUnauthorized(json_response)));
//         }

//         let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//         let validation = jsonwebtoken::Validation::default();
//         let claim = match decode::<TokenClaim>(
//             &token.unwrap(),
//             &DecodingKey::from_secret(&secret.as_ref()),
//             &validation,
//         ) {
//             Ok(claim) => claim.claims,
//             Err(_) => {
//                 let json_response = ErrorResponse {
//                     status: "401".to_string(),
//                     message: "Unauthorized".to_string(),
//                 };
//                 return ready(Err(ErrorUnauthorized(json_response)));
//             }
//         };
//         ready(Ok(JwtMiddleware {
//             user_id: claim.user_id,
//         }))
//     }
// }
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

        let res = self.service.call(request);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
