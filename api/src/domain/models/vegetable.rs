use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A vegetable groups one or more varieties of the same species or type.
/// For example, the `"pepper"` vegetable contains both `"pepper"` and `"red-pepper"`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Vegetable {
    pub id: String,
    pub name: String,
    pub variety_ids: Vec<String>,
}
