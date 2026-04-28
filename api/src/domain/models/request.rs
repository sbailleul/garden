use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::{
    variety::{Region, Variety},
    Coordinate, Matrix,
};

/// A single cell in the **domain** layout grid.
/// Pre-planted cells carry a resolved [`Variety`]; `Empty`/`Blocked` carry no data.
#[derive(Debug, Clone)]
pub enum LayoutCell {
    /// A pre-planted cell that fits in one 30 cm × 30 cm grid cell.
    SelfContained {
        variety: Variety,
        /// Number of plants per cell. Computed from the variety's spacing if absent.
        plants_per_cell: Option<u32>,
        /// Date when this plant was put in the ground (ISO 8601, e.g. `"2025-05-01"`).
        planted_date: Option<NaiveDate>,
    },
    /// The top-left (anchor) cell of a pre-planted multi-cell block.
    Overflowing {
        variety: Variety,
        /// Number of plants per cell. Computed from the variety's spacing if absent.
        plants_per_cell: Option<u32>,
        /// Block width in grid cells. Computed from the variety's spacing if absent.
        width_cells: Option<u32>,
        /// Block length in grid cells. Computed from the variety's spacing if absent.
        length_cells: Option<u32>,
        /// Date when this plant was put in the ground (ISO 8601, e.g. `"2025-05-01"`).
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
    /// Desired number of **plants** (placements) for this variety.
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
    /// When omitted the planner uses the variety's regional sowing calendar.
    #[serde(default)]
    #[schema(value_type = Option<String>, format = Date, example = "2025-03-15")]
    pub sowing_date: Option<NaiveDate>,
    /// Number of seeds (or seedlings) that were sown in this batch.
    pub seeds_sown: u32,
}

/// An enriched preference: carries the resolved variety instead of a bare ID.
#[derive(Debug, Clone)]
pub struct Preference {
    pub variety: Variety,
    /// Desired number of **plants** (placements) for this variety.
    pub quantity: Option<u32>,
}

/// An enriched sowing entry: carries the resolved variety together with its batches.
#[derive(Debug, Clone)]
pub struct SownEntry {
    pub variety: Variety,
    pub records: Vec<SowingRecord>,
}

#[derive(Debug, Clone)]
pub struct PlanParams {
    /// Planning period (start and end dates).
    /// When omitted, defaults to the current Monday-to-Sunday week.
    pub period: Option<Period>,
    pub region: Region,
    /// Preferred varieties with optional per-variety plant count.
    pub preferences: Vec<Preference>,
    /// Varieties already sown from seed, enriched with resolved variety data.
    pub sown: Vec<SownEntry>,
    /// Combined grid layout — defines dimensions and pre-filled cells.
    pub layout: Matrix<LayoutCell>,
}
