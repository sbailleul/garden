use actix_web::{test, web, App};
use garden::adapters::inbound::http::routes::configure;
use garden::adapters::outbound::memory::vegetable_repository::InMemoryVegetableRepository;
use garden::application::ports::vegetable_repository::VegetableRepository;

fn null_layout(rows: usize, cols: usize) -> serde_json::Value {
    let empty_cell = serde_json::json!({"type": "Empty"});
    let row: Vec<serde_json::Value> = vec![empty_cell; cols];
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
    let repo: Box<dyn VegetableRepository> = Box::new(InMemoryVegetableRepository);
    App::new()
        .app_data(web::Data::new(repo))
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

fn collect_placed_ids(body: &serde_json::Value) -> Vec<String> {
    body["payload"]["weeks"][0]["grid"]
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
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "sun": "FullSun",
        "soil": "Loamy",
        "region": "Temperate",
        "level": "Beginner",
        "layout": null_layout(10, 7)
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
    let found_typical = typical_summer
        .iter()
        .any(|id| placed.contains(&id.to_string()));
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
        "period": {"start": "2025-03-01", "end": "2025-05-31"},
        "region": "Mountain",
        "soil": "Clay",
        "layout": null_layout(7, 4)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(
        body["payload"]
            .get("weeks")
            .and_then(|w| w.as_array())
            .and_then(|a| a.first())
            .and_then(|w| w.get("grid"))
            .and_then(|g| g.as_array())
            .map(|a| !a.is_empty()),
        Some(true)
    );

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
        "period": {"start": "2025-06-01", "end": "2025-08-31"},
        "region": "Temperate",
        "preferences": [{"id": "basil"}],
        "layout": [
            [{"type": "SelfContained", "id": "tomato"}, {"type": "Empty"}, {"type": "Empty"}],
            [{"type": "Empty"}, {"type": "Empty"}, {"type": "Empty"}],
            [{"type": "Empty"}, {"type": "Empty"}, {"type": "Empty"}]
        ]
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    // Cell [0][0] must remain "tomato"
    assert_eq!(
        body["payload"]["weeks"][0]["grid"][0][0]["id"].as_str(),
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
        let r0c1 = body["payload"]["weeks"][0]["grid"][0][1]["id"].as_str() == Some("basil");
        let r1c0 = body["payload"]["weeks"][0]["grid"][1][0]["id"].as_str() == Some("basil");
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
        "period": {"start": "2024-12-01", "end": "2025-02-28"},
        "region": "Oceanic",
        "layout": null_layout(5, 5)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp_status_from_body(&body), None); // no error

    let placed = collect_placed_ids(&body);
    // In winter, no tomatoes or zucchinis
    assert!(
        !placed.contains(&"tomato".to_string()),
        "Tomato must not appear in winter"
    );
    assert!(
        !placed.contains(&"zucchini".to_string()),
        "Zucchini must not appear in winter"
    );

    // Winter vegetables like garlic or leek must be present
    let winter_vegs = ["garlic", "leek", "spinach", "cabbage"];
    let found_winter = winter_vegs
        .iter()
        .any(|id| placed.contains(&id.to_string()));
    assert!(
        found_winter,
        "Winter vegetables must be present: {:?}",
        winter_vegs
    );
}

fn resp_status_from_body(body: &serde_json::Value) -> Option<&str> {
    body.get("error").and_then(|e| e.as_str())
}

// ---------------------------------------------------------------------------
// Scenario 5: Sown tomato seeds are placed in the grid after their plant date
// ---------------------------------------------------------------------------
#[actix_web::test]
async fn scenario_sown_seeds_appear_in_plan() {
    let app = test::init_service(build_app()).await;
    // Tomato: days_to_plant=42; sown 2025-03-01 → ready 2025-04-12.
    // Planning window starts 2025-04-14 (after ready date) so tomato must appear.
    let payload = serde_json::json!({
        "period": {"start": "2025-04-14", "end": "2025-06-30"},
        "region": "Temperate",
        "sown": {
            "tomato": [{"sowingDate": "2025-03-01", "seedsSown": 2}]
        },
        "layout": null_layout(5, 5)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    // Collect placed ids across ALL weeks (merging may collapse multiple weeks).
    let placed: Vec<String> = body["payload"]["weeks"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .flat_map(|w| {
            w["grid"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .flat_map(|row| row.as_array().unwrap_or(&vec![]).to_owned())
                .filter_map(|cell| cell["id"].as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .collect();

    assert!(
        placed.contains(&"tomato".to_string()),
        "Sown tomato should appear in the plan after its plant_date. Placed: {:?}",
        placed
    );
}

// ---------------------------------------------------------------------------
// Scenario 6: Sowing schedule — tomato must appear in sowingTasks for the
//             week that is exactly days_to_plant before its transplant week
// ---------------------------------------------------------------------------
#[actix_web::test]
async fn scenario_sowing_tasks_computed_per_week() {
    let app = test::init_service(build_app()).await;
    // Planning period: 2025-03-03 → 2025-06-29 (Temperate).
    // Tomato (days_to_plant=42): first eligible month is May/June.
    // The first week where tomato could be transplanted is around 2025-05-05.
    // Therefore tomato should appear in sowingTasks for the week starting ~2025-03-24
    // (42 days before 2025-05-05).
    let payload = serde_json::json!({
        "period": {"start": "2025-03-03", "end": "2025-06-29"},
        "region": "Temperate",
        "layout": null_layout(5, 5)
    });
    let req = test::TestRequest::post()
        .uri("/api/plan")
        .set_json(&payload)
        .to_request();
    let body: serde_json::Value = test::call_and_read_body_json(&app, req).await;

    // At least one week must have a non-empty sowingTasks list.
    let has_sowing_tasks = body["payload"]["weeks"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .any(|w| {
            w["sowingTasks"]
                .as_array()
                .map(|a| !a.is_empty())
                .unwrap_or(false)
        });

    assert!(
        has_sowing_tasks,
        "At least one week must contain sowing tasks for the planning period 2025-03-03 → 2025-06-29"
    );

    // Every sowingTask entry must have id, name, and targetWeekStart fields.
    let all_valid = body["payload"]["weeks"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .flat_map(|w| w["sowingTasks"].as_array().unwrap_or(&vec![]).to_owned())
        .all(|task| {
            task["id"].is_string()
                && task["name"].is_string()
                && task["targetWeekStart"].is_string()
        });

    assert!(
        all_valid,
        "Every sowingTask must have id, name, and targetWeekStart"
    );
}
