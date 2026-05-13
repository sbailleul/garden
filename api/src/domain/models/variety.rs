use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::models::vegetable::Vegetable;

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

/// Per-region sowing and planting calendar for a variety.
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

/// The height layer a plant occupies in the garden.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stratum {
    pub id: String,
    pub name: String,
}

/// One way to cultivate a variety: its spacing, and the height stratum it occupies.
/// A variety may have multiple cultivation modes (e.g. bush vs. climbing).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CultivationMode {
    pub id: String,
    pub name: String,
    pub stratum: Stratum,
    /// Centre-to-centre spacing in centimetres.
    pub spacing_cm: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variety {
    pub id: String,
    pub vegetable: Arc<Vegetable>,
    pub name: String,
    pub latin_name: String,
    /// Per-region sowing and planting calendars.
    /// The presence of a [`RegionCalendar`] entry for a given region implies
    /// the variety can be grown there.
    pub calendars: Vec<RegionCalendar>,
    pub sun_requirement: Vec<SunExposure>,
    pub soil_types: Vec<SoilType>,
    /// Available cultivation modes (at least one guaranteed). The first is the default.
    pub cultivation_modes: Vec<CultivationMode>,
    /// Approximate number of days from planting/transplanting to first harvest.
    pub days_to_harvest: u32,
    /// Approximate number of days from sowing a seed to being ready for transplanting outdoors.
    pub days_to_plant: u32,
    pub lifecycle: Lifecycle,
    pub beginner_friendly: bool,
    pub category: Category,
}

impl Variety {
    /// Returns the default (first) cultivation mode.
    ///
    /// # Panics
    /// Panics when `cultivation_modes` is empty — every well-formed `Variety` must
    /// have at least one mode (enforced at the repository layer).
    pub fn default_mode(&self) -> &CultivationMode {
        self.cultivation_modes
            .first()
            .expect("variety must have at least one cultivation mode")
    }

    /// Returns the cultivation mode whose `id` matches `id`, falling back to
    /// [`default_mode`] when `id` is `None` or no match is found.
    pub fn mode_or_default(&self, id: Option<&str>) -> &CultivationMode {
        match id {
            Some(id) => self
                .cultivation_modes
                .iter()
                .find(|m| m.id == id)
                .unwrap_or_else(|| self.default_mode()),
            None => self.default_mode(),
        }
    }
}
