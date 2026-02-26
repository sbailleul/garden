use crate::data::vegetables::get_vegetable_by_id;
use crate::logic::companion::companion_score;
use crate::models::{
    garden::GardenGrid,
    request::{LayoutCell, PlanRequest, PlanResponse, PlannedCell},
    vegetable::Vegetable,
    Matrix,
};

/// Size of one grid cell in centimetres
pub const CELL_SIZE_CM: u32 = 30;

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
    let mut placed_ids: Vec<String> = grid
        .cells
        .iter()
        .flat_map(|r| r.iter())
        .filter_map(|c| c.vegetable.as_ref().map(|v| v.id.clone()))
        .collect();

    'outer: for vegetable in &candidates {
        // Each variety is placed at most once
        if placed_ids.contains(&vegetable.id) {
            continue;
        }

        // Find the best free cell
        let mut best_row = None;
        let mut best_col = None;
        let mut best_score = i32::MIN;

        for r in 0..rows {
            for c in 0..cols {
                if grid.cells[r][c].vegetable.is_some() || grid.cells[r][c].blocked {
                    continue;
                }
                let neighbor_ids: Vec<&str> = grid
                    .get_neighbors(r, c)
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
                .get_neighbors(r, c)
                .iter()
                .map(|v| v.name.clone())
                .collect();

            let reason = build_reason(vegetable, &neighbor_names, best_score);
            grid.cells[r][c].vegetable = Some(crate::models::garden::PlacedVegetable {
                id: vegetable.id.clone(),
                name: vegetable.name.clone(),
                reason,
            });
            placed_ids.push(vegetable.id.clone());
            global_score += best_score;
        } else {
            // Grid is full
            break 'outer;
        }
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
            vegetable.category_label(),
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
    let planned_grid: Matrix<PlannedCell> = grid
        .cells
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match &cell.vegetable {
                    Some(v) => PlannedCell {
                        id: Some(v.id.clone()),
                        name: Some(v.name.clone()),
                        reason: Some(v.reason.clone()),
                        blocked: false,
                    },
                    None => PlannedCell {
                        id: None,
                        name: None,
                        reason: None,
                        blocked: cell.blocked,
                    },
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

impl Vegetable {
    fn category_label(&self) -> &str {
        use crate::models::vegetable::Category;
        match self.category {
            Category::Fruit => "fruit",
            Category::Produce => "produce",
            Category::Herb => "herb",
            Category::Root => "root",
            Category::Bulb => "bulb",
            Category::Leafy => "leafy",
            Category::Pod => "pod",
        }
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
                if cell.id.is_some() {
                    assert!(
                        cell.reason
                            .as_deref()
                            .map(|r: &str| !r.is_empty())
                            .unwrap_or(false),
                        "Every placed cell must have a non-empty reason"
                    );
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
            first_cell.id.as_deref(),
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
                if cell.id.as_deref() == Some("tomato") {
                    tomato_pos = Some((r, c));
                }
                if cell.id.as_deref() == Some("fennel") {
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
            .all(|c| c.id.is_none());
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
            resp.grid[0][0].id.is_none(),
            "Blocked cell [0][0] must not have a vegetable"
        );
        assert!(
            resp.grid[0][0].blocked,
            "Cell [0][0] must be marked as blocked"
        );
        assert!(
            resp.grid[1][1].id.is_none(),
            "Blocked cell [1][1] must not have a vegetable"
        );
        assert!(
            resp.grid[1][1].blocked,
            "Cell [1][1] must be marked as blocked"
        );

        // Non-blocked cells must not be flagged
        assert!(!resp.grid[0][1].blocked, "Cell [0][1] must not be blocked");
        assert!(!resp.grid[1][0].blocked, "Cell [1][0] must not be blocked");
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
            .any(|c| c.id.is_some());
        assert!(
            !any_placed,
            "No vegetable must be placed on a fully blocked grid"
        );
    }
}
