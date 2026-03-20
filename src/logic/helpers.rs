use chrono::NaiveDate;

/// Size of one grid cell in centimetres
pub const CELL_SIZE_CM: u32 = 30;

/// How many grid cells a plant requires per axis: `ceil(spacing / 30)`, minimum 1.
/// Examples: 10 cm -> 1, 30 cm -> 1, 40 cm -> 2, 60 cm -> 2, 90 cm -> 3.
pub fn cell_span(spacing_cm: u32) -> u32 {
    spacing_cm.div_ceil(CELL_SIZE_CM).max(1)
}

/// Plants per cell:
/// - span == 1 (spacing <= 30 cm): `floor(30 / spacing)^2`
/// - span  > 1 (spacing  > 30 cm): 1 plant occupies the whole spanxspan block.
pub fn plants_per_cell(spacing_cm: u32) -> u32 {
    if cell_span(spacing_cm) > 1 {
        1
    } else {
        let per_axis = (CELL_SIZE_CM / spacing_cm.max(1)).max(1);
        per_axis * per_axis
    }
}

/// Adjusts `days_to_harvest` for pre-placed vegetables based on user-provided
/// planting date and planning start.
///
/// Formula requested by product:
/// `planning_start - planted_date + base_days_to_harvest`.
pub fn adjusted_days_to_harvest(
    base_days_to_harvest: u32,
    planted_date: Option<NaiveDate>,
    planning_start: NaiveDate,
) -> u32 {
    match planted_date {
        None => base_days_to_harvest,
        Some(date) => {
            let adjusted = (planning_start - date).num_days() + i64::from(base_days_to_harvest);
            adjusted.clamp(0, i64::from(u32::MAX)) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_span_values() {
        assert_eq!(cell_span(10), 1, "10 cm fits in 1 cell");
        assert_eq!(cell_span(30), 1, "30 cm fits in 1 cell");
        assert_eq!(cell_span(31), 2, "31 cm needs 2 cells");
        assert_eq!(cell_span(60), 2, "60 cm needs 2 cells");
        assert_eq!(cell_span(90), 3, "90 cm needs 3 cells");
    }
}
