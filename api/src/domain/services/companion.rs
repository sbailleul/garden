use crate::domain::models::garden::PlacedVariety;
use crate::domain::models::variety::CultivationMode;
use crate::domain::models::vegetable::Vegetable;

pub const GOOD_COMPANION_SCORE: i32 = 2;
pub const BAD_COMPANION_SCORE: i32 = -3;
/// Applied per co-occupant in a lower stratum when a canopy plant casts shade.
pub const SHADE_PENALTY: i32 = -2;

/// Calculates the companion score of a vegetable against its neighbours.
/// +2 per good companion, -3 per bad companion.
pub fn companion_score(vegetable: &Vegetable, neighbor_vegetable_ids: &[&str]) -> i32 {
    let mut score = 0;
    for neighbor_id in neighbor_vegetable_ids {
        if vegetable.good_companions.iter().any(|c| c == neighbor_id) {
            score += GOOD_COMPANION_SCORE;
        }
        if vegetable.bad_companions.iter().any(|c| c == neighbor_id) {
            score += BAD_COMPANION_SCORE;
        }
    }
    score
}

/// Returns a shade penalty for co-occupants that are shorter than the placed plant.
///
/// Any co-occupant whose `max_height_cm` is strictly less than the placed plant's
/// `max_height_cm` is considered shaded and incurs a `-2` penalty.
pub fn shade_penalty(mode: &CultivationMode, co_occupants: &[&PlacedVariety]) -> i32 {
    let shaded = co_occupants
        .iter()
        .filter(|v| v.max_height_cm < mode.max_height_cm)
        .count();
    SHADE_PENALTY * shaded as i32
}

/// Returns true if the two vegetables are compatible (neither appears in the other's bad_companions list).
#[cfg(test)]
pub fn is_compatible(a: &Vegetable, b: &Vegetable) -> bool {
    !a.bad_companions.iter().any(|c| c == &b.id) && !b.bad_companions.iter().any(|c| c == &a.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_fixtures::get_vegetable_by_id;

    fn get(id: &str) -> Vegetable {
        get_vegetable_by_id(id).unwrap_or_else(|| panic!("Vegetable '{}' not found", id))
    }

    #[test]
    fn test_good_companion_positive_score() {
        let tomato = get("tomato");
        // basil is a good companion of tomato
        let score = companion_score(&tomato, &["basil"]);
        assert_eq!(
            score, GOOD_COMPANION_SCORE,
            "Tomato + basil must give a positive score"
        );
    }

    #[test]
    fn test_bad_companion_negative_score() {
        let tomato = get("tomato");
        // fennel is a bad companion of tomato
        let score = companion_score(&tomato, &["fennel"]);
        assert_eq!(
            score, BAD_COMPANION_SCORE,
            "Tomato + fennel must give a negative score"
        );
    }

    #[test]
    fn test_neutral_companion_score_zero() {
        let lettuce = get("lettuce");
        // thyme is neither good nor bad for lettuce
        let score = companion_score(&lettuce, &["thyme"]);
        assert_eq!(score, 0, "Neutral vegetables must give a score of 0");
    }

    #[test]
    fn test_multiple_neighbors_cumulative() {
        let tomato = get("tomato");
        // basil (+2) + carrot (+2) + fennel (-3) = 1
        let score = companion_score(&tomato, &["basil", "carrot", "fennel"]);
        assert_eq!(score, 1);
    }

    #[test]
    fn test_no_neighbors_score_zero() {
        let tomato = get("tomato");
        let score = companion_score(&tomato, &[]);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_is_compatible_good_pair() {
        let tomato = get("tomato");
        let basil = get("basil");
        // Tomato and basil are good companions → compatible
        assert!(is_compatible(&tomato, &basil));
    }

    #[test]
    fn test_is_compatible_bad_pair() {
        let tomato = get("tomato");
        let fennel = get("fennel");
        // Tomato + fennel → incompatible
        assert!(!is_compatible(&tomato, &fennel));
    }

    #[test]
    fn test_is_compatible_symmetric() {
        let tomato = get("tomato");
        let fennel = get("fennel");
        assert_eq!(
            is_compatible(&tomato, &fennel),
            is_compatible(&fennel, &tomato),
            "Compatibility must be symmetric"
        );
    }

    #[test]
    fn test_is_compatible_neutral_pair() {
        let lettuce = get("lettuce");
        let radish = get("radish");
        // Lettuce and radish → compatible (good companions)
        assert!(is_compatible(&lettuce, &radish));
    }

    // ---------------------------------------------------------------------------
    // shade_penalty
    // ---------------------------------------------------------------------------

    fn make_placed_for_shade(max_height_cm: u32) -> PlacedVariety {
        use crate::domain::models::variety::Lifecycle;
        PlacedVariety {
            id: "test".into(),
            vegetable_id: "test".into(),
            name: "Test".into(),
            reason: "Test".into(),
            plants_per_cell: 1,
            span: 1,
            anchor: crate::domain::models::Coordinate { row: 0, col: 0 },
            planted_week: 0,
            days_to_harvest: 60,
            estimated_harvest_date: chrono::NaiveDate::from_ymd_opt(2025, 8, 1).unwrap(),
            lifecycle: Lifecycle::Annual,
            stratum_id: "intermediate".into(),
            cultivation_mode_id: "test-standard".into(),
            max_height_cm,
        }
    }

    fn make_mode_for_shade(max_height_cm: u32) -> CultivationMode {
        use crate::domain::models::variety::Stratum;
        CultivationMode {
            id: "test-mode".into(),
            name: "Test".into(),
            stratum: Stratum {
                id: "intermediate".into(),
                name: "Intermediate".into(),
            },
            spacing_cm: 40,
            min_height_cm: 40,
            max_height_cm,
        }
    }

    #[test]
    fn test_shade_penalty_taller_plant_shades_shorter_co_occupant() {
        // Placed plant max 120 cm > co-occupant max 40 cm → penalty applied
        let mode = make_mode_for_shade(120);
        let short = make_placed_for_shade(40);
        let penalty = shade_penalty(&mode, &[&short]);
        assert_eq!(
            penalty, SHADE_PENALTY,
            "Taller plant must shade shorter co-occupant"
        );
    }

    #[test]
    fn test_shade_penalty_equal_height_no_penalty() {
        // Same height → no shade
        let mode = make_mode_for_shade(120);
        let same = make_placed_for_shade(120);
        let penalty = shade_penalty(&mode, &[&same]);
        assert_eq!(penalty, 0, "Equal-height plants must not shade each other");
    }

    #[test]
    fn test_shade_penalty_shorter_plant_no_penalty() {
        // Placed plant max 40 cm < co-occupant max 120 cm → no penalty
        let mode = make_mode_for_shade(40);
        let tall = make_placed_for_shade(120);
        let penalty = shade_penalty(&mode, &[&tall]);
        assert_eq!(
            penalty, 0,
            "Shorter placed plant must not shade taller co-occupant"
        );
    }

    #[test]
    fn test_shade_penalty_no_co_occupants() {
        let mode = make_mode_for_shade(220);
        let penalty = shade_penalty(&mode, &[]);
        assert_eq!(penalty, 0, "No co-occupants means no shade penalty");
    }

    #[test]
    fn test_shade_penalty_multiple_co_occupants() {
        // Placed plant 220 cm; two short co-occupants (40 cm each) → 2 × penalty
        let mode = make_mode_for_shade(220);
        let short1 = make_placed_for_shade(40);
        let short2 = make_placed_for_shade(40);
        let penalty = shade_penalty(&mode, &[&short1, &short2]);
        assert_eq!(penalty, SHADE_PENALTY * 2);
    }

    #[test]
    fn test_shade_penalty_mixed_co_occupants() {
        // Placed plant 220 cm; one short (40 cm) + one tall (220 cm) → only 1 × penalty
        let mode = make_mode_for_shade(220);
        let short = make_placed_for_shade(40);
        let tall = make_placed_for_shade(220);
        let penalty = shade_penalty(&mode, &[&short, &tall]);
        assert_eq!(penalty, SHADE_PENALTY);
    }
}
