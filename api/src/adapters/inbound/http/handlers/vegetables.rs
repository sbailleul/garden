use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::domain::models::hateoas::{CompanionsApiResponse, VegetableApiResponse};
#[allow(unused_imports)]
use crate::domain::models::response::ErrorResponse;

use crate::{
    application::{
        ports::vegetable_repository::VegetableRepository,
        use_cases::vegetables::{GetCompanionsUseCase, GetVegetableUseCase, ListVegetablesUseCase},
    },
    domain::models::{
        hateoas::{link, ApiResponse, PaginatedResponse, Pagination},
        response::{CompanionsResponse, VegetableResponse},
    },
};

/// Extract the primary language tag from the `Accept-Language` header.
/// E.g. `"fr-FR,fr;q=0.9,en;q=0.8"` → `"fr"`. Falls back to `"en"`.
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

/// GET /api/vegetables
/// Returns all vegetables from the database.
#[utoipa::path(
    get,
    path = "/api/vegetables",
    tag = "vegetables",
    responses(
        (status = 200, description = "Paginated list of all vegetables",
         body = VegetablesApiResponse),
    )
)]
#[get("/vegetables")]
pub async fn list_vegetables(
    req: HttpRequest,
    repo: web::Data<Box<dyn VegetableRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    match ListVegetablesUseCase::new(repo.as_ref().as_ref())
        .execute(&locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch vegetables: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(vegetables) => {
            let total = vegetables.len();
            let items: Vec<ApiResponse<VegetableResponse>> = vegetables
                .into_iter()
                .map(|v| {
                    let id = v.id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/vegetables/{id}"), Method::GET),
                    );
                    links.insert(
                        "companions".into(),
                        link(format!("/api/vegetables/{id}/companions"), Method::GET),
                    );
                    ApiResponse::new(VegetableResponse { vegetable: v }, links)
                })
                .collect();
            let mut collection_links = std::collections::HashMap::new();
            collection_links.insert("self".into(), link("/api/vegetables", Method::GET));
            HttpResponse::Ok().json(PaginatedResponse::new(
                items,
                collection_links,
                Pagination {
                    page: 1,
                    per_page: total,
                    total,
                    total_pages: 1,
                },
            ))
        }
    }
}

/// GET /api/vegetables/{id}
/// Returns a single vegetable by id.
#[utoipa::path(
    get,
    path = "/api/vegetables/{id}",
    tag = "vegetables",
    params(
        ("id" = String, Path, description = "Vegetable identifier (e.g. `tomato`, `basil`)")
    ),
    responses(
        (status = 200, description = "Vegetable found", body = VegetableApiResponse),
        (status = 404, description = "Vegetable not found",  body = ErrorResponse),
    )
)]
#[get("/vegetables/{id}")]
pub async fn get_vegetable(
    req: HttpRequest,
    path: web::Path<String>,
    repo: web::Data<Box<dyn VegetableRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();
    match GetVegetableUseCase::new(repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch vegetable '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Vegetable '{}' not found.", id)
        })),
        Ok(Some(vegetable)) => {
            let mut links = std::collections::HashMap::new();
            links.insert(
                "self".into(),
                link(format!("/api/vegetables/{id}"), Method::GET),
            );
            links.insert(
                "companions".into(),
                link(format!("/api/vegetables/{id}/companions"), Method::GET),
            );
            links.insert("collection".into(), link("/api/vegetables", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(VegetableResponse { vegetable }, links))
        }
    }
}

/// GET /api/vegetables/{id}/companions
/// Returns good and bad companions for a given vegetable.
#[utoipa::path(
    get,
    path = "/api/vegetables/{id}/companions",
    tag = "vegetables",
    params(
        ("id" = String, Path, description = "Vegetable identifier (e.g. `tomato`)")
    ),
    responses(
        (status = 200, description = "Companion planting info", body = CompanionsApiResponse),
        (status = 404, description = "Vegetable not found",     body = ErrorResponse),
    )
)]
#[get("/vegetables/{id}/companions")]
pub async fn get_companions(
    req: HttpRequest,
    path: web::Path<String>,
    repo: web::Data<Box<dyn VegetableRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();
    match GetCompanionsUseCase::new(repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch companions for '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Vegetable '{}' not found.", id)
        })),
        Ok(Some(data)) => {
            let mut links = std::collections::HashMap::new();
            links.insert(
                "self".into(),
                link(format!("/api/vegetables/{id}/companions"), Method::GET),
            );
            links.insert(
                "vegetable".into(),
                link(format!("/api/vegetables/{id}"), Method::GET),
            );
            HttpResponse::Ok().json(ApiResponse::new(
                CompanionsResponse {
                    id: data.vegetable.id,
                    name: data.vegetable.name,
                    good: data.good,
                    bad: data.bad,
                },
                links,
            ))
        }
    }
}
