use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Season {
    Printemps,
    Ete,
    Automne,
    Hiver,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SoilType {
    Argileux,
    Sableux,
    Limoneux,
    Calcaire,
    Humifere,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SunExposure {
    PleinSoleil,
    MiOmbre,
    Ombre,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Region {
    Tempere,
    Mediterraneen,
    Oceanique,
    Continental,
    Montagnard,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Category {
    Fruit,
    Legume,
    Aromate,
    Racine,
    Bulbe,
    Feuille,
    Gousse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
