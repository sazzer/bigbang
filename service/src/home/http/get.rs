use crate::http::{hal::HalDocument, Response, SimpleRespondable};
use actix_http::http::{
    header::{CacheControl, CacheDirective},
    StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Output {
    pub name: String,
}

pub async fn handle() -> Response<SimpleRespondable<HalDocument>> {
    let hal_document = HalDocument::new(Output {
        name: "Graham".to_owned(),
    })
    .with_link("self", "/")
    .with_link("self", "/");

    SimpleRespondable::from(hal_document)
        .with_status_code(StatusCode::OK)
        .with_header(CacheControl(vec![CacheDirective::NoCache]))
        .into()
}
