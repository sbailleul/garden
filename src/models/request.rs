use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::{
    vegetable::{Region, SoilType, SunExposure, Vegetable},
    Coordinate, Matrix,
};

/// Vegetable domain struct for use in responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VegetableResponse {
    #[serde(flatten)]
    pub vegetable: Vegetable,
}

/// A single cell in the **request** layout grid.
/// Uses the same `{"type":...}` tag as `PlannedCell` but only carries the data
/// relevant for input: `id` for pre-planted cells, nothing for `Empty`/`Blocked`.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum LayoutCell {
    /// A pre-planted cell that fits in one 30 cm × 30 cm grid cell.
    SelfContained {
        id: String,
        /// Number of plants per cell. Computed from the vegetable's spacing if absent.
        plants_per_cell: Option<u32>,
        /// Date when this plant was put in the ground (ISO 8601, e.g. `"2025-05-01"`).
        /// When provided, it is used to free the cell after harvest and compute
        /// `estimatedHarvestDate` in the response.
        #[schema(value_type = Option<String>, format = Date, example = "2025-05-01")]
        planted_date: Option<NaiveDate>,
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
        /// Date when this plant was put in the ground (ISO 8601, e.g. `"2025-05-01"`).
        /// When provided, it is used to free the cell after harvest and compute
        /// `estimatedHarvestDate` in the response.
        #[schema(value_type = Option<String>, format = Date, example = "2025-05-01")]
        planted_date: Option<NaiveDate>,
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

/// The date range of the planning period.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    /// First day of the planning period (ISO 8601, e.g. `"2025-06-01"`).
    #[schema(value_type = String, format = Date, example = "2025-06-01")]
    pub start: NaiveDate,
    /// Last day of the planning period (ISO 8601, e.g. `"2025-08-31"`).
    #[schema(value_type = String, format = Date, example = "2025-08-31")]
    pub end: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlanRequest {
    /// Planning period (start and end dates).
    /// When omitted, defaults to the current Monday-to-Sunday week.
    #[serde(default)]
    pub period: Option<Period>,
    pub sun: Option<SunExposure>,
    pub soil: Option<SoilType>,
    pub region: Region,
    pub level: Option<Level>,
    /// Preferred vegetables with optional per-vegetable plant count.
    pub preferences: Option<Vec<PreferenceEntry>>,
    /// Combined grid layout — defines dimensions and pre-filled cells.
    /// Each cell is a `LayoutCell` object: `{"type":"Empty"}` (free),
    /// `{"type":"SelfContained","id":"..."}` (pre-planted), or `{"type":"Blocked"}` (blocked).
    pub layout: Matrix<LayoutCell>,
}

/// A cell in the planned garden grid (response output).
///
/// Three occupied variants, plus `Empty` and `Blocked`:
/// - `SelfContained` — a plant whose spacing ≤ 30 cm; fits entirely in one cell.
/// - `Overflowing`   — the anchor (top-left) cell of a plant that spans multiple cells.
/// - `Overflowed`    — a continuation cell covered by a neighbouring anchor; carries only a
///   back-reference so clients can look up the full data from the anchor.
/// - `Empty`         — free, unoccupied, non-blocked cell.
/// - `Blocked`       — non-plantable zone (path, alley, obstacle).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum PlannedCell {
    /// A plant that fits entirely within one 30 cm × 30 cm cell.
    SelfContained {
        id: String,
        name: String,
        reason: String,
        plants_per_cell: u32,
        /// Estimated date the plant will be ready to harvest.
        #[serde(skip_serializing_if = "Option::is_none")]
        estimated_harvest_date: Option<NaiveDate>,
    },
    /// The anchor (top-left) cell of a plant that overflows into neighbouring cells.
    Overflowing {
        id: String,
        name: String,
        reason: String,
        plants_per_cell: u32,
        width_cells: u32,
        length_cells: u32,
        /// Estimated date the plant will be ready to harvest.
        #[serde(skip_serializing_if = "Option::is_none")]
        estimated_harvest_date: Option<NaiveDate>,
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

/// A snapshot of the garden layout for one week of the planning period.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyPlan {
    /// The Monday-to-Sunday date range this snapshot covers.
    pub period: Period,

    pub week_count: u16,
    /// Full garden grid for this week (same dimensions as the request layout).
    pub grid: Matrix<PlannedCell>,
    /// Cumulative companion-planting score for plants placed **this week**.
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlanResponse {
    pub rows: usize,
    pub cols: usize,
    /// One entry per week in the requested planning period.
    pub weeks: Vec<WeeklyPlan>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_cell_selfcontained_planted_date_deserializes_from_camel_case() {
        let json = r#"{"type": "SelfContained","id":"tomato","plantedDate":"2025-05-01"}"#;
        let cell: LayoutCell = serde_json::from_str(json).expect("should deserialize");

        match cell {
            LayoutCell::SelfContained {
                id, planted_date, ..
            } => {
                assert_eq!(id, "tomato");
                assert_eq!(
                    planted_date,
                    Some(NaiveDate::from_ymd_opt(2025, 5, 1).unwrap())
                );
            }
            other => panic!("expected SelfContained, got {other:?}"),
        }
    }
}
