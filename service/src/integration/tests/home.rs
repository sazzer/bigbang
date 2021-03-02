use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
pub async fn get_home() {
    let test_suite = crate::integration::TestSuite::new().await;

    let response = test_suite
        .inject(TestRequest::get().uri("/").to_request())
        .await;

    check!(response.status == 200);

    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "name": "bigbang",
      "version": "0.1.0",
      "_links": {
        "self": {
          "href": "/"
        }
      }
    }
    "###);
}
