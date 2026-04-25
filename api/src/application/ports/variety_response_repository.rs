use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::application::ports::{Page, RepositoryError};
use crate::domain::models::variety::{Category, Lifecycle, RegionCalendar, SoilType, SunExposure};

/// HTTP-facing flat representation of a variety.
///
/// Returned directly by the [`VarietyResponseRepository`] so handlers never
/// need to build a full domain [`Variety`] (with its embedded [`Vegetable`])
/// just to flatten it back down for the API response.
///
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
    pub spacing_cm: u32,
    pub days_to_harvest: u32,
    pub days_to_plant: u32,
    pub lifecycle: Lifecycle,
    pub beginner_friendly: bool,
    pub category: Category,
}

/// Outbound port: provides read access to the variety catalogue returning
/// [`VarietyResponse`] objects directly, without the full embedded [`Vegetable`]
/// that the planning use case requires.
///
/// Use this port for HTTP listing / detail endpoints.
/// Use [`VarietyRepository`] for the planning use case.
///
/// [`VarietyRepository`]: crate::application::ports::variety_repository::VarietyRepository
#[async_trait]
pub trait VarietyResponseRepository: Send + Sync {
    async fn get_by_id(
        &self,
        id: &str,
        locale: &str,
    ) -> Result<Option<VarietyResponse>, RepositoryError>;

    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<VarietyResponse>, RepositoryError>;

    async fn list_page_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<VarietyResponse>, RepositoryError>;
}
