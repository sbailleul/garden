use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::domain::models::{
    vegetable::{Region, SoilType, SunExposure},
    Coordinate, Matrix,
};

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

/// One sowing event: an optional date and a seed count.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SowingRecord {
    /// Date when the seeds were sown (ISO 8601, e.g. `"2025-03-15"`).
    /// When omitted the planner uses the vegetable's regional sowing calendar.
    #[serde(default)]
    #[schema(value_type = Option<String>, format = Date, example = "2025-03-15")]
    pub sowing_date: Option<NaiveDate>,
    /// Number of seeds (or seedlings) that were sown in this batch.
    pub seeds_sown: u32,
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
    /// Vegetables already sown from seed, keyed by vegetable id.
    /// Each entry is a list of sowing batches, each with an optional date and a seed count.
    /// Example: `{ "tomato": [{ "sowingDate": "2025-03-15", "seedsSown": 10 }] }`
    #[serde(default)]
    pub sown: HashMap<String, Vec<SowingRecord>>,
    /// Combined grid layout — defines dimensions and pre-filled cells.
    /// Each cell is a `LayoutCell` object: `{"type":"Empty"}` (free),
    /// `{"type":"SelfContained","id":"..."}` (pre-planted), or `{"type":"Blocked"}` (blocked).
    pub layout: Matrix<LayoutCell>,
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
