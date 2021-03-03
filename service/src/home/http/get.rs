use crate::home::HomeLinksUseCase;
use crate::http::{hal::HalDocument, Response, SimpleRespondable};
use actix_http::http::{
    header::{CacheControl, CacheDirective},
    StatusCode,
};
use actix_web::web::Data;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct HomeDocument {
    pub name: &'static str,
    pub version: &'static str,
}

pub async fn handle(
    home_links: Data<Arc<HomeLinksUseCase>>,
) -> Response<SimpleRespondable<HalDocument>> {
    let mut hal_document = HalDocument::new(HomeDocument {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    });

    let links = home_links.generate_links().await;
    for (name, link) in links {
        hal_document = hal_document.with_link(name, link);
    }

    SimpleRespondable::from(hal_document)
        .with_status_code(StatusCode::OK)
        .with_header(CacheControl(vec![
            CacheDirective::Public,
            CacheDirective::MaxAge(3600),
        ]))
        .into()
}
