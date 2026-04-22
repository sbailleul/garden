use crate::domain::models::{
    garden::GardenGrid,
    request::Period,
    response::{PlannedCell, SowingTask, WeeklyPlan},
    variety::Variety,
    Matrix,
};

/// Merges consecutive [`WeeklyPlan`]s that have identical grids.
///
/// When two or more adjacent weeks produce the same garden layout the function
/// collapses them into a single entry: `period.start` is kept from the first
/// entry, `period.end` is taken from the last, `week_count` is incremented by
/// the number of weeks merged, and `score` is accumulated.
pub fn merge_consecutive_plans(plans: Vec<WeeklyPlan>) -> Vec<WeeklyPlan> {
    let mut merged: Vec<WeeklyPlan> = Vec::new();
    for plan in plans {
        match merged.last_mut() {
            Some(last) if last.grid == plan.grid => {
                last.period.end = plan.period.end;
                last.week_count += plan.week_count;
                last.score += plan.score;
                last.sowing_tasks.extend(plan.sowing_tasks);
            }
            _ => merged.push(plan),
        }
    }
    merged
}

/// Converts the current garden grid into a [`WeeklyPlan`] snapshot.
pub fn build_weekly_plan(
    week: Period,
    grid: &GardenGrid,
    score: i32,
    sowing_tasks: Vec<SowingTask>,
) -> WeeklyPlan {
    WeeklyPlan {
        period: week,
        grid: build_grid_cells(grid),
        score,
        week_count: 1,
        sowing_tasks,
    }
}

/// Converts a [`GardenGrid`] into the `Matrix<PlannedCell>` used in API responses.
pub fn build_grid_cells(grid: &GardenGrid) -> Matrix<PlannedCell> {
    grid.cells
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, cell)| match &cell.variety {
                    Some(v)
                        if (row_idx, col_idx) == (v.anchor.row, v.anchor.col) && v.span == 1 =>
                    {
                        PlannedCell::SelfContained {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: v.reason.clone(),
                            plants_per_cell: v.plants_per_cell,
                            estimated_harvest_date: v.estimated_harvest_date,
                        }
                    }
                    Some(v) if (row_idx, col_idx) == (v.anchor.row, v.anchor.col) => {
                        PlannedCell::Overflowing {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            reason: v.reason.clone(),
                            plants_per_cell: v.plants_per_cell,
                            width_cells: v.span,
                            length_cells: v.span,
                            estimated_harvest_date: v.estimated_harvest_date,
                        }
                    }
                    Some(v) => PlannedCell::Overflowed {
                        covered_by: v.anchor,
                    },
                    None if cell.blocked => PlannedCell::Blocked,
                    None => PlannedCell::Empty,
                })
                .collect()
        })
        .collect()
}

/// Generates a descriptive reason string for a planted variety.
pub fn build_reason(variety: &Variety, neighbor_names: &[String], score: i32) -> String {
    if neighbor_names.is_empty() {
        return format!(
            "First placed ({}{}) ",
            variety.category,
            if variety.beginner_friendly {
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
        variety.name,
        qualifier,
        neighbors_str,
        if variety.beginner_friendly {
            " (beginner-friendly)"
        } else {
            ""
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_merge_consecutive_identical_plans() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        let mid = NaiveDate::from_ymd_opt(2025, 6, 8).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 6, 22).unwrap();

        let grid = vec![vec![crate::domain::models::response::PlannedCell::Empty]];

        let plans = vec![
            WeeklyPlan {
                period: Period { start, end: mid },
                grid: grid.clone(),
                score: 10,
                week_count: 1,
                sowing_tasks: vec![],
            },
            WeeklyPlan {
                period: Period {
                    start: mid + chrono::Duration::days(1),
                    end,
                },
                grid,
                score: 10,
                week_count: 1,
                sowing_tasks: vec![],
            },
        ];

        let merged = merge_consecutive_plans(plans);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].week_count, 2);
        assert_eq!(merged[0].score, 20);
    }
}
