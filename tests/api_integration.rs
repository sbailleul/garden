use actix_web::{test, web, App};
use garden::api::routes::configure;

fn null_layout(rows: usize, cols: usize) -> serde_json::Value {
    let row: Vec<serde_json::Value> = vec![serde_json::Value::Null; cols];
    let layout: Vec<serde_json::Value> = (0..rows)
        .map(|_| serde_json::Value::Array(row.clone()))
        .collect();
    serde_json::Value::Array(layout)
}

fn build_app() -> actix_web::App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .configure(configure)
        .app_data(web::JsonConfig::default().error_handler(|err, _req| {
            let message = format!("{err}");
            actix_web::error::InternalError::from_response(
                err,
                actix_web::HttpResponse::BadRequest().json(serde_json::json!({ "error": message })),
            )
            .into()
        }))
}

// ---------------------------------------------------------------------------
// GET /api/vegetables
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_vegetables_returns_200() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get().uri("/api/vegetables").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_vegetables_returns_array() {
    let app = test::init_service(build_app()).await;
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
    let app = test::init_service(build_app()).await;
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
            item["payload"].get("seasons").is_some(),
            "Each vegetable must have a 'seasons' field"
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

// ---------------------------------------------------------------------------
// GET /api/vegetables/{id}/companions
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_companions_known_id_returns_200() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato/companions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_companions_returns_good_and_bad() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato/companions")
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
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/nonexistent-vegetable/companions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_companions_unknown_id_returns_error_message() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/nonexistent-vegetable/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let error_msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !error_msg.is_empty(),
        "An error message must be returned for an unknown id"
    );
}

// ---------------------------------------------------------------------------
// POST /api/plan
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_minimal_request_returns_200() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": null_layout(10, 7)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_post_plan_minimal_has_grid_and_score() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": null_layout(10, 7)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["payload"]
            .get("grid")
            .map(|v| v.is_array())
            .unwrap_or(false),
        "Response must contain a grid"
    );
    assert!(
        body["payload"].get("score").is_some(),
        "Response must contain a score"
    );
    assert!(
        body["payload"]
            .get("warnings")
            .map(|v| v.is_array())
            .unwrap_or(false),
        "Response must contain warnings"
    );
}

#[actix_web::test]
async fn test_post_plan_full_request_returns_200() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Spring",
        "sun": "FullSun",
        "soil": "Loamy",
        "region": "Temperate",
        "level": "Beginner",
        "preferences": [{"id": "tomato"}, {"id": "basil"}],
        "layout": null_layout(10, 10)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_post_plan_score_is_non_negative_for_compatible_garden() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Summer",
        "preferences": [{"id": "tomato"}, {"id": "basil"}, {"id": "carrot"}],
        "layout": null_layout(7, 7)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let score = body["payload"]
        .get("score")
        .and_then(|s| s.as_i64())
        .unwrap_or(-999);
    assert!(
        score >= 0,
        "Score must be >= 0 for a garden with good companion associations (score = {score})"
    );
}

#[actix_web::test]
async fn test_post_plan_invalid_zero_dimensions_returns_400() {
    let app = test::init_service(build_app()).await;
    // Empty layout triggers validation error → 400
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": []
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_post_plan_invalid_returns_error_message() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": []
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let error_msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        !error_msg.is_empty(),
        "A readable error message must be returned"
    );
}

#[actix_web::test]
async fn test_post_plan_with_existing_layout_preserved() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": [
            ["tomato", null],
            [null, null]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let first_cell_id = body["payload"]["grid"][0][0]["id"].as_str().unwrap_or("");
    assert_eq!(
        first_cell_id, "tomato",
        "Existing cell [0][0] must remain 'tomato'"
    );
}

#[actix_web::test]
async fn test_post_plan_malformed_json_returns_400() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .insert_header(("content-type", "application/json"))
        .set_payload("{invalid json}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

// ---------------------------------------------------------------------------
// POST /api/plan — blocked cells
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_blocked_cells_never_planted() {
    let app = test::init_service(build_app()).await;
    // 3×3 grid; middle row fully blocked
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": [
            [null, null, null],
            [true, true, true],
            [null, null, null]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    let row1 = body["payload"]["grid"][1].as_array().unwrap();
    for cell in row1 {
        assert!(cell["id"].is_null(), "Blocked cell must have no vegetable");
        assert_eq!(cell["type"], "blocked", "Blocked cell must have type='blocked'");
    }
}

#[actix_web::test]
async fn test_post_plan_blocked_flag_false_on_plantable_cells() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "season": "Summer",
        "layout": [
            [null, null, null],
            [true, true, true],
            [null, null, null]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    for r in [0usize, 2usize] {
        let row = body["payload"]["grid"][r].as_array().unwrap();
        for cell in row {
            assert_ne!(
                cell["type"], "blocked",
                "Non-blocked cell must not have type='blocked' (row {r})"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// HATEOAS — _links
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_vegetables_items_have_links() {
    let app = test::init_service(build_app()).await;
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

#[actix_web::test]
async fn test_get_vegetable_by_id_returns_200() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_vegetable_by_id_unknown_returns_404() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/nonexistent-vegetable")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_vegetable_by_id_returns_links() {
    let app = test::init_service(build_app()).await;
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

#[actix_web::test]
async fn test_get_companions_returns_links() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomato/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body
        .get("_links")
        .expect("Companions response must have _links");
    assert_eq!(
        links["self"]["href"].as_str().unwrap(),
        "/api/vegetables/tomato/companions"
    );
    assert_eq!(links["self"]["method"].as_str().unwrap(), "GET");
    assert_eq!(
        links["vegetable"]["href"].as_str().unwrap(),
        "/api/vegetables/tomato"
    );
    assert_eq!(links["vegetable"]["method"].as_str().unwrap(), "GET");
}

#[actix_web::test]
async fn test_post_plan_returns_links() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({ "season": "Summer", "layout": null_layout(4, 4) });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let links = body.get("_links").expect("Plan response must have _links");
    assert_eq!(links["self"]["href"].as_str().unwrap(), "/api/plan");
    assert_eq!(links["self"]["method"].as_str().unwrap(), "POST");
    assert_eq!(
        links["vegetables"]["href"].as_str().unwrap(),
        "/api/vegetables"
    );
    assert_eq!(links["vegetables"]["method"].as_str().unwrap(), "GET");
}
