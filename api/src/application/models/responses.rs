use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::variety::{
    Category, CultivationMode, Lifecycle, RegionCalendar, SoilType, SunExposure,
};

/// HTTP-facing flat representation of a variety.
///
/// Returned directly by the [`VarietyResponseRepository`] so handlers never
/// need to build a full domain [`Variety`] (with its embedded [`Vegetable`])
/// just to flatten it back down for the API response.
///
/// [`VarietyResponseRepository`]: crate::application::ports::variety_response_repository::VarietyResponseRepository
/// [`Variety`]: crate::domain::models::variety::Variety
/// [`Vegetable`]: crate::domain::models::vegetable::Vegetable
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct VarietyResponse {
    pub id: String,
    pub vegetable_id: String,
    pub name: String,
    pub latin_name: String,
    pub calendars: Vec<RegionCalendar>,
    pub sun_requirement: Vec<SunExposure>,
    pub soil_types: Vec<SoilType>,
    pub cultivation_modes: Vec<CultivationMode>,
    pub days_to_harvest: u32,
    pub days_to_plant: u32,
    pub lifecycle: Lifecycle,
    pub beginner_friendly: bool,
    pub category: Category,
}
