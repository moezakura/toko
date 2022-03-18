use crate::repositories::auth::AuthRepository;
use actix_web::body::EitherBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures::future::{ok, Ready};
use std::{future::Future, pin::Pin, rc::Rc};

enum AuthErrorType {
    Empty(String),
    Unauthorized(String),
    None(),
}

pub struct AuthTokenFilterService {
    pub target_path: Vec<String>,
    pub auth_repository: AuthRepository,
}

impl AuthTokenFilterService {
    pub fn new(target_path: Vec<impl Into<String>>, auth_repository: AuthRepository) -> Self {
        let target_path = target_path.into_iter().map(|i| i.into()).collect();

        AuthTokenFilterService {
            target_path,
            auth_repository,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthTokenFilterService
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthTokenFilterServiceMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthTokenFilterServiceMiddleware {
            service: Rc::new(service),
            auth_repository: self.auth_repository.clone(),
        })
    }
}

pub struct AuthTokenFilterServiceMiddleware<S> {
    service: Rc<S>,
    auth_repository: AuthRepository,
}

impl<S, B> Service<ServiceRequest> for AuthTokenFilterServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let auth_repository = self.auth_repository.clone();

        Box::pin(async move {
            let mut error_type = AuthErrorType::None();
            let headers = req.headers();
            let auth_header = headers.get("authorization");
            let no_auth = auth_header.is_none();

            if no_auth {
                error_type =
                    AuthErrorType::Empty(String::from("authorization header is required."));
            } else {
                let auth_token = auth_header.unwrap().to_str().unwrap();
                let verified_future = auth_repository.verify_token(String::from(auth_token));
                let verified = verified_future.await;
                match verified {
                    Err(_e) => {
                        error_type =
                            AuthErrorType::Unauthorized(String::from("authorization error"));
                        // print log
                    }
                    Ok(verified) => {
                        if !verified {
                            error_type =
                                AuthErrorType::Unauthorized(String::from("authorization error"));
                        }
                    }
                }
            }

            match error_type {
                AuthErrorType::Empty(e) => {
                    println!("error_type empty: {}", e);
                    let forbidden = HttpResponse::BadRequest().finish().map_into_right_body();
                    Ok(req.into_response(forbidden))
                }
                AuthErrorType::Unauthorized(e) => {
                    println!("error_type unauthorized: {}", e);
                    let unauthorized = HttpResponse::Unauthorized().finish().map_into_right_body();
                    Ok(req.into_response(unauthorized))
                }
                AuthErrorType::None() => service
                    .call(req)
                    .await
                    .map(ServiceResponse::map_into_left_body),
            }

            //service.call(req).await
        })
    }
}
