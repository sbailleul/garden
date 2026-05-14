use async_trait::async_trait;
use serde::Deserialize;

pub use crate::application::models::responses::VarietyResponse;
use crate::application::ports::{Page, RepositoryError};
use crate::domain::models::variety::{Category, Lifecycle, Region, SoilType, SunExposure};

/// Filters for the variety listing endpoints.
///
/// All fields are optional — an absent filter means "no restriction".
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VarietyListFilter {
    pub category: Option<Category>,
    pub lifecycle: Option<Lifecycle>,
    pub beginner_friendly: Option<bool>,
    pub sun_requirement: Option<SunExposure>,
    pub soil_type: Option<SoilType>,
    pub region: Option<Region>,
    pub vegetable_id: Option<String>,
    pub search: Option<String>,
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
        filter: &VarietyListFilter,
    ) -> Result<Page<VarietyResponse>, RepositoryError>;

    async fn list_page_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
        filter: &VarietyListFilter,
    ) -> Result<Page<VarietyResponse>, RepositoryError>;
}
