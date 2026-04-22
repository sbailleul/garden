use std::collections::HashMap;

use crate::application::ports::variety_repository::VarietyRepository;
use crate::domain::models::{request::PlanRequest, response::PlanResponse, variety::Variety};
use crate::domain::services::{filter::filter_candidates_base, planner::plan_garden};

/// Use case: generate an optimised garden plan.
///
/// Responsibilities:
/// 1. Fetch the full variety catalogue via the outbound port.
/// 2. Apply pre-filtering constraints from the request.
/// 3. Build a lookup map so the domain planner can resolve pre-placed
///    variety IDs from the layout without depending on any port.
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
        let db = self.repo.get_all(locale).await.map_err(|e| e.to_string())?;
        let candidates = filter_candidates_base(&db, request);
        let lookup: HashMap<String, Variety> = db.into_iter().map(|v| (v.id.clone(), v)).collect();
        plan_garden(candidates, request, |id| lookup.get(id).cloned())
    }
}
