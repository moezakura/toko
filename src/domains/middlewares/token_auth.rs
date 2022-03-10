use actix_web::error::ErrorUnauthorized;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TokenAuthMiddleware<S>;
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
    type Response = ServiceResponse<B>;
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
            None => return Box::pin(async move { Err(ErrorUnauthorized("must be API KEY")) }),
        };
        println!("your token: {:?}", auth);

        let fut = self.service.call(_req);

        Box::pin(async {
            let res = fut.await?;
            Ok(res)
        })
    }
}
