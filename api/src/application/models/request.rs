use std::collections::HashMap;

use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::models::{
    request::{Level, Period, PreferenceEntry, SowingRecord},
    variety::{Region, SoilType, SunExposure},
    Coordinate, Matrix,
};

/// HTTP-facing layout cell, deserialized from the `layout` array in `POST /api/plan`.
/// Pre-planted cells reference a variety by ID; the use case resolves IDs to [`Variety`]
/// objects before passing the layout to the domain planner.
#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum LayoutCell {
    /// A pre-planted cell that fits in one 30 cm × 30 cm grid cell.
    #[serde(rename_all = "camelCase")]
    SelfContained {
        id: String,
        /// Number of plants per cell. Computed from the variety's spacing if absent.
        plants_per_cell: Option<u32>,
        /// Date when this plant was put in the ground (ISO 8601, e.g. `"2025-05-01"`).
        /// When provided, it is used to free the cell after harvest and compute
        /// `estimatedHarvestDate` in the response.
        #[schema(value_type = Option<String>, format = Date, example = "2025-05-01")]
        planted_date: Option<chrono::NaiveDate>,
    },
    /// The top-left (anchor) cell of a pre-planted multi-cell block.
    #[serde(rename_all = "camelCase")]
    Overflowing {
        id: String,
        /// Number of plants per cell. Computed from the variety's spacing if absent.
        plants_per_cell: Option<u32>,
        /// Block width in grid cells. Computed from the variety's spacing if absent.
        width_cells: Option<u32>,
        /// Block length in grid cells. Computed from the variety's spacing if absent.
        length_cells: Option<u32>,
        /// Date when this plant was put in the ground (ISO 8601, e.g. `"2025-05-01"`).
        /// When provided, it is used to free the cell after harvest and compute
        /// `estimatedHarvestDate` in the response.
        #[schema(value_type = Option<String>, format = Date, example = "2025-05-01")]
        planted_date: Option<chrono::NaiveDate>,
    },
    /// A continuation cell of a multi-cell block (skipped — anchor handles placement).
    #[serde(rename_all = "camelCase")]
    Overflowed { covered_by: Coordinate },
    /// Free, unoccupied, non-blocked cell.
    Empty,
    /// Non-plantable zone (path, alley, obstacle).
    Blocked,
}

/// HTTP-facing planning request, deserialized from the `POST /api/plan` body.
///
/// Validated and converted to [`PlanParams`] by the use case before the domain
/// services are invoked.
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
    /// Preferred varieties with optional per-variety plant count.
    pub preferences: Option<Vec<PreferenceEntry>>,
    /// Variety IDs to exclude from planning — these will never be auto-placed
    /// regardless of other filters. Pre-placed cells in `layout` are not affected.
    #[serde(default)]
    pub exclusions: Vec<String>,
    /// Varieties already sown from seed, keyed by variety id.
    /// Each entry is a list of sowing batches, each with an optional date and a seed count.
    /// Example: `{ "tomato": [{ "sowingDate": "2025-03-15", "seedsSown": 10 }] }`
    #[serde(default)]
    pub sown: HashMap<String, Vec<SowingRecord>>,
    /// Combined grid layout — defines dimensions and pre-filled cells.
    /// Each cell is a `LayoutCell` object: `{"type":"Empty"}` (free),
    /// `{"type":"SelfContained","id":"..."}` (pre-planted), or `{"type":"Blocked"}` (blocked).
    #[schema(value_type = Vec<Vec<LayoutCell>>)]
    pub layout: Matrix<LayoutCell>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

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
