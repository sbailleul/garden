use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A vegetable groups one or more varieties of the same species or type.
/// For example, the `"pepper"` vegetable contains both `"pepper"` and `"red-pepper"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Vegetable {
    pub id: String,
    pub name: String,
    pub variety_ids: Vec<String>,
    /// Identifiers of vegetables that benefit this vegetable when planted nearby.
    pub good_companions: Vec<String>,
    /// Identifiers of vegetables that harm this vegetable when planted nearby.
    pub bad_companions: Vec<String>,
}
