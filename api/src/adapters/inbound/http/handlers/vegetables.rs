use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{
    ErrorResponse, VarietiesApiResponse, VegetableApiResponse, VegetablesApiResponse,
};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse, PaginatedResponse, Pagination},
        localization::parse_locale,
    },
    application::{
        ports::{variety_repository::VarietyRepository, vegetable_repository::VegetableRepository},
        use_cases::{
            varieties::ListVarietiesByVegetableUseCase,
            vegetables::{GetVegetableUseCase, ListVegetablesUseCase},
        },
    },
    domain::models::{variety::Variety, vegetable::Vegetable},
};

/// GET /api/vegetables
/// Returns all vegetables from the database.
#[utoipa::path(
    get,
    path = "/api/vegetables",
    tag = "vegetables",
    params(
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
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
            let items: Vec<ApiResponse<Vegetable>> = vegetables
                .into_iter()
                .map(|v| {
                    let id = v.id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/vegetables/{id}"), Method::GET),
                    );
                    ApiResponse::new(v, links)
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
        ("id" = String, Path, description = "Vegetable identifier (e.g. `tomato`, `brassica`)"),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Vegetable found", body = VegetableApiResponse),
        (status = 404, description = "Vegetable not found", body = ErrorResponse),
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
            links.insert("collection".into(), link("/api/vegetables", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(vegetable, links))
        }
    }
}

/// GET /api/vegetables/{id}/varieties
/// Returns all varieties that belong to the given vegetable.
#[utoipa::path(
    get,
    path = "/api/vegetables/{id}/varieties",
    tag = "vegetables",
    params(
        ("id" = String, Path, description = "Vegetable identifier (e.g. `pepper`, `brassica`)"),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Paginated list of varieties for this vegetable", body = VarietiesApiResponse),
        (status = 404, description = "Vegetable not found", body = ErrorResponse),
    )
)]
#[get("/vegetables/{id}/varieties")]
pub async fn get_varieties_by_vegetable(
    req: HttpRequest,
    path: web::Path<String>,
    vegetable_repo: web::Data<Box<dyn VegetableRepository>>,
    variety_repo: web::Data<Box<dyn VarietyRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();

    // 404 if the vegetable doesn't exist
    match GetVegetableUseCase::new(vegetable_repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch vegetable '{id}': {e}");
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }));
        }
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Vegetable '{}' not found.", id)
            }));
        }
        Ok(Some(_)) => {}
    }

    match ListVarietiesByVegetableUseCase::new(variety_repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch varieties for vegetable '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(varieties) => {
            let total = varieties.len();
            let items: Vec<ApiResponse<Variety>> = varieties
                .into_iter()
                .map(|v| {
                    let vid = v.id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/varieties/{vid}"), Method::GET),
                    );
                    links.insert(
                        "companions".into(),
                        link(format!("/api/varieties/{vid}/companions"), Method::GET),
                    );
                    ApiResponse::new(v, links)
                })
                .collect();
            let mut collection_links = std::collections::HashMap::new();
            collection_links.insert(
                "self".into(),
                link(format!("/api/vegetables/{id}/varieties"), Method::GET),
            );
            collection_links.insert(
                "vegetable".into(),
                link(format!("/api/vegetables/{id}"), Method::GET),
            );
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
