use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Coordinate, Matrix};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacedVegetable {
    pub id: String,
    pub name: String,
    pub reason: String,
    /// Number of individual plants that fit in this 30 cm × 30 cm cell.
    pub plants_per_cell: u32,
    /// How many grid cells this plant occupies per axis.
    pub span: u32,
    /// Top-left cell of this plant's block.
    pub anchor: Coordinate,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub vegetable: Option<PlacedVegetable>,
    /// True when the cell is a path, alley or other non-plantable zone.
    pub blocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                        vegetable: None,
                        blocked: false,
                    })
                    .collect()
            })
            .collect();
        Self { rows, cols, cells }
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> Vec<&PlacedVegetable> {
        let mut neighbors = Vec::new();
        let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in directions {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < self.rows as i32 && nc >= 0 && nc < self.cols as i32 {
                if let Some(ref v) = self.cells[nr as usize][nc as usize].vegetable {
                    neighbors.push(v);
                }
            }
        }
        neighbors
    }

    /// Returns true when every cell in the `span × span` block starting at `(row, col)` is free.
    pub fn is_block_free(&self, row: usize, col: usize, span: usize) -> bool {
        if row + span > self.rows || col + span > self.cols {
            return false;
        }
        for dr in 0..span {
            for dc in 0..span {
                let cell = &self.cells[row + dr][col + dc];
                if cell.vegetable.is_some() || cell.blocked {
                    return false;
                }
            }
        }
        true
    }

    /// Returns all distinct already-placed neighbours on the perimeter of a `span × span` block.
    pub fn get_block_neighbors(
        &self,
        row: usize,
        col: usize,
        span: usize,
    ) -> Vec<&PlacedVegetable> {
        let mut seen: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        let mut neighbors: Vec<&PlacedVegetable> = Vec::new();
        let s = span as i32;
        let r0 = row as i32;
        let c0 = col as i32;

        let mut check = |r: i32, c: i32| {
            if r < 0 || c < 0 || r >= self.rows as i32 || c >= self.cols as i32 {
                return;
            }
            let key = (r as usize, c as usize);
            if seen.insert(key) {
                if let Some(ref v) = self.cells[r as usize][c as usize].vegetable {
                    neighbors.push(v);
                }
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
}
