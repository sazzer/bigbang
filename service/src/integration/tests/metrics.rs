#[actix_rt::test]
pub async fn get_metrics() {
    let _test_suite = crate::integration::TestSuite::new().await;
}
