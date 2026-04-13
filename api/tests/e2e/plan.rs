use crate::common::{build_app_postgres, null_layout};
use actix_web::test;

// ---------------------------------------------------------------------------
// POST /api/plan — basic
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_minimal_request_returns_200() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
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
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "layout": null_layout(10, 7)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["payload"]["weeks"][0]
            .get("grid")
            .map(|v| v.is_array())
            .unwrap_or(false),
        "Response must contain weeks[0].grid"
    );
    assert!(
        body["payload"]["weeks"][0].get("score").is_some(),
        "Response must contain weeks[0].score"
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
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-03-01", "end": "2025-05-31"},
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
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "preferences": [{"id": "tomato"}, {"id": "basil"}, {"id": "carrot"}],
        "layout": null_layout(7, 7)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let score = body["payload"]["weeks"][0]
        .get("score")
        .and_then(|s| s.as_i64())
        .unwrap_or(-999);
    assert!(
        score >= 0,
        "Score must be >= 0 for a garden with good companion associations (score = {score})"
    );
}

// ---------------------------------------------------------------------------
// POST /api/plan — validation errors
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_invalid_zero_dimensions_returns_400() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
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
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
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
async fn test_post_plan_malformed_json_returns_400() {
    let app = test::init_service(build_app_postgres().await).await;
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .insert_header(("content-type", "application/json"))
        .set_payload("{invalid json}")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

// ---------------------------------------------------------------------------
// POST /api/plan — existing layout
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_with_existing_layout_preserved() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "layout": [
            [{"type": "SelfContained", "id": "tomato"}, {"type": "Empty"}],
            [{"type": "Empty"}, {"type": "Empty"}]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    let first_cell_id = body["payload"]["weeks"][0]["grid"][0][0]["id"]
        .as_str()
        .unwrap_or("");
    assert_eq!(
        first_cell_id, "tomato",
        "Existing cell [0][0] must remain 'tomato'"
    );
}

#[actix_web::test]
async fn test_post_plan_existing_layout_planted_date_sets_estimated_harvest_date() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-02", "end": "2025-06-08"},
        "region": "Temperate",
        "layout": [
            [{"type": "SelfContained", "id": "tomato", "plantedDate": "2025-05-01"}]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert!(
        body["payload"]["weeks"][0]["grid"][0][0]["estimatedHarvestDate"].is_string(),
        "estimatedHarvestDate must be present when plantedDate is supplied: {body}"
    );
}

// ---------------------------------------------------------------------------
// POST /api/plan — blocked cells
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_blocked_cells_never_planted() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "layout": [
            [{"type": "Empty"}, {"type": "Empty"}, {"type": "Empty"}],
            [{"type": "Blocked"}, {"type": "Blocked"}, {"type": "Blocked"}],
            [{"type": "Empty"}, {"type": "Empty"}, {"type": "Empty"}]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    let row1 = body["payload"]["weeks"][0]["grid"][1].as_array().unwrap();
    for cell in row1 {
        assert!(cell["id"].is_null(), "Blocked cell must have no vegetable");
        assert_eq!(
            cell["type"], "Blocked",
            "Blocked cell must have type='Blocked'"
        );
    }
}

#[actix_web::test]
async fn test_post_plan_blocked_flag_false_on_plantable_cells() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "layout": [
            [{"type": "Empty"}, {"type": "Empty"}, {"type": "Empty"}],
            [{"type": "Blocked"}, {"type": "Blocked"}, {"type": "Blocked"}],
            [{"type": "Empty"}, {"type": "Empty"}, {"type": "Empty"}]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    for r in [0usize, 2usize] {
        let row = body["payload"]["weeks"][0]["grid"][r].as_array().unwrap();
        for cell in row {
            assert_ne!(
                cell["type"], "Blocked",
                "Non-blocked cell must not have type='Blocked' (row {r})"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// POST /api/plan — sown entries
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_with_sown_entries_is_accepted() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "sown": {
            "tomato": [{"sowingDate": "2025-03-15", "seedsSown": 10}],
            "pepper": [{"sowingDate": "2025-02-20", "seedsSown": 6}, {"seedsSown": 4}]
        },
        "layout": null_layout(4, 4)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

// ---------------------------------------------------------------------------
// POST /api/plan — HATEOAS _links
// ---------------------------------------------------------------------------

#[actix_web::test]
async fn test_post_plan_returns_links() {
    let app = test::init_service(build_app_postgres().await).await;
    let payload = serde_json::json!({
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "layout": null_layout(4, 4)
    });
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
