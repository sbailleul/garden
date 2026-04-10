use actix_web::{http::Method, post, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::domain::models::hateoas::PlanApiResponse;
#[allow(unused_imports)]
use crate::domain::models::response::ErrorResponse;

use crate::{
    application::{
        ports::vegetable_repository::VegetableRepository, use_cases::plan_garden::PlanGardenUseCase,
    },
    domain::models::{
        hateoas::{link, ApiResponse},
        request::PlanRequest,
    },
};

fn parse_locale(req: &HttpRequest) -> String {
    req.headers()
        .get("accept-language")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(',').next().map(|tag| {
                tag.split(';')
                    .next()
                    .unwrap_or(tag)
                    .split('-')
                    .next()
                    .unwrap_or(tag)
                    .trim()
                    .to_lowercase()
            })
        })
        .unwrap_or_else(|| "en".to_string())
}

/// POST /api/plan
/// Generates an optimised garden plan based on the provided constraints.
#[utoipa::path(
    post,
    path = "/api/plan",
    tag = "plan",
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
    repo: web::Data<Box<dyn VegetableRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let request = body.into_inner();
    let use_case = PlanGardenUseCase::new(repo.as_ref().as_ref());
    match use_case.execute(&request, &locale).await {
        Ok(response) => {
            let mut links = std::collections::HashMap::new();
            links.insert("self".into(), link("/api/plan", Method::POST));
            links.insert("vegetables".into(), link("/api/vegetables", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(response, links))
        }
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({ "error": e })),
    }
}
