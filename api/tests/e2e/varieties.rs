use crate::common::build_app_postgres;
use actix_web::test;

// ---------------------------------------------------------------------------
// GET /api/varieties
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_varieties_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/varieties").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_varieties_returns_array() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/varieties").to_request();
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
async fn test_get_varieties_items_have_required_fields() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/varieties").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        assert!(
            item["payload"].get("id").is_some(),
            "Each variety must have an 'id' field"
        );
        assert!(
            item["payload"].get("name").is_some(),
            "Each variety must have a 'name' field"
        );
    }
}

#[actix_web::test]
async fn test_get_varieties_items_have_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/varieties").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        let id = item["payload"]["id"].as_str().unwrap();
        let links = item.get("_links").expect("Each variety must have _links");
        assert_eq!(
            links["self"]["href"].as_str().unwrap(),
            format!("/api/varieties/{id}")
        );
        assert_eq!(links["self"]["method"].as_str().unwrap(), "GET");
    }
}

#[actix_web::test]
async fn test_get_varieties_french_locale() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties")
        .insert_header(("Accept-Language", "fr"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let tomato = body["payload"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["payload"]["id"].as_str() == Some("tomato"))
        .expect("tomato variety must be present");
    assert_eq!(tomato["payload"]["name"].as_str().unwrap(), "Tomate");
}

// ---------------------------------------------------------------------------
// GET /api/varieties/{id}
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_single_variety_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/tomato")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_single_variety_fields() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/brassica")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["payload"]["id"].as_str().unwrap(), "brassica");
    assert_eq!(body["payload"]["name"].as_str().unwrap(), "Brassica");
}

#[actix_web::test]
async fn test_get_single_variety_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/tomato")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body.get("_links").expect("Response must have _links");
    assert_eq!(
        links["self"]["href"].as_str().unwrap(),
        "/api/varieties/tomato"
    );
    assert_eq!(links["self"]["method"].as_str().unwrap(), "GET");
    assert_eq!(
        links["collection"]["href"].as_str().unwrap(),
        "/api/varieties"
    );
}

#[actix_web::test]
async fn test_get_single_variety_not_found() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/does-not-exist")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_single_variety_french_locale() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/pepper")
        .insert_header(("Accept-Language", "fr"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["payload"]["name"].as_str().unwrap(), "Poivron");
}

// ---------------------------------------------------------------------------
// Variety grouping — pepper and red-pepper share the same variety
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_pepper_and_red_pepper_share_variety() {
    let app = test::init_service(build_app_postgres().await).await;

    let req_pepper = test::TestRequest::get()
        .uri("/api/vegetables/pepper")
        .to_request();
    let body_pepper: serde_json::Value = test::call_and_read_body_json(&app, req_pepper).await;
    let pepper_variety = body_pepper["payload"]["varietyId"].as_str();

    let req_red = test::TestRequest::get()
        .uri("/api/vegetables/red-pepper")
        .to_request();
    let body_red: serde_json::Value = test::call_and_read_body_json(&app, req_red).await;
    let red_variety = body_red["payload"]["varietyId"].as_str();

    assert_eq!(
        pepper_variety,
        Some("pepper"),
        "pepper vegetable must have varietyId = 'pepper'"
    );
    assert_eq!(
        red_variety,
        Some("pepper"),
        "red-pepper vegetable must have varietyId = 'pepper'"
    );
    assert_eq!(
        pepper_variety, red_variety,
        "pepper and red-pepper must share the same variety"
    );
}

#[actix_web::test]
async fn test_brassica_groups_three_vegetables() {
    let app = test::init_service(build_app_postgres().await).await;
    for veg_id in ["cabbage", "broccoli", "cauliflower"] {
        let req = test::TestRequest::get()
            .uri(&format!("/api/vegetables/{veg_id}"))
            .to_request();
        let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
        assert_eq!(
            body["payload"]["varietyId"].as_str(),
            Some("brassica"),
            "{veg_id} must have varietyId = 'brassica'"
        );
    }
}
