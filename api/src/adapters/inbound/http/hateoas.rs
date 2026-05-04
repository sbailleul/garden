use std::collections::HashMap;

use actix_web::http::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::application::ports::Page;

use crate::adapters::inbound::http::dto::VarietyResponse;
use crate::domain::models::{
    group::Group,
    response::{CompanionsResponse, PlanResponse},
    vegetable::Vegetable,
};

/// Error response returned for 4xx responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

/// Serde adapter for `actix_web::http::Method` (serialises as its uppercase string).
mod method_serde {
    use actix_web::http::Method;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(method: &Method, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(method.as_str())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Method, D::Error> {
        let s = String::deserialize(d)?;
        Method::from_bytes(s.as_bytes()).map_err(serde::de::Error::custom)
    }
}

/// A single HAL-style hyperlink.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Link {
    pub href: String,
    /// HTTP method to use for this link (e.g. `GET`, `POST`).
    #[schema(value_type = String, example = "GET")]
    #[serde(with = "method_serde")]
    pub method: Method,
}

/// Map of relation name -> link, serialised as the `_links` field in responses.
pub type Links = HashMap<String, Link>;

/// Helper to build a `Link` from an href and an HTTP method.
pub fn link(href: impl Into<String>, method: Method) -> Link {
    Link {
        href: href.into(),
        method,
    }
}

/// Pagination metadata included in responses that return lists.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
}

/// Query parameters accepted by paginated list endpoints.
#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PaginationParams {
    /// Page number (1-based). Defaults to `1`.
    #[serde(default = "PaginationParams::default_page")]
    pub page: usize,
    /// Number of items per page. Defaults to `20`.
    #[serde(default = "PaginationParams::default_size")]
    pub size: usize,
}

impl PaginationParams {
    fn default_page() -> usize {
        1
    }
    fn default_size() -> usize {
        20
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Self::default_page(),
            size: Self::default_size(),
        }
    }
}

/// Generic single-item response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[aliases(
    VarietyApiResponse   = ApiResponse<VarietyResponse>,
    VegetableApiResponse     = ApiResponse<Vegetable>,
    PlanApiResponse        = ApiResponse<PlanResponse>,
    CompanionsApiResponse  = ApiResponse<CompanionsResponse>,
    GroupApiResponse       = ApiResponse<Group>
)]
pub struct ApiResponse<T> {
    pub payload: T,
    /// HAL-style hypermedia links.
    #[schema(value_type = HashMap<String, Link>)]
    #[serde(rename = "_links")]
    pub links: Links,
}

impl<T> ApiResponse<T> {
    pub fn new(payload: T, links: Links) -> Self {
        Self { payload, links }
    }
}

/// Generic paginated list response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[aliases(
    VarietiesApiResponse   = PaginatedResponse<VarietyApiResponse>,
    VegetablesApiResponse    = PaginatedResponse<VegetableApiResponse>,
    GroupsApiResponse        = PaginatedResponse<GroupApiResponse>,
)]
pub struct PaginatedResponse<T> {
    pub payload: Vec<T>,
    /// HAL-style hypermedia links.
    #[schema(value_type = HashMap<String, Link>)]
    #[serde(rename = "_links")]
    pub links: Links,
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    pub fn new(payload: Vec<T>, links: Links, pagination: Pagination) -> Self {
        Self {
            payload,
            links,
            pagination,
        }
    }
}

/// Extension trait that converts a `Page<T>` (application layer) into an HTTP
/// `Pagination` metadata block, so handlers do not have to repeat the
/// `total_pages` calculation.
pub trait IntoHttpPagination {
    fn to_pagination(&self, page: usize, size: usize) -> Pagination;
}

impl<T> IntoHttpPagination for Page<T> {
    fn to_pagination(&self, page: usize, size: usize) -> Pagination {
        let total = self.total;
        let total_pages = total.div_ceil(size).max(1);
        Pagination {
            page,
            per_page: size,
            total,
            total_pages,
        }
    }
}
