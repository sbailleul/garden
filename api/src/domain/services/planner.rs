use chrono::{Datelike, Duration, NaiveDate};
use std::collections::{HashMap, HashSet};

use crate::domain::models::{
    garden::GardenGrid,
    request::{PlanParams, PreferenceEntry, SowingRecord},
    response::{PlanResponse, SowingTask},
    variety::{Month, Variety},
    warnings::Warnings,
};
use crate::domain::services::allocation::build_placement_queue;
use crate::domain::services::filter::filter_varieties;
use crate::domain::services::grid::{
    count_grid_occupancy, initialize_grid, validate_layout, GridOccupancy, GridSize,
};
pub use crate::domain::services::helpers::{cell_span, CELL_SIZE_CM};
use crate::domain::services::placement::{
    fill_remaining_cells, harvest_plants, place_candidates, PlacementWeek,
};
use crate::domain::services::response::{build_reason, build_weekly_plan, merge_consecutive_plans};
use crate::domain::services::schedule::weeks_for_period;

/// One pre-germinated batch of a single variety ready to transplant on `plant_date`.
struct SownBatch {
    variety: Variety,
    plant_date: NaiveDate,
    seeds_sown: u32,
}

/// Converts the `sown` map from the request into a flat list of [`SownBatch`]es.
/// `plant_date` = `sowing_date + days_to_plant`; falls back to `planning_start`
/// when no sowing date is provided.
fn compute_sown_batches(
    sown: &HashMap<String, Vec<SowingRecord>>,
    planning_start: NaiveDate,
    lookup: &impl Fn(&str) -> Option<Variety>,
) -> Vec<SownBatch> {
    let mut batches = Vec::new();
    for (id, records) in sown {
        if let Some(variety) = lookup(id) {
            for record in records {
                let plant_date = record
                    .sowing_date
                    .map(|d| d + Duration::days(variety.days_to_plant as i64))
                    .unwrap_or(planning_start);
                batches.push(SownBatch {
                    variety: variety.clone(),
                    plant_date,
                    seeds_sown: record.seeds_sown,
                });
            }
        }
    }
    batches
}

/// For each planning week, returns the list of varieties to sow that week
/// so they will be ready to transplant during a future planning week.
fn compute_sowing_tasks_by_week(
    weeks: &[crate::domain::models::request::Period],
    base_candidates: &[Variety],
    request: &PlanParams,
) -> Vec<Vec<SowingTask>> {
    weeks
        .iter()
        .enumerate()
        .map(|(w_idx, week)| {
            let mut tasks: Vec<SowingTask> = Vec::new();
            let mut seen_ids: HashSet<String> = HashSet::new();
            for future_week in weeks.iter().skip(w_idx) {
                let future_month = Month::from_u32(future_week.start.month());
                let future_candidates = filter_varieties(base_candidates, request, future_month);
                for veg in &future_candidates {
                    if veg.days_to_plant > 0 {
                        let sow_date = future_week.start - Duration::days(veg.days_to_plant as i64);
                        if sow_date >= week.start
                            && sow_date <= week.end
                            && seen_ids.insert(veg.id.clone())
                        {
                            tasks.push(SowingTask {
                                id: veg.id.clone(),
                                name: veg.name.clone(),
                                target_week_start: future_week.start,
                            });
                        }
                    }
                }
            }
            tasks
        })
        .collect()
}

impl Warnings {
    /// Planner warning text when no week can be generated for the period.
    fn no_weeks_to_plan() -> String {
        "No weeks to plan in the provided date range.".to_string()
    }

    /// Adds planner warning for an empty planning period.
    fn add_no_weeks_to_plan(&mut self) {
        self.add(Self::no_weeks_to_plan());
    }

    /// Planner warning text when non-blocked cells remain empty.
    fn empty_cells_not_filled(empty_cells: usize) -> String {
        format!(
            "{empty_cells} empty cell(s): not enough compatible varieties to fill the entire grid."
        )
    }
}

fn empty_cells_warning(grid: &GardenGrid) -> Option<String> {
    let empty = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| c.variety.is_none() && !c.blocked)
        .count();
    (empty > 0).then(|| Warnings::empty_cells_not_filled(empty))
}

/// Returns a warning string when non-blocked cells remain unplanted, otherwise `None`.
pub fn plan_garden(
    base_candidates: Vec<Variety>,
    request: &PlanParams,
    lookup: impl Fn(&str) -> Option<Variety>,
) -> Result<PlanResponse, String> {
    let mut warnings = Warnings::new();

    let weeks = weeks_for_period(&request.period, &mut warnings);

    let GridSize(rows, cols) = validate_layout(&request.layout)?;

    let planning_start = weeks
        .first()
        .map(|w| w.start)
        .or_else(|| request.period.as_ref().map(|p| p.start))
        .unwrap_or(NaiveDate::MIN);
    // Compute all sown batches before moving `lookup` into initialize_grid.
    let sown_batches = compute_sown_batches(&request.sown, planning_start, &lookup);
    // Pre-compute sowing tasks for each week.
    let sowing_tasks_by_week = compute_sowing_tasks_by_week(&weeks, &base_candidates, request);
    let mut grid = initialize_grid(
        rows,
        cols,
        &request.layout,
        planning_start,
        &request.region,
        lookup,
        &mut warnings,
    );
    let preferences = request.preferences.as_deref().unwrap_or(&[]);
    let mut weekly_plans = Vec::with_capacity(weeks.len());

    for (week_idx, (week, sowing_tasks)) in weeks.into_iter().zip(sowing_tasks_by_week).enumerate()
    {
        // Free cells occupied by plants that have matured by this week.
        harvest_plants(&mut grid, week_idx);

        // Filter candidates for the current week's month.
        let week_candidates = filter_varieties(
            &base_candidates,
            request,
            Month::from_u32(week.start.month()),
        );

        let GridOccupancy(occupied, blocked_count) = count_grid_occupancy(&grid);
        let available_cells = (rows * cols).saturating_sub(blocked_count);
        let free_cells = available_cells.saturating_sub(occupied);

        // Aggregate all sown batches whose plant_date has arrived by this week.
        let mut active_sown_counts: HashMap<String, u32> = HashMap::new();
        let mut sown_variety_map: HashMap<String, Variety> = HashMap::new();
        for batch in &sown_batches {
            if batch.plant_date <= week.start {
                *active_sown_counts
                    .entry(batch.variety.id.clone())
                    .or_insert(0) += batch.seeds_sown;
                sown_variety_map
                    .entry(batch.variety.id.clone())
                    .or_insert_with(|| batch.variety.clone());
            }
        }

        // Sown varieties ready this week that didn't pass the calendar filter — bypass it.
        let sown_extra: Vec<Variety> = sown_variety_map
            .iter()
            .filter(|(id, _)| !week_candidates.iter().any(|c| &c.id == *id))
            .map(|(_, v)| v.clone())
            .collect();

        // Sown preferences come before regular preferences for placement priority.
        let sown_prefs: Vec<PreferenceEntry> = active_sown_counts
            .into_iter()
            .map(|(id, count)| PreferenceEntry {
                id,
                quantity: Some(count),
            })
            .collect();

        let extended_candidates: Vec<Variety> =
            week_candidates.iter().cloned().chain(sown_extra).collect();
        let combined_prefs: Vec<PreferenceEntry> = sown_prefs
            .into_iter()
            .chain(preferences.iter().cloned())
            .collect();

        let week_score = if free_cells > 0 && !extended_candidates.is_empty() {
            // Phase 1: place varieties with an explicit quantity (in preference order).
            let (queue, placements_map) =
                build_placement_queue(&extended_candidates, &combined_prefs, free_cells);
            let pw = PlacementWeek {
                rows,
                cols,
                week_idx,
                week_start: week.start,
            };
            let score_p1 = place_candidates(&mut grid, &queue, &placements_map, &pw, build_reason);

            // Phase 2: iteratively fill every remaining free cell.
            let score_p2 = fill_remaining_cells(&mut grid, &extended_candidates, &pw, build_reason);

            score_p1 + score_p2
        } else {
            0
        };

        weekly_plans.push(build_weekly_plan(week, &grid, week_score, sowing_tasks));
    }

    if weekly_plans.is_empty() {
        warnings.add_no_weeks_to_plan();
    } else {
        warnings.add_optional(empty_cells_warning(&grid));
    }

    let weekly_plans = merge_consecutive_plans(weekly_plans);

    Ok(PlanResponse {
        rows,
        cols,
        weeks: weekly_plans,
        warnings: warnings.into_vec(),
    })
}
