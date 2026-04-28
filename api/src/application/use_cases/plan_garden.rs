use std::collections::HashMap;

use crate::application::models::request::{LayoutCell as RawLayoutCell, PlanRequest};
use crate::application::ports::variety_repository::{VarietyFilter, VarietyRepository};
use crate::domain::models::request::{LayoutCell, Level, PlanParams, Preference, SownEntry};
use crate::domain::models::{response::PlanResponse, variety::Variety};
use crate::domain::services::{filter::filter_candidates_base, planner::plan_garden};

/// Use case: generate an optimised garden plan.
///
/// Responsibilities:
/// 1. Fetch only the varieties that pass the SQL-level filter constraints via
///    [`VarietyRepository::get_for_planning`].
/// 2. Sort the pre-filtered candidates by preference / French consumption rank
///    (application-level logic, not expressible in SQL).
/// 3. Fetch the full catalogue via [`VarietyRepository::get_all`] and enrich
///    preferences, sown entries, and layout cells with resolved [`Variety`] objects.
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
        // SQL-filtered candidates — avoid loading the full catalogue into memory.
        let filtered = self
            .repo
            .get_for_planning(&filter, locale)
            .await
            .map_err(|e| e.to_string())?;
        // Full catalogue for resolving pre-placed layout cells and enriching preferences/sown.
        let all: Vec<Variety> = self.repo.get_all(locale).await.map_err(|e| e.to_string())?;
        let lookup: HashMap<String, Variety> = all.into_iter().map(|v| (v.id.clone(), v)).collect();

        // Enrich preferences with resolved Variety objects (unknown IDs are silently dropped).
        let preferences: Vec<Preference> = request
            .preferences
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|p| {
                lookup.get(&p.id).map(|v| Preference {
                    variety: v.clone(),
                    quantity: p.quantity,
                })
            })
            .collect();

        // Enrich sown entries with resolved Variety objects.
        let sown: Vec<SownEntry> = request
            .sown
            .iter()
            .filter_map(|(id, records)| {
                lookup.get(id).map(|v| SownEntry {
                    variety: v.clone(),
                    records: records.clone(),
                })
            })
            .collect();

        // Enrich layout cells with resolved Variety objects (unknown IDs → Empty).
        let layout =
            request
                .layout
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| match cell {
                            RawLayoutCell::SelfContained {
                                id,
                                plants_per_cell,
                                planted_date,
                            } => lookup.get(id).map_or(LayoutCell::Empty, |v| {
                                LayoutCell::SelfContained {
                                    variety: v.clone(),
                                    plants_per_cell: *plants_per_cell,
                                    planted_date: *planted_date,
                                }
                            }),
                            RawLayoutCell::Overflowing {
                                id,
                                plants_per_cell,
                                width_cells,
                                length_cells,
                                planted_date,
                            } => lookup.get(id).map_or(LayoutCell::Empty, |v| {
                                LayoutCell::Overflowing {
                                    variety: v.clone(),
                                    plants_per_cell: *plants_per_cell,
                                    width_cells: *width_cells,
                                    length_cells: *length_cells,
                                    planted_date: *planted_date,
                                }
                            }),
                            RawLayoutCell::Overflowed { covered_by } => LayoutCell::Overflowed {
                                covered_by: *covered_by,
                            },
                            RawLayoutCell::Empty => LayoutCell::Empty,
                            RawLayoutCell::Blocked => LayoutCell::Blocked,
                        })
                        .collect()
                })
                .collect();

        let params = PlanParams {
            period: request.period.clone(),
            region: request.region.clone(),
            preferences,
            sown,
            layout,
        };

        // Sort by preferences / French consumption rank (application logic).
        let candidates = filter_candidates_base(&filtered, &params);
        plan_garden(candidates, &params)
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
