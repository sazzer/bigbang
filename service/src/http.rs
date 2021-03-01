use actix_http::{
    http::{HeaderMap, StatusCode},
    Error, Response as HttpResponse,
};
use actix_web::Responder;
use futures::future::{ok, Ready};
use serde::Serialize;

/// Trait that anything able to represent a response can implement.
pub trait Respondable {
    type Body: Serialize;

    /// Generate the status code for the response
    fn status_code(&self) -> StatusCode {
        StatusCode::OK
    }

    /// Generate any headers for the response
    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }

    /// Retrieve the body of the response
    fn body(self) -> Self::Body;
}

impl<T> Respondable for T
where
    T: Serialize,
{
    type Body = T;

    fn body(self) -> Self::Body {
        self
    }
}

impl<R> From<R> for Response<R>
where
    R: Respondable,
    R::Body: Serialize,
{
    fn from(respondable: R) -> Self {
        Self(respondable)
    }
}

/// Wrapper for any HTTP Response, implementing the standard requirements.
pub struct Response<R>(pub R)
where
    R: Respondable,
    R::Body: Serialize;

impl<R> Responder for Response<R>
where
    R: Respondable,
    R::Body: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
        let mut response = HttpResponse::build(self.0.status_code());

        for (key, value) in self.0.headers().iter() {
            response.set_header(key, value.clone());
        }

        let built = response.json(self.0.body());

        ok(built)
    }
}
