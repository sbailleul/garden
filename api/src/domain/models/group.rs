use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A group is a high-level category that contains several vegetables
/// (e.g. "Bulbes" groups onion, garlic, leek, and chive).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: String,
    pub name: String,
}
