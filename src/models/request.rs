use std::collections::HashMap;

use actix_web::http::Method;
use serde::{Deserialize, Serialize};

use crate::models::{
    vegetable::{Region, Season, SoilType, SunExposure, Vegetable},
    Matrix,
};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
    #[serde(with = "method_serde")]
    pub method: Method,
}

/// Map of relation name → link, serialised as the `_links` field in responses.
pub type Links = HashMap<String, Link>;

/// Helper to build a `Link` from an href and an HTTP method.
pub fn link(href: impl Into<String>, method: Method) -> Link {
    Link {
        href: href.into(),
        method,
    }
}

/// Pagination metadata included in responses that return lists.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
}

/// Generic single-item response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub payload: T,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[serde(rename = "_links")]
    pub links: Links,
}

impl<T> ApiResponse<T> {
    pub fn new(payload: T, links: Links) -> Self {
        Self {
            payload,
            errors: vec![],
            links,
        }
    }
}

/// Generic paginated list response envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub payload: Vec<T>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub pagination: Pagination,
}

impl<T> PaginatedResponse<T> {
    pub fn new(payload: Vec<T>, links: Links, pagination: Pagination) -> Self {
        Self {
            payload,
            errors: vec![],
            links,
            pagination,
        }
    }
}

/// Vegetable domain struct for use in responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegetableResponse {
    #[serde(flatten)]
    pub vegetable: Vegetable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Level {
    Beginner,
    Expert,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    /// Garden width in metres
    pub width_m: f32,
    /// Garden length in metres
    pub length_m: f32,
    pub season: Season,
    pub sun: Option<SunExposure>,
    pub soil: Option<SoilType>,
    pub region: Option<Region>,
    pub level: Option<Level>,
    /// Preferred vegetables (ids) chosen by the user
    pub preferences: Option<Vec<String>>,
    /// Existing layout: nullable string grid (vegetable ids or null)
    pub existing_layout: Option<Matrix<Option<String>>>,
    /// Blocked cells grid (paths, alleys, obstacles): true = not plantable
    pub blocked_cells: Option<Matrix<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedCell {
    pub id: Option<String>,
    pub name: Option<String>,
    pub reason: Option<String>,
    /// True when the cell is a non-plantable zone (path, alley, etc.).
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanResponse {
    pub grid: Matrix<PlannedCell>,
    pub rows: usize,
    pub cols: usize,
    pub score: i32,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanionsResponse {
    pub id: String,
    pub name: String,
    pub good: Vec<CompanionInfo>,
    pub bad: Vec<CompanionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanionInfo {
    pub id: String,
    pub name: String,
}
