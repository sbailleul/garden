use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{
    CompanionsApiResponse, ErrorResponse, VarietyApiResponse,
};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse, IntoHttpPagination, PaginatedResponse, PaginationParams},
        localization::parse_locale,
    },
    application::{
        ports::variety_response_repository::{VarietyResponse, VarietyResponseRepository},
        use_cases::varieties::{GetVarietyUseCase, ListVarietiesUseCase},
    },
};

/// GET /api/varieties
/// Returns all varieties from the database.
#[utoipa::path(
    get,
    path = "/api/varieties",
    tag = "varieties",
    params(
        ("page" = Option<usize>, Query, description = "Page number (1-based, default: 1)."),
        ("size" = Option<usize>, Query, description = "Items per page (default: 20)."),
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
    query: web::Query<PaginationParams>,
    repo: web::Data<Box<dyn VarietyResponseRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let page = query.page.max(1);
    let size = query.size.max(1);
    match ListVarietiesUseCase::new(repo.as_ref().as_ref())
        .execute(&locale, page, size)
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
