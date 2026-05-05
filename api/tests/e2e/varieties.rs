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
        assert!(
            item["payload"].get("calendars").is_some(),
            "Each variety must have a 'calendars' field"
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
        assert!(
            links.get("companions").is_some(),
            "Each variety must have a companions link"
        );
        assert_eq!(links["companions"]["method"].as_str().unwrap(), "GET");
    }
}

// ---------------------------------------------------------------------------
// GET /api/varieties/{id}
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_variety_by_id_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/tomato")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_variety_by_id_unknown_returns_404() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties/nonexistent-variety")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_variety_by_id_returns_links() {
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
    assert!(
        links.get("companions").is_some(),
        "Response must have a companions link"
    );
    assert_eq!(links["companions"]["method"].as_str().unwrap(), "GET");
    assert_eq!(
        links["collection"]["href"].as_str().unwrap(),
        "/api/varieties"
    );
    assert_eq!(links["collection"]["method"].as_str().unwrap(), "GET");
}

// ---------------------------------------------------------------------------
// GET /api/varieties — filter tests
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_filter_by_lifecycle_annual() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?lifecycle=Annual&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected at least one Annual variety");
    for item in items {
        assert_eq!(
            item["payload"]["lifecycle"].as_str().unwrap(),
            "Annual",
            "all returned varieties must be Annual"
        );
    }
}

#[actix_web::test]
async fn test_filter_by_lifecycle_perennial() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?lifecycle=Perennial&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected at least one Perennial variety");
    for item in items {
        assert_eq!(
            item["payload"]["lifecycle"].as_str().unwrap(),
            "Perennial",
            "all returned varieties must be Perennial"
        );
    }
    // Thyme and rosemary are both Perennial in the seed data
    let ids: Vec<&str> = items
        .iter()
        .filter_map(|i| i["payload"]["id"].as_str())
        .collect();
    assert!(
        ids.contains(&"thyme"),
        "thyme must appear in Perennial results"
    );
}

#[actix_web::test]
async fn test_filter_by_category_herb() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?category=Herb&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected at least one Herb variety");
    for item in items {
        assert_eq!(
            item["payload"]["category"].as_str().unwrap(),
            "Herb",
            "all returned varieties must have category Herb"
        );
    }
}

#[actix_web::test]
async fn test_filter_by_beginner_friendly_true() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?beginner_friendly=true&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected beginner-friendly varieties");
    for item in items {
        assert_eq!(
            item["payload"]["beginnerFriendly"].as_bool().unwrap(),
            true,
            "all returned varieties must have beginnerFriendly == true"
        );
    }
}

#[actix_web::test]
async fn test_filter_by_beginner_friendly_false_excludes_true() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?beginner_friendly=false&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    for item in items {
        assert_eq!(
            item["payload"]["beginnerFriendly"].as_bool().unwrap(),
            false,
            "all returned varieties must have beginnerFriendly == false"
        );
    }
    // fennel has beginner_friendly=false in seed data
    let ids: Vec<&str> = items
        .iter()
        .filter_map(|i| i["payload"]["id"].as_str())
        .collect();
    assert!(
        ids.contains(&"fennel"),
        "fennel (beginner_friendly=false) must appear"
    );
}

#[actix_web::test]
async fn test_filter_by_sun_requirement_full_sun() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?sun_requirement=FullSun&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected at least one FullSun variety");
    for item in items {
        let sun: Vec<&str> = item["payload"]["sunRequirement"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|s| s.as_str())
            .collect();
        assert!(
            sun.contains(&"FullSun"),
            "all returned varieties must include FullSun in sunRequirement"
        );
    }
}

#[actix_web::test]
async fn test_filter_by_soil_type_loamy() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?soil_type=Loamy&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected at least one Loamy variety");
    for item in items {
        let soils: Vec<&str> = item["payload"]["soilTypes"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|s| s.as_str())
            .collect();
        assert!(
            soils.contains(&"Loamy"),
            "all returned varieties must include Loamy in soilTypes"
        );
    }
}

#[actix_web::test]
async fn test_filter_by_vegetable_id() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?vegetable_id=pepper&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "expected pepper varieties");
    for item in items {
        assert_eq!(
            item["payload"]["vegetableId"].as_str().unwrap(),
            "pepper",
            "all returned varieties must belong to vegetable pepper"
        );
    }
}

#[actix_web::test]
async fn test_filter_no_match_returns_empty_payload() {
    let app = test::init_service(build_app_postgres().await).await;
    // Root + Perennial has no seed data
    let req = test::TestRequest::get()
        .uri("/api/varieties?category=Root&lifecycle=Perennial")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        items.is_empty(),
        "Root+Perennial combination should yield no seed results"
    );
    assert_eq!(
        body["pagination"]["total"].as_u64().unwrap(),
        0,
        "total must be 0 for empty results"
    );
}

#[actix_web::test]
async fn test_search_by_name_returns_matching_varieties() {
    let app = test::init_service(build_app_postgres().await).await;
    // "tom" is a substring of "Tomato"
    let req = test::TestRequest::get()
        .uri("/api/varieties?search=tom&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        !items.is_empty(),
        "search=tom must return at least one result"
    );
    for item in items {
        let name = item["payload"]["name"].as_str().unwrap().to_lowercase();
        assert!(
            name.contains("tom"),
            "all returned varieties must have 'tom' in their name, got: {name}"
        );
    }
}

#[actix_web::test]
async fn test_search_by_name_is_case_insensitive() {
    let app = test::init_service(build_app_postgres().await).await;
    let req_lower = test::TestRequest::get()
        .uri("/api/varieties?search=tomato&size=100")
        .to_request();
    let req_upper = test::TestRequest::get()
        .uri("/api/varieties?search=TOMATO&size=100")
        .to_request();
    let body_lower: serde_json::Value = test::call_and_read_body_json(&app, req_lower).await;
    let body_upper: serde_json::Value = test::call_and_read_body_json(&app, req_upper).await;
    assert_eq!(
        body_lower["pagination"]["total"], body_upper["pagination"]["total"],
        "search must be case-insensitive"
    );
}

#[actix_web::test]
async fn test_search_no_match_returns_empty_payload() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?search=zzznomatch999")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        items.is_empty(),
        "non-matching search must return empty payload"
    );
    assert_eq!(
        body["pagination"]["total"].as_u64().unwrap(),
        0,
        "total must be 0 for non-matching search"
    );
}

#[actix_web::test]
async fn test_combined_filter_category_and_lifecycle() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/varieties?category=Herb&lifecycle=Perennial&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        !items.is_empty(),
        "expected Herb+Perennial varieties (thyme, rosemary, …)"
    );
    for item in items {
        assert_eq!(item["payload"]["category"].as_str().unwrap(), "Herb");
        assert_eq!(item["payload"]["lifecycle"].as_str().unwrap(), "Perennial");
    }
}
