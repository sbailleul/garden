use actix_web::{http::Method, post, web, HttpResponse, Responder};

use crate::{
    data::vegetables::get_all_vegetables,
    logic::{filter::filter_vegetables, planner::plan_garden},
    models::request::{link, ApiResponse, PlanRequest},
};

/// POST /api/plan
/// Generates an optimised garden plan based on the provided constraints.
#[post("/plan")]
pub async fn post_plan(body: web::Json<PlanRequest>) -> impl Responder {
    let request = body.into_inner();

    if request.width_m <= 0.0 || request.length_m <= 0.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Garden dimensions (width_m, length_m) must be strictly positive."
        }));
    }

    let db = get_all_vegetables();
    let candidates = filter_vegetables(&db, &request);

    match plan_garden(candidates, &request) {
        Ok(response) => {
            let mut links = std::collections::HashMap::new();
            links.insert("self".into(), link("/api/plan", Method::POST));
            links.insert("vegetables".into(), link("/api/vegetables", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(response, links))
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}
