use crate::domain::models::{
    request::PlanParams,
    variety::{Month, RegionCalendar, Variety},
};

/// Returns the French household consumption rank for a variety ID.
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
    db: &[Variety],
    request: &PlanParams,
    month_filter: Option<Month>,
) -> Vec<Variety> {
    let preferences = request.preferences.clone();

    let mut filtered: Vec<Variety> = db
        .iter()
        .filter(|v| {
            // Filter by region and/or month via calendars.
            // sun / soil / level / exclusions are already handled at SQL level.
            match month_filter {
                Some(month) => v
                    .calendars
                    .iter()
                    .any(|c| c.region == request.region && is_active_month(c, month)),
                None => v.calendars.iter().any(|c| c.region == request.region),
            }
        })
        .cloned()
        .collect();

    // Sort: preferences first (preserving preference order), then by French consumption rank
    filtered.sort_by(|a, b| {
        let a_pos = preferences.iter().position(|p| p.variety.id == a.id);
        let b_pos = preferences.iter().position(|p| p.variety.id == b.id);
        match (a_pos, b_pos) {
            (Some(ai), Some(bi)) => ai.cmp(&bi),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => french_rank(&a.id).cmp(&french_rank(&b.id)),
        }
    });

    filtered
}

/// Filters varieties by all request constraints **including** the calendar month
/// (used to check against per-region sowing/planting windows), then sorts by
/// priority (preferences first, then French consumption rank).
pub fn filter_varieties(db: &[Variety], request: &PlanParams, month: Month) -> Vec<Variety> {
    filter_and_sort_internal(db, request, Some(month))
}

/// Filters varieties by all request constraints **except** month,
/// then sorts by priority. Used by the planner when month is applied
/// per-week internally.
pub fn filter_candidates_base(db: &[Variety], request: &PlanParams) -> Vec<Variety> {
    filter_and_sort_internal(db, request, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{
        request::{LayoutCell, Period, PlanParams, Preference},
        variety::{Month, Region},
    };
    use crate::domain::test_fixtures::{get_all_varieties, get_variety_by_id};
    use chrono::{Duration, NaiveDate};

    fn make_request_for_month(month: u32) -> PlanParams {
        let start = NaiveDate::from_ymd_opt(2025, month, 1).unwrap();
        PlanParams {
            // 2m × 3m → 7 cols × 10 rows
            layout: vec![vec![LayoutCell::Empty; 7]; 10],
            period: Some(Period {
                start,
                end: start + Duration::days(6),
            }),
            region: Region::Temperate,
            preferences: vec![],
            sown: vec![],
        }
    }

    #[test]
    fn test_filter_by_month_june() {
        let db = get_all_varieties();
        let req = make_request_for_month(6);
        let result = filter_varieties(&db, &req, Month::June);
        for v in &result {
            assert!(
                v.calendars.iter().any(|c| is_active_month(c, Month::June)),
                "Variety {} has no June calendar entry",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_by_season_excludes_wrong_season() {
        let db = get_all_varieties();
        // Tomato is a summer crop: must be excluded when filtering for December
        let req = make_request_for_month(12);
        let result = filter_varieties(&db, &req, Month::December);
        assert!(
            !result.iter().any(|v| v.id == "tomato"),
            "Tomato must not appear in December"
        );
    }

    #[test]
    fn test_filter_preferences_boost() {
        let db = get_all_varieties();
        let basil = get_variety_by_id("basil").unwrap();
        let req = PlanParams {
            preferences: vec![Preference {
                variety: basil,
                quantity: None,
            }],
            ..make_request_for_month(6)
        };
        let result = filter_varieties(&db, &req, Month::June);
        if let Some(first) = result.first() {
            assert_eq!(first.id, "basil", "Basil (preferred) must be first");
        }
    }

    #[test]
    fn test_filter_by_region() {
        let db = get_all_varieties();
        let req = PlanParams {
            region: Region::Mountain,
            ..make_request_for_month(5)
        };
        let result = filter_varieties(&db, &req, Month::May);
        for v in &result {
            assert!(
                v.calendars.iter().any(|c| c.region == Region::Mountain),
                "Variety {} has no Mountain calendar entry",
                v.id
            );
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
        let db = get_all_varieties();
        // No preferences — June candidates should be ordered by French rank.
        // Tomato (rank 1) must appear before carrot (rank 2).
        let req = make_request_for_month(6);
        let result = filter_varieties(&db, &req, Month::June);
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
