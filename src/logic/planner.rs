use std::collections::HashMap;

use crate::data::vegetables::get_vegetable_by_id;
use crate::logic::companion::companion_score;
use crate::models::{
    garden::GardenGrid,
    request::{LayoutCell, PlanRequest, PlanResponse, PlannedCell, PreferenceEntry},
    vegetable::Vegetable,
    Matrix,
};

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

/// Distributes `available` cells across the candidate pool.
/// Pass 1: honour explicit `quantity` preferences (capped at remaining).
/// Pass 2: split leftover cells evenly (round-robin extra to earlier entries).
fn compute_allocation(
    candidates: &[Vegetable],
    preferences: &[PreferenceEntry],
    available: usize,
) -> HashMap<String, usize> {
    let mut allocation: HashMap<String, usize> = HashMap::new();
    let mut remaining = available;

    // Pass 1: explicit quantities
    for pref in preferences {
        if let Some(qty) = pref.quantity {
            if candidates.iter().any(|v| v.id == pref.id) {
                let alloc = (qty as usize).min(remaining);
                allocation.insert(pref.id.clone(), alloc);
                remaining = remaining.saturating_sub(alloc);
            }
        }
    }

    // Pass 2: evenly distribute remainder
    let pool: Vec<&Vegetable> = candidates
        .iter()
        .filter(|v| !allocation.contains_key(&v.id))
        .collect();
    if !pool.is_empty() {
        let base = remaining / pool.len();
        let extra = remaining % pool.len();
        for (i, v) in pool.iter().enumerate() {
            let count = if i < extra { base + 1 } else { base };
            allocation.insert(v.id.clone(), count);
        }
    }

    allocation
}

/// Greedy placement algorithm on the grid.
/// For each candidate vegetable (sorted by priority), chooses the free cell
/// that maximises the companion score against already-placed neighbours.
pub fn plan_garden(
    candidates: Vec<Vegetable>,
    request: &PlanRequest,
) -> Result<PlanResponse, String> {
    if request.layout.is_empty() {
        return Err("Layout must contain at least one row.".into());
    }
    let rows = request.layout.len();
    let cols = request.layout[0].len();
    if cols == 0 {
        return Err("Layout rows must not be empty.".into());
    }
    let total_cells = rows * cols;

    let mut grid = GardenGrid::new(rows, cols);
    let mut warnings: Vec<String> = Vec::new();
    let mut global_score: i32 = 0;

    // Pre-fill the grid from the unified layout (blocked zones and pre-planted vegetables).
    for (r, row) in request.layout.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                LayoutCell::Blocked(true) => {
                    grid.cells[r][c].blocked = true;
                }
                LayoutCell::Planted(id) => {
                    if let Some(v) = get_vegetable_by_id(id) {
                        grid.cells[r][c].vegetable = Some(crate::models::garden::PlacedVegetable {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: "Present in the existing layout.".into(),
                            plants_per_cell: plants_per_cell(v.spacing_cm),
                            span: 1, // pre-placed cells occupy exactly one cell
                            anchor_row: r,
                            anchor_col: c,
                        });
                    } else {
                        warnings.push(format!(
                            "Vegetable '{id}' not found in the database, skipped."
                        ));
                    }
                }
                _ => {} // Free(()) or Blocked(false) — nothing to do
            }
        }
    }

    let occupied: usize = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| c.vegetable.is_some())
        .count();
    let blocked_count: usize = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| c.blocked)
        .count();
    let available_cells = total_cells.saturating_sub(blocked_count);
    if occupied >= available_cells {
        warnings.push("The grid is already fully occupied by the existing layout.".into());
        return Ok(build_response(grid, rows, cols, global_score, warnings));
    }

    // Greedy placement of candidates
    let preferences_slice = request.preferences.as_deref().unwrap_or(&[]);
    let allocation = compute_allocation(
        &candidates,
        preferences_slice,
        available_cells.saturating_sub(occupied),
    );

    // Convert cell allocations to placement counts.
    // A span×span plant uses span² cells per placement.
    // Guarantee at least 1 placement for any candidate with a non-zero cell budget.
    let placements_map: HashMap<String, usize> = candidates
        .iter()
        .map(|v| {
            let cps = (cell_span(v.spacing_cm) as usize).pow(2);
            let cells = allocation.get(&v.id).copied().unwrap_or(0);
            let n = if cells > 0 { (cells / cps).max(1) } else { 0 };
            (v.id.clone(), n)
        })
        .collect();

    // Expand the candidate list: repeat each vegetable according to its placement count.
    let expanded_candidates: Vec<&Vegetable> = candidates
        .iter()
        .flat_map(|v| {
            let n = placements_map.get(&v.id).copied().unwrap_or(0);
            std::iter::repeat_n(v, n)
        })
        .collect();

    let mut placed_counts: HashMap<String, usize> = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter_map(|c| c.vegetable.as_ref().map(|v| v.id.clone()))
        .fold(HashMap::new(), |mut map, id| {
            *map.entry(id).or_insert(0) += 1;
            map
        });

    'outer: for vegetable in &expanded_candidates {
        let max_count = placements_map.get(&vegetable.id).copied().unwrap_or(0);
        let current_count = placed_counts.get(&vegetable.id).copied().unwrap_or(0);
        if current_count >= max_count {
            continue;
        }

        let span = cell_span(vegetable.spacing_cm) as usize;

        // Find the best free span×span block
        let mut best_row = None;
        let mut best_col = None;
        let mut best_score = i32::MIN;

        for r in 0..=rows.saturating_sub(span) {
            for c in 0..=cols.saturating_sub(span) {
                if !grid.is_block_free(r, c, span) {
                    continue;
                }
                let neighbor_ids: Vec<&str> = grid
                    .get_block_neighbors(r, c, span)
                    .iter()
                    .map(|v| v.id.as_str())
                    .collect();
                let score = companion_score(vegetable, &neighbor_ids);
                if score > best_score {
                    best_score = score;
                    best_row = Some(r);
                    best_col = Some(c);
                }
            }
        }

        if let (Some(r), Some(c)) = (best_row, best_col) {
            let neighbor_names: Vec<String> = grid
                .get_block_neighbors(r, c, span)
                .iter()
                .map(|v| v.name.clone())
                .collect();

            let reason = build_reason(vegetable, &neighbor_names, best_score);
            let ppc = plants_per_cell(vegetable.spacing_cm);

            // Fill every cell in the span×span block
            for dr in 0..span {
                for dc in 0..span {
                    grid.cells[r + dr][c + dc].vegetable =
                        Some(crate::models::garden::PlacedVegetable {
                            id: vegetable.id.clone(),
                            name: vegetable.name.clone(),
                            reason: reason.clone(),
                            plants_per_cell: ppc,
                            span: span as u32,
                            anchor_row: r,
                            anchor_col: c,
                        });
                }
            }
            placed_counts
                .entry(vegetable.id.clone())
                .and_modify(|n| *n += 1)
                .or_insert(1);
            global_score += best_score;
        } else if span == 1 {
            // No free single cell exists — grid is genuinely full.
            break 'outer;
        }
        // For span > 1: no suitable block found — skip (grid may still have free single cells).
    }

    // Warn if plantable cells remain empty
    let empty: usize = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|c| c.vegetable.is_none() && !c.blocked)
        .count();
    if empty > 0 {
        warnings.push(format!(
            "{empty} empty cell(s): not enough compatible vegetables to fill the entire grid."
        ));
    }

    Ok(build_response(grid, rows, cols, global_score, warnings))
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

fn build_response(
    grid: GardenGrid,
    rows: usize,
    cols: usize,
    score: i32,
    warnings: Vec<String>,
) -> PlanResponse {
    use crate::models::request::CoveredBy;

    let planned_grid: Matrix<PlannedCell> = grid
        .cells
        .iter()
        .enumerate()
        .map(|(ro, row)| {
            row.iter()
                .enumerate()
                .map(|(co, cell)| match &cell.vegetable {
                    Some(v) if ro == v.anchor_row && co == v.anchor_col && v.span == 1 => {
                        PlannedCell::SelfContained {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: v.reason.clone(),
                            plants_per_cell: v.plants_per_cell,
                        }
                    }
                    Some(v) if ro == v.anchor_row && co == v.anchor_col => {
                        PlannedCell::Overflowing {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: v.reason.clone(),
                            plants_per_cell: v.plants_per_cell,
                            width_cells: v.span,
                            length_cells: v.span,
                        }
                    }
                    Some(v) => PlannedCell::Overflowed {
                        covered_by: CoveredBy {
                            row: v.anchor_row,
                            col: v.anchor_col,
                        },
                    },
                    None if cell.blocked => PlannedCell::Blocked,
                    None => PlannedCell::Empty,
                })
                .collect()
        })
        .collect();

    PlanResponse {
        grid: planned_grid,
        rows,
        cols,
        score,
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vegetables::{get_all_vegetables, get_vegetable_by_id};
    use crate::logic::filter::filter_vegetables;
    use crate::models::{
        request::{LayoutCell, PlanRequest},
        vegetable::Season,
    };

    fn meters_to_cells(meters: f32) -> usize {
        ((meters * 100.0) / 30.0_f32).ceil() as usize
    }

    fn minimal_request(width: f32, length: f32, season: Season) -> PlanRequest {
        let cols = meters_to_cells(width);
        let rows = meters_to_cells(length);
        PlanRequest {
            season,
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            layout: vec![vec![LayoutCell::Free(()); cols]; rows],
        }
    }

    #[test]
    fn test_grid_dimensions_1m_x_1m() {
        let req = minimal_request(1.0, 1.0, Season::Summer);
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // 1m = 100cm / 30 = 3.33 → ceil = 4 cells
        assert_eq!(resp.rows, 4);
        assert_eq!(resp.cols, 4);
    }

    #[test]
    fn test_grid_dimensions_2m_x_3m() {
        let req = minimal_request(2.0, 3.0, Season::Summer);
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
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
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        for row in &resp.grid {
            for cell in row {
                match cell {
                    PlannedCell::SelfContained { reason, .. }
                    | PlannedCell::Overflowing { reason, .. } => {
                        assert!(!reason.is_empty(), "Anchor cell must have a non-empty reason");
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
                vec![LayoutCell::Planted("tomato".into()), LayoutCell::Free(())],
                vec![LayoutCell::Free(()), LayoutCell::Free(())],
            ],
            ..minimal_request(0.6, 0.6, Season::Summer)
        };
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // Cell [0][0] must still be "tomato"
        let first_cell = &resp.grid[0][0];
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
        for (r, row) in resp.grid.iter().enumerate() {
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
        let all_empty = resp
            .grid
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
                vec![LayoutCell::Blocked(true), LayoutCell::Free(())],
                vec![LayoutCell::Free(()), LayoutCell::Blocked(true)],
            ],
            ..minimal_request(0.6, 0.6, Season::Summer)
        };
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();

        // Blocked cells must carry no vegetable and be flagged
        assert!(
            !resp.grid[0][0].is_placed(),
            "Blocked cell [0][0] must not have a vegetable"
        );
        assert!(
            resp.grid[0][0].is_blocked(),
            "Cell [0][0] must be marked as blocked"
        );
        assert!(
            !resp.grid[1][1].is_placed(),
            "Blocked cell [1][1] must not have a vegetable"
        );
        assert!(
            resp.grid[1][1].is_blocked(),
            "Cell [1][1] must be marked as blocked"
        );

        // Non-blocked cells must not be flagged
        assert!(!resp.grid[0][1].is_blocked(), "Cell [0][1] must not be blocked");
        assert!(!resp.grid[1][0].is_blocked(), "Cell [1][0] must not be blocked");
    }

    #[test]
    fn test_fully_blocked_grid_returns_no_placements() {
        // 0.9m × 0.9m → 3×3 grid; mark every cell as blocked
        let req = PlanRequest {
            layout: vec![
                vec![LayoutCell::Blocked(true); 3],
                vec![LayoutCell::Blocked(true); 3],
                vec![LayoutCell::Blocked(true); 3],
            ],
            ..minimal_request(0.9, 0.9, Season::Summer)
        };
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let any_placed = resp
            .grid
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
        // 3×3 grid, request 3 basil plants
        let req = PlanRequest {
            preferences: Some(vec![PreferenceEntry {
                id: "basil".into(),
                quantity: Some(3),
            }]),
            ..minimal_request(0.9, 0.9, Season::Summer)
        };
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        let basil_count = resp
            .grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| c.id() == Some("basil"))
            .count();
        assert_eq!(basil_count, 3, "Basil must be placed exactly 3 times");
    }

    #[test]
    fn test_grid_fully_filled_without_preferences() {
        // 4×4 grid, no preferences → all 16 unblocked cells must be filled
        let req = minimal_request((4.0 * 30.0) / 100.0, (4.0 * 30.0) / 100.0, Season::Summer);
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
        let resp = plan_garden(candidates, &req).unwrap();
        // A cell is "used" when it is placed (SelfContained, Overflowing, or Overflowed).
        let empty = resp
            .grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| matches!(c, PlannedCell::Empty))
            .count();
        assert_eq!(empty, 0, "All cells must be filled: {empty} empty cells remain");
    }

    #[test]
    fn test_french_rank_used_as_fallback() {
        // Small grid, no preferences → tomato (rank 1) must be placed
        let req = minimal_request(0.6, 0.6, Season::Summer);
        let candidates = filter_vegetables(&get_all_vegetables(), &req);
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
    fn test_compute_allocation_distributes_evenly() {
        use crate::data::vegetables::get_vegetable_by_id;
        let tomato = get_vegetable_by_id("tomato").unwrap();
        let carrot = get_vegetable_by_id("carrot").unwrap();
        let leek = get_vegetable_by_id("leek").unwrap();
        let candidates = vec![tomato, carrot, leek];
        let allocation = compute_allocation(&candidates, &[], 10);
        // compute_allocation distributes raw cells: 10 / 3 = base 3, extra 1
        // → first candidate gets 4 cells, rest get 3 cells
        assert_eq!(allocation["tomato"], 4);
        assert_eq!(allocation["carrot"], 3);
        assert_eq!(allocation["leek"], 3);
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
    fn test_multi_cell_plant_fills_block() {
        use crate::data::vegetables::get_vegetable_by_id;
        // Tomato: 60 cm spacing → span=2 → must occupy a 2×2 block in the grid.
        let tomato = get_vegetable_by_id("tomato").unwrap();
        // 2m × 2m → ceil(200/30)=7 × 7 grid — plenty of room for a 2×2 block.
        let req = minimal_request(2.0, 2.0, Season::Summer);
        let resp = plan_garden(vec![tomato], &req).unwrap();

        // Anchor cells: those with id == "tomato" (SelfContained or Overflowing)
        let anchor_cells: Vec<(usize, usize)> = resp
            .grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| c.id() == Some("tomato"))
                    .map(move |(c, _)| (r, c))
            })
            .collect();

        assert!(!anchor_cells.is_empty(), "Tomato must be placed at least once");

        // Each anchor must be Overflowing with widthCells=2, lengthCells=2
        for (r, c) in &anchor_cells {
            let cell = &resp.grid[*r][*c];
            assert_eq!(cell.width_cells(), Some(2), "Anchor [{r},{c}] must have widthCells=2");
            assert_eq!(cell.length_cells(), Some(2), "Anchor [{r},{c}] must have lengthCells=2");
            assert!(cell.covered_by().is_none(), "Anchor [{r},{c}] must not have coveredBy");
        }

        // Continuation cells: those pointing back to a tomato anchor
        let anchor_set: std::collections::HashSet<(usize, usize)> =
            anchor_cells.iter().cloned().collect();
        let continuation_count = resp
            .grid
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
