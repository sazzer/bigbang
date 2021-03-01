use actix_web::test::TestRequest;
use assert2::check;

#[actix_rt::test]
pub async fn get_home() {
    let test_suite = crate::integration::TestSuite::new().await;

    let response = test_suite
        .inject(TestRequest::get().uri("/").to_request())
        .await;

    check!(response.status == 200);
}
