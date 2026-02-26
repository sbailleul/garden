use actix_web::{get, http::Method, web, HttpResponse, Responder};

use crate::{
    data::vegetables::{get_all_vegetables, get_vegetable_by_id},
    models::request::{
        link, ApiResponse, CompanionInfo, CompanionsResponse,
        Pagination, PaginatedResponse, VegetableResponse,
    },
};

/// GET /api/vegetables
/// Returns all vegetables from the in-memory database.
#[get("/vegetables")]
pub async fn list_vegetables() -> impl Responder {
    let vegetables = get_all_vegetables();
    let total = vegetables.len();
    let items: Vec<ApiResponse<VegetableResponse>> = vegetables
        .into_iter()
        .map(|v| {
            let id = v.id.clone();
            let mut links = std::collections::HashMap::new();
            links.insert("self".into(), link(format!("/api/vegetables/{id}"), Method::GET));
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
        Pagination { page: 1, per_page: total, total, total_pages: 1 },
    ))
}

/// GET /api/vegetables/{id}
/// Returns a single vegetable by id.
#[get("/vegetables/{id}")]
pub async fn get_vegetable(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match get_vegetable_by_id(&id) {
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Vegetable '{}' not found.", id)
        })),
        Some(vegetable) => {
            let mut links = std::collections::HashMap::new();
            links.insert("self".into(), link(format!("/api/vegetables/{id}"), Method::GET));
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
                CompanionsResponse { id: vegetable.id, name: vegetable.name, good, bad },
                links,
            ))
        }
    }
}
