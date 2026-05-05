use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{
    CompanionsApiResponse, ErrorResponse, VarietiesApiResponse, VegetableApiResponse,
    VegetablesApiResponse,
};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse, IntoHttpPagination, PaginatedResponse, PaginationParams},
        localization::parse_locale,
    },
    application::{
        ports::{
            variety_response_repository::{
                VarietyListFilter, VarietyResponse, VarietyResponseRepository,
            },
            vegetable_repository::VegetableRepository,
        },
        use_cases::{
            varieties::ListVarietiesByVegetableUseCase,
            vegetables::{
                GetVegetableCompanionsUseCase, GetVegetableUseCase, ListVegetablesUseCase,
            },
        },
    },
    domain::models::{
        response::CompanionsResponse,
        variety::{Category, Lifecycle, Region, SoilType, SunExposure},
        vegetable::Vegetable,
    },
};

fn default_page() -> usize {
    1
}
fn default_size() -> usize {
    20
}

/// Combined pagination + filter query parameters for the
/// `GET /api/vegetables/{id}/varieties` endpoint.
#[derive(Debug, serde::Deserialize)]
pub struct VarietyByVegetableQueryParams {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_size")]
    pub size: usize,
    pub category: Option<Category>,
    pub lifecycle: Option<Lifecycle>,
    pub beginner_friendly: Option<bool>,
    pub sun_requirement: Option<SunExposure>,
    pub soil_type: Option<SoilType>,
    pub region: Option<Region>,
    pub search: Option<String>,
}

impl VarietyByVegetableQueryParams {
    fn into_filter(self) -> VarietyListFilter {
        VarietyListFilter {
            category: self.category,
            lifecycle: self.lifecycle,
            beginner_friendly: self.beginner_friendly,
            sun_requirement: self.sun_requirement,
            soil_type: self.soil_type,
            region: self.region,
            vegetable_id: None,
            search: self.search,
        }
    }
}

/// GET /api/vegetables
/// Returns all vegetables from the database.
#[utoipa::path(
    get,
    path = "/api/vegetables",
    tag = "vegetables",
    params(
        ("page" = Option<usize>, Query, description = "Page number (1-based, default: 1)."),
        ("size" = Option<usize>, Query, description = "Items per page (default: 20)."),
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
    query: web::Query<PaginationParams>,
    repo: web::Data<Box<dyn VegetableRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let page = query.page.max(1);
    let size = query.size.max(1);
    match ListVegetablesUseCase::new(repo.as_ref().as_ref())
        .execute(&locale, page, size)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch vegetables: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(result) => {
            let pagination = result.to_pagination(page, size);
            let items: Vec<ApiResponse<Vegetable>> = result
                .items
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
                    ApiResponse::new(v, links)
                })
                .collect();
            let mut collection_links = std::collections::HashMap::new();
            collection_links.insert("self".into(), link("/api/vegetables", Method::GET));
            HttpResponse::Ok().json(PaginatedResponse::new(items, collection_links, pagination))
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
            links.insert(
                "companions".into(),
                link(format!("/api/vegetables/{id}/companions"), Method::GET),
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
        ("page" = Option<usize>, Query, description = "Page number (1-based, default: 1)."),
        ("size" = Option<usize>, Query, description = "Items per page (default: 20)."),
        ("category" = Option<String>, Query, description = "Filter by category (e.g. `Fruit`, `Herb`)."),
        ("lifecycle" = Option<String>, Query, description = "Filter by lifecycle (`Annual`, `Biennial`, `Perennial`)."),
        ("beginner_friendly" = Option<bool>, Query, description = "Filter to beginner-friendly varieties only."),
        ("sun_requirement" = Option<String>, Query, description = "Filter by sun exposure (`FullSun`, `PartialShade`, `Shade`)."),
        ("soil_type" = Option<String>, Query, description = "Filter by soil type (`Clay`, `Sandy`, `Loamy`, `Chalky`, `Humus`)."),
        ("region" = Option<String>, Query, description = "Filter by region (`Temperate`, `Mediterranean`, `Oceanic`, `Continental`, `Mountain`)."),
        ("search" = Option<String>, Query, description = "Case-insensitive substring search on the translated variety name."),
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
    query: web::Query<VarietyByVegetableQueryParams>,
    vegetable_repo: web::Data<Box<dyn VegetableRepository>>,
    variety_repo: web::Data<Box<dyn VarietyResponseRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();
    let page = query.page.max(1);
    let size = query.size.max(1);
    let filter = query.into_inner().into_filter();

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
        .execute(&id, &locale, page, size, &filter)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch varieties for vegetable '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(result) => {
            let pagination = result.to_pagination(page, size);
            let items: Vec<ApiResponse<VarietyResponse>> = result
                .items
                .into_iter()
                .map(|v| {
                    let vid = v.id.clone();
                    let vegetable_id = v.vegetable_id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/varieties/{vid}"), Method::GET),
                    );
                    links.insert(
                        "companions".into(),
                        link(
                            format!("/api/vegetables/{vegetable_id}/companions"),
                            Method::GET,
                        ),
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
            HttpResponse::Ok().json(PaginatedResponse::new(items, collection_links, pagination))
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
        ("id" = String, Path, description = "Vegetable identifier (e.g. `tomato`, `brassica`)"),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Companion planting info", body = CompanionsApiResponse),
        (status = 404, description = "Vegetable not found",    body = ErrorResponse),
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
    match GetVegetableCompanionsUseCase::new(repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch companions for vegetable '{id}': {e}");
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
