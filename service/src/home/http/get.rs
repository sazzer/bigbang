use crate::http::{hal::HalDocument, Response, SimpleRespondable};
use actix_http::http::{
    header::{CacheControl, CacheDirective},
    StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HomeDocument {
    pub name: &'static str,
    pub version: &'static str,
}

pub async fn handle() -> Response<SimpleRespondable<HalDocument>> {
    let hal_document = HalDocument::new(HomeDocument {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    })
    .with_link("self", "/");

    SimpleRespondable::from(hal_document)
        .with_status_code(StatusCode::OK)
        .with_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(3600),
        ]))
        .into()
}
