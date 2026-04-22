use crate::common::build_app_postgres;
use actix_web::test;

// ---------------------------------------------------------------------------
// GET /api/varieties/{id}/companions
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_companions_known_id_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/tomato/companions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_companions_returns_good_and_bad() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/tomato/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["payload"]
            .get("good")
            .map(|v| v.is_array())
            .unwrap_or(false),
        "Response must contain a 'good' array"
    );
    assert!(
        body["payload"]
            .get("bad")
            .map(|v| v.is_array())
            .unwrap_or(false),
        "Response must contain a 'bad' array"
    );
}

#[actix_web::test]
async fn test_get_companions_unknown_id_returns_404() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/nonexistent-variety/companions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_companions_unknown_id_returns_error_message() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/nonexistent-variety/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let error_msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !error_msg.is_empty(),
        "An error message must be returned for an unknown id"
    );
}

#[actix_web::test]
async fn test_get_companions_returns_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/tomato/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body
        .get("_links")
        .expect("Companions response must have _links");
    assert_eq!(
        links["self"]["href"].as_str().unwrap(),
        "/api/varieties/tomato/companions"
    );
    assert_eq!(links["self"]["method"].as_str().unwrap(), "GET");
    assert_eq!(
        links["variety"]["href"].as_str().unwrap(),
        "/api/varieties/tomato"
    );
    assert_eq!(links["variety"]["method"].as_str().unwrap(), "GET");
}
