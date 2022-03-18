use actix_web::dev::fn_factory;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use controllers::get_list::GetListController;
use env_logger::Env;
use mongodb::options::ClientOptions;
use mongodb::Client;
use toko::controllers;
use toko::domains::middlewares::token_auth::AuthTokenFilterService;
use toko::domains::models::controller_inject::IControllerInject;
use toko::repositories::auth::AuthRepository;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let client_options = ClientOptions::parse("mongodb://root:example@toko-mongo:27017").await;
    let client_options = match client_options {
        Ok(client_options) => client_options,
        Err(error) => {
            panic!("Failed to create client to mongo: {:?}", error)
        }
    };
    let client = Client::with_options(client_options);
    let client = match client {
        Ok(client) => client,
        Err(error) => {
            panic!("Failed to connect mongo: {:?}", error)
        }
    };
    let db = client.database("toko");

    let auth_repository = AuthRepository::new(db.clone());
    let controller_inject = IControllerInject {
        auth_repo: auth_repository,
    };
    let controller_inject_item = web::Data::new(controller_inject);

    HttpServer::new(move || {
        let auth_repository = AuthRepository::new(db.clone());

        App::new()
            .wrap(Logger::default())
            .wrap(AuthTokenFilterService::new(
                vec!["/member/"],
                auth_repository,
            ))
            .app_data(controller_inject_item.clone())
            .route(
                "/",
                web::get().service(fn_factory(|| async { Ok(GetListController) })),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
