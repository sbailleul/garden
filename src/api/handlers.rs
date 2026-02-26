use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{
    data::vegetables::{get_all_vegetables, get_vegetable_by_id},
    logic::{filter::filter_vegetables, planner::plan_garden},
    models::request::{CompanionInfo, CompanionsResponse, PlanRequest},
};

/// GET /api/vegetables
/// Returns all vegetables from the in-memory database.
#[get("/vegetables")]
pub async fn list_vegetables() -> impl Responder {
    let vegetables = get_all_vegetables();
    HttpResponse::Ok().json(vegetables)
}

/// GET /api/vegetables/{id}/companions
/// Returns good and bad companions for a given vegetable.
#[get("/vegetables/{id}/companions")]
pub async fn get_companions(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let all = get_all_vegetables();

    match get_vegetable_by_id(&id) {
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Vegetable '{}' not found.", id)
        })),
        Some(vegetable) => {
            let good: Vec<CompanionInfo> = vegetable
                .good_companions
                .iter()
                .filter_map(|cid| {
                    all.iter().find(|v| &v.id == cid).map(|v| CompanionInfo {
                        id: v.id.clone(),
                        name: v.name.clone(),
                    })
                })
                .collect();

            let bad: Vec<CompanionInfo> = vegetable
                .bad_companions
                .iter()
                .filter_map(|cid| {
                    all.iter().find(|v| &v.id == cid).map(|v| CompanionInfo {
                        id: v.id.clone(),
                        name: v.name.clone(),
                    })
                })
                .collect();

            HttpResponse::Ok().json(CompanionsResponse {
                id: vegetable.id,
                name: vegetable.name,
                good,
                bad,
            })
        }
    }
}

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
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}
