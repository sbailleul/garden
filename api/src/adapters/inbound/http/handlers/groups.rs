use actix_web::{get, http::Method, web, HttpRequest, HttpResponse, Responder};
// Types referenced only in #[utoipa::path] attributes — used at proc-macro expansion time.
#[allow(unused_imports)]
use crate::adapters::inbound::http::hateoas::{
    ErrorResponse, GroupApiResponse, GroupsApiResponse, VegetablesApiResponse,
};

use crate::{
    adapters::inbound::http::{
        hateoas::{link, ApiResponse, IntoHttpPagination, PaginatedResponse, PaginationParams},
        localization::parse_locale,
    },
    application::{
        ports::{group_repository::GroupRepository, vegetable_repository::VegetableRepository},
        use_cases::{
            groups::{GetGroupUseCase, ListGroupsUseCase},
            vegetables::ListVegetablesByGroupUseCase,
        },
    },
    domain::models::{group::Group, vegetable::Vegetable},
};

/// GET /api/groups
#[utoipa::path(
    get,
    path = "/api/groups",
    tag = "groups",
    params(
        ("page" = Option<usize>, Query, description = "Page number (1-based, default: 1)."),
        ("size" = Option<usize>, Query, description = "Items per page (default: 20)."),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Paginated list of all groups", body = GroupsApiResponse),
    )
)]
#[get("/groups")]
pub async fn list_groups(
    req: HttpRequest,
    query: web::Query<PaginationParams>,
    repo: web::Data<Box<dyn GroupRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let page = query.page.max(1);
    let size = query.size.max(1);
    match ListGroupsUseCase::new(repo.as_ref().as_ref())
        .execute(&locale, page, size)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch groups: {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(result) => {
            let pagination = result.to_pagination(page, size);
            let items: Vec<ApiResponse<Group>> = result
                .items
                .into_iter()
                .map(|g| {
                    let id = g.id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/groups/{id}"), Method::GET),
                    );
                    links.insert(
                        "vegetables".into(),
                        link(format!("/api/groups/{id}/vegetables"), Method::GET),
                    );
                    ApiResponse::new(g, links)
                })
                .collect();
            let mut collection_links = std::collections::HashMap::new();
            collection_links.insert("self".into(), link("/api/groups", Method::GET));
            HttpResponse::Ok().json(PaginatedResponse::new(items, collection_links, pagination))
        }
    }
}

/// GET /api/groups/{id}
#[utoipa::path(
    get,
    path = "/api/groups/{id}",
    tag = "groups",
    params(
        ("id" = String, Path, description = "Group identifier (e.g. `bulbes`, `legumes-fruits`)"),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Group found", body = GroupApiResponse),
        (status = 404, description = "Group not found", body = ErrorResponse),
    )
)]
#[get("/groups/{id}")]
pub async fn get_group(
    req: HttpRequest,
    path: web::Path<String>,
    repo: web::Data<Box<dyn GroupRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();
    match GetGroupUseCase::new(repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch group '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Group '{}' not found.", id)
        })),
        Ok(Some(group)) => {
            let mut links = std::collections::HashMap::new();
            links.insert(
                "self".into(),
                link(format!("/api/groups/{id}"), Method::GET),
            );
            links.insert(
                "vegetables".into(),
                link(format!("/api/groups/{id}/vegetables"), Method::GET),
            );
            links.insert("collection".into(), link("/api/groups", Method::GET));
            HttpResponse::Ok().json(ApiResponse::new(group, links))
        }
    }
}

/// GET /api/groups/{id}/vegetables
#[utoipa::path(
    get,
    path = "/api/groups/{id}/vegetables",
    tag = "groups",
    params(
        ("id" = String, Path, description = "Group identifier (e.g. `bulbes`, `legumes-fruits`)"),
        ("page" = Option<usize>, Query, description = "Page number (1-based, default: 1)."),
        ("size" = Option<usize>, Query, description = "Items per page (default: 20)."),
        ("Accept-Language" = Option<String>, Header, description = "BCP 47 language tag (e.g. `fr`, `en`). Falls back to `en`.")
    ),
    responses(
        (status = 200, description = "Paginated list of vegetables in this group", body = VegetablesApiResponse),
        (status = 404, description = "Group not found", body = ErrorResponse),
    )
)]
#[get("/groups/{id}/vegetables")]
pub async fn list_vegetables_by_group(
    req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<PaginationParams>,
    group_repo: web::Data<Box<dyn GroupRepository>>,
    vegetable_repo: web::Data<Box<dyn VegetableRepository>>,
) -> impl Responder {
    let locale = parse_locale(&req);
    let id = path.into_inner();
    let page = query.page.max(1);
    let size = query.size.max(1);

    // 404 if group doesn't exist
    match GetGroupUseCase::new(group_repo.as_ref().as_ref())
        .execute(&id, &locale)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch group '{id}': {e}");
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }));
        }
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": format!("Group '{}' not found.", id)
            }));
        }
        Ok(Some(_)) => {}
    }

    match ListVegetablesByGroupUseCase::new(vegetable_repo.as_ref().as_ref())
        .execute(&id, &locale, page, size)
        .await
    {
        Err(e) => {
            log::error!("Failed to fetch vegetables for group '{id}': {e}");
            HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "Internal server error" }))
        }
        Ok(result) => {
            let pagination = result.to_pagination(page, size);
            let items: Vec<ApiResponse<Vegetable>> = result
                .items
                .into_iter()
                .map(|v| {
                    let vid = v.id.clone();
                    let mut links = std::collections::HashMap::new();
                    links.insert(
                        "self".into(),
                        link(format!("/api/vegetables/{vid}"), Method::GET),
                    );
                    links.insert(
                        "companions".into(),
                        link(format!("/api/vegetables/{vid}/companions"), Method::GET),
                    );
                    ApiResponse::new(v, links)
                })
                .collect();
            let mut collection_links = std::collections::HashMap::new();
            collection_links.insert(
                "self".into(),
                link(format!("/api/groups/{id}/vegetables"), Method::GET),
            );
            collection_links.insert(
                "group".into(),
                link(format!("/api/groups/{id}"), Method::GET),
            );
            HttpResponse::Ok().json(PaginatedResponse::new(items, collection_links, pagination))
        }
    }
}
