use actix_web::{web, HttpResponse, Responder};

use crate::module::{Modules, ModulesExt};

pub async fn health() -> impl Responder {
    tracing::debug!("Access healtch checl endpoint from user!");
    HttpResponse::NoContent()
}

pub async fn health_db(module: web::Data<Modules>) -> impl Responder {
    module
        .health_check_use_case()
        .diagnose_db_conn()
        .await
        .map_or_else(
            |e| {
                tracing::error!("{e:?}");
                HttpResponse::ServiceUnavailable().finish()
            },
            |_| HttpResponse::NoContent().finish(),
        )
}
