pub mod hal;

use actix_http::{
    http::{header::Header, HeaderMap, StatusCode},
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

/// Simple implementation of the Respondable trait.
pub struct SimpleRespondable<T>
where
    T: Serialize,
{
    status_code: StatusCode,
    headers: HeaderMap,
    body: T,
}

impl<T> SimpleRespondable<T>
where
    T: Serialize,
{
    /// Create a new instance of the `SimpleRespondable` struct wrapping the provided body
    pub fn new(body: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
            body,
        }
    }

    /// Specify the status code to use
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    /// Specify a header to use
    pub fn with_header<H>(mut self, header: H) -> Self
    where
        H: Header,
    {
        let name = H::name();
        match header.try_into() {
            Ok(value) => {
                self.headers.append(name, value);
            }
            Err(_) => {
                tracing::error!(name = ?name, "Failed to process header");
            }
        };

        self
    }
}

impl<T> Respondable for SimpleRespondable<T>
where
    T: Serialize,
{
    type Body = T;

    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    fn body(self) -> Self::Body {
        self.body
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
