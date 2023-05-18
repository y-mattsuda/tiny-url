use actix_web::{web, HttpResponse, Responder};

use crate::{
    model::url::{JsonDataToShorten, JsonUrl},
    module::{Modules, ModulesExt},
};

pub async fn shorten_long_url(
    req_body: web::Json<JsonDataToShorten>,
    module: web::Data<Modules>,
) -> impl Responder {
    module
        .url_use_case()
        .get_short_url(req_body.long_url.as_str())
        .await
        .map_or_else(
            |e| {
                tracing::error!("{e:?}");
                HttpResponse::InternalServerError().finish()
            },
            |url| {
                let ju: JsonUrl = url.into();
                HttpResponse::Created().json(ju)
            },
        )
}

pub async fn redirect_to_long_url(
    path: web::Path<String>,
    module: web::Data<Modules>,
) -> impl Responder {
    module
        .url_use_case()
        .find_original_long_url(&path)
        .await
        .map_or_else(
            |e| {
                tracing::error!("{e:?}");
                HttpResponse::InternalServerError().finish()
            },
            |url| {
                url.map_or_else(
                    || HttpResponse::NotFound().finish(),
                    |url| {
                        HttpResponse::MovedPermanently()
                            .append_header(("location", url.long.0))
                            .finish()
                    },
                )
            },
        )
}
