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

fn collect_placed_ids(body: &serde_json::Value) -> Vec<String> {
    body["payload"]["grid"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .flat_map(|row| row.as_array().unwrap_or(&vec![]).to_owned())
        .filter_map(|cell| cell["id"].as_str().map(String::from))
        .collect()
}

// ---------------------------------------------------------------------------
// Scenario 1: Small summer garden, full sun, loamy soil, beginner
// ---------------------------------------------------------------------------
#[actix_web::test]
async fn scenario_small_summer_garden() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "widthM": 2.0,
        "lengthM": 3.0,
        "season": "Summer",
        "sun": "FullSun",
        "soil": "Loamy",
        "region": "Temperate",
        "level": "Beginner"
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    assert_eq!(body["payload"]["rows"], 10, "2m x 3m → 10 rows");
    assert_eq!(body["payload"]["cols"], 7, "2m x 3m → 7 cols");

    let placed = collect_placed_ids(&body);
    // A beginner summer garden must contain at least one typical summer vegetable
    let typical_summer = ["tomato", "zucchini", "cucumber", "green-bean", "radish"];
    let found_typical = typical_summer.iter().any(|id| placed.contains(&id.to_string()));
    assert!(
        found_typical,
        "Summer garden must contain at least one typical vegetable: {:?}. Found: {:?}",
        typical_summer, placed
    );

    // No advanced vegetables should appear in a beginner garden
    let advanced_vegs = ["pepper", "fennel", "eggplant", "celery", "asparagus"];
    for id in &advanced_vegs {
        assert!(
            !placed.contains(&id.to_string()),
            "Advanced vegetable '{}' must not appear in a beginner garden",
            id
        );
    }
}

// ---------------------------------------------------------------------------
// Scenario 2: Small spring garden, mountain region, clay soil
// ---------------------------------------------------------------------------
#[actix_web::test]
async fn scenario_spring_cool_climate() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "widthM": 1.0,
        "lengthM": 2.0,
        "season": "Spring",
        "region": "Mountain",
        "soil": "Clay"
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(body["payload"].get("grid").and_then(|g| g.as_array()).map(|a| !a.is_empty()), Some(true));

    // Only spring vegetables must be placed
    let summer_only_vegs = ["tomato", "cucumber", "zucchini", "eggplant", "corn"];
    let placed = collect_placed_ids(&body);
    for id in &summer_only_vegs {
        assert!(
            !placed.contains(&id.to_string()),
            "Summer-only vegetable '{}' must not appear in a spring garden",
            id
        );
    }
}

// ---------------------------------------------------------------------------
// Scenario 3: Garden with existing tomatoes → basil must be placed adjacent
// ---------------------------------------------------------------------------
#[actix_web::test]
async fn scenario_existing_tomatoes_add_companions() {
    let app = test::init_service(build_app()).await;
    // 3x3 grid, tomato at [0][0], placing summer vegetables with basil as preference
    let payload = serde_json::json!({
        "widthM": 0.9,
        "lengthM": 0.9,
        "season": "Summer",
        "preferences": ["basil"],
        "existingLayout": [
            ["tomato", null, null],
            [null, null, null],
            [null, null, null]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    // Cell [0][0] must remain "tomato"
    assert_eq!(
        body["payload"]["grid"][0][0]["id"].as_str(),
        Some("tomato"),
        "Existing tomato must be preserved"
    );

    // Basil must be placed somewhere in the grid
    let placed = collect_placed_ids(&body);
    assert!(
        placed.contains(&"basil".to_string()),
        "Basil (preferred and good companion of tomato) must be placed in the grid"
    );

    // Basil must be adjacent to tomato at [0][0]:
    // adjacent positions: [0][1] and [1][0]
    let basil_adjacent_to_tomato = {
        let r0c1 = body["payload"]["grid"][0][1]["id"].as_str() == Some("basil");
        let r1c0 = body["payload"]["grid"][1][0]["id"].as_str() == Some("basil");
        r0c1 || r1c0
    };
    assert!(
        basil_adjacent_to_tomato,
        "Basil must be placed adjacent to tomato to maximise the companion score"
    );
}

// ---------------------------------------------------------------------------
// Scenario 4: Winter garden → only season-appropriate vegetables
// ---------------------------------------------------------------------------
#[actix_web::test]
async fn scenario_winter_garden() {
    let app = test::init_service(build_app()).await;
    let payload = serde_json::json!({
        "widthM": 1.5,
        "lengthM": 1.5,
        "season": "Winter",
        "region": "Oceanic"
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp_status_from_body(&body), None); // no error

    let placed = collect_placed_ids(&body);
    // In winter, no tomatoes or zucchinis
    assert!(!placed.contains(&"tomato".to_string()), "Tomato must not appear in winter");
    assert!(!placed.contains(&"zucchini".to_string()), "Zucchini must not appear in winter");

    // Winter vegetables like garlic or leek must be present
    let winter_vegs = ["garlic", "leek", "spinach", "cabbage"];
    let found_winter = winter_vegs.iter().any(|id| placed.contains(&id.to_string()));
    assert!(found_winter, "Winter vegetables must be present: {:?}", winter_vegs);
}

fn resp_status_from_body(body: &serde_json::Value) -> Option<&str> {
    body.get("error").and_then(|e| e.as_str())
}
