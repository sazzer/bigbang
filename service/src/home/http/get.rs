use crate::http::{Response, SimpleRespondable};
use actix_http::http::{
    header::{CacheControl, CacheDirective},
    StatusCode,
};

pub async fn handle() -> Response<SimpleRespondable<String>> {
    SimpleRespondable::new("Hello, World".to_owned())
        .with_status_code(StatusCode::FOUND)
        .with_header(CacheControl(vec![CacheDirective::NoCache]))
        .into()
}
