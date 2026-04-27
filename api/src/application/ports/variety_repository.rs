use async_trait::async_trait;

use crate::application::ports::{Page, RepositoryError};
use crate::domain::models::request::{Level, PlanRequest};
use crate::domain::models::variety::{Region, SoilType, SunExposure, Variety};

/// Filters derived from a [`PlanRequest`] that can be pushed down to the
/// database, avoiding a full-catalogue fetch for the planning use case.
#[derive(Debug, Clone)]
pub struct VarietyFilter {
    /// Only varieties whose calendar includes this region are returned.
    pub region: Region,
    /// When set, only varieties that tolerate this sun exposure are returned.
    pub sun: Option<SunExposure>,
    /// When set, only varieties compatible with this soil type are returned.
    pub soil: Option<SoilType>,
    /// When `true`, only beginner-friendly varieties are returned.
    pub beginner_only: bool,
    /// Variety IDs that must be excluded from the result.
    pub exclusions: Vec<String>,
}

impl From<&PlanRequest> for VarietyFilter {
    fn from(req: &PlanRequest) -> Self {
        Self {
            region: req.region.clone(),
            sun: req.sun.clone(),
            soil: req.soil.clone(),
            beginner_only: matches!(req.level, Some(Level::Beginner)),
            exclusions: req.exclusions.clone(),
        }
    }
}

/// Outbound port: provides access to the variety catalogue.
/// The application layer defines this trait; adapters implement it.
/// The `locale` parameter is a BCP-47 language tag (e.g. `"en"`, `"fr"`); the
/// implementation falls back to `"en"` when no translation is available.
#[async_trait]
pub trait VarietyRepository: Send + Sync {
    async fn get_all(&self, locale: &str) -> Result<Vec<Variety>, RepositoryError>;
    async fn get_by_id(&self, id: &str, locale: &str) -> Result<Option<Variety>, RepositoryError>;
    async fn get_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
    ) -> Result<Vec<Variety>, RepositoryError>;
    async fn list_page(
        &self,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Variety>, RepositoryError>;
    async fn list_page_by_vegetable_id(
        &self,
        vegetable_id: &str,
        locale: &str,
        page: usize,
        size: usize,
    ) -> Result<Page<Variety>, RepositoryError>;
    /// Returns the varieties that pass all SQL-pushable constraints from
    /// `filter`, ordered by `v.id`. Sorting by preferences / French rank
    /// is left to the caller since it is application-level logic.
    async fn get_for_planning(
        &self,
        filter: &VarietyFilter,
        locale: &str,
    ) -> Result<Vec<Variety>, RepositoryError>;
}
