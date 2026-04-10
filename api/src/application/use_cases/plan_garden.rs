use std::collections::HashMap;

use crate::application::ports::vegetable_repository::VegetableRepository;
use crate::domain::models::{request::PlanRequest, response::PlanResponse, vegetable::Vegetable};
use crate::domain::services::{filter::filter_candidates_base, planner::plan_garden};

/// Use case: generate an optimised garden plan.
///
/// Responsibilities:
/// 1. Fetch the full vegetable catalogue via the outbound port.
/// 2. Apply pre-filtering constraints from the request.
/// 3. Build a lookup map so the domain planner can resolve pre-placed
///    vegetable IDs from the layout without depending on any port.
/// 4. Delegate planning to the domain service.
pub struct PlanGardenUseCase<'a> {
    repo: &'a dyn VegetableRepository,
}

impl<'a> PlanGardenUseCase<'a> {
    pub fn new(repo: &'a dyn VegetableRepository) -> Self {
        Self { repo }
    }

    pub async fn execute(
        &self,
        request: &PlanRequest,
        locale: &str,
    ) -> Result<PlanResponse, String> {
        let db = self.repo.get_all(locale).await.map_err(|e| e.to_string())?;
        let candidates = filter_candidates_base(&db, request);
        let lookup: HashMap<String, Vegetable> =
            db.into_iter().map(|v| (v.id.clone(), v)).collect();
        plan_garden(candidates, request, |id| lookup.get(id).cloned())
    }
}
