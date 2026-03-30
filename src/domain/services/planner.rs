use chrono::Datelike;

use crate::domain::models::{
    garden::GardenGrid,
    request::PlanRequest,
    response::PlanResponse,
    vegetable::{Month, Vegetable},
    warnings::Warnings,
};
use crate::domain::services::allocation::build_placement_queue;
use crate::domain::services::filter::filter_vegetables;
use crate::domain::services::grid::{
    count_grid_occupancy, initialize_grid, validate_layout, GridOccupancy, GridSize,
};
pub use crate::domain::services::helpers::{cell_span, CELL_SIZE_CM};
use crate::domain::services::placement::{
    fill_remaining_cells, harvest_plants, place_candidates, PlacementWeek,
};
use crate::domain::services::response::{build_reason, build_weekly_plan, merge_consecutive_plans};
use crate::domain::services::schedule::weeks_for_period;

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
            "{empty_cells} empty cell(s): not enough compatible vegetables to fill the entire grid."
        )
    }
}

fn empty_cells_warning(grid: &GardenGrid) -> Option<String> {
    let empty = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| c.vegetable.is_none() && !c.blocked)
        .count();
    (empty > 0).then(|| Warnings::empty_cells_not_filled(empty))
}

/// Returns a warning string when non-blocked cells remain unplanted, otherwise `None`.
pub fn plan_garden(
    base_candidates: Vec<Vegetable>,
    request: &PlanRequest,
    lookup: impl Fn(&str) -> Option<Vegetable>,
) -> Result<PlanResponse, String> {
    let mut warnings = Warnings::new();

    let weeks = weeks_for_period(&request.period, &mut warnings);

    let GridSize(rows, cols) = validate_layout(&request.layout)?;

    let planning_start = weeks
        .first()
        .map(|w| w.start)
        .or_else(|| request.period.as_ref().map(|p| p.start))
        .unwrap_or(chrono::NaiveDate::MIN);
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

    for (week_idx, week) in weeks.into_iter().enumerate() {
        // Free cells occupied by plants that have matured by this week.
        harvest_plants(&mut grid, week_idx);

        // Filter candidates for the current week's month.
        let week_candidates = filter_vegetables(
            &base_candidates,
            request,
            Month::from_u32(week.start.month()),
        );

        let GridOccupancy(occupied, blocked_count) = count_grid_occupancy(&grid);
        let available_cells = (rows * cols).saturating_sub(blocked_count);
        let free_cells = available_cells.saturating_sub(occupied);

        let week_score = if free_cells > 0 && !week_candidates.is_empty() {
            // Phase 1: place vegetables with an explicit quantity (in preference order).
            let (queue, placements_map) =
                build_placement_queue(&week_candidates, preferences, free_cells);
            let pw = PlacementWeek {
                rows,
                cols,
                week_idx,
                week_start: week.start,
            };
            let score_p1 = place_candidates(&mut grid, &queue, &placements_map, &pw, build_reason);

            // Phase 2: iteratively fill every remaining free cell.
            let score_p2 = fill_remaining_cells(&mut grid, &week_candidates, &pw, build_reason);

            score_p1 + score_p2
        } else {
            0
        };

        weekly_plans.push(build_weekly_plan(week, &grid, week_score));
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
