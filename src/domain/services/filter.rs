use crate::domain::models::{
    request::{Level, PlanRequest},
    vegetable::{Month, RegionCalendar, Vegetable},
};

/// Returns the French household consumption rank for a vegetable ID.
/// Rank 1 = most consumed; unknown IDs get rank 999.
pub fn french_rank(id: &str) -> usize {
    match id {
        "tomato" => 1,
        "carrot" => 2,
        "leek" => 3,
        "lettuce" => 4,
        "green-bean" => 5,
        "zucchini" => 6,
        "cucumber" => 7,
        "onion" => 8,
        "cabbage" => 9,
        "spinach" => 10,
        "pepper" => 11,
        "red-pepper" => 12,
        "broccoli" => 13,
        "eggplant" => 14,
        "cauliflower" => 15,
        "pea" => 16,
        "beet" => 17,
        "radish" => 18,
        "potato" => 19,
        "garlic" => 20,
        "pumpkin" => 21,
        "celery" => 22,
        "fennel" => 23,
        "turnip" => 24,
        "asparagus" => 25,
        "artichoke" => 26,
        "strawberry" => 27,
        "basil" => 28,
        "parsley" => 29,
        "chive" => 30,
        "mint" => 31,
        "thyme" => 32,
        "rosemary" => 33,
        "maïs" => 34,
        _ => 999,
    }
}

/// Returns `true` when `month` is an active sowing or planting month
/// (outdoor or indoor) for the given [`RegionCalendar`].
pub(crate) fn is_active_month(cal: &RegionCalendar, month: Month) -> bool {
    cal.sowing.outdoor.contains(&month)
        || cal.sowing.indoor.contains(&month)
        || cal.planting.outdoor.contains(&month)
        || cal.planting.indoor.contains(&month)
}

/// Internal helper: filters and sorts candidates, optionally restricting to a given month.
fn filter_and_sort_internal(
    db: &[Vegetable],
    request: &PlanRequest,
    month_filter: Option<Month>,
) -> Vec<Vegetable> {
    let preferences = request.preferences.clone().unwrap_or_default();
    let is_beginner = matches!(request.level, Some(Level::Beginner));

    let mut filtered: Vec<Vegetable> = db
        .iter()
        .filter(|v| {
            // Filter by region and/or month via calendars
            let region_match = match month_filter {
                Some(month) => v
                    .calendars
                    .iter()
                    .any(|c| c.region == request.region && is_active_month(c, month)),
                None => v.calendars.iter().any(|c| c.region == request.region),
            };
            if !region_match {
                return false;
            }
            // Filter by sun exposure
            if let Some(ref sun) = request.sun {
                if !v.sun_requirement.contains(sun) {
                    return false;
                }
            }
            // Filter by soil type
            if let Some(ref soil) = request.soil {
                if !v.soil_types.contains(soil) {
                    return false;
                }
            }
            // Filter by skill level
            if is_beginner && !v.beginner_friendly {
                return false;
            }
            true
        })
        .cloned()
        .collect();

    // Sort: preferences first (preserving preference order), then by French consumption rank
    filtered.sort_by(|a, b| {
        let a_pos = preferences.iter().position(|p| p.id == a.id);
        let b_pos = preferences.iter().position(|p| p.id == b.id);
        match (a_pos, b_pos) {
            (Some(ai), Some(bi)) => ai.cmp(&bi),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => french_rank(&a.id).cmp(&french_rank(&b.id)),
        }
    });

    filtered
}

/// Filters vegetables by all request constraints **including** the calendar month
/// (used to check against per-region sowing/planting windows), then sorts by
/// priority (preferences first, then French consumption rank).
pub fn filter_vegetables(db: &[Vegetable], request: &PlanRequest, month: Month) -> Vec<Vegetable> {
    filter_and_sort_internal(db, request, Some(month))
}

/// Filters vegetables by all request constraints **except** month,
/// then sorts by priority. Used by the planner when month is applied
/// per-week internally.
pub fn filter_candidates_base(db: &[Vegetable], request: &PlanRequest) -> Vec<Vegetable> {
    filter_and_sort_internal(db, request, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::outbound::memory::vegetable_repository::get_all_vegetables;
    use crate::domain::models::{
        request::{LayoutCell, Level, Period, PlanRequest, PreferenceEntry},
        vegetable::{Month, Region, SoilType, SunExposure},
    };
    use chrono::{Duration, NaiveDate};

    fn make_request_for_month(month: u32) -> PlanRequest {
        let start = NaiveDate::from_ymd_opt(2025, month, 1).unwrap();
        PlanRequest {
            // 2m × 3m → 7 cols × 10 rows
            layout: vec![vec![LayoutCell::Empty; 7]; 10],
            period: Some(Period {
                start,
                end: start + Duration::days(6),
            }),
            sun: None,
            soil: None,
            region: Region::Temperate,
            level: None,
            preferences: None,
        }
    }

    #[test]
    fn test_filter_by_month_june() {
        let db = get_all_vegetables();
        let req = make_request_for_month(6);
        let result = filter_vegetables(&db, &req, Month::June);
        for v in &result {
            assert!(
                v.calendars.iter().any(|c| is_active_month(c, Month::June)),
                "Vegetable {} has no June calendar entry",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_by_season_excludes_wrong_season() {
        let db = get_all_vegetables();
        // Tomato is a summer crop: must be excluded when filtering for December
        let req = make_request_for_month(12);
        let result = filter_vegetables(&db, &req, Month::December);
        assert!(
            !result.iter().any(|v| v.id == "tomato"),
            "Tomato must not appear in December"
        );
    }

    #[test]
    fn test_filter_by_beginner_excludes_advanced() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            level: Some(Level::Beginner),
            ..make_request_for_month(6)
        };
        let result = filter_vegetables(&db, &req, Month::June);
        for v in &result {
            assert!(
                v.beginner_friendly,
                "Vegetable {} is not beginner-friendly",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_preferences_boost() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            preferences: Some(vec![PreferenceEntry {
                id: "basil".into(),
                quantity: None,
            }]),
            ..make_request_for_month(6)
        };
        let result = filter_vegetables(&db, &req, Month::June);
        if let Some(first) = result.first() {
            assert_eq!(first.id, "basil", "Basil (preferred) must be first");
        }
    }

    #[test]
    fn test_filter_by_soil() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            soil: Some(SoilType::Sandy),
            ..make_request_for_month(4)
        };
        let result = filter_vegetables(&db, &req, Month::April);
        for v in &result {
            assert!(
                v.soil_types.contains(&SoilType::Sandy),
                "Vegetable {} is not suited for sandy soil",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_by_sun() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            sun: Some(SunExposure::Shade),
            ..make_request_for_month(4)
        };
        let result = filter_vegetables(&db, &req, Month::April);
        for v in &result {
            assert!(
                v.sun_requirement.contains(&SunExposure::Shade),
                "Vegetable {} does not tolerate shade",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_by_region() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            region: Region::Mountain,
            ..make_request_for_month(5)
        };
        let result = filter_vegetables(&db, &req, Month::May);
        for v in &result {
            assert!(
                v.calendars.iter().any(|c| c.region == Region::Mountain),
                "Vegetable {} has no Mountain calendar entry",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_empty_result_incompatible_constraints() {
        let db = get_all_vegetables();
        // Shade + June + chalky soil + Mountain + beginner → very few vegetables
        let req = PlanRequest {
            sun: Some(SunExposure::Shade),
            soil: Some(SoilType::Chalky),
            region: Region::Mountain,
            level: Some(Level::Beginner),
            ..make_request_for_month(6)
        };
        let result = filter_vegetables(&db, &req, Month::June);
        for v in &result {
            assert!(v
                .calendars
                .iter()
                .any(|c| c.region == Region::Mountain && is_active_month(c, Month::June)));
            assert!(v.sun_requirement.contains(&SunExposure::Shade));
            assert!(v.soil_types.contains(&SoilType::Chalky));
            assert!(v.beginner_friendly);
        }
    }

    #[test]
    fn test_french_rank_known() {
        assert_eq!(super::french_rank("tomato"), 1);
        assert_eq!(super::french_rank("maïs"), 34);
    }

    #[test]
    fn test_french_rank_unknown() {
        assert_eq!(super::french_rank("dragon"), 999);
    }

    #[test]
    fn test_sort_uses_french_rank_for_non_preferences() {
        let db = get_all_vegetables();
        // No preferences — June candidates should be ordered by French rank.
        // Tomato (rank 1) must appear before carrot (rank 2).
        let req = make_request_for_month(6);
        let result = filter_vegetables(&db, &req, Month::June);
        let tomato_pos = result.iter().position(|v| v.id == "tomato");
        let carrot_pos = result.iter().position(|v| v.id == "carrot");
        if let (Some(tp), Some(cp)) = (tomato_pos, carrot_pos) {
            assert!(
                tp < cp,
                "Tomato (rank 1) must appear before carrot (rank 2); got positions {tp} vs {cp}"
            );
        }
    }
}
