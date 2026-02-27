use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod garden;
pub mod request;
pub mod vegetable;

/// Convenience alias for a two-dimensional grid.
pub type Matrix<T> = Vec<Vec<T>>;

/// A zero-based (row, col) position within the garden grid.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}
