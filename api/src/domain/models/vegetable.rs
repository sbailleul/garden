use std::fmt;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

/// Maps a calendar month (1–12) to the corresponding planting [`Season`].
/// Spring: Mar–May, Summer: Jun–Aug, Autumn: Sep–Nov, Winter: Dec–Feb.
pub fn season_for_month(month: u32) -> Season {
    match month {
        3..=5 => Season::Spring,
        6..=8 => Season::Summer,
        9..=11 => Season::Autumn,
        _ => Season::Winter,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum SoilType {
    Clay,
    Sandy,
    Loamy,
    Chalky,
    Humus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum SunExposure {
    FullSun,
    PartialShade,
    Shade,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Region {
    Temperate,
    Mediterranean,
    Oceanic,
    Continental,
    Mountain,
}

/// Calendar month — used in sowing and planting windows.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from_u32(m: u32) -> Self {
        match m {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            _ => Month::December,
        }
    }

    pub fn to_u32(self) -> u32 {
        match self {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
    }
}

/// Sowing or planting window — distinguishes direct outdoor from under-cover months.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalendarWindow {
    /// Months for sowing / planting directly in open ground.
    pub outdoor: Vec<Month>,
    /// Months for sowing / planting under cover or in a greenhouse.
    pub indoor: Vec<Month>,
}

/// Per-region sowing and planting calendar for a vegetable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegionCalendar {
    pub region: Region,
    /// Recommended months for sowing seeds.
    pub sowing: CalendarWindow,
    /// Recommended months for planting seedlings / transplanting.
    pub planting: CalendarWindow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Category {
    Fruit,
    Produce,
    Herb,
    Root,
    Bulb,
    Leafy,
    Pod,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// Plant lifecycle: how many growing seasons the plant lives.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "PascalCase")]
pub enum Lifecycle {
    /// Completes its full life cycle in a single growing season.
    Annual,
    /// Requires two growing seasons to complete its life cycle.
    Biennial,
    /// Lives for three or more years, re-growing each season.
    Perennial,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Vegetable {
    pub id: String,
    pub name: String,
    pub latin_name: String,
    /// Per-region sowing and planting calendars.
    /// The presence of a [`RegionCalendar`] entry for a given region implies
    /// the vegetable can be grown there.
    pub calendars: Vec<RegionCalendar>,
    pub sun_requirement: Vec<SunExposure>,
    pub soil_types: Vec<SoilType>,
    pub spacing_cm: u32,
    /// Approximate number of days from planting/transplanting to first harvest.
    pub days_to_harvest: u32,
    /// Approximate number of days from sowing a seed to being ready for transplanting outdoors.
    pub days_to_plant: u32,
    pub lifecycle: Lifecycle,
    pub good_companions: Vec<String>,
    pub bad_companions: Vec<String>,
    pub beginner_friendly: bool,
    pub category: Category,
}
