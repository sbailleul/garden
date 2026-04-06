use chrono::{Datelike, NaiveDate};

use crate::domain::models::vegetable::{Region, Vegetable};

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

/// Infers the planting date for a pre-placed vegetable from its regional calendar.
///
/// Selects the most recent planting month (outdoor preferred, indoor as fallback)
/// that falls on or before `planning_start`. Falls back to `planning_start` when
/// no suitable calendar entry exists for the given region.
pub fn infer_planted_date(
    vegetable: &Vegetable,
    region: &Region,
    planning_start: NaiveDate,
) -> NaiveDate {
    let inferred = vegetable
        .calendars
        .iter()
        .find(|c| &c.region == region)
        .and_then(|calendar| {
            let months: &[_] = if !calendar.planting.outdoor.is_empty() {
                &calendar.planting.outdoor
            } else {
                &calendar.planting.indoor
            };

            let planning_year = planning_start.year();
            months
                .iter()
                .flat_map(|&m| {
                    let mn = m.to_u32();
                    [planning_year, planning_year - 1]
                        .into_iter()
                        .filter_map(move |y| NaiveDate::from_ymd_opt(y, mn, 1))
                })
                .filter(|&d| d <= planning_start)
                .max()
        });

    inferred.unwrap_or(planning_start)
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

    #[test]
    fn test_infer_planted_date_picks_most_recent_before_planning_start() {
        use crate::domain::models::vegetable::{
            CalendarWindow, Category, Lifecycle, Month, RegionCalendar, SoilType, SunExposure,
            Vegetable,
        };
        let veg = Vegetable {
            id: "tomato".into(),
            name: "Tomato".into(),
            latin_name: "Solanum lycopersicum".into(),
            calendars: vec![RegionCalendar {
                region: Region::Temperate,
                sowing: CalendarWindow {
                    outdoor: vec![],
                    indoor: vec![Month::February, Month::March],
                },
                planting: CalendarWindow {
                    outdoor: vec![Month::April, Month::May],
                    indoor: vec![],
                },
            }],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy],
            spacing_cm: 50,
            days_to_harvest: 80,
            days_to_plant: 42,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![],
            bad_companions: vec![],
            beginner_friendly: true,
            category: Category::Fruit,
        };

        // Planning starts in June: most recent planting month on/before June is May
        let planning_start = NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
        let result = infer_planted_date(&veg, &Region::Temperate, planning_start);
        assert_eq!(result, NaiveDate::from_ymd_opt(2026, 5, 1).unwrap());
    }

    #[test]
    fn test_infer_planted_date_falls_back_to_previous_year() {
        use crate::domain::models::vegetable::{
            CalendarWindow, Category, Lifecycle, Month, RegionCalendar, SoilType, SunExposure,
            Vegetable,
        };
        let veg = Vegetable {
            id: "tomato".into(),
            name: "Tomato".into(),
            latin_name: "Solanum lycopersicum".into(),
            calendars: vec![RegionCalendar {
                region: Region::Temperate,
                sowing: CalendarWindow {
                    outdoor: vec![],
                    indoor: vec![],
                },
                planting: CalendarWindow {
                    outdoor: vec![Month::April, Month::May],
                    indoor: vec![],
                },
            }],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy],
            spacing_cm: 50,
            days_to_harvest: 80,
            days_to_plant: 42,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![],
            bad_companions: vec![],
            beginner_friendly: true,
            category: Category::Fruit,
        };

        // Planning starts in March: all 2026 planting months (Apr, May) are in the future
        // so the inference falls back to May 2025
        let planning_start = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let result = infer_planted_date(&veg, &Region::Temperate, planning_start);
        assert_eq!(result, NaiveDate::from_ymd_opt(2025, 5, 1).unwrap());
    }

    #[test]
    fn test_infer_planted_date_returns_planning_start_for_unknown_region() {
        use crate::domain::models::vegetable::{
            CalendarWindow, Category, Lifecycle, Month, RegionCalendar, SoilType, SunExposure,
            Vegetable,
        };
        let veg = Vegetable {
            id: "tomato".into(),
            name: "Tomato".into(),
            latin_name: "Solanum lycopersicum".into(),
            calendars: vec![RegionCalendar {
                region: Region::Temperate,
                sowing: CalendarWindow {
                    outdoor: vec![],
                    indoor: vec![],
                },
                planting: CalendarWindow {
                    outdoor: vec![Month::April],
                    indoor: vec![],
                },
            }],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy],
            spacing_cm: 50,
            days_to_harvest: 80,
            days_to_plant: 42,
            lifecycle: Lifecycle::Annual,
            good_companions: vec![],
            bad_companions: vec![],
            beginner_friendly: true,
            category: Category::Fruit,
        };

        let planning_start = NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
        let result = infer_planted_date(&veg, &Region::Mediterranean, planning_start);
        assert_eq!(result, planning_start);
    }
}
