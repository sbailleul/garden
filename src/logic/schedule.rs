use chrono::{Datelike, Duration, Local, NaiveDate};
use log::trace;

use crate::models::garden::GardenGrid;

/// A 7-day planning interval.
pub struct WeekRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// Returns `(monday, sunday)` for the current calendar week.
pub fn current_week() -> (NaiveDate, NaiveDate) {
    let today = Local::now().date_naive();
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let sunday = monday + Duration::days(6);
    (monday, sunday)
}

/// Generates 7-day intervals from `start` to `end` (inclusive).
/// The last interval may be shorter than 7 days if the period doesn't divide evenly.
pub fn generate_weeks(start: NaiveDate, end: NaiveDate) -> Vec<WeekRange> {
    let mut weeks = Vec::new();
    let mut week_start = start;
    while week_start <= end {
        let week_end = (week_start + Duration::days(6)).min(end);
        weeks.push(WeekRange {
            start: week_start,
            end: week_end,
        });
        week_start += Duration::days(7);
    }
    weeks
}

/// Snaps a planning period so that it always starts on Monday and ends on Sunday.
/// Returns the (possibly adjusted) `WeekRange` and a flag indicating whether any
/// adjustment was made.
pub fn normalize_period(start: NaiveDate, end: NaiveDate) -> (WeekRange, bool) {
    let normalized_start = start - Duration::days(start.weekday().num_days_from_monday() as i64);
    let normalized_end = end + Duration::days((6 - end.weekday().num_days_from_monday()) as i64);
    let changed = normalized_start != start || normalized_end != end;
    (
        WeekRange {
            start: normalized_start,
            end: normalized_end,
        },
        changed,
    )
}

/// Removes any plant whose harvest week (`planted_week + ⌈days_to_harvest / 7⌉`) is ≤
/// `current_week_idx`, freeing those cells for new plantings.
pub fn harvest_plants(grid: &mut GardenGrid, current_week_idx: usize) {
    for row in &mut grid.cells {
        for cell in row.iter_mut() {
            if let Some(ref v) = cell.vegetable {
                let harvest_week = v.planted_week + (v.days_to_harvest as usize).div_ceil(7);
                if harvest_week <= current_week_idx {
                    trace!(
                        "harvest_plants: harvesting '{}' planted week {} (harvest_week={harvest_week})",
                        v.id, v.planted_week
                    );
                    cell.vegetable = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;

    #[test]
    fn test_current_week_is_monday_to_sunday() {
        let (monday, sunday) = current_week();
        assert_eq!(monday.weekday(), Weekday::Mon);
        assert_eq!(sunday.weekday(), Weekday::Sun);
        assert_eq!(sunday - monday, Duration::days(6));
    }

    #[test]
    fn test_generate_weeks_single_week() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 8).unwrap(); // Sunday
        let weeks = generate_weeks(start, end);
        assert_eq!(weeks.len(), 1);
        let ws = weeks[0].start;
        let we = weeks[0].end;
        assert_eq!(ws, start);
        assert_eq!(we, end);
    }

    #[test]
    fn test_generate_weeks_multiple() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 6, 22).unwrap();
        let weeks = generate_weeks(start, end);
        assert_eq!(weeks.len(), 3);
    }

    #[test]
    fn test_normalize_period_already_aligned() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 8, 31).unwrap(); // Sunday
        let (WeekRange { start: ns, end: ne }, changed) = normalize_period(start, end);
        assert!(!changed);
        assert_eq!(ns, start);
        assert_eq!(ne, end);
    }

    #[test]
    fn test_normalize_period_start_snapped_to_monday() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 4).unwrap(); // Wednesday
        let end = NaiveDate::from_ymd_opt(2025, 8, 31).unwrap(); // Sunday
        let (WeekRange { start: ns, end: ne }, changed) = normalize_period(start, end);
        assert!(changed);
        assert_eq!(ns, NaiveDate::from_ymd_opt(2025, 6, 2).unwrap());
        assert_eq!(ne, end);
    }

    #[test]
    fn test_normalize_period_end_snapped_to_sunday() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(); // Thursday
        let (WeekRange { start: ns, end: ne }, changed) = normalize_period(start, end);
        assert!(changed);
        assert_eq!(ns, start);
        assert_eq!(ne, NaiveDate::from_ymd_opt(2025, 8, 3).unwrap());
    }

    #[test]
    fn test_normalize_period_both_adjusted() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(); // Sunday
        let end = NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(); // Thursday
        let (WeekRange { start: ns, end: ne }, changed) = normalize_period(start, end);
        assert!(changed);
        assert_eq!(ns, NaiveDate::from_ymd_opt(2025, 5, 26).unwrap());
        assert_eq!(ne, NaiveDate::from_ymd_opt(2025, 8, 3).unwrap());
    }
}
