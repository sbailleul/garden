use crate::common::build_app_postgres;
use actix_web::test;

// ---------------------------------------------------------------------------
// GET /api/vegetables
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_vegetables_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/vegetables").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_vegetables_returns_array() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/vegetables").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["payload"].is_array(),
        "Response must contain a payload array"
    );
    assert!(
        !body["payload"].as_array().unwrap().is_empty(),
        "Payload array must not be empty"
    );
    assert!(
        body.get("pagination").is_some(),
        "Response must contain pagination metadata"
    );
}

#[actix_web::test]
async fn test_get_vegetables_items_have_required_fields() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/vegetables").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        assert!(
            item["payload"].get("id").is_some(),
            "Each vegetable must have an 'id' field"
        );
        assert!(
            item["payload"].get("name").is_some(),
            "Each vegetable must have a 'name' field"
        );
        assert!(
            item["payload"].get("calendars").is_some(),
            "Each vegetable must have a 'calendars' field"
        );
        assert!(
            item["payload"].get("goodCompanions").is_some(),
            "Each vegetable must have 'goodCompanions'"
        );
        assert!(
            item["payload"].get("badCompanions").is_some(),
            "Each vegetable must have 'badCompanions'"
        );
    }
}

#[actix_web::test]
async fn test_get_vegetables_items_have_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/vegetables").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        let id = item["payload"]["id"].as_str().unwrap();
        let links = item.get("_links").expect("Each vegetable must have _links");
        assert_eq!(
            links["self"]["href"].as_str().unwrap(),
            format!("/api/vegetables/{id}")
        );
        assert_eq!(links["self"]["method"].as_str().unwrap(), "GET");
        assert_eq!(
            links["companions"]["href"].as_str().unwrap(),
            format!("/api/vegetables/{id}/companions")
        );
        assert_eq!(links["companions"]["method"].as_str().unwrap(), "GET");
    }
}

// ---------------------------------------------------------------------------
// GET /api/vegetables/{id}
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_vegetable_by_id_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_vegetable_by_id_unknown_returns_404() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/nonexistent-vegetable")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_vegetable_by_id_returns_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body.get("_links").expect("Response must have _links");
    assert_eq!(
        links["self"]["href"].as_str().unwrap(),
        "/api/vegetables/tomato"
    );
    assert_eq!(links["self"]["method"].as_str().unwrap(), "GET");
    assert_eq!(
        links["companions"]["href"].as_str().unwrap(),
        "/api/vegetables/tomato/companions"
    );
    assert_eq!(links["companions"]["method"].as_str().unwrap(), "GET");
    assert_eq!(
        links["collection"]["href"].as_str().unwrap(),
        "/api/vegetables"
    );
    assert_eq!(links["collection"]["method"].as_str().unwrap(), "GET");
}
