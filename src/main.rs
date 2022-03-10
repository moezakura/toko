use actix_web::dev::fn_factory;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

use controllers::get_list::GetListController;
use domains::middlewares::token_auth::TokenAuth;
use toko::{controllers, domains};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).wrap(TokenAuth).route(
            "/",
            web::get().service(fn_factory(|| async { Ok(GetListController) })),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
