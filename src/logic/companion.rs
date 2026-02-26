use crate::models::vegetable::Vegetable;

pub const GOOD_COMPANION_SCORE: i32 = 2;
pub const BAD_COMPANION_SCORE: i32 = -3;

/// Calculates the companion score of a vegetable against its neighbours.
/// +2 per good companion, -3 per bad companion.
pub fn companion_score(vegetable: &Vegetable, neighbor_ids: &[&str]) -> i32 {
    let mut score = 0;
    for neighbor_id in neighbor_ids {
        if vegetable.good_companions.iter().any(|c| c == neighbor_id) {
            score += GOOD_COMPANION_SCORE;
        }
        if vegetable.bad_companions.iter().any(|c| c == neighbor_id) {
            score += BAD_COMPANION_SCORE;
        }
    }
    score
}

/// Returns true if the two vegetables are compatible (neither appears in the other's bad_companions list).
pub fn is_compatible(a: &Vegetable, b: &Vegetable) -> bool {
    !a.bad_companions.iter().any(|c| c == &b.id) && !b.bad_companions.iter().any(|c| c == &a.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vegetables::get_vegetable_by_id;

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
}
