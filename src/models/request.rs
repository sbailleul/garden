use serde::{Deserialize, Serialize};

use crate::models::vegetable::{Region, Season, SoilType, SunExposure};

#[derive(Debug, Clone, Deserialize)]
pub struct PlanRequest {
    /// Garden width in metres
    pub width_m: f32,
    /// Garden length in metres
    pub length_m: f32,
    pub season: Season,
    pub sun: Option<SunExposure>,
    pub soil: Option<SoilType>,
    pub region: Option<Region>,
    /// "Beginner" or "Expert"
    pub level: Option<String>,
    /// Preferred vegetables (ids) chosen by the user
    pub preferences: Option<Vec<String>>,
    /// Existing layout: nullable string grid (vegetable ids or null)
    pub existing_layout: Option<Vec<Vec<Option<String>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedCell {
    pub id: Option<String>,
    pub name: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanResponse {
    pub grid: Vec<Vec<PlannedCell>>,
    pub rows: usize,
    pub cols: usize,
    pub score: i32,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionsResponse {
    pub id: String,
    pub name: String,
    pub good: Vec<CompanionInfo>,
    pub bad: Vec<CompanionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionInfo {
    pub id: String,
    pub name: String,
}
