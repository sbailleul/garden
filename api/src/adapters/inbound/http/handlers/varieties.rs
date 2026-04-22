use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{
    CompanionsApiResponse, ErrorResponse, VarietyApiResponse,
};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse, PaginatedResponse, Pagination},
        localization::parse_locale,
    },
    application::{
        ports::variety_repository::VarietyRepository,
        use_cases::varieties::{GetCompanionsUseCase, GetVarietyUseCase, ListVarietiesUseCase},
    },
    domain::models::{response::CompanionsResponse, variety::Variety},
};

/// GET /api/varieties
/// Returns all varieties from the database.
#[utoipa::path(
    get,
    path = "/api/varieties",
    tag = "varieties",
    params(
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Paginated list of all varieties",
         body = VarietiesApiResponse),
    )
)]
#[get("/varieties")]
pub async fn list_varieties(
    req: HttpRequest,
    repo: web::Data<Box<dyn VarietyRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    match ListVarietiesUseCase::new(repo.as_ref().as_ref())
        .execute(&locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch varieties: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(varieties) => {
            let total = varieties.len();
            let items: Vec<ApiResponse<Variety>> = varieties
                .into_iter()
                .map(|v| {
                    let id = v.id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/varieties/{id}"), Method::GET),
                    );
                    links.insert(
                        "companions".into(),
                        link(format!("/api/varieties/{id}/companions"), Method::GET),
                    );
                    ApiResponse::new(v, links)
                })
                .collect();
            let mut collection_links = std::collections::HashMap::new();
            collection_links.insert("self".into(), link("/api/varieties", Method::GET));
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

/// GET /api/varieties/{id}
/// Returns a single variety by id.
#[utoipa::path(
    get,
    path = "/api/varieties/{id}",
    tag = "varieties",
    params(
        ("id" = String, Path, description = "Variety identifier (e.g. `tomato`, `basil`)"),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Variety found", body = VarietyApiResponse),
        (status = 404, description = "Variety not found",  body = ErrorResponse),
    )
)]
#[get("/varieties/{id}")]
pub async fn get_variety(
    req: HttpRequest,
    path: web::Path<String>,
    repo: web::Data<Box<dyn VarietyRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();
    match GetVarietyUseCase::new(repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch variety '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Variety '{}' not found.", id)
        })),
        Ok(Some(variety)) => {
            let mut links = std::collections::HashMap::new();
            links.insert(
                "self".into(),
                link(format!("/api/varieties/{id}"), Method::GET),
            );
            links.insert(
                "companions".into(),
                link(format!("/api/varieties/{id}/companions"), Method::GET),
            );
            links.insert("collection".into(), link("/api/varieties", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(variety, links))
        }
    }
}

/// GET /api/varieties/{id}/companions
/// Returns good and bad companions for a given variety.
#[utoipa::path(
    get,
    path = "/api/varieties/{id}/companions",
    tag = "varieties",
    params(
        ("id" = String, Path, description = "Variety identifier (e.g. `tomato`)"),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Companion planting info", body = CompanionsApiResponse),
        (status = 404, description = "Variety not found",     body = ErrorResponse),
    )
)]
#[get("/varieties/{id}/companions")]
pub async fn get_companions(
    req: HttpRequest,
    path: web::Path<String>,
    repo: web::Data<Box<dyn VarietyRepository>>,
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
            "error": format!("Variety '{}' not found.", id)
        })),
        Ok(Some(data)) => {
            let mut links = std::collections::HashMap::new();
            links.insert(
                "self".into(),
                link(format!("/api/varieties/{id}/companions"), Method::GET),
            );
            links.insert(
                "variety".into(),
                link(format!("/api/varieties/{id}"), Method::GET),
            );
            HttpResponse::Ok().json(ApiResponse::new(
                CompanionsResponse {
                    id: data.variety.id,
                    name: data.variety.name,
                    good: data.good,
                    bad: data.bad,
                },
                links,
            ))
        }
    }
}
