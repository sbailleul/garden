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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Vegetable {
    pub id: String,
    pub name: String,
    pub latin_name: String,
    pub seasons: Vec<Season>,
    pub sun_requirement: Vec<SunExposure>,
    pub soil_types: Vec<SoilType>,
    pub regions: Vec<Region>,
    pub spacing_cm: u32,
    pub good_companions: Vec<String>,
    pub bad_companions: Vec<String>,
    pub beginner_friendly: bool,
    pub category: Category,
}
