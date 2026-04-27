use std::collections::HashMap;

use crate::application::models::request::PlanRequest;
use crate::application::ports::variety_repository::{VarietyFilter, VarietyRepository};
use crate::domain::models::request::{Level, PlanParams};
use crate::domain::models::{response::PlanResponse, variety::Variety};
use crate::domain::services::{filter::filter_candidates_base, planner::plan_garden};

/// Use case: generate an optimised garden plan.
///
/// Responsibilities:
/// 1. Fetch only the varieties that pass the SQL-level filter constraints via
///    [`VarietyRepository::get_for_planning`].
/// 2. Sort the pre-filtered candidates by preference / French consumption rank
///    (application-level logic, not expressible in SQL).
/// 3. Fetch the full catalogue via [`VarietyRepository::get_all`] to build a
///    lookup map so the domain planner can resolve pre-placed variety IDs from
///    the layout — these may not appear in the filtered candidates.
/// 4. Delegate planning to the domain service.
pub struct PlanGardenUseCase<'a> {
    repo: &'a dyn VarietyRepository,
}

impl<'a> PlanGardenUseCase<'a> {
    pub fn new(repo: &'a dyn VarietyRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        request: &PlanRequest,
        locale: &str,
    ) -> Result<PlanResponse, String> {
        let filter = VarietyFilter::from(request);
        let params = PlanParams::from(request);
        // SQL-filtered candidates — avoid loading the full catalogue into memory.
        let filtered = self
            .repo
            .get_for_planning(&filter, locale)
            .await
            .map_err(|e| e.to_string())?;
        // Sort by preferences / French consumption rank (application logic).
        let candidates = filter_candidates_base(&filtered, &params);
        // Full catalogue for resolving pre-placed layout cells.
        let all: Vec<Variety> = self.repo.get_all(locale).await.map_err(|e| e.to_string())?;
        let lookup: HashMap<String, Variety> = all.into_iter().map(|v| (v.id.clone(), v)).collect();
        plan_garden(candidates, &params, |id| lookup.get(id).cloned())
    }
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
