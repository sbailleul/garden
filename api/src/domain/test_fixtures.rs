//! Variety and Vegetable fixture data for unit tests in the domain layer.
//! This module exists solely for `#[cfg(test)]` use and is never compiled
//! into production binaries.

use std::sync::Arc;

use crate::domain::models::variety::Month::*;
use crate::domain::models::variety::{
    CalendarWindow, Category, Lifecycle, Region, RegionCalendar, SoilType, SunExposure, Variety,
};
use crate::domain::models::vegetable::Vegetable;

pub fn get_variety_by_id(id: &str) -> Option<Variety> {
    get_all_varieties().into_iter().find(|v| v.id == id)
}

pub fn get_vegetable_by_id(id: &str) -> Option<Vegetable> {
    get_all_vegetables().into_iter().find(|v| v.id == id)
}

pub fn get_all_varieties() -> Vec<Variety> {
    vec![
        Variety {
            id: "tomato".into(),
            vegetable: Arc::new(get_vegetable_by_id("tomato").unwrap()),
            name: "Tomato".into(),
            latin_name: "Solanum lycopersicum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 60,
            days_to_harvest: 75,
            days_to_plant: 42,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Variety {
            id: "zucchini".into(),
            vegetable: Arc::new(get_vegetable_by_id("zucchini").unwrap()),
            name: "Zucchini".into(),
            latin_name: "Cucurbita pepo".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 90,
            days_to_harvest: 55,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Variety {
            id: "carrot".into(),
            vegetable: Arc::new(get_vegetable_by_id("carrot").unwrap()),
            name: "Carrot".into(),
            latin_name: "Daucus carota".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Sandy, SoilType::Loamy],
            spacing_cm: 10,
            days_to_harvest: 75,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Root,
        },
        Variety {
            id: "basil".into(),
            vegetable: Arc::new(get_vegetable_by_id("basil").unwrap()),
            name: "Basil".into(),
            latin_name: "Ocimum basilicum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 20,
            days_to_harvest: 30,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Herb,
        },
        Variety {
            id: "lettuce".into(),
            vegetable: Arc::new(get_vegetable_by_id("lettuce").unwrap()),
            name: "Lettuce".into(),
            latin_name: "Lactuca sativa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::PartialShade, SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 30,
            days_to_harvest: 45,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Leafy,
        },
        Variety {
            id: "radish".into(),
            vegetable: Arc::new(get_vegetable_by_id("radish").unwrap()),
            name: "Radish".into(),
            latin_name: "Raphanus sativus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Sandy, SoilType::Loamy],
            spacing_cm: 5,
            days_to_harvest: 25,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Root,
        },
        Variety {
            id: "onion".into(),
            vegetable: Arc::new(get_vegetable_by_id("onion").unwrap()),
            name: "Onion".into(),
            latin_name: "Allium cepa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, July, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy],
            spacing_cm: 10,
            days_to_harvest: 100,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Bulb,
        },
        Variety {
            id: "garlic".into(),
            vegetable: Arc::new(get_vegetable_by_id("garlic").unwrap()),
            name: "Garlic".into(),
            latin_name: "Allium sativum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![October, November, March],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![September, October, November, February, March],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![October, November, March, April],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![October, November, December, March],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy],
            spacing_cm: 10,
            days_to_harvest: 240,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Bulb,
        },
        Variety {
            id: "leek".into(),
            vegetable: Arc::new(get_vegetable_by_id("leek").unwrap()),
            name: "Leek".into(),
            latin_name: "Allium porrum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April],
                        indoor: vec![January, November, December],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Humus],
            spacing_cm: 15,
            days_to_harvest: 120,
            days_to_plant: 60,
            lifecycle: Lifecycle::Biennial,
            beginner_friendly: true,
            category: Category::Bulb,
        },
        Variety {
            id: "green-bean".into(),
            vegetable: Arc::new(get_vegetable_by_id("green-bean").unwrap()),
            name: "Green Bean".into(),
            latin_name: "Phaseolus vulgaris".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Humus],
            spacing_cm: 15,
            days_to_harvest: 55,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Pod,
        },
        Variety {
            id: "cucumber".into(),
            vegetable: Arc::new(get_vegetable_by_id("cucumber").unwrap()),
            name: "Cucumber".into(),
            latin_name: "Cucumis sativus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 60,
            days_to_harvest: 55,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Variety {
            id: "pepper".into(),
            vegetable: Arc::new(get_vegetable_by_id("pepper").unwrap()),
            name: "Pepper".into(),
            latin_name: "Capsicum annuum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 50,
            days_to_harvest: 70,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Variety {
            id: "pea".into(),
            vegetable: Arc::new(get_vegetable_by_id("pea").unwrap()),
            name: "Pea".into(),
            latin_name: "Pisum sativum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, September, October, November],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 10,
            days_to_harvest: 60,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Pod,
        },
        Variety {
            id: "cabbage".into(),
            vegetable: Arc::new(get_vegetable_by_id("brassica").unwrap()),
            name: "Cabbage".into(),
            latin_name: "Brassica oleracea".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February, November, December],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, July, August],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![February, March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 50,
            days_to_harvest: 90,
            days_to_plant: 35,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Variety {
            id: "broccoli".into(),
            vegetable: Arc::new(get_vegetable_by_id("brassica").unwrap()),
            name: "Broccoli".into(),
            latin_name: "Brassica oleracea var. italica".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, July, August],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![February, March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 50,
            days_to_harvest: 80,
            days_to_plant: 35,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Variety {
            id: "parsley".into(),
            vegetable: Arc::new(get_vegetable_by_id("parsley").unwrap()),
            name: "Parsley".into(),
            latin_name: "Petroselinum crispum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August, September],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            February, March, April, May, June, July, August, September, October,
                        ],
                        indoor: vec![January],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 20,
            days_to_harvest: 75,
            days_to_plant: 21,
            lifecycle: Lifecycle::Biennial,
            beginner_friendly: true,
            category: Category::Herb,
        },
        Variety {
            id: "thyme".into(),
            vegetable: Arc::new(get_vegetable_by_id("thyme").unwrap()),
            name: "Thyme".into(),
            latin_name: "Thymus vulgaris".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Sandy, SoilType::Chalky, SoilType::Loamy],
            spacing_cm: 30,
            days_to_harvest: 90,
            days_to_plant: 28,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: true,
            category: Category::Herb,
        },
        Variety {
            id: "rosemary".into(),
            vegetable: Arc::new(get_vegetable_by_id("rosemary").unwrap()),
            name: "Rosemary".into(),
            latin_name: "Salvia rosmarinus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            March, April, May, June, July, August, September, October, November,
                        ],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, September, October, November],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September, October],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, September, October, November],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Sandy, SoilType::Chalky],
            spacing_cm: 40,
            days_to_harvest: 90,
            days_to_plant: 28,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: true,
            category: Category::Herb,
        },
        Variety {
            id: "beet".into(),
            vegetable: Arc::new(get_vegetable_by_id("beet").unwrap()),
            name: "Beet".into(),
            latin_name: "Beta vulgaris".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Clay],
            spacing_cm: 15,
            days_to_harvest: 60,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Root,
        },
        Variety {
            id: "spinach".into(),
            vegetable: Arc::new(get_vegetable_by_id("spinach").unwrap()),
            name: "Spinach".into(),
            latin_name: "Spinacia oleracea".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            February, March, April, May, August, September, October, November,
                        ],
                        indoor: vec![January, February, November, December],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::PartialShade, SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 15,
            days_to_harvest: 40,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Leafy,
        },
        Variety {
            id: "fennel".into(),
            vegetable: Arc::new(get_vegetable_by_id("fennel").unwrap()),
            name: "Fennel".into(),
            latin_name: "Foeniculum vulgare".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy],
            spacing_cm: 30,
            days_to_harvest: 90,
            days_to_plant: 21,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: false,
            category: Category::Herb,
        },
        Variety {
            id: "eggplant".into(),
            vegetable: Arc::new(get_vegetable_by_id("eggplant").unwrap()),
            name: "Eggplant".into(),
            latin_name: "Solanum melongena".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 60,
            days_to_harvest: 75,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Variety {
            id: "celery".into(),
            vegetable: Arc::new(get_vegetable_by_id("celery").unwrap()),
            name: "Celery".into(),
            latin_name: "Apium graveolens".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 30,
            days_to_harvest: 100,
            days_to_plant: 60,
            lifecycle: Lifecycle::Biennial,
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Variety {
            id: "potato".into(),
            vegetable: Arc::new(get_vegetable_by_id("potato").unwrap()),
            name: "Potato".into(),
            latin_name: "Solanum tuberosum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Humus],
            spacing_cm: 35,
            days_to_harvest: 90,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Root,
        },
        Variety {
            id: "maïs".into(),
            vegetable: Arc::new(get_vegetable_by_id("maïs").unwrap()),
            name: "Corn".into(),
            latin_name: "Zea mays".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 40,
            days_to_harvest: 80,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Pod,
        },
        Variety {
            id: "pumpkin".into(),
            vegetable: Arc::new(get_vegetable_by_id("pumpkin").unwrap()),
            name: "Pumpkin".into(),
            latin_name: "Cucurbita maxima".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 120,
            days_to_harvest: 100,
            days_to_plant: 21,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Variety {
            id: "chive".into(),
            vegetable: Arc::new(get_vegetable_by_id("chive").unwrap()),
            name: "Chive".into(),
            latin_name: "Allium schoenoprasum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March, April, May, June, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 20,
            days_to_harvest: 60,
            days_to_plant: 21,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: true,
            category: Category::Herb,
        },
        Variety {
            id: "mint".into(),
            vegetable: Arc::new(get_vegetable_by_id("mint").unwrap()),
            name: "Mint".into(),
            latin_name: "Mentha".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, June, July, August],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::PartialShade, SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus, SoilType::Clay],
            spacing_cm: 30,
            days_to_harvest: 60,
            days_to_plant: 0,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: true,
            category: Category::Herb,
        },
        Variety {
            id: "strawberry".into(),
            vegetable: Arc::new(get_vegetable_by_id("strawberry").unwrap()),
            name: "Strawberry".into(),
            latin_name: "Fragaria × ananassa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May, August, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Sandy, SoilType::Humus],
            spacing_cm: 30,
            days_to_harvest: 90,
            days_to_plant: 30,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: true,
            category: Category::Fruit,
        },
        Variety {
            id: "turnip".into(),
            vegetable: Arc::new(get_vegetable_by_id("turnip").unwrap()),
            name: "Turnip".into(),
            latin_name: "Brassica rapa".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![
                            February, March, April, May, July, August, September, October,
                        ],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, August, September],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun, SunExposure::PartialShade],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 20,
            days_to_harvest: 40,
            days_to_plant: 0,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: true,
            category: Category::Root,
        },
        Variety {
            id: "cauliflower".into(),
            vegetable: Arc::new(get_vegetable_by_id("brassica").unwrap()),
            name: "Cauliflower".into(),
            latin_name: "Brassica oleracea var. botrytis".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September, October],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, July, August],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, September, October, November],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May, July, August],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, September],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mountain,
                    sowing: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Clay, SoilType::Chalky],
            spacing_cm: 60,
            days_to_harvest: 80,
            days_to_plant: 35,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Variety {
            id: "red-pepper".into(),
            vegetable: Arc::new(get_vegetable_by_id("pepper").unwrap()),
            name: "Red Pepper".into(),
            latin_name: "Capsicum annuum".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![January, February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 50,
            days_to_harvest: 70,
            days_to_plant: 60,
            lifecycle: Lifecycle::Annual,
            beginner_friendly: false,
            category: Category::Fruit,
        },
        Variety {
            id: "asparagus".into(),
            vegetable: Arc::new(get_vegetable_by_id("asparagus").unwrap()),
            name: "Asparagus".into(),
            latin_name: "Asparagus officinalis".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Temperate,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Continental,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![February, March],
                        indoor: vec![January, February],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Sandy, SoilType::Loamy],
            spacing_cm: 45,
            days_to_harvest: 730,
            days_to_plant: 90,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: false,
            category: Category::Leafy,
        },
        Variety {
            id: "artichoke".into(),
            vegetable: Arc::new(get_vegetable_by_id("artichoke").unwrap()),
            name: "Artichoke".into(),
            latin_name: "Cynara cardunculus var. scolymus".into(),
            calendars: vec![
                RegionCalendar {
                    region: Region::Mediterranean,
                    sowing: CalendarWindow {
                        outdoor: vec![March, April, May],
                        indoor: vec![February, March],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![April, May, June],
                        indoor: vec![],
                    },
                },
                RegionCalendar {
                    region: Region::Oceanic,
                    sowing: CalendarWindow {
                        outdoor: vec![April, May],
                        indoor: vec![March, April],
                    },
                    planting: CalendarWindow {
                        outdoor: vec![May, June, July],
                        indoor: vec![],
                    },
                },
            ],
            sun_requirement: vec![SunExposure::FullSun],
            soil_types: vec![SoilType::Loamy, SoilType::Humus],
            spacing_cm: 80,
            days_to_harvest: 365,
            days_to_plant: 60,
            lifecycle: Lifecycle::Perennial,
            beginner_friendly: false,
            category: Category::Leafy,
        },
    ]
}

pub fn get_all_vegetables() -> Vec<Vegetable> {
    vec![
        Vegetable {
            id: "tomato".into(),
            name: "Tomato".into(),
            variety_ids: vec!["tomato".into()],
            good_companions: vec![
                "basil".into(),
                "carrot".into(),
                "parsley".into(),
                "garlic".into(),
                "onion".into(),
            ],
            bad_companions: vec!["fennel".into(), "brassica".into()],
        },
        Vegetable {
            id: "zucchini".into(),
            name: "Zucchini".into(),
            variety_ids: vec!["zucchini".into()],
            good_companions: vec!["green-bean".into(), "maïs".into(), "radish".into()],
            bad_companions: vec!["potato".into()],
        },
        Vegetable {
            id: "carrot".into(),
            name: "Carrot".into(),
            variety_ids: vec!["carrot".into()],
            good_companions: vec![
                "tomato".into(),
                "onion".into(),
                "leek".into(),
                "lettuce".into(),
                "radish".into(),
            ],
            bad_companions: vec!["dill".into(), "fennel".into()],
        },
        Vegetable {
            id: "basil".into(),
            name: "Basil".into(),
            variety_ids: vec!["basil".into()],
            good_companions: vec!["tomato".into(), "pepper".into(), "asparagus".into()],
            bad_companions: vec!["sage".into(), "thyme".into()],
        },
        Vegetable {
            id: "lettuce".into(),
            name: "Lettuce".into(),
            variety_ids: vec!["lettuce".into()],
            good_companions: vec![
                "carrot".into(),
                "radish".into(),
                "strawberry".into(),
                "cucumber".into(),
            ],
            bad_companions: vec!["parsley".into(), "celery".into()],
        },
        Vegetable {
            id: "radish".into(),
            name: "Radish".into(),
            variety_ids: vec!["radish".into()],
            good_companions: vec![
                "carrot".into(),
                "lettuce".into(),
                "tomato".into(),
                "cucumber".into(),
            ],
            bad_companions: vec!["hyssop".into()],
        },
        Vegetable {
            id: "onion".into(),
            name: "Onion".into(),
            variety_ids: vec!["onion".into()],
            good_companions: vec![
                "carrot".into(),
                "tomato".into(),
                "beet".into(),
                "lettuce".into(),
            ],
            bad_companions: vec!["green-bean".into(), "pea".into(), "garlic".into()],
        },
        Vegetable {
            id: "garlic".into(),
            name: "Garlic".into(),
            variety_ids: vec!["garlic".into()],
            good_companions: vec![
                "tomato".into(),
                "rose".into(),
                "strawberry".into(),
                "carrot".into(),
            ],
            bad_companions: vec!["onion".into(), "green-bean".into(), "pea".into()],
        },
        Vegetable {
            id: "leek".into(),
            name: "Leek".into(),
            variety_ids: vec!["leek".into()],
            good_companions: vec!["carrot".into(), "celery".into(), "lettuce".into()],
            bad_companions: vec!["green-bean".into(), "pea".into()],
        },
        Vegetable {
            id: "green-bean".into(),
            name: "Green Bean".into(),
            variety_ids: vec!["green-bean".into()],
            good_companions: vec![
                "zucchini".into(),
                "maïs".into(),
                "potato".into(),
                "radish".into(),
            ],
            bad_companions: vec![
                "onion".into(),
                "garlic".into(),
                "fennel".into(),
                "leek".into(),
            ],
        },
        Vegetable {
            id: "cucumber".into(),
            name: "Cucumber".into(),
            variety_ids: vec!["cucumber".into()],
            good_companions: vec![
                "radish".into(),
                "lettuce".into(),
                "green-bean".into(),
                "maïs".into(),
            ],
            bad_companions: vec!["tomato".into(), "potato".into(), "fennel".into()],
        },
        Vegetable {
            id: "pepper".into(),
            name: "Pepper".into(),
            variety_ids: vec!["pepper".into(), "red-pepper".into()],
            good_companions: vec!["basil".into(), "tomato".into(), "carrot".into()],
            bad_companions: vec!["fennel".into(), "brassica".into()],
        },
        Vegetable {
            id: "pea".into(),
            name: "Pea".into(),
            variety_ids: vec!["pea".into()],
            good_companions: vec![
                "carrot".into(),
                "radish".into(),
                "lettuce".into(),
                "brassica".into(),
            ],
            bad_companions: vec!["onion".into(), "garlic".into(), "fennel".into()],
        },
        Vegetable {
            id: "brassica".into(),
            name: "Brassica".into(),
            variety_ids: vec!["cabbage".into(), "broccoli".into(), "cauliflower".into()],
            good_companions: vec!["celery".into(), "onion".into(), "pea".into()],
            bad_companions: vec!["tomato".into(), "strawberry".into(), "fennel".into()],
        },
        Vegetable {
            id: "parsley".into(),
            name: "Parsley".into(),
            variety_ids: vec!["parsley".into()],
            good_companions: vec!["tomato".into(), "asparagus".into(), "rose".into()],
            bad_companions: vec!["lettuce".into()],
        },
        Vegetable {
            id: "thyme".into(),
            name: "Thyme".into(),
            variety_ids: vec!["thyme".into()],
            good_companions: vec!["brassica".into(), "tomato".into(), "eggplant".into()],
            bad_companions: vec!["basil".into()],
        },
        Vegetable {
            id: "rosemary".into(),
            name: "Rosemary".into(),
            variety_ids: vec!["rosemary".into()],
            good_companions: vec!["brassica".into(), "green-bean".into(), "sage".into()],
            bad_companions: vec!["cucumber".into(), "pumpkin".into()],
        },
        Vegetable {
            id: "beet".into(),
            name: "Beet".into(),
            variety_ids: vec!["beet".into()],
            good_companions: vec!["onion".into(), "lettuce".into(), "radish".into()],
            bad_companions: vec!["green-bean".into(), "mustard".into()],
        },
        Vegetable {
            id: "spinach".into(),
            name: "Spinach".into(),
            variety_ids: vec!["spinach".into()],
            good_companions: vec!["strawberry".into(), "tomato".into(), "radish".into()],
            bad_companions: vec!["beet".into(), "sorrel".into()],
        },
        Vegetable {
            id: "fennel".into(),
            name: "Fennel".into(),
            variety_ids: vec!["fennel".into()],
            good_companions: vec![],
            bad_companions: vec![
                "tomato".into(),
                "green-bean".into(),
                "pepper".into(),
                "carrot".into(),
                "brassica".into(),
                "pea".into(),
                "cucumber".into(),
            ],
        },
        Vegetable {
            id: "eggplant".into(),
            name: "Eggplant".into(),
            variety_ids: vec!["eggplant".into()],
            good_companions: vec!["basil".into(), "thyme".into(), "pepper".into()],
            bad_companions: vec!["fennel".into()],
        },
        Vegetable {
            id: "celery".into(),
            name: "Celery".into(),
            variety_ids: vec!["celery".into()],
            good_companions: vec!["leek".into(), "brassica".into(), "tomato".into()],
            bad_companions: vec!["lettuce".into(), "garlic".into()],
        },
        Vegetable {
            id: "potato".into(),
            name: "Potato".into(),
            variety_ids: vec!["potato".into()],
            good_companions: vec!["green-bean".into(), "brassica".into(), "maïs".into()],
            bad_companions: vec!["tomato".into(), "cucumber".into(), "zucchini".into()],
        },
        Vegetable {
            id: "maïs".into(),
            name: "Corn".into(),
            variety_ids: vec!["maïs".into()],
            good_companions: vec!["green-bean".into(), "zucchini".into(), "potato".into()],
            bad_companions: vec!["tomato".into(), "celery".into()],
        },
        Vegetable {
            id: "pumpkin".into(),
            name: "Pumpkin".into(),
            variety_ids: vec!["pumpkin".into()],
            good_companions: vec!["maïs".into(), "green-bean".into(), "onion".into()],
            bad_companions: vec!["potato".into(), "rosemary".into()],
        },
        Vegetable {
            id: "chive".into(),
            name: "Chive".into(),
            variety_ids: vec!["chive".into()],
            good_companions: vec![
                "carrot".into(),
                "tomato".into(),
                "rose".into(),
                "strawberry".into(),
            ],
            bad_companions: vec!["green-bean".into(), "pea".into()],
        },
        Vegetable {
            id: "mint".into(),
            name: "Mint".into(),
            variety_ids: vec!["mint".into()],
            good_companions: vec!["brassica".into(), "tomato".into(), "pea".into()],
            bad_companions: vec!["parsley".into()],
        },
        Vegetable {
            id: "strawberry".into(),
            name: "Strawberry".into(),
            variety_ids: vec!["strawberry".into()],
            good_companions: vec![
                "lettuce".into(),
                "spinach".into(),
                "garlic".into(),
                "onion".into(),
            ],
            bad_companions: vec!["brassica".into(), "fennel".into()],
        },
        Vegetable {
            id: "turnip".into(),
            name: "Turnip".into(),
            variety_ids: vec!["turnip".into()],
            good_companions: vec!["pea".into(), "green-bean".into()],
            bad_companions: vec!["mustard".into(), "radish".into()],
        },
        Vegetable {
            id: "asparagus".into(),
            name: "Asparagus".into(),
            variety_ids: vec!["asparagus".into()],
            good_companions: vec!["tomato".into(), "parsley".into(), "basil".into()],
            bad_companions: vec!["onion".into(), "garlic".into()],
        },
        Vegetable {
            id: "artichoke".into(),
            name: "Artichoke".into(),
            variety_ids: vec!["artichoke".into()],
            good_companions: vec![],
            bad_companions: vec![],
        },
    ]
}
