use std::collections::HashMap;

use crate::domain::models::{request::PreferenceEntry, variety::Variety};
use crate::domain::services::helpers::cell_span;

/// Distributes cells for varieties that have an explicit `quantity` preference.
/// Returns a map of `id -> cell count` only for those varieties; everything else
/// (auto-fill candidates) is handled by a separate iterative fill phase.
pub fn compute_explicit_allocation(
    candidates: &[Variety],
    preferences: &[PreferenceEntry],
    available: usize,
) -> HashMap<String, usize> {
    let mut allocation: HashMap<String, usize> = HashMap::new();
    let mut remaining = available;

    for pref in preferences {
        if let Some(qty) = pref.quantity {
            if let Some(v) = candidates.iter().find(|v| v.id == pref.id) {
                let cells_per_plant = (cell_span(v.spacing_cm) as usize).pow(2);
                let cells_needed = (qty as usize).saturating_mul(cells_per_plant);
                let alloc = cells_needed.min(remaining);
                allocation.insert(pref.id.clone(), alloc);
                remaining = remaining.saturating_sub(alloc);
            }
        }
    }

    allocation
}

/// Converts explicit-preference allocations into an ordered placement queue
/// (each variety repeated by its allocated count) and a per-variety placement cap.
/// Varieties without an explicit quantity are not in the queue; they are handled
/// by the iterative fill phase.
pub fn build_placement_queue<'a>(
    candidates: &'a [Variety],
    preferences: &[PreferenceEntry],
    free_cells: usize,
) -> (Vec<&'a Variety>, HashMap<String, usize>) {
    let allocation = compute_explicit_allocation(candidates, preferences, free_cells);

    // Convert cell allocations -> placement counts (one placement = span^2 cells).
    let placements_map: HashMap<String, usize> = candidates
        .iter()
        .filter(|v| allocation.contains_key(&v.id))
        .map(|v| {
            let cells_per_slot = (cell_span(v.spacing_cm) as usize).pow(2);
            let cells = allocation.get(&v.id).copied().unwrap_or(0);
            let n = if cells > 0 {
                (cells / cells_per_slot).max(1)
            } else {
                0
            };
            (v.id.clone(), n)
        })
        .collect();

    // Expand: repeat each variety in preference order by its placement count.
    let queue: Vec<&Variety> = preferences
        .iter()
        .filter_map(|p| candidates.iter().find(|v| v.id == p.id))
        .flat_map(|v| {
            let n = placements_map.get(&v.id).copied().unwrap_or(0);
            std::iter::repeat_n(v, n)
        })
        .collect();
    (queue, placements_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_fixtures::get_variety_by_id;

    #[test]
    fn test_compute_explicit_allocation_honours_quantities() {
        let basil = get_variety_by_id("basil").unwrap();
        let tomato = get_variety_by_id("tomato").unwrap();
        let carrot = get_variety_by_id("carrot").unwrap();
        let candidates = vec![basil, tomato, carrot];
        let preferences = vec![
            PreferenceEntry {
                id: "basil".into(),
                quantity: Some(2),
            },
            PreferenceEntry {
                id: "tomato".into(),
                quantity: Some(1),
            },
        ];
        let allocation = compute_explicit_allocation(&candidates, &preferences, 20);
        assert_eq!(allocation["basil"], 2, "basil: 2 plants x 1 cell");
        assert_eq!(allocation["tomato"], 4, "tomato: 1 plant x 4 cells");
        assert!(
            !allocation.contains_key("carrot"),
            "carrot has no explicit quantity"
        );
    }
}
