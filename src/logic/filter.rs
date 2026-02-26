use crate::models::{
    request::{Level, PlanRequest},
    vegetable::Vegetable,
};

/// Filters vegetables according to request constraints and sorts by priority.
/// User preferences are moved to the top, followed by a stable alphabetical sort.
pub fn filter_vegetables(db: &[Vegetable], request: &PlanRequest) -> Vec<Vegetable> {
    let preferences = request.preferences.clone().unwrap_or_default();
    let is_beginner = matches!(request.level, Some(Level::Beginner));

    let mut filtered: Vec<Vegetable> = db
        .iter()
        .filter(|v| {
            // Filter by season
            if !v.seasons.contains(&request.season) {
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
            // Filter by region
            if let Some(ref region) = request.region {
                if !v.regions.contains(region) {
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

    // Sort: preferences first, then alphabetical
    filtered.sort_by(|a, b| {
        let a_pref = preferences.contains(&a.id);
        let b_pref = preferences.contains(&b.id);
        match (a_pref, b_pref) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vegetables::get_all_vegetables;
    use crate::models::{
        request::{Level, PlanRequest},
        vegetable::{Region, Season, SoilType, SunExposure},
    };

    fn make_request(season: Season) -> PlanRequest {
        PlanRequest {
            width_m: 2.0,
            length_m: 3.0,
            season,
            sun: None,
            soil: None,
            region: None,
            level: None,
            preferences: None,
            existing_layout: None,
            blocked_cells: None,
        }
    }

    #[test]
    fn test_filter_by_season_summer() {
        let db = get_all_vegetables();
        let req = make_request(Season::Summer);
        let result = filter_vegetables(&db, &req);
        for v in &result {
            assert!(
                v.seasons.contains(&Season::Summer),
                "Vegetable {} does not grow in summer",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_by_season_excludes_wrong_season() {
        let db = get_all_vegetables();
        // Tomato only grows in summer → must be excluded in winter
        let req = make_request(Season::Winter);
        let result = filter_vegetables(&db, &req);
        assert!(
            !result.iter().any(|v| v.id == "tomato"),
            "Tomato must not appear in winter"
        );
    }

    #[test]
    fn test_filter_by_beginner_excludes_advanced() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            level: Some(Level::Beginner),
            ..make_request(Season::Summer)
        };
        let result = filter_vegetables(&db, &req);
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
            preferences: Some(vec!["basil".into()]),
            ..make_request(Season::Summer)
        };
        let result = filter_vegetables(&db, &req);
        if let Some(first) = result.first() {
            assert_eq!(first.id, "basil", "Basil (preferred) must be first");
        }
    }

    #[test]
    fn test_filter_by_soil() {
        let db = get_all_vegetables();
        let req = PlanRequest {
            soil: Some(SoilType::Sandy),
            ..make_request(Season::Spring)
        };
        let result = filter_vegetables(&db, &req);
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
            ..make_request(Season::Spring)
        };
        let result = filter_vegetables(&db, &req);
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
            region: Some(Region::Mountain),
            ..make_request(Season::Spring)
        };
        let result = filter_vegetables(&db, &req);
        for v in &result {
            assert!(
                v.regions.contains(&Region::Mountain),
                "Vegetable {} is not suited for mountain region",
                v.id
            );
        }
    }

    #[test]
    fn test_filter_empty_result_incompatible_constraints() {
        let db = get_all_vegetables();
        // Shade + summer + chalky soil + mountain + beginner → very few vegetables
        let req = PlanRequest {
            sun: Some(SunExposure::Shade),
            soil: Some(SoilType::Chalky),
            region: Some(Region::Mountain),
            level: Some(Level::Beginner),
            ..make_request(Season::Summer)
        };
        let result = filter_vegetables(&db, &req);
        for v in &result {
            assert!(v.seasons.contains(&Season::Summer));
            assert!(v.sun_requirement.contains(&SunExposure::Shade));
            assert!(v.soil_types.contains(&SoilType::Chalky));
            assert!(v.regions.contains(&Region::Mountain));
            assert!(v.beginner_friendly);
        }
    }
}
