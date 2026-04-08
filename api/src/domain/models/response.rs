use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::{request::Period, vegetable::Vegetable, Coordinate, Matrix};

/// Vegetable domain struct for use in responses.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VegetableResponse {
    #[serde(flatten)]
    pub vegetable: Vegetable,
}

/// A cell in the planned garden grid (response output).
///
/// Three occupied variants, plus `Empty` and `Blocked`:
/// - `SelfContained` - a plant whose spacing <= 30 cm; fits entirely in one cell.
/// - `Overflowing`   - the anchor (top-left) cell of a plant that spans multiple cells.
/// - `Overflowed`    - a continuation cell covered by a neighbouring anchor; carries only a
///   back-reference so clients can look up the full data from the anchor.
/// - `Empty`         - free, unoccupied, non-blocked cell.
/// - `Blocked`       - non-plantable zone (path, alley, obstacle).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum PlannedCell {
    /// A plant that fits entirely within one 30 cm x 30 cm cell.
    SelfContained {
        id: String,
        name: String,
        reason: String,
        plants_per_cell: u32,
        /// Estimated date the plant will be ready to harvest.
        #[schema(value_type = String, format = Date, example = "2025-08-01")]
        estimated_harvest_date: NaiveDate,
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
        #[schema(value_type = String, format = Date, example = "2025-08-01")]
        estimated_harvest_date: NaiveDate,
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

/// A vegetable that should be sown during a given planning week so it is
/// ready to transplant into the garden during a future week.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SowingTask {
    /// Vegetable identifier.
    pub id: String,
    /// Human-readable vegetable name.
    pub name: String,
    /// Start date of the target transplanting week.
    #[schema(value_type = String, format = Date, example = "2025-05-05")]
    pub target_week_start: NaiveDate,
}

/// A snapshot of the garden layout for one week of the planning period.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct WeeklyPlan {
    /// The Monday-to-Sunday date range this snapshot covers.
    pub period: Period,

    pub week_count: u16,
    /// Full garden grid for this week (same dimensions as the request layout).
    #[schema(value_type = Vec<Vec<PlannedCell>>)]
    pub grid: Matrix<PlannedCell>,
    /// Cumulative companion-planting score for plants placed **this week**.
    pub score: i32,
    /// Vegetables to sow this week so they are ready to transplant during a
    /// future planning week.
    pub sowing_tasks: Vec<SowingTask>,
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
