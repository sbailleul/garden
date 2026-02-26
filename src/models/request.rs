use std::collections::HashMap;

use actix_web::http::Method;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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

/// A single cell in the request layout grid.
/// - `null` JSON → `Free` — plantable, empty
/// - `"tomato"` JSON → `Planted` — pre-placed vegetable
/// - `true` JSON → `Blocked` — non-plantable zone; `false` is treated as free
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum LayoutCell {
    Planted(String),
    Blocked(bool),
    Free(()),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Level {
    Beginner,
    Expert,
}

/// A single preference entry with an optional desired cell count.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceEntry {
    pub id: String,
    /// Desired number of grid cells for this vegetable (defaults to 1 when omitted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    pub season: Season,
    pub sun: Option<SunExposure>,
    pub soil: Option<SoilType>,
    pub region: Option<Region>,
    pub level: Option<Level>,
    /// Preferred vegetables with optional per-vegetable cell count.
    pub preferences: Option<Vec<PreferenceEntry>>,
    /// Combined grid layout — defines dimensions and pre-filled cells.
    /// Each cell is: `null` (free), `"id"` (pre-planted), or `true` (blocked).
    pub layout: Matrix<LayoutCell>,
}

#[skip_serializing_none]
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
