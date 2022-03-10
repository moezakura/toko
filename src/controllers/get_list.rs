use actix_web::{dev::*, http::header, *};
use futures_util::future::LocalBoxFuture;
use std::convert::Infallible;

pub struct GetListController;

impl Service<ServiceRequest> for GetListController {
    type Response = ServiceResponse;
    type Error = Infallible;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::always_ready!();

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let (req, _) = req.into_parts();

        let headers = req.headers();
        let al = headers.get("Accept-Language");
        println!("{:?}", al);

        let res = HttpResponse::Ok()
            .insert_header(header::ContentType::plaintext())
            .body("Hello world!");

        Box::pin(async move { Ok(ServiceResponse::new(req, res)) })
    }
}
