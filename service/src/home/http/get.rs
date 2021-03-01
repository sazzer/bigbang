use crate::http::Response;

pub async fn handle() -> Response<String> {
    "Hello".to_owned().into()
}
