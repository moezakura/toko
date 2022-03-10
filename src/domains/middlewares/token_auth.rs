use crate::domains::models::error_response::ErrorResponse;
use actix_web::body::EitherBody;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct TokenAuth;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for TokenAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = TokenAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenAuthMiddleware { service }))
    }
}

pub struct TokenAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TokenAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth: Result<&str, Error>;
        let _req = req;

        let headers = _req.headers();
        let _auth = headers.get("authorization");

        auth = match _auth {
            Some(auth) => Ok(auth.to_str().unwrap()),
            None => {
                return Box::pin(async move {
                    let (req, _p1) = _req.into_parts();
                    let err_response = ErrorResponse {
                        code: StatusCode::UNAUTHORIZED.as_u16(),
                        error: String::from("unauthorized"),
                        message: String::from("need authorization header"),
                    };

                    let res = HttpResponse::build(StatusCode::UNAUTHORIZED)
                        .content_type("application/json")
                        .json(err_response)
                        .map_into_right_body();

                    Ok(ServiceResponse::new(req, res))
                });
            }
        };
        println!("your token: {:?}", auth);

        let res = self.service.call(_req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
