use crate::common::build_app_postgres;
use actix_web::test;

// ---------------------------------------------------------------------------
// GET /api/groups
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_list_groups_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/groups").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_list_groups_returns_paginated_payload() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/groups").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["payload"].is_array(),
        "response must contain a payload array"
    );
    assert!(
        !body["payload"].as_array().unwrap().is_empty(),
        "payload must not be empty"
    );
    assert!(
        body.get("pagination").is_some(),
        "response must contain pagination"
    );
}

#[actix_web::test]
async fn test_list_groups_items_have_required_fields() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/groups").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        assert!(item["payload"].get("id").is_some(), "group must have id");
        assert!(
            item["payload"].get("name").is_some(),
            "group must have name"
        );
    }
}

#[actix_web::test]
async fn test_list_groups_items_have_hateoas_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get().uri("/api/groups").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        let id = item["payload"]["id"].as_str().unwrap();
        let links = item.get("_links").expect("group must have _links");
        assert_eq!(
            links["self"]["href"].as_str().unwrap(),
            format!("/api/groups/{id}")
        );
        assert_eq!(
            links["vegetables"]["href"].as_str().unwrap(),
            format!("/api/groups/{id}/vegetables")
        );
    }
}

#[actix_web::test]
async fn test_list_groups_french_locale() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups")
        .insert_header(("Accept-Language", "fr"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let bulbes = body["payload"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["payload"]["id"].as_str() == Some("bulbes"))
        .expect("bulbes group not found");
    assert_eq!(bulbes["payload"]["name"].as_str().unwrap(), "Bulbes");
}

#[actix_web::test]
async fn test_list_groups_english_locale() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups")
        .insert_header(("Accept-Language", "en"))
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let bulbes = body["payload"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["payload"]["id"].as_str() == Some("bulbes"))
        .expect("bulbes group not found");
    assert_eq!(bulbes["payload"]["name"].as_str().unwrap(), "Bulbs");
}

// ---------------------------------------------------------------------------
// GET /api/groups/{id}
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_group_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_group_returns_404_for_unknown() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/does-not-exist")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_group_has_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body.get("_links").expect("response must have _links");
    assert_eq!(
        links["self"]["href"].as_str().unwrap(),
        "/api/groups/bulbes"
    );
    assert_eq!(
        links["vegetables"]["href"].as_str().unwrap(),
        "/api/groups/bulbes/vegetables"
    );
    assert_eq!(links["collection"]["href"].as_str().unwrap(), "/api/groups");
}

// ---------------------------------------------------------------------------
// GET /api/groups/{id}/vegetables
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_list_vegetables_by_group_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes/vegetables")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_list_vegetables_by_group_returns_404_for_unknown_group() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/does-not-exist/vegetables")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_list_vegetables_by_group_contains_bulb_vegetables() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes/vegetables?size=50")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let ids: Vec<&str> = body["payload"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["payload"]["id"].as_str().unwrap())
        .collect();
    assert!(ids.contains(&"onion"), "bulbes group must contain onion");
    assert!(ids.contains(&"garlic"), "bulbes group must contain garlic");
}

#[actix_web::test]
async fn test_list_vegetables_by_group_has_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes/vegetables")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body.get("_links").expect("response must have _links");
    assert_eq!(
        links["self"]["href"].as_str().unwrap(),
        "/api/groups/bulbes/vegetables"
    );
    assert_eq!(
        links["group"]["href"].as_str().unwrap(),
        "/api/groups/bulbes"
    );
}

#[actix_web::test]
async fn test_list_vegetables_by_group_pagination() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes/vegetables?page=1&size=2")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["pagination"]["page"].as_u64().unwrap(), 1);
    assert_eq!(body["pagination"]["perPage"].as_u64().unwrap(), 2);
    assert!(
        body["pagination"]["total"].as_u64().unwrap() >= 2,
        "bulbes group has at least 4 vegetables"
    );
}

#[actix_web::test]
async fn test_vegetable_has_group_id_field() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::get()
        .uri("/api/groups/bulbes/vegetables?size=50")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body["payload"].as_array().unwrap() {
        assert_eq!(
            item["payload"]["groupId"].as_str().unwrap(),
            "bulbes",
            "vegetable returned in bulbes group must have groupId=bulbes"
        );
    }
}
