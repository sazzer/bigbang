use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
pub async fn get_metrics() {
    let test_suite = crate::integration::TestSuite::new().await;

    let response = test_suite
        .inject(TestRequest::get().uri("/metrics").to_request())
        .await;

    check!(response.status == 404);
}
