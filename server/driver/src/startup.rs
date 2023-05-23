use core::panic;
use std::net::SocketAddr;

use actix_web::{web, App, HttpServer};

use crate::{
    module::Modules,
    routes::{
        health::{health, health_db},
        url::{redirect_to_long_url, shorten_long_url},
    },
};

pub async fn startup(modules: Modules) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    tracing::info!("Server listening on {addr}");

    let data = web::Data::new(modules);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/health")
                    .service(web::resource("").route(web::get().to(health)))
                    .service(web::resource("/db").route(web::get().to(health_db))),
            )
            .service(web::resource("/shorten").route(web::post().to(shorten_long_url)))
            .service(web::resource("/{short_url}").route(web::get().to(redirect_to_long_url)))
    })
    .bind(addr)?
    .run()
    .await
    .map_or_else(|_| panic!("Server cannot launch!"), |_| Ok(()))
}

pub fn init_app() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
}
