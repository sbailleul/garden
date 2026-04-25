use std::collections::HashMap;

use chrono::NaiveDate;

use crate::domain::models::{garden::GardenGrid, variety::Variety, Coordinate};
use crate::domain::services::companion::companion_score;
use crate::domain::services::helpers::{cell_span, plants_per_cell};

/// Scans the grid for the free `span x span` block that maximises the companion score
/// for `variety`. Returns `Some((coordinate, score))` or `None` when no valid block exists.
pub fn find_best_block(
    grid: &GardenGrid,
    variety: &Variety,
    rows: usize,
    cols: usize,
) -> Option<(Coordinate, i32)> {
    let span = cell_span(variety.spacing_cm) as usize;
    let vegetable = &variety.vegetable;
    let mut best: Option<(Coordinate, i32)> = None;

    for r in 0..=rows.saturating_sub(span) {
        for c in 0..=cols.saturating_sub(span) {
            if !grid.is_block_free(r, c, span) {
                continue;
            }
            let score = {
                let neighbor_veg_ids: Vec<&str> = grid
                    .get_block_neighbors(Coordinate { row: r, col: c }, span)
                    .iter()
                    .map(|v| v.vegetable_id.as_str())
                    .collect();
                companion_score(vegetable, &neighbor_veg_ids)
            };
            if best.is_none_or(|(_, s)| score > s) {
                best = Some((Coordinate { row: r, col: c }, score));
            }
        }
    }

    best
}

/// Fills a single `span x span` block starting at `(row, col)` with `variety`.
pub fn fill_block(
    grid: &mut GardenGrid,
    variety: &Variety,
    coordinate: Coordinate,
    reason: &str,
    week_idx: usize,
    week_start: NaiveDate,
) {
    let span = cell_span(variety.spacing_cm) as usize;
    let ppc = plants_per_cell(variety.spacing_cm);
    for dr in 0..span {
        for dc in 0..span {
            grid.cells[coordinate.row + dr][coordinate.col + dc].variety =
                Some(crate::domain::models::garden::PlacedVariety {
                    id: variety.id.clone(),
                    vegetable_id: variety.vegetable.id.clone(),
                    name: variety.name.clone(),
                    reason: reason.to_owned(),
                    plants_per_cell: ppc,
                    span: span as u32,
                    anchor: coordinate,
                    planted_week: week_idx,
                    days_to_harvest: variety.days_to_harvest,
                    estimated_harvest_date: week_start
                        + chrono::Duration::days(variety.days_to_harvest as i64),
                    lifecycle: variety.lifecycle.clone(),
                });
        }
    }
}

/// Shared context for a single planning week passed to placement functions.
pub struct PlacementWeek {
    pub rows: usize,
    pub cols: usize,
    pub week_idx: usize,
    pub week_start: NaiveDate,
}

/// Iterates over the placement queue and greedily places each variety on the grid.
/// Returns the cumulative companion score.
pub fn place_candidates(
    grid: &mut GardenGrid,
    queue: &[&Variety],
    placements_map: &HashMap<String, usize>,
    week: &PlacementWeek,
    build_reason_fn: impl Fn(&Variety, &[String], i32) -> String,
) -> i32 {
    let mut global_score: i32 = 0;

    // Seed placement counts from anything already in the grid (pre-filled cells).
    let mut placed_counts: HashMap<String, usize> = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter_map(|c| c.variety.as_ref().map(|v| v.id.clone()))
        .fold(HashMap::new(), |mut map, id| {
            *map.entry(id).or_insert(0) += 1;
            map
        });

    'outer: for variety in queue {
        let max_count = placements_map.get(&variety.id).copied().unwrap_or(0);
        if placed_counts.get(&variety.id).copied().unwrap_or(0) >= max_count {
            continue;
        }

        let span = cell_span(variety.spacing_cm) as usize;

        match find_best_block(grid, variety, week.rows, week.cols) {
            None if span == 1 => {
                break 'outer; // no free single cell - grid is full
            }
            None => {
                continue; // no spanxspan block; smaller plants may still fit
            }
            Some((coordinate, score)) => {
                let neighbor_names: Vec<String> = grid
                    .get_block_neighbors(coordinate, span)
                    .iter()
                    .map(|v| v.name.clone())
                    .collect();
                let reason = build_reason_fn(variety, &neighbor_names, score);
                fill_block(
                    grid,
                    variety,
                    coordinate,
                    &reason,
                    week.week_idx,
                    week.week_start,
                );
                placed_counts
                    .entry(variety.id.clone())
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
                global_score += score;
            }
        }
    }
    global_score
}

/// Phase 2 - iterative greedy fill.
///
/// After explicit preferences have been placed, tries every candidate in priority
/// order and places the best available block for each. Repeats until a full pass
/// over all candidates produces zero new placements (grid is genuinely full or no
/// candidate fits anywhere). This ensures that cells left vacant by large-span
/// plants that could not find a free block are filled by smaller alternatives.
pub fn fill_remaining_cells(
    grid: &mut GardenGrid,
    candidates: &[Variety],
    week: &PlacementWeek,
    build_reason_fn: impl Fn(&Variety, &[String], i32) -> String,
) -> i32 {
    let mut total_score: i32 = 0;

    loop {
        let mut placements_this_pass = 0usize;

        for variety in candidates {
            match find_best_block(grid, variety, week.rows, week.cols) {
                None => continue,
                Some((coordinate, score)) => {
                    let span = cell_span(variety.spacing_cm) as usize;
                    let neighbor_names: Vec<String> = grid
                        .get_block_neighbors(coordinate, span)
                        .iter()
                        .map(|v| v.name.clone())
                        .collect();
                    let reason = build_reason_fn(variety, &neighbor_names, score);
                    fill_block(
                        grid,
                        variety,
                        coordinate,
                        &reason,
                        week.week_idx,
                        week.week_start,
                    );
                    total_score += score;
                    placements_this_pass += 1;
                }
            }
        }

        if placements_this_pass == 0 {
            break;
        }
    }
    total_score
}

/// Harvests plants by clearing cells where the plant has reached its harvest week.
/// Perennial plants are never removed — they re-grow the following season.
pub fn harvest_plants(grid: &mut GardenGrid, current_week_idx: usize) {
    use crate::domain::models::variety::Lifecycle;
    for row in &mut grid.cells {
        for cell in row.iter_mut() {
            if let Some(ref v) = cell.variety {
                if v.lifecycle == Lifecycle::Perennial {
                    continue;
                }
                let harvest_week = v.planted_week + (v.days_to_harvest as usize).div_ceil(7);
                if harvest_week <= current_week_idx {
                    cell.variety = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harvest_frees_cells_for_replanting() {
        let mut grid = GardenGrid::new(1, 1);
        grid.cells[0][0].variety = Some(crate::domain::models::garden::PlacedVariety {
            id: "test".into(),
            vegetable_id: "test".into(),
            name: "Test".into(),
            reason: "Test".into(),
            plants_per_cell: 1,
            span: 1,
            anchor: Coordinate { row: 0, col: 0 },
            planted_week: 0,
            days_to_harvest: 7,
            estimated_harvest_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 8).unwrap(),
            lifecycle: crate::domain::models::variety::Lifecycle::Annual,
        });

        harvest_plants(&mut grid, 1);
        assert!(grid.cells[0][0].variety.is_none());
    }

    #[test]
    fn test_harvest_keeps_perennial_plants() {
        let mut grid = GardenGrid::new(1, 1);
        grid.cells[0][0].variety = Some(crate::domain::models::garden::PlacedVariety {
            id: "asparagus".into(),
            vegetable_id: "asparagus".into(),
            name: "Asparagus".into(),
            reason: "Test".into(),
            plants_per_cell: 1,
            span: 1,
            anchor: Coordinate { row: 0, col: 0 },
            planted_week: 0,
            days_to_harvest: 7,
            estimated_harvest_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 8).unwrap(),
            lifecycle: crate::domain::models::variety::Lifecycle::Perennial,
        });

        harvest_plants(&mut grid, 100);
        assert!(
            grid.cells[0][0].variety.is_some(),
            "Perennial plants must not be removed after harvest"
        );
    }
}
