use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{
    CompanionsApiResponse, ErrorResponse, VarietyApiResponse,
};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse, IntoHttpPagination, PaginatedResponse},
        localization::parse_locale,
    },
    application::{
        ports::variety_response_repository::{
            VarietyListFilter, VarietyResponse, VarietyResponseRepository,
        },
        use_cases::varieties::{GetVarietyUseCase, ListVarietiesUseCase},
    },
    domain::models::variety::{Category, Lifecycle, Region, SoilType, SunExposure},
};

fn default_page() -> usize {
    1
}
fn default_size() -> usize {
    20
}

/// Combined pagination + filter query parameters for variety listing endpoints.
#[derive(Debug, serde::Deserialize)]
pub struct VarietyQueryParams {
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
    pub vegetable_id: Option<String>,
    pub search: Option<String>,
}

impl VarietyQueryParams {
    fn into_filter(self) -> VarietyListFilter {
        VarietyListFilter {
            category: self.category,
            lifecycle: self.lifecycle,
            beginner_friendly: self.beginner_friendly,
            sun_requirement: self.sun_requirement,
            soil_type: self.soil_type,
            region: self.region,
            vegetable_id: self.vegetable_id,
            search: self.search,
        }
    }
}

/// GET /api/varieties
/// Returns all varieties from the database.
#[utoipa::path(
    get,
    path = "/api/varieties",
    tag = "varieties",
    params(
        ("page" = Option<usize>, Query, description = "Page number (1-based, default: 1)."),
        ("size" = Option<usize>, Query, description = "Items per page (default: 20)."),
        ("category" = Option<String>, Query, description = "Filter by category (e.g. `Fruit`, `Herb`)."),
        ("lifecycle" = Option<String>, Query, description = "Filter by lifecycle (`Annual`, `Biennial`, `Perennial`)."),
        ("beginner_friendly" = Option<bool>, Query, description = "Filter to beginner-friendly varieties only."),
        ("sun_requirement" = Option<String>, Query, description = "Filter by sun exposure (`FullSun`, `PartialShade`, `Shade`)."),
        ("soil_type" = Option<String>, Query, description = "Filter by soil type (`Clay`, `Sandy`, `Loamy`, `Chalky`, `Humus`)."),
        ("region" = Option<String>, Query, description = "Filter by region (`Temperate`, `Mediterranean`, `Oceanic`, `Continental`, `Mountain`)."),
        ("vegetable_id" = Option<String>, Query, description = "Filter by parent vegetable identifier."),
        ("search" = Option<String>, Query, description = "Case-insensitive substring search on the translated variety name."),
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
    query: web::Query<VarietyQueryParams>,
    repo: web::Data<Box<dyn VarietyResponseRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let page = query.page.max(1);
    let size = query.size.max(1);
    let filter = query.into_inner().into_filter();
    match ListVarietiesUseCase::new(repo.as_ref().as_ref())
        .execute(&locale, page, size, &filter)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch varieties: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(result) => {
            let pagination = result.to_pagination(page, size);
            let items: Vec<ApiResponse<VarietyResponse>> = result
                .items
                .into_iter()
                .map(|v| {
                    let id = v.id.clone();
                    let vegetable_id = v.vegetable_id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/varieties/{id}"), Method::GET),
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
            collection_links.insert("self".into(), link("/api/varieties", Method::GET));
            HttpResponse::Ok().json(PaginatedResponse::new(items, collection_links, pagination))
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
    repo: web::Data<Box<dyn VarietyResponseRepository>>,
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
            let vegetable_id = variety.vegetable_id.clone();
            let mut links = std::collections::HashMap::new();
            links.insert(
                "self".into(),
                link(format!("/api/varieties/{id}"), Method::GET),
            );
            links.insert(
                "companions".into(),
                link(
                    format!("/api/vegetables/{vegetable_id}/companions"),
                    Method::GET,
                ),
            );
            links.insert("collection".into(), link("/api/varieties", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(variety, links))
        }
    }
}
