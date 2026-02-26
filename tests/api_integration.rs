use actix_web::{test, web, App};
use garden::api::routes::configure;

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
        .app_data(
            web::JsonConfig::default().error_handler(|err, _req| {
                let message = format!("{err}");
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest()
                        .json(serde_json::json!({ "error": message })),
                )
                .into()
            }),
        )
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
    assert!(body.is_array(), "Response must be a JSON array");
    assert!(!body.as_array().unwrap().is_empty(), "Array must not be empty");
}

#[actix_web::test]
async fn test_get_vegetables_items_have_required_fields() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get().uri("/api/vegetables").to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    for item in body.as_array().unwrap() {
        assert!(item.get("id").is_some(), "Each vegetable must have an 'id' field");
        assert!(item.get("name").is_some(), "Each vegetable must have a 'name' field");
        assert!(item.get("seasons").is_some(), "Each vegetable must have a 'seasons' field");
        assert!(item.get("good_companions").is_some(), "Each vegetable must have 'good_companions'");
        assert!(item.get("bad_companions").is_some(), "Each vegetable must have 'bad_companions'");
    }
}

// ---------------------------------------------------------------------------
// GET /api/vegetables/{id}/companions
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_get_companions_known_id_returns_200() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomate/companions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn test_get_companions_returns_good_and_bad() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/tomate/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body.get("good").map(|v| v.is_array()).unwrap_or(false),
        "Response must contain a 'good' array"
    );
    assert!(
        body.get("bad").map(|v| v.is_array()).unwrap_or(false),
        "Response must contain a 'bad' array"
    );
}

#[actix_web::test]
async fn test_get_companions_unknown_id_returns_404() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/legume-inexistant/companions")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_get_companions_unknown_id_returns_error_message() {
    let app = test::init_service(build_app()).await;
    let req = test::TestRequest::get()
        .uri("/api/vegetables/legume-inexistant/companions")
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let error_msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(!error_msg.is_empty(), "An error message must be returned for an unknown id");
}

// ---------------------------------------------------------------------------
// POST /api/plan
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_minimal_request_returns_200() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "width_m": 2.0,
        "length_m": 3.0,
        "season": "Summer"
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
        "width_m": 2.0,
        "length_m": 3.0,
        "season": "Summer"
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(body.get("grid").map(|v| v.is_array()).unwrap_or(false), "Response must contain a grid");
    assert!(body.get("score").is_some(), "Response must contain a score");
    assert!(body.get("warnings").map(|v| v.is_array()).unwrap_or(false), "Response must contain warnings");
}

#[actix_web::test]
async fn test_post_plan_full_request_returns_200() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "width_m": 3.0,
        "length_m": 3.0,
        "season": "Spring",
        "sun": "FullSun",
        "soil": "Loamy",
        "region": "Temperate",
        "level": "Beginner",
        "preferences": ["tomate", "basilic"]
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
        "width_m": 2.0,
        "length_m": 2.0,
        "season": "Summer",
        "preferences": ["tomate", "basilic", "carotte"]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let score = body.get("score").and_then(|s| s.as_i64()).unwrap_or(-999);
    assert!(score >= 0, "Score must be >= 0 for a garden with good companion associations (score = {score})");
}

#[actix_web::test]
async fn test_post_plan_invalid_zero_dimensions_returns_400() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "width_m": 0.0,
        "length_m": 3.0,
        "season": "Summer"
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
        "width_m": 0.0,
        "length_m": 3.0,
        "season": "Summer"
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let error_msg = body.get("error").and_then(|v| v.as_str()).unwrap_or("");
    assert!(!error_msg.is_empty(), "A readable error message must be returned");
}

#[actix_web::test]
async fn test_post_plan_with_existing_layout_preserved() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "width_m": 0.6,
        "length_m": 0.6,
        "season": "Summer",
        "existing_layout": [
            ["tomate", null],
            [null, null]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let first_cell_id = body["grid"][0][0]["id"].as_str().unwrap_or("");
    assert_eq!(first_cell_id, "tomate", "Existing cell [0][0] must remain 'tomate'");
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
