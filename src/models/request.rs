use std::collections::HashMap;

use actix_web::http::Method;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{
    vegetable::{Region, Season, SoilType, SunExposure, Vegetable},
    Coordinate, Matrix,
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Link {
    pub href: String,
    /// HTTP method to use for this link (e.g. `GET`, `POST`).
    #[schema(value_type = String, example = "GET")]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub total_pages: usize,
}

/// Generic single-item response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[aliases(
    VegetableApiResponse   = ApiResponse<VegetableResponse>,
    PlanApiResponse        = ApiResponse<PlanResponse>,
    CompanionsApiResponse  = ApiResponse<CompanionsResponse>
)]
pub struct ApiResponse<T> {
    pub payload: T,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    /// HAL-style hypermedia links.
    #[schema(value_type = HashMap<String, Link>)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub payload: Vec<T>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    /// HAL-style hypermedia links.
    #[schema(value_type = HashMap<String, Link>)]
    #[serde(rename = "_links")]
    pub links: Links,
    pub pagination: Pagination,
}

/// OpenAPI schema for the paginated vegetables list.
/// Uses the concrete alias `VegetableApiResponse` so utoipa emits a
/// resolvable `$ref` for each item instead of the bare generic `ApiResponse`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VegetableListResponse {
    pub payload: Vec<VegetableApiResponse>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
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
            errors: vec![],
            links,
            pagination,
        }
    }
}

/// Vegetable domain struct for use in responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VegetableResponse {
    #[serde(flatten)]
    pub vegetable: Vegetable,
}

/// A single cell in the **request** layout grid.
/// Uses the same `{"type":...}` tag as `PlannedCell` but only carries the data
/// relevant for input: `id` for pre-planted cells, nothing for `empty`/`blocked`.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LayoutCell {
    /// A pre-planted cell that fits in one 30 cm × 30 cm grid cell.
    SelfContained {
        id: String,
        /// Number of plants per cell. Computed from the vegetable's spacing if absent.
        plants_per_cell: Option<u32>,
    },
    /// The top-left (anchor) cell of a pre-planted multi-cell block.
    Overflowing {
        id: String,
        /// Number of plants per cell. Computed from the vegetable's spacing if absent.
        plants_per_cell: Option<u32>,
        /// Block width in grid cells. Computed from the vegetable's spacing if absent.
        width_cells: Option<u32>,
        /// Block length in grid cells. Computed from the vegetable's spacing if absent.
        length_cells: Option<u32>,
    },
    /// A continuation cell of a multi-cell block (skipped — anchor handles placement).
    Overflowed { covered_by: Coordinate },
    /// Free, unoccupied, non-blocked cell.
    Empty,
    /// Non-plantable zone (path, alley, obstacle).
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Level {
    Beginner,
    Expert,
}

/// A single preference entry with an optional desired plant count.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceEntry {
    pub id: String,
    /// Desired number of **plants** (placements) for this vegetable.
    /// Each plant may occupy more than one cell depending on its spacing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    pub season: Season,
    pub sun: Option<SunExposure>,
    pub soil: Option<SoilType>,
    pub region: Option<Region>,
    pub level: Option<Level>,
    /// Preferred vegetables with optional per-vegetable plant count.
    pub preferences: Option<Vec<PreferenceEntry>>,
    /// Combined grid layout — defines dimensions and pre-filled cells.
    /// Each cell is a `LayoutCell` object: `{"type":"empty"}` (free),
    /// `{"type":"selfContained","id":"..."}` (pre-planted), or `{"type":"blocked"}` (blocked).
    pub layout: Matrix<LayoutCell>,
}

/// A cell in the planned garden grid (response output).
///
/// Three occupied variants, plus `Empty` and `Blocked`:
/// - `selfContained` — a plant whose spacing ≤ 30 cm; fits entirely in one cell.
/// - `overflowing`   — the anchor (top-left) cell of a plant that spans multiple cells.
/// - `overflowed`    — a continuation cell covered by a neighbouring anchor; carries only a
///   back-reference so clients can look up the full data from the anchor.
/// - `empty`         — free, unoccupied, non-blocked cell.
/// - `blocked`       — non-plantable zone (path, alley, obstacle).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PlannedCell {
    /// A plant that fits entirely within one 30 cm × 30 cm cell.
    SelfContained {
        id: String,
        name: String,
        reason: String,
        plants_per_cell: u32,
    },
    /// The anchor (top-left) cell of a plant that overflows into neighbouring cells.
    Overflowing {
        id: String,
        name: String,
        reason: String,
        plants_per_cell: u32,
        width_cells: u32,
        length_cells: u32,
    },
    /// A continuation cell covered by a multi-cell plant's anchor.
    /// All plant data lives on the anchor cell; this cell only holds a back-reference.
    Overflowed { covered_by: Coordinate },
    /// A free, unoccupied, non-blocked cell.
    Empty,
    /// A non-plantable zone (path, alley, obstacle).
    Blocked,
}

impl PlannedCell {
    /// Returns the vegetable id if this cell is an anchor (`SelfContained` or `Overflowing`).
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::SelfContained { id, .. } | Self::Overflowing { id, .. } => Some(id),
            _ => None,
        }
    }

    /// Returns `true` if this cell carries or is part of a plant placement.
    pub fn is_placed(&self) -> bool {
        !matches!(self, Self::Empty | Self::Blocked)
    }

    /// Returns `true` if this cell is a non-plantable zone.
    pub fn is_blocked(&self) -> bool {
        matches!(self, Self::Blocked)
    }

    /// Returns the `coveredBy` reference for `Overflowed` cells, `None` otherwise.
    pub fn covered_by(&self) -> Option<&Coordinate> {
        match self {
            Self::Overflowed { covered_by } => Some(covered_by),
            _ => None,
        }
    }

    /// Returns `widthCells` for `Overflowing` anchor cells, `None` otherwise.
    pub fn width_cells(&self) -> Option<u32> {
        match self {
            Self::Overflowing { width_cells, .. } => Some(*width_cells),
            _ => None,
        }
    }

    /// Returns `lengthCells` for `Overflowing` anchor cells, `None` otherwise.
    pub fn length_cells(&self) -> Option<u32> {
        match self {
            Self::Overflowing { length_cells, .. } => Some(*length_cells),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlanResponse {
    pub grid: Matrix<PlannedCell>,
    pub rows: usize,
    pub cols: usize,
    pub score: i32,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompanionsResponse {
    pub id: String,
    pub name: String,
    pub good: Vec<CompanionInfo>,
    pub bad: Vec<CompanionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompanionInfo {
    pub id: String,
    pub name: String,
}

/// Error response returned for 4xx responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}
