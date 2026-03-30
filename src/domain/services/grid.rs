use chrono::{Duration, NaiveDate};

use crate::domain::models::{
    garden::GardenGrid, request::LayoutCell, vegetable::Vegetable, warnings::Warnings, Coordinate,
};
use crate::domain::services::helpers::{adjusted_days_to_harvest, cell_span, plants_per_cell};

/// Grid dimensions returned by layout validation: `(rows, cols)`.
pub struct GridSize(pub usize, pub usize);

/// Grid occupancy counts returned by the occupancy check: `(occupied, blocked)`.
pub struct GridOccupancy(pub usize, pub usize);

/// A deferred continuation cell and its anchor coordinate: `(position, anchor)`.
struct DeferredCell(Coordinate, Coordinate);

/// Validates that the layout has at least one non-empty row.
/// Returns `GridSize(rows, cols)` on success.
pub fn validate_layout(layout: &[Vec<LayoutCell>]) -> Result<GridSize, String> {
    if layout.is_empty() {
        return Err("Layout must contain at least one row.".into());
    }
    let cols = layout[0].len();
    if cols == 0 {
        return Err("Layout rows must not be empty.".into());
    }
    Ok(GridSize(layout.len(), cols))
}

/// Creates a blank grid and pre-fills it from the unified layout array:
/// blocked zones (`true`) and pre-placed vegetables (`"id"`).
/// Returns the grid and any warnings produced (e.g. unknown vegetable IDs).
/// The `lookup` closure resolves a vegetable ID to a `Vegetable` without any I/O dependency.
pub fn initialize_grid(
    rows: usize,
    cols: usize,
    layout: &[Vec<LayoutCell>],
    planning_start: NaiveDate,
    lookup: impl Fn(&str) -> Option<Vegetable>,
    warnings: &mut Warnings,
) -> GardenGrid {
    let mut grid = GardenGrid::new(rows, cols);
    // Continuation cells are collected here and resolved after all anchors are placed.
    let mut deferred: Vec<DeferredCell> = Vec::new();

    for (r, row) in layout.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            match cell {
                LayoutCell::Blocked => {
                    grid.cells[r][c].blocked = true;
                }
                LayoutCell::SelfContained {
                    id,
                    plants_per_cell: ppc_input,
                    planted_date,
                } => {
                    if let Some(v) = lookup(id) {
                        let ppc = ppc_input.unwrap_or_else(|| plants_per_cell(v.spacing_cm));
                        let adjusted_days = adjusted_days_to_harvest(
                            v.days_to_harvest,
                            *planted_date,
                            planning_start,
                        );
                        grid.cells[r][c].vegetable =
                            Some(crate::domain::models::garden::PlacedVegetable {
                                id: v.id.clone(),
                                name: v.name.clone(),
                                reason: "Present in the existing layout.".into(),
                                plants_per_cell: ppc,
                                span: 1,
                                anchor: Coordinate { row: r, col: c },
                                planted_week: 0,
                                days_to_harvest: adjusted_days,
                                estimated_harvest_date: planted_date
                                    .map(|d| d + Duration::days(adjusted_days as i64)),
                            });
                    } else {
                        warnings.add(format!(
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
                    planted_date,
                } => {
                    if let Some(v) = lookup(id) {
                        let span = cell_span(v.spacing_cm);
                        let ppc = ppc_input.unwrap_or_else(|| plants_per_cell(v.spacing_cm));
                        let w = width_cells.unwrap_or(span);
                        let l = length_cells.unwrap_or(span);
                        let adjusted_days = adjusted_days_to_harvest(
                            v.days_to_harvest,
                            *planted_date,
                            planning_start,
                        );
                        grid.cells[r][c].vegetable =
                            Some(crate::domain::models::garden::PlacedVegetable {
                                id: v.id.clone(),
                                name: v.name.clone(),
                                reason: "Present in the existing layout.".into(),
                                plants_per_cell: ppc,
                                span: w.max(l),
                                anchor: Coordinate { row: r, col: c },
                                planted_week: 0,
                                days_to_harvest: adjusted_days,
                                estimated_harvest_date: planted_date
                                    .map(|d| d + Duration::days(adjusted_days as i64)),
                            });
                    } else {
                        warnings.add(format!(
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
                grid.cells[position_row][position_col].vegetable = Some(anchor_veg);
            } else {
                warnings.add(format!(
                    "Continuation cell [{position_row},{position_col}] references an unplanted anchor [{covered_by_row},{covered_by_col}], skipped."
                ));
            }
        } else {
            warnings.add(format!(
                "Continuation cell [{position_row},{position_col}] references out-of-bounds anchor [{covered_by_row},{covered_by_col}], skipped."
            ));
        }
    }

    grid
}

/// Returns `GridOccupancy(occupied, blocked)` cell counts for the given grid.
pub fn count_grid_occupancy(grid: &GardenGrid) -> GridOccupancy {
    let flat = || grid.cells.iter().flat_map(|r| r.iter());
    let occupied = flat().filter(|c| c.vegetable.is_some()).count();
    let blocked = flat().filter(|c| c.blocked).count();
    GridOccupancy(occupied, blocked)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_zero_width_returns_error() {
        let layout: Vec<Vec<LayoutCell>> = vec![];
        let result = validate_layout(&layout);
        assert!(result.is_err());
    }
}
