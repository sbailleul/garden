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
    let req = test::TestRequest::get()
        .uri("/api/vegetables?size=50")
        .to_request();
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
            item["payload"].get("groupId").is_some(),
            "Each vegetable must have a 'groupId' field"
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
    }
}

#[actix_web::test]
async fn test_get_vegetables_french_locale() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables?size=50")
        .insert_header(("Accept-Language", "fr"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let tomato = body["payload"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["payload"]["id"].as_str() == Some("tomato"))
        .expect("tomato vegetable must be present");
    assert_eq!(tomato["payload"]["name"].as_str().unwrap(), "Tomate");
}

// ---------------------------------------------------------------------------
// GET /api/vegetables/{id}
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_single_vegetable_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_single_vegetable_fields() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/brassica")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["payload"]["id"].as_str().unwrap(), "brassica");
    assert_eq!(body["payload"]["name"].as_str().unwrap(), "Brassica");
    assert_eq!(
        body["payload"]["groupId"].as_str().unwrap(),
        "legumes-feuilles",
        "brassica belongs to legumes-feuilles group"
    );
}

#[actix_web::test]
async fn test_get_single_vegetable_has_group_id() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(
        body["payload"]["groupId"].as_str().unwrap(),
        "legumes-fruits",
        "tomato belongs to legumes-fruits group"
    );
}

#[actix_web::test]
async fn test_get_single_vegetable_links() {
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
        links["collection"]["href"].as_str().unwrap(),
        "/api/vegetables"
    );
}

#[actix_web::test]
async fn test_get_single_vegetable_not_found() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/does-not-exist")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_single_vegetable_french_locale() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper")
        .insert_header(("Accept-Language", "fr"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["payload"]["name"].as_str().unwrap(), "Poivron");
}

// ---------------------------------------------------------------------------
// Vegetable grouping — pepper and red-pepper share the same vegetable
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_pepper_and_red_pepper_share_vegetable() {
    let app = test::init_service(build_app_postgres().await).await;

    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let variety_ids: Vec<&str> = body["payload"]["varietyIds"]
        .as_array()
        .expect("varietyIds must be an array")
        .iter()
        .filter_map(|v| v.as_str())
        .collect();

    assert!(
        variety_ids.contains(&"pepper"),
        "pepper vegetable must contain variety 'pepper'"
    );
    assert!(
        variety_ids.contains(&"red-pepper"),
        "pepper vegetable must contain variety 'red-pepper'"
    );
}

#[actix_web::test]
async fn test_brassica_groups_three_varieties() {
    let app = test::init_service(build_app_postgres().await).await;

    let req = test::TestRequest::get()
        .uri("/api/vegetables/brassica")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let variety_ids: Vec<&str> = body["payload"]["varietyIds"]
        .as_array()
        .expect("varietyIds must be an array")
        .iter()
        .filter_map(|v| v.as_str())
        .collect();

    for var_id in ["cabbage", "broccoli", "cauliflower"] {
        assert!(
            variety_ids.contains(&var_id),
            "brassica vegetable must contain variety '{var_id}'"
        );
    }
}

// ---------------------------------------------------------------------------
// GET /api/vegetables/{id}/varieties
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_varieties_by_vegetable_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper/varieties")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_varieties_by_vegetable_returns_matching_varieties() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper/varieties")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(!items.is_empty(), "pepper must have at least one variety");
    let ids: Vec<&str> = items
        .iter()
        .filter_map(|item| item["payload"]["id"].as_str())
        .collect();
    assert!(ids.contains(&"pepper"), "pepper variety must be present");
    assert!(
        ids.contains(&"red-pepper"),
        "red-pepper variety must be present"
    );
    for item in items {
        assert_eq!(
            item["payload"]["vegetableId"].as_str().unwrap(),
            "pepper",
            "all returned varieties must have vegetableId == 'pepper'"
        );
    }
}

#[actix_web::test]
async fn test_get_varieties_by_vegetable_returns_404_for_unknown() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/does-not-exist/varieties")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_varieties_by_vegetable_links_contain_self_and_vegetable() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/brassica/varieties")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["_links"]["self"]["href"].as_str().is_some(),
        "_links.self must be present"
    );
    assert!(
        body["_links"]["vegetable"]["href"].as_str().is_some(),
        "_links.vegetable must be present"
    );
}

// ---------------------------------------------------------------------------
// GET /api/vegetables/{id}/varieties — filter tests
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_varieties_by_vegetable_filter_lifecycle_annual() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper/varieties?lifecycle=Annual&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        !items.is_empty(),
        "expected at least one Annual pepper variety"
    );
    for item in items {
        assert_eq!(
            item["payload"]["vegetableId"].as_str().unwrap(),
            "pepper",
            "all returned varieties must belong to pepper"
        );
        assert_eq!(
            item["payload"]["lifecycle"].as_str().unwrap(),
            "Annual",
            "all returned varieties must be Annual"
        );
    }
}

#[actix_web::test]
async fn test_varieties_by_vegetable_filter_beginner_friendly() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper/varieties?beginner_friendly=true&size=100")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        !items.is_empty(),
        "expected beginner-friendly pepper varieties"
    );
    for item in items {
        assert_eq!(
            item["payload"]["beginnerFriendly"].as_bool().unwrap(),
            true,
            "all returned varieties must be beginner-friendly"
        );
    }
}

#[actix_web::test]
async fn test_varieties_by_vegetable_filter_no_match_returns_empty() {
    let app = test::init_service(build_app_postgres().await).await;
    // Biennial has no seed data, so this should return an empty list
    let req = test::TestRequest::get()
        .uri("/api/vegetables/pepper/varieties?lifecycle=Biennial")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let items = body["payload"].as_array().expect("payload must be array");
    assert!(
        items.is_empty(),
        "Biennial filter must yield no pepper varieties"
    );
}
