use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate};
use log::{debug, info, trace, warn};

use crate::data::vegetables::get_vegetable_by_id;
use crate::logic::schedule::{harvest_plants, weeks_for_period};
use crate::logic::{companion::companion_score, filter::filter_vegetables};
use crate::models::{
    garden::{GardenGrid, PlantedAt},
    request::{
        LayoutCell, Period, PlanRequest, PlanResponse, PlannedCell, PreferenceEntry, WeeklyPlan,
    },
    vegetable::{Month, Vegetable},
    Coordinate, Matrix,
};

/// Grid dimensions returned by layout validation: `(rows, cols)`.
struct GridSize(usize, usize);

/// Grid occupancy counts returned by the occupancy check: `(occupied, blocked)`.
struct GridOccupancy(usize, usize);

/// A deferred continuation cell and its anchor coordinate: `(position, anchor)`.
struct DeferredCell(Coordinate, Coordinate);

/// Size of one grid cell in centimetres
pub const CELL_SIZE_CM: u32 = 30;

/// How many grid cells a plant requires per axis: `ceil(spacing / 30)`, minimum 1.
/// Examples: 10 cm → 1, 30 cm → 1, 40 cm → 2, 60 cm → 2, 90 cm → 3.
pub fn cell_span(spacing_cm: u32) -> u32 {
    spacing_cm.div_ceil(CELL_SIZE_CM).max(1)
}

/// Plants per cell:
/// - span == 1 (spacing ≤ 30 cm): `floor(30 / spacing)^2`
/// - span  > 1 (spacing  > 30 cm): 1 plant occupies the whole span×span block.
fn plants_per_cell(spacing_cm: u32) -> u32 {
    if cell_span(spacing_cm) > 1 {
        1
    } else {
        let per_axis = (CELL_SIZE_CM / spacing_cm.max(1)).max(1);
        per_axis * per_axis
    }
}

/// Distributes cells for vegetables that have an **explicit** `quantity` preference.
/// Returns a map of `id → cell count` only for those vegetables; everything else
/// (auto-fill candidates) is handled by a separate iterative fill phase.
fn compute_explicit_allocation(
    candidates: &[Vegetable],
    preferences: &[PreferenceEntry],
    available: usize,
) -> HashMap<String, usize> {
    let mut allocation: HashMap<String, usize> = HashMap::new();
    let mut remaining = available;

    for pref in preferences {
        if let Some(qty) = pref.quantity {
            if let Some(v) = candidates.iter().find(|v| v.id == pref.id) {
                let cells_per_plant = (cell_span(v.spacing_cm) as usize).pow(2);
                let cells_needed = (qty as usize).saturating_mul(cells_per_plant);
                let alloc = cells_needed.min(remaining);
                allocation.insert(pref.id.clone(), alloc);
                remaining = remaining.saturating_sub(alloc);
            }
        }
    }

    allocation
}

/// Validates that the layout has at least one non-empty row.
/// Returns `GridSize(rows, cols)` on success.
fn validate_layout(layout: &[Vec<LayoutCell>]) -> Result<GridSize, String> {
    if layout.is_empty() {
        warn!("validate_layout: rejected — layout has no rows");
        return Err("Layout must contain at least one row.".into());
    }
    let cols = layout[0].len();
    if cols == 0 {
        warn!("validate_layout: rejected — first row is empty");
        return Err("Layout rows must not be empty.".into());
    }
    debug!("validate_layout: {}×{} grid accepted", layout.len(), cols);
    Ok(GridSize(layout.len(), cols))
}

/// Creates a blank grid and pre-fills it from the unified layout array:
/// blocked zones (`true`) and pre-placed vegetables (`"id"`).
/// Returns the grid and any warnings produced (e.g. unknown vegetable IDs).
fn initialize_grid(
    rows: usize,
    cols: usize,
    layout: &[Vec<LayoutCell>],
    warnings: &mut Vec<String>,
) -> GardenGrid {
    debug!("initialize_grid: building {rows}×{cols} grid from layout");
    let mut grid = GardenGrid::new(rows, cols);
    // Continuation cells are collected here and resolved after all anchors are placed.
    let mut deferred: Vec<DeferredCell> = Vec::new();

    for (r, row) in layout.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                LayoutCell::Blocked => {
                    trace!("initialize_grid: [{r},{c}] marked as blocked");
                    grid.cells[r][c].blocked = true;
                }
                LayoutCell::SelfContained {
                    id,
                    plants_per_cell: ppc_input,
                } => {
                    if let Some(v) = get_vegetable_by_id(id) {
                        debug!("initialize_grid: [{r},{c}] pre-filled with '{}'", v.id);
                        let ppc = ppc_input.unwrap_or_else(|| plants_per_cell(v.spacing_cm));
                        grid.cells[r][c].vegetable = Some(crate::models::garden::PlacedVegetable {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: "Present in the existing layout.".into(),
                            plants_per_cell: ppc,
                            span: 1,
                            anchor: Coordinate { row: r, col: c },
                            planted_at: PlantedAt {
                                week: 0,
                                date: None,
                            },
                            days_to_harvest: v.days_to_harvest,
                        });
                    } else {
                        warn!("initialize_grid: vegetable '{id}' not found, skipping [{r},{c}]");
                        warnings.push(format!(
                            "Vegetable '{id}' not found in the database, skipped."
                        ));
                    }
                }
                LayoutCell::Overflowed { covered_by } => {
                    deferred.push(DeferredCell(Coordinate { row: r, col: c }, *covered_by));
                }
                LayoutCell::Overflowing {
                    id,
                    plants_per_cell: ppc_input,
                    width_cells,
                    length_cells,
                } => {
                    if let Some(v) = get_vegetable_by_id(id) {
                        debug!("initialize_grid: [{r},{c}] pre-filled with '{}'", v.id);
                        let span = cell_span(v.spacing_cm);
                        let ppc = ppc_input.unwrap_or_else(|| plants_per_cell(v.spacing_cm));
                        let w = width_cells.unwrap_or(span);
                        let l = length_cells.unwrap_or(span);
                        grid.cells[r][c].vegetable = Some(crate::models::garden::PlacedVegetable {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: "Present in the existing layout.".into(),
                            plants_per_cell: ppc,
                            span: w.max(l),
                            anchor: Coordinate { row: r, col: c },
                            planted_at: PlantedAt {
                                week: 0,
                                date: None,
                            },
                            days_to_harvest: v.days_to_harvest,
                        });
                    } else {
                        warn!("initialize_grid: vegetable '{id}' not found, skipping [{r},{c}]");
                        warnings.push(format!(
                            "Vegetable '{id}' not found in the database, skipped."
                        ));
                    }
                }
                LayoutCell::Empty => {}
            }
        }
    }

    // Resolve continuation cells now that all anchors are in the grid.
    for DeferredCell(pos, covered_by) in deferred {
        let Coordinate {
            row: position_row,
            col: position_col,
        } = pos;
        let Coordinate {
            row: covered_by_row,
            col: covered_by_col,
        } = covered_by;
        if covered_by_row < rows && covered_by_col < cols {
            if let Some(anchor_veg) = grid.cells[covered_by_row][covered_by_col].vegetable.clone() {
                trace!(
                    "initialize_grid: [{position_row},{position_col}] continuation of anchor [{covered_by_row},{covered_by_col}] ('{}')",
                    anchor_veg.id
                );
                grid.cells[position_row][position_col].vegetable = Some(anchor_veg);
            } else {
                warn!(
                    "initialize_grid: [{position_row},{position_col}] Overflowed references [{covered_by_row},{covered_by_col}] which has no planted anchor, skipping"
                );
                warnings.push(format!(
                    "Continuation cell [{position_row},{position_col}] references an unplanted anchor [{covered_by_row},{covered_by_col}], skipped."
                ));
            }
        } else {
            warn!(
                "initialize_grid: [{position_row},{position_col}] Overflowed references out-of-bounds anchor [{covered_by_row},{covered_by_col}]"
            );
            warnings.push(format!(
                "Continuation cell [{position_row},{position_col}] references out-of-bounds anchor [{covered_by_row},{covered_by_col}], skipped."
            ));
        }
    }

    grid
}

/// Returns `GridOccupancy(occupied, blocked)` cell counts for the given grid.
fn count_grid_occupancy(grid: &GardenGrid) -> GridOccupancy {
    let flat = || grid.cells.iter().flat_map(|r| r.iter());
    let occupied = flat().filter(|c| c.vegetable.is_some()).count();
    let blocked = flat().filter(|c| c.blocked).count();
    debug!("count_grid_occupancy: {occupied} occupied, {blocked} blocked");
    GridOccupancy(occupied, blocked)
}

/// Converts explicit-preference allocations into an ordered placement queue
/// (each vegetable repeated by its allocated count) and a per-vegetable placement cap.
/// Vegetables without an explicit quantity are NOT in the queue; they are handled
/// by the iterative fill phase.
fn build_placement_queue<'a>(
    candidates: &'a [Vegetable],
    preferences: &[PreferenceEntry],
    free_cells: usize,
) -> (Vec<&'a Vegetable>, HashMap<String, usize>) {
    debug!(
        "build_placement_queue: {} candidates, {} free cells",
        candidates.len(),
        free_cells
    );
    let allocation = compute_explicit_allocation(candidates, preferences, free_cells);

    // Convert cell allocations → placement counts (one placement = span² cells).
    let placements_map: HashMap<String, usize> = candidates
        .iter()
        .filter(|v| allocation.contains_key(&v.id))
        .map(|v| {
            let cells_per_slot = (cell_span(v.spacing_cm) as usize).pow(2);
            let cells = allocation.get(&v.id).copied().unwrap_or(0);
            let n = if cells > 0 {
                (cells / cells_per_slot).max(1)
            } else {
                0
            };
            debug!(
                "build_placement_queue: '{}' → {} cell(s) → {} placement(s) (span {})",
                v.id,
                cells,
                n,
                cell_span(v.spacing_cm)
            );
            (v.id.clone(), n)
        })
        .collect();

    // Expand: repeat each vegetable in preference order by its placement count.
    let queue: Vec<&Vegetable> = preferences
        .iter()
        .filter_map(|p| candidates.iter().find(|v| v.id == p.id))
        .flat_map(|v| {
            let n = placements_map.get(&v.id).copied().unwrap_or(0);
            std::iter::repeat_n(v, n)
        })
        .collect();

    debug!("build_placement_queue: queue length = {}", queue.len());
    (queue, placements_map)
}

/// Scans the grid for the free `span × span` block that maximises the companion score
/// for `vegetable`. Returns `Some((row, col, score))` or `None` when no valid block exists.
fn find_best_block(
    grid: &GardenGrid,
    vegetable: &Vegetable,
    rows: usize,
    cols: usize,
) -> Option<(Coordinate, i32)> {
    let span = cell_span(vegetable.spacing_cm) as usize;
    let mut best: Option<(Coordinate, i32)> = None;

    for r in 0..=rows.saturating_sub(span) {
        for c in 0..=cols.saturating_sub(span) {
            if !grid.is_block_free(r, c, span) {
                continue;
            }
            let neighbor_ids: Vec<&str> = grid
                .get_block_neighbors(Coordinate { row: r, col: c }, span)
                .iter()
                .map(|v| v.id.as_str())
                .collect();
            let score = companion_score(vegetable, &neighbor_ids);
            trace!(
                "find_best_block: '{}' at [{r},{c}] span={span} score={score}",
                vegetable.id
            );
            if best.is_none_or(|(_, s)| score > s) {
                best = Some((Coordinate { row: r, col: c }, score));
            }
        }
    }

    if let Some((Coordinate { row, col }, s)) = best {
        debug!(
            "find_best_block: best block for '{}' at [{row},{col}] score={s}",
            vegetable.id
        );
    } else {
        debug!(
            "find_best_block: no free {span}×{span} block for '{}'",
            vegetable.id
        );
    }

    best
}

/// Fills a single `span × span` block starting at `(row, col)` with `vegetable`.
fn fill_block(
    grid: &mut GardenGrid,
    vegetable: &Vegetable,
    coordinate: Coordinate,
    reason: &str,
    week_idx: usize,
    week_start: NaiveDate,
) {
    let span = cell_span(vegetable.spacing_cm) as usize;
    let ppc = plants_per_cell(vegetable.spacing_cm);
    debug!(
        "fill_block: placing '{}' at [{},{}] span={span} plants_per_cell={ppc} week={week_idx}",
        vegetable.id, coordinate.row, coordinate.col
    );
    for dr in 0..span {
        for dc in 0..span {
            grid.cells[coordinate.row + dr][coordinate.col + dc].vegetable =
                Some(crate::models::garden::PlacedVegetable {
                    id: vegetable.id.clone(),
                    name: vegetable.name.clone(),
                    reason: reason.to_owned(),
                    plants_per_cell: ppc,
                    span: span as u32,
                    anchor: coordinate,
                    planted_at: PlantedAt {
                        week: week_idx,
                        date: Some(week_start),
                    },
                    days_to_harvest: vegetable.days_to_harvest,
                });
        }
    }
}

/// Iterates over the placement queue and greedily places each vegetable on the grid.
/// Returns the cumulative companion score.
fn place_candidates(
    grid: &mut GardenGrid,
    queue: &[&Vegetable],
    placements_map: &HashMap<String, usize>,
    rows: usize,
    cols: usize,
    week_idx: usize,
    week_start: NaiveDate,
) -> i32 {
    let mut global_score: i32 = 0;

    // Seed placement counts from anything already in the grid (pre-filled cells).
    let mut placed_counts: HashMap<String, usize> = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter_map(|c| c.vegetable.as_ref().map(|v| v.id.clone()))
        .fold(HashMap::new(), |mut map, id| {
            *map.entry(id).or_insert(0) += 1;
            map
        });

    'outer: for vegetable in queue {
        let max_count = placements_map.get(&vegetable.id).copied().unwrap_or(0);
        if placed_counts.get(&vegetable.id).copied().unwrap_or(0) >= max_count {
            trace!(
                "place_candidates: '{}' reached its cap of {max_count}, skipping",
                vegetable.id
            );
            continue;
        }

        let span = cell_span(vegetable.spacing_cm) as usize;

        match find_best_block(grid, vegetable, rows, cols) {
            None if span == 1 => {
                debug!("place_candidates: no free cells left — stopping early");
                break 'outer; // no free single cell — grid is full
            }
            None => {
                debug!(
                    "place_candidates: no {span}×{span} block for '{}', skipping",
                    vegetable.id
                );
                continue; // no span×span block; smaller plants may still fit
            }
            Some((coordinate, score)) => {
                let neighbor_names: Vec<String> = grid
                    .get_block_neighbors(coordinate, span)
                    .iter()
                    .map(|v| v.name.clone())
                    .collect();
                let reason = build_reason(vegetable, &neighbor_names, score);
                fill_block(grid, vegetable, coordinate, &reason, week_idx, week_start);
                placed_counts
                    .entry(vegetable.id.clone())
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
                global_score += score;
            }
        }
    }

    info!("place_candidates: finished — cumulative score = {global_score}");
    global_score
}

/// Phase 2 — iterative greedy fill.
///
/// After explicit preferences have been placed, tries every candidate in priority
/// order and places the best available block for each. Repeats until a full pass
/// over all candidates produces zero new placements (grid is genuinely full or no
/// candidate fits anywhere). This ensures that cells left vacant by large-span
/// plants that could not find a free block are filled by smaller alternatives.
fn fill_remaining_cells(
    grid: &mut GardenGrid,
    candidates: &[Vegetable],
    rows: usize,
    cols: usize,
    week_idx: usize,
    week_start: NaiveDate,
) -> i32 {
    let mut total_score: i32 = 0;
    let mut pass = 0usize;

    loop {
        pass += 1;
        let mut placements_this_pass = 0usize;

        for vegetable in candidates {
            match find_best_block(grid, vegetable, rows, cols) {
                None => continue,
                Some((coordinate, score)) => {
                    let span = cell_span(vegetable.spacing_cm) as usize;
                    let neighbor_names: Vec<String> = grid
                        .get_block_neighbors(coordinate, span)
                        .iter()
                        .map(|v| v.name.clone())
                        .collect();
                    let reason = build_reason(vegetable, &neighbor_names, score);
                    debug!(
                        "fill_remaining_cells pass {pass}: placing '{}' at [{},{}] score={score}",
                        vegetable.id, coordinate.row, coordinate.col
                    );
                    fill_block(grid, vegetable, coordinate, &reason, week_idx, week_start);
                    total_score += score;
                    placements_this_pass += 1;
                }
            }
        }

        debug!("fill_remaining_cells pass {pass}: {placements_this_pass} placement(s) made");

        if placements_this_pass == 0 {
            break;
        }
    }

    info!("fill_remaining_cells: done after {pass} pass(es), score gained = {total_score}");
    total_score
}
fn empty_cells_warning(grid: &GardenGrid) -> Option<String> {
    let empty = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| c.vegetable.is_none() && !c.blocked)
        .count();
    if empty > 0 {
        warn!("empty_cells_warning: {empty} cell(s) left unplanted");
    }
    (empty > 0).then(|| {
        format!("{empty} empty cell(s): not enough compatible vegetables to fill the entire grid.")
    })
}

/// Returns a warning string when non-blocked cells remain unplanted, otherwise `None`.
pub fn plan_garden(
    base_candidates: Vec<Vegetable>,
    request: &PlanRequest,
) -> Result<PlanResponse, String> {
    let mut warnings: Vec<String> = Vec::new();

    let weeks = weeks_for_period(&request.period, &mut warnings);

    info!(
        "plan_garden: starting — {} candidate(s), {} → {}",
        base_candidates.len(),
        weeks
            .first()
            .map_or_else(|| "-".to_string(), |w| w.start.to_string()),
        weeks
            .last()
            .map_or_else(|| "-".to_string(), |w| w.end.to_string()),
    );

    let GridSize(rows, cols) = validate_layout(&request.layout)?;

    let mut grid = initialize_grid(rows, cols, &request.layout, &mut warnings);
    let preferences = request.preferences.as_deref().unwrap_or(&[]);
    let mut weekly_plans: Vec<WeeklyPlan> = Vec::with_capacity(weeks.len());

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
        info!(
            "plan_garden week {week_idx} ({}): {free_cells} free, month={}, {} candidate(s)",
            week.start,
            week.start.month(),
            week_candidates.len()
        );

        let week_score = if free_cells > 0 && !week_candidates.is_empty() {
            // Phase 1: place vegetables with an explicit quantity (in preference order).
            let (queue, placements_map) =
                build_placement_queue(&week_candidates, preferences, free_cells);
            let score_p1 = place_candidates(
                &mut grid,
                &queue,
                &placements_map,
                rows,
                cols,
                week_idx,
                week.start,
            );

            // Phase 2: iteratively fill every remaining free cell.
            let score_p2 = fill_remaining_cells(
                &mut grid,
                &week_candidates,
                rows,
                cols,
                week_idx,
                week.start,
            );

            score_p1 + score_p2
        } else {
            0
        };

        weekly_plans.push(build_weekly_plan(week, &grid, week_score));
    }

    if weekly_plans.is_empty() {
        warnings.push("No weeks to plan in the provided date range.".into());
    } else if let Some(w) = empty_cells_warning(&grid) {
        warnings.push(w);
    }

    let weekly_plans = merge_consecutive_plans(weekly_plans);

    info!(
        "plan_garden: done — {} week(s), warnings={}",
        weekly_plans.len(),
        warnings.len()
    );

    Ok(PlanResponse {
        rows,
        cols,
        weeks: weekly_plans,
        warnings,
    })
}

/// Merges consecutive [`WeeklyPlan`]s that have identical grids.
///
/// When two or more adjacent weeks produce the same garden layout the function
/// collapses them into a single entry: `period.start` is kept from the first
/// entry, `period.end` is taken from the last, `week_count` is incremented by
/// the number of weeks merged, and `score` is accumulated.
fn merge_consecutive_plans(plans: Vec<WeeklyPlan>) -> Vec<WeeklyPlan> {
    let mut merged: Vec<WeeklyPlan> = Vec::new();
    for plan in plans {
        match merged.last_mut() {
            Some(last) if last.grid == plan.grid => {
                last.period.end = plan.period.end;
                last.week_count += plan.week_count;
                last.score += plan.score;
            }
            _ => merged.push(plan),
        }
    }
    merged
}

/// Converts the current garden grid into a [`WeeklyPlan`] snapshot.
fn build_weekly_plan(week: Period, grid: &GardenGrid, score: i32) -> WeeklyPlan {
    WeeklyPlan {
        period: week,
        grid: build_grid_cells(grid),
        score,
        week_count: 1,
    }
}

/// Converts a [`GardenGrid`] into the `Matrix<PlannedCell>` used in API responses.
fn build_grid_cells(grid: &GardenGrid) -> Matrix<PlannedCell> {
    grid.cells
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, cell)| match &cell.vegetable {
                    Some(v)
                        if (row_idx, col_idx) == (v.anchor.row, v.anchor.col) && v.span == 1 =>
                    {
                        PlannedCell::SelfContained {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: v.reason.clone(),
                            plants_per_cell: v.plants_per_cell,
                            estimated_harvest_date: v
                                .planted_at
                                .date
                                .map(|d| d + Duration::days(v.days_to_harvest as i64)),
                        }
                    }
                    Some(v) if (row_idx, col_idx) == (v.anchor.row, v.anchor.col) => {
                        PlannedCell::Overflowing {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: v.reason.clone(),
                            plants_per_cell: v.plants_per_cell,
                            width_cells: v.span,
                            length_cells: v.span,
                            estimated_harvest_date: v
                                .planted_at
                                .date
                                .map(|d| d + Duration::days(v.days_to_harvest as i64)),
                        }
                    }
                    Some(v) => PlannedCell::Overflowed {
                        covered_by: v.anchor,
                    },
                    None if cell.blocked => PlannedCell::Blocked,
                    None => PlannedCell::Empty,
                })
                .collect()
        })
        .collect()
}

fn build_reason(vegetable: &Vegetable, neighbor_names: &[String], score: i32) -> String {
    if neighbor_names.is_empty() {
        return format!(
            "First placed ({}{}) ",
            vegetable.category,
            if vegetable.beginner_friendly {
                ", beginner-friendly"
            } else {
                ""
            }
        );
    }
    let neighbors_str = neighbor_names.join(", ");
    let qualifier = if score > 0 {
        "good companion with"
    } else if score < 0 {
        "constrained placement near"
    } else {
        "neutral with"
    };
    format!(
        "{} {} {}{}",
        vegetable.name,
        qualifier,
        neighbors_str,
        if vegetable.beginner_friendly {
            " (beginner-friendly)"
        } else {
            ""
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vegetables::{get_all_vegetables, get_vegetable_by_id};
    use crate::logic::filter::{filter_candidates_base, filter_vegetables};
    use crate::models::{
        request::{LayoutCell, Period, PlanRequest, PlannedCell},
        vegetable::{Month, Season},
    };
    use chrono::NaiveDate;

    fn meters_to_cells(meters: f32) -> usize {
        ((meters * 100.0) / 30.0_f32).ceil() as usize
    }

    fn season_to_dates(season: &Season) -> Period {
        // All dates are chosen to already fall on a Monday so that
        // normalize_period is a no-op and test behaviour stays deterministic.
        let start = match season {
            Season::Spring => NaiveDate::from_ymd_opt(2025, 3, 3).unwrap(), // Monday
            Season::Summer => NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(), // Monday
            Season::Autumn => NaiveDate::from_ymd_opt(2025, 9, 1).unwrap(), // Monday
            Season::Winter => NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(), // Monday
        };
        let end = start + chrono::Duration::days(6); // Sunday
        Period { start, end }
    }

    /// Returns a reference to the first week's grid in the response.
    fn first_grid(resp: &PlanResponse) -> &Matrix<PlannedCell> {
        &resp.weeks[0].grid
    }

    fn minimal_request(width: f32, length: f32, season: Season) -> PlanRequest {
        let cols = meters_to_cells(width);
        let rows = meters_to_cells(length);
        PlanRequest {
            period: Some(season_to_dates(&season)),
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            layout: vec![vec![LayoutCell::Empty; cols]; rows],
        }
    }

    #[test]
    fn test_grid_dimensions_1m_x_1m() {
        let req = minimal_request(1.0, 1.0, Season::Summer);
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // 1m = 100cm / 30 = 3.33 → ceil = 4 cells
        assert_eq!(resp.rows, 4);
        assert_eq!(resp.cols, 4);
    }

    #[test]
    fn test_grid_dimensions_2m_x_3m() {
        let req = minimal_request(2.0, 3.0, Season::Summer);
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // 2m → ceil(200/30) = 7, 3m → ceil(300/30) = 10
        assert_eq!(resp.cols, 7);
        assert_eq!(resp.rows, 10);
    }

    #[test]
    fn test_invalid_zero_width_returns_error() {
        let req = minimal_request(0.0, 2.0, Season::Summer);
        let result = plan_garden(vec![], &req);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_negative_returns_error() {
        let req = minimal_request(-1.0, 2.0, Season::Summer);
        let result = plan_garden(vec![], &req);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_cells_have_reason() {
        let req = minimal_request(1.0, 1.0, Season::Summer);
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        for row in first_grid(&resp) {
            for cell in row {
                match cell {
                    PlannedCell::SelfContained { reason, .. }
                    | PlannedCell::Overflowing { reason, .. } => {
                        assert!(
                            !reason.is_empty(),
                            "Anchor cell must have a non-empty reason"
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_existing_layout_preserved() {
        let req = PlanRequest {
            layout: vec![
                vec![
                    LayoutCell::SelfContained {
                        id: "tomato".into(),
                        plants_per_cell: None,
                    },
                    LayoutCell::Empty,
                ],
                vec![LayoutCell::Empty, LayoutCell::Empty],
            ],
            ..minimal_request(0.6, 0.6, Season::Summer)
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // Cell [0][0] must still be "tomato"
        let first_cell = &first_grid(&resp)[0][0];
        assert_eq!(
            first_cell.id(),
            Some("tomato"),
            "Existing cell must be preserved"
        );
    }

    #[test]
    fn test_bad_companions_not_adjacent_when_alternatives_exist() {
        // Tomato and fennel are incompatible — on a large grid they must not be adjacent
        let tomato = get_vegetable_by_id("tomato").unwrap();
        let fennel = get_vegetable_by_id("fennel").unwrap();
        let candidates = vec![tomato, fennel];
        let req = minimal_request(3.0, 3.0, Season::Summer);
        let resp = plan_garden(candidates, &req).unwrap();

        // Find positions of tomato and fennel
        let mut tomato_pos = None;
        let mut fennel_pos = None;
        for (r, row) in first_grid(&resp).iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                if cell.id() == Some("tomato") {
                    tomato_pos = Some((r, c));
                }
                if cell.id() == Some("fennel") {
                    fennel_pos = Some((r, c));
                }
            }
        }

        if let (Some((tr, tc)), Some((fr, fc))) = (tomato_pos, fennel_pos) {
            let row_dist = (tr as i32 - fr as i32).abs();
            let col_dist = (tc as i32 - fc as i32).abs();
            // They must not be direct neighbours (Manhattan distance > 1)
            let adjacent = (row_dist == 1 && col_dist == 0) || (row_dist == 0 && col_dist == 1);
            assert!(
                !adjacent,
                "Tomato [{tr},{tc}] and fennel [{fr},{fc}] must not be adjacent"
            );
        }
    }

    #[test]
    fn test_empty_candidates_returns_empty_grid() {
        let req = minimal_request(1.0, 1.0, Season::Summer);
        let resp = plan_garden(vec![], &req).unwrap();
        let all_empty = first_grid(&resp)
            .iter()
            .flat_map(|r: &Vec<PlannedCell>| r.iter())
            .all(|c| !c.is_placed());
        assert!(all_empty, "Grid must be empty when there are no candidates");
    }

    #[test]
    fn test_blocked_cells_are_never_planted() {
        // 2x2 grid (0.6m x 0.6m) with [0][0] and [1][1] blocked
        let req = PlanRequest {
            layout: vec![
                vec![LayoutCell::Blocked, LayoutCell::Empty],
                vec![LayoutCell::Empty, LayoutCell::Blocked],
            ],
            ..minimal_request(0.6, 0.6, Season::Summer)
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();

        // Blocked cells must carry no vegetable and be flagged
        assert!(
            !first_grid(&resp)[0][0].is_placed(),
            "Blocked cell [0][0] must not have a vegetable"
        );
        assert!(
            first_grid(&resp)[0][0].is_blocked(),
            "Cell [0][0] must be marked as blocked"
        );
        assert!(
            !first_grid(&resp)[1][1].is_placed(),
            "Blocked cell [1][1] must not have a vegetable"
        );
        assert!(
            first_grid(&resp)[1][1].is_blocked(),
            "Cell [1][1] must be marked as blocked"
        );

        // Non-blocked cells must not be flagged
        assert!(
            !first_grid(&resp)[0][1].is_blocked(),
            "Cell [0][1] must not be blocked"
        );
        assert!(
            !first_grid(&resp)[1][0].is_blocked(),
            "Cell [1][0] must not be blocked"
        );
    }

    #[test]
    fn test_fully_blocked_grid_returns_no_placements() {
        // 0.9m × 0.9m → 3×3 grid; mark every cell as blocked
        let req = PlanRequest {
            layout: vec![
                vec![LayoutCell::Blocked; 3],
                vec![LayoutCell::Blocked; 3],
                vec![LayoutCell::Blocked; 3],
            ],
            ..minimal_request(0.9, 0.9, Season::Summer)
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let any_placed = first_grid(&resp)
            .iter()
            .flat_map(|r: &Vec<PlannedCell>| r.iter())
            .any(|c| c.is_placed());
        assert!(
            !any_placed,
            "No vegetable must be placed on a fully blocked grid"
        );
    }

    #[test]
    fn test_preference_quantity_places_multiple_instances() {
        use crate::models::request::PreferenceEntry;
        // 3×3 grid, request 3 basil plants.
        // The fill phase may add more basil — the quantity is a guaranteed minimum.
        let req = PlanRequest {
            preferences: Some(vec![PreferenceEntry {
                id: "basil".into(),
                quantity: Some(3),
            }]),
            ..minimal_request(0.9, 0.9, Season::Summer)
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let basil_count = first_grid(&resp)
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| c.id() == Some("basil"))
            .count();
        assert!(
            basil_count >= 3,
            "Basil must be placed at least 3 times (got {basil_count})"
        );
    }

    #[test]
    fn test_preference_quantity_is_plant_count_not_cell_count() {
        use crate::models::request::PreferenceEntry;
        // Tomato: spacing=60cm → span=2 → occupies 2×2=4 cells per plant.
        // Requesting quantity=2 means at least 2 plants (8 cells), not 2 cells.
        let req = PlanRequest {
            preferences: Some(vec![PreferenceEntry {
                id: "tomato".into(),
                quantity: Some(2),
            }]),
            ..minimal_request(1.8, 1.8, Season::Summer)
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let tomato_anchors = first_grid(&resp)
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| c.id() == Some("tomato"))
            .count();
        assert!(
            tomato_anchors >= 2,
            "quantity=2 for tomato (span=2) must yield at least 2 plant placements, got {tomato_anchors}"
        );
    }

    #[test]
    fn test_grid_fully_filled_without_preferences() {
        // 4×4 grid, no preferences → all 16 unblocked cells must be filled
        let req = minimal_request((4.0 * 30.0) / 100.0, (4.0 * 30.0) / 100.0, Season::Summer);
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // A cell is "used" when it is placed (SelfContained, Overflowing, or Overflowed).
        let empty = first_grid(&resp)
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| matches!(c, PlannedCell::Empty))
            .count();
        assert_eq!(
            empty, 0,
            "All cells must be filled: {empty} empty cells remain"
        );
    }

    #[test]
    fn test_french_rank_used_as_fallback() {
        // Small grid, no preferences → tomato (rank 1) must be placed
        let req = minimal_request(0.6, 0.6, Season::Summer);
        let candidates = filter_vegetables(&get_all_vegetables(), &req, Month::June);
        assert!(
            !candidates.is_empty(),
            "Summer must yield at least one candidate"
        );
        assert_eq!(
            candidates[0].id, "tomato",
            "Tomato (french rank 1) must be the first candidate in summer with no preferences"
        );
    }

    #[test]
    fn test_compute_explicit_allocation_honours_quantities() {
        use crate::data::vegetables::get_vegetable_by_id;
        use crate::models::request::PreferenceEntry;
        let basil = get_vegetable_by_id("basil").unwrap(); // span=1, 1 cell/plant
        let tomato = get_vegetable_by_id("tomato").unwrap(); // span=2, 4 cells/plant
        let carrot = get_vegetable_by_id("carrot").unwrap(); // span=1, no preference
        let candidates = vec![basil, tomato, carrot];
        let preferences = vec![
            PreferenceEntry {
                id: "basil".into(),
                quantity: Some(2),
            },
            PreferenceEntry {
                id: "tomato".into(),
                quantity: Some(1),
            },
        ];
        // basil: 2 plants × 1 cell = 2 cells
        // tomato: 1 plant × 4 cells = 4 cells
        // carrot: no explicit quantity — absent from the map
        let allocation = compute_explicit_allocation(&candidates, &preferences, 20);
        assert_eq!(allocation["basil"], 2, "basil: 2 plants × 1 cell");
        assert_eq!(allocation["tomato"], 4, "tomato: 1 plant × 4 cells");
        assert!(
            !allocation.contains_key("carrot"),
            "carrot has no explicit quantity"
        );
    }

    #[test]
    fn test_fill_phase_covers_cells_left_by_unplaceable_large_plants() {
        use crate::models::request::PreferenceEntry;
        // Grid too small for pumpkin (span=4 needs a 4×4 block) — request 1 pumpkin.
        // The fill phase must cover the cells pumpkin could not occupy.
        let req = PlanRequest {
            preferences: Some(vec![PreferenceEntry {
                id: "pumpkin".into(),
                quantity: Some(1),
            }]),
            ..minimal_request(0.9, 0.9, Season::Summer) // 3×3 grid
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let empty = first_grid(&resp)
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| matches!(c, PlannedCell::Empty))
            .count();
        assert_eq!(
            empty, 0,
            "Fill phase must cover cells pumpkin could not occupy; {empty} empty cell(s) remain"
        );
    }

    #[test]
    fn test_harvest_frees_cells_for_replanting() {
        // Multi-week request: plants placed in week 0 with short days_to_harvest
        // are harvested and their cells freed for new plantings in later weeks.
        let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(); // ~9 weeks
        let req = PlanRequest {
            period: Some(Period { start, end }),
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            layout: vec![vec![LayoutCell::Empty]],
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // After merging identical grids the total number of original weeks
        // (sum of week_count) must still reflect that multiple weeks were processed.
        let total_weeks: usize = resp.weeks.iter().map(|w| w.week_count as usize).sum();
        assert!(
            total_weeks > 1,
            "Multi-week request must process multiple WeeklyPlans (got {total_weeks})"
        );
        // Each week must have the correct grid dimensions
        for w in &resp.weeks {
            assert_eq!(w.grid.len(), 1, "Grid must have 1 row");
            assert_eq!(w.grid[0].len(), 1, "Grid must have 1 col");
        }
    }

    #[test]
    fn test_cell_span_values() {
        assert_eq!(cell_span(10), 1, "10 cm fits in 1 cell");
        assert_eq!(cell_span(30), 1, "30 cm fits in 1 cell");
        assert_eq!(cell_span(31), 2, "31 cm needs 2 cells");
        assert_eq!(cell_span(60), 2, "60 cm needs 2 cells");
        assert_eq!(cell_span(90), 3, "90 cm needs 3 cells");
    }

    #[test]
    fn test_merge_consecutive_identical_plans() {
        // An all-blocked layout produces the same grid every week, so all weeks
        // must be merged into a single WeeklyPlan with week_count = 3.
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 22).unwrap(); // Sunday after 3 weeks
        let req = PlanRequest {
            period: Some(Period { start, end }),
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            layout: vec![vec![LayoutCell::Blocked]],
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        assert_eq!(
            resp.weeks.len(),
            1,
            "Three identical blocked-grid weeks must merge into one WeeklyPlan"
        );
        assert_eq!(
            resp.weeks[0].week_count, 3,
            "Merged WeeklyPlan must count all 3 original weeks"
        );
        assert_eq!(
            resp.weeks[0].period.start, start,
            "Merged period must retain the earliest start date"
        );
        assert_eq!(
            resp.weeks[0].period.end, end,
            "Merged period must retain the latest end date"
        );
    }

    #[test]
    fn test_plan_garden_emits_warning_on_unadjusted_period() {
        // A period already on Mon-Sun must NOT produce a normalisation warning.
        let req = minimal_request(0.9, 0.9, Season::Summer); // season_to_dates uses Mon-Sun
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let has_normalisation_warning = resp
            .warnings
            .iter()
            .any(|w| w.contains("adjusted to full weeks"));
        assert!(
            !has_normalisation_warning,
            "No normalisation warning expected for an already-aligned period"
        );
    }

    #[test]
    fn test_plan_garden_no_period_uses_current_week() {
        // No period → must default to current Mon-Sun week and produce exactly 1 weekly plan.
        let req = PlanRequest {
            period: None,
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            layout: vec![vec![LayoutCell::Empty; 3]; 3],
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        assert_eq!(
            resp.weeks.len(),
            1,
            "A None period defaults to the current week → exactly 1 WeeklyPlan"
        );
        // The week must start on a Monday (weekday index 0).
        use chrono::Datelike;
        assert_eq!(
            resp.weeks[0].period.start.weekday(),
            chrono::Weekday::Mon,
            "Default period must start on Monday"
        );
        assert_eq!(
            resp.weeks[0].period.end.weekday(),
            chrono::Weekday::Sun,
            "Default period must end on Sunday"
        );
        // No normalisation warning because current_week() already returns Mon-Sun.
        assert!(
            !resp
                .warnings
                .iter()
                .any(|w| w.contains("adjusted to full weeks")),
            "No normalisation warning expected when period defaults to current week"
        );
    }

    #[test]
    fn test_plan_garden_emits_warning_when_period_adjusted() {
        use crate::models::request::Period;
        let req = PlanRequest {
            period: Some(Period {
                start: NaiveDate::from_ymd_opt(2025, 6, 4).unwrap(), // Wednesday
                end: NaiveDate::from_ymd_opt(2025, 8, 28).unwrap(),  // Thursday
            }),
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            layout: vec![vec![LayoutCell::Empty; 3]; 3],
        };
        let candidates = filter_candidates_base(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let has_normalisation_warning = resp
            .warnings
            .iter()
            .any(|w| w.contains("adjusted to full weeks"));
        assert!(
            has_normalisation_warning,
            "A normalisation warning must be emitted when the period is adjusted"
        );
    }

    #[test]
    fn test_multi_cell_plant_fills_block() {
        use crate::data::vegetables::get_vegetable_by_id;
        // Tomato: 60 cm spacing → span=2 → must occupy a 2×2 block in the grid.
        let tomato = get_vegetable_by_id("tomato").unwrap();
        // 2m × 2m → ceil(200/30)=7 × 7 grid — plenty of room for a 2×2 block.
        let req = minimal_request(2.0, 2.0, Season::Summer);
        let resp = plan_garden(vec![tomato], &req).unwrap();
        let grid = first_grid(&resp);
        // Anchor cells: those with id == "tomato" (SelfContained or Overflowing)
        let anchor_cells: Vec<(usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| c.id() == Some("tomato"))
                    .map(move |(c, _)| (r, c))
            })
            .collect();

        assert!(
            !anchor_cells.is_empty(),
            "Tomato must be placed at least once"
        );

        // Each anchor must be Overflowing with widthCells=2, lengthCells=2
        for (r, c) in &anchor_cells {
            let cell = &grid[*r][*c];
            assert_eq!(
                cell.width_cells(),
                Some(2),
                "Anchor [{r},{c}] must have widthCells=2"
            );
            assert_eq!(
                cell.length_cells(),
                Some(2),
                "Anchor [{r},{c}] must have lengthCells=2"
            );
            assert!(
                cell.covered_by().is_none(),
                "Anchor [{r},{c}] must not have coveredBy"
            );
        }

        // Continuation cells: those pointing back to a tomato anchor
        let anchor_set: std::collections::HashSet<(usize, usize)> =
            anchor_cells.iter().cloned().collect();
        let continuation_count = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, cell)| (r, c, cell)))
            .filter(|(r, c, cell)| {
                cell.covered_by()
                    .map(|cb| anchor_set.contains(&(cb.row, cb.col)))
                    .unwrap_or(false)
                    && !anchor_set.contains(&(*r, *c))
            })
            .count();

        // Each 2×2 block has 1 anchor + 3 continuation cells
        assert_eq!(
            continuation_count,
            anchor_cells.len() * 3,
            "Each 2×2 block must have 3 continuation cells; got {} anchors, {} continuations",
            anchor_cells.len(),
            continuation_count
        );
    }
}
