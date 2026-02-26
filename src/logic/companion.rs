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
    !a.bad_companions.iter().any(|c| c == &b.id)
        && !b.bad_companions.iter().any(|c| c == &a.id)
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
        let tomate = get("tomate");
        // basil is a good companion of tomato
        let score = companion_score(&tomate, &["basilic"]);
        assert_eq!(score, GOOD_COMPANION_SCORE, "Tomato + basil must give a positive score");
    }

    #[test]
    fn test_bad_companion_negative_score() {
        let tomate = get("tomate");
        // fennel is a bad companion of tomato
        let score = companion_score(&tomate, &["fenouil"]);
        assert_eq!(score, BAD_COMPANION_SCORE, "Tomato + fennel must give a negative score");
    }

    #[test]
    fn test_neutral_companion_score_zero() {
        let salade = get("salade");
        // thyme is neither good nor bad for lettuce
        let score = companion_score(&salade, &["thym"]);
        assert_eq!(score, 0, "Neutral vegetables must give a score of 0");
    }

    #[test]
    fn test_multiple_neighbors_cumulative() {
        let tomate = get("tomate");
        // basil (+2) + carrot (+2) + fennel (-3) = 1
        let score = companion_score(&tomate, &["basilic", "carotte", "fenouil"]);
        assert_eq!(score, 1);
    }

    #[test]
    fn test_no_neighbors_score_zero() {
        let tomate = get("tomate");
        let score = companion_score(&tomate, &[]);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_is_compatible_good_pair() {
        let tomate = get("tomate");
        let basilic = get("basilic");
        // Tomato and basil are good companions → compatible
        assert!(is_compatible(&tomate, &basilic));
    }

    #[test]
    fn test_is_compatible_bad_pair() {
        let tomate = get("tomate");
        let fenouil = get("fenouil");
        // Tomato + fennel → incompatible
        assert!(!is_compatible(&tomate, &fenouil));
    }

    #[test]
    fn test_is_compatible_symmetric() {
        let tomate = get("tomate");
        let fenouil = get("fenouil");
        assert_eq!(
            is_compatible(&tomate, &fenouil),
            is_compatible(&fenouil, &tomate),
            "Compatibility must be symmetric"
        );
    }

    #[test]
    fn test_is_compatible_neutral_pair() {
        let salade = get("salade");
        let radis = get("radis");
        // Lettuce and radish → compatible (good companions)
        assert!(is_compatible(&salade, &radis));
    }
}
