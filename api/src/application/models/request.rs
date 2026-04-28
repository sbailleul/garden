use std::collections::HashMap;

use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::models::{
    request::{LayoutCell, Level, Period, PreferenceEntry, SowingRecord},
    variety::{Region, SoilType, SunExposure},
    Matrix,
};

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
