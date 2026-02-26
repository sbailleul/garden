use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::Matrix;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacedVegetable {
    pub id: String,
    pub name: String,
    pub reason: String,
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
}
