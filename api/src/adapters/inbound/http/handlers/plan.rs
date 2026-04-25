use actix_web::{http::Method, post, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{ErrorResponse, PlanApiResponse};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse},
        localization::parse_locale,
    },
    application::{
        ports::variety_repository::VarietyRepository, use_cases::plan_garden::PlanGardenUseCase,
    },
    domain::models::request::PlanRequest,
};

/// POST /api/plan
/// Generates an optimised garden plan based on the provided constraints.
#[utoipa::path(
    post,
    path = "/api/plan",
    tag = "plan",
    params(
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    request_body(
        content = PlanRequest,
        description = "Planning constraints and grid layout",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Optimised garden plan",        body = PlanApiResponse),
        (status = 400, description = "Validation error or bad JSON", body = ErrorResponse),
    )
)]
#[post("/plan")]
pub async fn post_plan(
    req: HttpRequest,
    body: web::Json<PlanRequest>,
    repo: web::Data<Box<dyn VarietyRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let request = body.into_inner();
    let use_case = PlanGardenUseCase::new(repo.as_ref().as_ref());
    match use_case.execute(&request, &locale).await {
        Ok(response) => {
            let mut links = std::collections::HashMap::new();
            links.insert("self".into(), link("/api/plan", Method::POST));
            links.insert("varieties".into(), link("/api/varieties", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(response, links))
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}
