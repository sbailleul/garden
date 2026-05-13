use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::models::{variety::Lifecycle, Coordinate, Matrix};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlacedVariety {
    pub id: String,
    pub vegetable_id: String,
    pub name: String,
    pub reason: String,
    /// Number of individual plants that fit in this 30 cm × 30 cm cell.
    pub plants_per_cell: u32,
    /// How many grid cells this plant occupies per axis.
    pub span: u32,
    /// Top-left cell of this plant's block.
    pub anchor: Coordinate,
    /// 0-based index of the week in which this plant was placed.
    pub planted_week: usize,
    /// Days until this plant is ready to harvest (copied from the variety catalogue).
    pub days_to_harvest: u32,
    /// Calendar date when this plant is expected to be ready for harvest.
    pub estimated_harvest_date: chrono::NaiveDate,
    /// Plant lifecycle — `Perennial` plants are never removed from the grid after harvest.
    pub lifecycle: Lifecycle,
    /// Identifier of the height stratum this placement occupies.
    pub stratum_id: String,
    /// Identifier of the cultivation mode used for this placement.
    pub cultivation_mode_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    /// Occupants keyed by stratum id — one `PlacedVariety` per stratum layer.
    pub layers: HashMap<String, PlacedVariety>,
    /// True when the cell is a path, alley or other non-plantable zone.
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GardenGrid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Matrix<Cell>,
}

impl GardenGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = (0..rows)
            .map(|_| {
                (0..cols)
                    .map(|_| Cell {
                        layers: HashMap::new(),
                        blocked: false,
                    })
                    .collect()
            })
            .collect();
        Self { rows, cols, cells }
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<&PlacedVariety> {
        let mut neighbors = Vec::new();
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                neighbors.extend(self.cells[nr as usize][nc as usize].layers.values());
            }
        }
        neighbors
    }

    /// Returns `true` when every cell in the `span × span` block starting at
    /// `(row, col)` has no occupant in `stratum_id` and is not blocked.
    pub fn is_block_free_for_stratum(
        &self,
        row: usize,
        col: usize,
        span: usize,
        stratum_id: &str,
    ) -> bool {
        if row + span > self.rows || col + span > self.cols {
            return false;
        }
        for dr in 0..span {
            for dc in 0..span {
                let cell = &self.cells[row + dr][col + dc];
                if cell.blocked || cell.layers.contains_key(stratum_id) {
                    return false;
                }
            }
        }
        true
    }

    /// Returns all distinct already-placed neighbours on the perimeter of a `span × span` block.
    pub fn get_block_neighbors(&self, coordinate: Coordinate, span: usize) -> Vec<&PlacedVariety> {
        let mut seen: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        let mut neighbors: Vec<&PlacedVariety> = Vec::new();
        let s = span as i32;
        let r0 = coordinate.row as i32;
        let c0 = coordinate.col as i32;

        let mut check = |r: i32, c: i32| {
            if r < 0 || c < 0 || r >= self.rows as i32 || c >= self.cols as i32 {
                return;
            }
            let key = (r as usize, c as usize);
            if seen.insert(key) {
                neighbors.extend(self.cells[r as usize][c as usize].layers.values());
            }
        };

        for d in 0..s {
            check(r0 - 1, c0 + d); // top edge
            check(r0 + s, c0 + d); // bottom edge
            check(r0 + d, c0 - 1); // left edge
            check(r0 + d, c0 + s); // right edge
        }
        neighbors
    }

    /// Returns all plants already occupying any stratum OTHER than `stratum_id`
    /// within the `span × span` block starting at `(row, col)`.
    /// Used to compute shade penalties when placing a canopy plant over lower layers.
    pub fn get_block_co_occupants(
        &self,
        coordinate: Coordinate,
        span: usize,
        stratum_id: &str,
    ) -> Vec<&PlacedVariety> {
        let mut seen_anchors: std::collections::HashSet<(usize, usize, &str)> =
            std::collections::HashSet::new();
        let mut co_occupants: Vec<&PlacedVariety> = Vec::new();

        for dr in 0..span {
            for dc in 0..span {
                let cell = &self.cells[coordinate.row + dr][coordinate.col + dc];
                for (sid, pv) in &cell.layers {
                    if sid != stratum_id {
                        let key = (pv.anchor.row, pv.anchor.col, sid.as_str());
                        if seen_anchors.insert(key) {
                            co_occupants.push(pv);
                        }
                    }
                }
            }
        }
        co_occupants
    }
}
