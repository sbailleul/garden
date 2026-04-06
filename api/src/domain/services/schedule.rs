use chrono::{Datelike, Duration, Local};

use crate::domain::models::{request::Period, warnings::Warnings};

impl Warnings {
    /// Adds schedule warning when the requested period is normalized to full weeks.
    fn add_period_adjusted_to_full_weeks(
        &mut self,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    ) {
        self.add(format!(
            "Planning period adjusted to full weeks: {start} (Monday) → {end} (Sunday)."
        ));
    }
}

/// Returns `(monday, sunday)` for the current calendar week.
pub fn current_week() -> Period {
    let today = Local::now().date_naive();
    let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
    let sunday = monday + Duration::days(6);
    Period {
        start: monday,
        end: sunday,
    }
}

/// Generates 7-day intervals from `start` to `end` (inclusive).
/// The last interval may be shorter than 7 days if the period doesn't divide evenly.
pub fn generate_weeks(period: Period) -> Vec<Period> {
    let mut weeks = Vec::new();
    let Period { mut start, end } = period;
    while start <= end {
        let week_end = (start + Duration::days(6)).min(end);
        weeks.push(Period {
            start,
            end: week_end,
        });
        start += Duration::days(7);
    }
    weeks
}

pub struct NormalizedPeriod {
    period: Period,
    has_changed: bool,
}
/// Snaps a planning period so that it always starts on Monday and ends on Sunday.
/// Returns the (possibly adjusted) `Period` and a flag indicating whether any
/// adjustment was made.
pub fn normalize_period(period: &Period) -> NormalizedPeriod {
    let Period { start, end } = period.clone();
    let normalized_start = start - Duration::days(start.weekday().num_days_from_monday() as i64);
    let normalized_end = end + Duration::days((6 - end.weekday().num_days_from_monday()) as i64);
    let changed = normalized_start != start || normalized_end != end;
    NormalizedPeriod {
        period: Period {
            start: normalized_start,
            end: normalized_end,
        },
        has_changed: changed,
    }
}

/// Resolves a planning period from an optional `(start, end)` pair:
/// - falls back to the current Monday–Sunday week when `period` is `None`
/// - snaps any non-Monday start or non-Sunday end to full-week boundaries
///
/// Returns the ordered `Vec<Period>` and any warning messages emitted
/// (e.g. when the period was adjusted to full-week boundaries).
pub fn weeks_for_period(period: &Option<Period>, warnings: &mut Warnings) -> Vec<Period> {
    let period = period.clone().unwrap_or_else(current_week);

    let normalized = normalize_period(&period);
    if normalized.has_changed {
        warnings.add_period_adjusted_to_full_weeks(normalized.period.start, normalized.period.end);
    }

    generate_weeks(normalized.period)
}

/// Removes any plant whose harvest week (`planted_week + ⌈days_to_harvest / 7⌉`) is ≤
/// `current_week_idx`, freeing those cells for new plantings.
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Weekday};

    #[test]
    fn test_current_week_is_monday_to_sunday() {
        let Period { start, end } = current_week();
        assert_eq!(start.weekday(), Weekday::Mon);
        assert_eq!(end.weekday(), Weekday::Sun);
        assert_eq!(end - start, Duration::days(6));
    }

    #[test]
    fn test_generate_weeks_single_week() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 6, 8).unwrap(); // Sunday
        let weeks = generate_weeks(Period { start, end });
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
        let weeks = generate_weeks(Period { start, end });
        assert_eq!(weeks.len(), 3);
    }

    #[test]
    fn test_normalize_period_already_aligned() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 8, 31).unwrap(); // Sunday
        let NormalizedPeriod {
            period: Period { start: ns, end: ne },
            has_changed: changed,
        } = normalize_period(&Period { start, end });
        assert!(!changed);
        assert_eq!(ns, start);
        assert_eq!(ne, end);
    }

    #[test]
    fn test_normalize_period_start_snapped_to_monday() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 4).unwrap(); // Wednesday
        let end: NaiveDate = NaiveDate::from_ymd_opt(2025, 8, 31).unwrap(); // Sunday
        let NormalizedPeriod {
            period: Period { start: ns, end: ne },
            has_changed: changed,
        } = normalize_period(&Period { start, end });
        assert!(changed);
        assert_eq!(ns, NaiveDate::from_ymd_opt(2025, 6, 2).unwrap());
        assert_eq!(ne, end);
    }

    #[test]
    fn test_normalize_period_end_snapped_to_sunday() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 2).unwrap(); // Monday
        let end = NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(); // Thursday
        let NormalizedPeriod {
            period: Period { start: ns, end: ne },
            has_changed: changed,
        } = normalize_period(&Period { start, end });
        assert!(changed);
        assert_eq!(ns, start);
        assert_eq!(ne, NaiveDate::from_ymd_opt(2025, 8, 3).unwrap());
    }

    #[test]
    fn test_normalize_period_both_adjusted() {
        let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(); // Sunday
        let end = NaiveDate::from_ymd_opt(2025, 7, 31).unwrap(); // Thursday
        let NormalizedPeriod {
            period: Period { start: ns, end: ne },
            has_changed: changed,
        } = normalize_period(&Period { start, end });
        assert!(changed);
        assert_eq!(ns, NaiveDate::from_ymd_opt(2025, 5, 26).unwrap());
        assert_eq!(ne, NaiveDate::from_ymd_opt(2025, 8, 3).unwrap());
    }
}
