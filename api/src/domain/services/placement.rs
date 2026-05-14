use std::collections::HashMap;

use chrono::NaiveDate;

use crate::domain::models::{
    garden::GardenGrid,
    variety::{CultivationMode, Variety},
    Coordinate,
};
use crate::domain::services::companion::{companion_score, shade_penalty};
use crate::domain::services::helpers::{cell_span, plants_per_cell};

/// Scans the grid for the free `span × span` block (in `mode`'s stratum) that
/// maximises the companion + shade score for `variety`. Returns
/// `Some((coordinate, score))` or `None` when no valid block exists.
pub fn find_best_block(
    grid: &GardenGrid,
    variety: &Variety,
    mode: &CultivationMode,
    rows: usize,
    cols: usize,
) -> Option<(Coordinate, i32)> {
    let span = cell_span(mode.spacing_cm) as usize;
    let vegetable = &variety.vegetable;
    let stratum_id = &mode.stratum.id;
    let mut best: Option<(Coordinate, i32)> = None;

    for r in 0..=rows.saturating_sub(span) {
        for c in 0..=cols.saturating_sub(span) {
            if !grid.is_block_free_for_stratum(r, c, span, stratum_id) {
                continue;
            }
            let coord = Coordinate { row: r, col: c };
            let neighbor_veg_ids: Vec<&str> = grid
                .get_block_neighbors(coord, span)
                .iter()
                .map(|v| v.vegetable_id.as_str())
                .collect();
            let co_occupants = grid.get_block_co_occupants(coord, span, stratum_id);
            let score =
                companion_score(vegetable, &neighbor_veg_ids) + shade_penalty(mode, &co_occupants);
            if best.is_none_or(|(_, s)| score > s) {
                best = Some((coord, score));
            }
        }
    }

    best
}

/// Fills a single `span × span` block starting at `coordinate` with `variety`
/// in the given `mode`'s stratum layer.
pub fn fill_block(
    grid: &mut GardenGrid,
    variety: &Variety,
    mode: &CultivationMode,
    coordinate: Coordinate,
    reason: &str,
    week_idx: usize,
    week_start: NaiveDate,
) {
    let span = cell_span(mode.spacing_cm) as usize;
    let ppc = plants_per_cell(mode.spacing_cm);
    let placed = crate::domain::models::garden::PlacedVariety {
        id: variety.id.clone(),
        vegetable_id: variety.vegetable.id.clone(),
        name: variety.name.clone(),
        reason: reason.to_owned(),
        plants_per_cell: ppc,
        span: span as u32,
        anchor: coordinate,
        planted_week: week_idx,
        days_to_harvest: variety.days_to_harvest,
        estimated_harvest_date: week_start + chrono::Duration::days(variety.days_to_harvest as i64),
        lifecycle: variety.lifecycle.clone(),
        stratum_id: mode.stratum.id.clone(),
        cultivation_mode_id: mode.id.clone(),
        max_height_cm: mode.max_height_cm,
    };
    for dr in 0..span {
        for dc in 0..span {
            grid.cells[coordinate.row + dr][coordinate.col + dc]
                .layers
                .insert(mode.stratum.id.clone(), placed.clone());
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
/// `queue` is a slice of `(variety, mode)` pairs so that each preference can
/// specify a particular cultivation mode. Returns the cumulative companion score.
pub fn place_candidates(
    grid: &mut GardenGrid,
    queue: &[(Variety, CultivationMode)],
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
        .flat_map(|c| c.layers.values())
        .map(|v| v.id.clone())
        .fold(HashMap::new(), |mut map, id| {
            *map.entry(id).or_insert(0) += 1;
            map
        });

    'outer: for (variety, mode) in queue {
        let max_count = placements_map.get(&variety.id).copied().unwrap_or(0);
        if placed_counts.get(&variety.id).copied().unwrap_or(0) >= max_count {
            continue;
        }

        let span = cell_span(mode.spacing_cm) as usize;

        match find_best_block(grid, variety, mode, week.rows, week.cols) {
            None if span == 1 => {
                break 'outer; // no free single cell in this stratum — grid full for this layer
            }
            None => {
                continue; // no span×span block; smaller plants may still fit
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
                    mode,
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

/// Phase 2 — iterative greedy fill.
///
/// After explicit preferences have been placed, tries every candidate (using its
/// default cultivation mode) and places the best available block. Repeats until a
/// full pass produces zero new placements. This ensures cells left vacant by
/// large-span plants are filled by smaller alternatives.
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
            let mode = variety.default_mode();
            match find_best_block(grid, variety, mode, week.rows, week.cols) {
                None => continue,
                Some((coordinate, score)) => {
                    let span = cell_span(mode.spacing_cm) as usize;
                    let neighbor_names: Vec<String> = grid
                        .get_block_neighbors(coordinate, span)
                        .iter()
                        .map(|v| v.name.clone())
                        .collect();
                    let reason = build_reason_fn(variety, &neighbor_names, score);
                    fill_block(
                        grid,
                        variety,
                        mode,
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
            cell.layers.retain(|_, v| {
                v.lifecycle == Lifecycle::Perennial
                    || v.planted_week + (v.days_to_harvest as usize).div_ceil(7) > current_week_idx
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::garden::PlacedVariety;
    use crate::domain::models::variety::Lifecycle;

    fn make_placed(
        lifecycle: Lifecycle,
        days_to_harvest: u32,
        planted_week: usize,
    ) -> PlacedVariety {
        PlacedVariety {
            id: "test".into(),
            vegetable_id: "test".into(),
            name: "Test".into(),
            reason: "Test".into(),
            plants_per_cell: 1,
            span: 1,
            anchor: Coordinate { row: 0, col: 0 },
            planted_week,
            days_to_harvest,
            estimated_harvest_date: chrono::NaiveDate::from_ymd_opt(2025, 6, 8).unwrap(),
            lifecycle,
            stratum_id: "ground-cover".into(),
            cultivation_mode_id: "test-standard".into(),
            max_height_cm: 40,
        }
    }

    #[test]
    fn test_harvest_frees_cells_for_replanting() {
        let mut grid = GardenGrid::new(1, 1);
        grid.cells[0][0]
            .layers
            .insert("ground-cover".into(), make_placed(Lifecycle::Annual, 7, 0));

        harvest_plants(&mut grid, 1);
        assert!(grid.cells[0][0].layers.is_empty());
    }

    #[test]
    fn test_harvest_keeps_perennial_plants() {
        let mut grid = GardenGrid::new(1, 1);
        grid.cells[0][0]
            .layers
            .insert("canopy".into(), make_placed(Lifecycle::Perennial, 7, 0));

        harvest_plants(&mut grid, 100);
        assert!(
            !grid.cells[0][0].layers.is_empty(),
            "Perennial plants must not be removed after harvest"
        );
    }
}
