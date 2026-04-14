use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A variety groups one or more vegetables of the same species or type.
/// For example, the `"pepper"` variety contains both `"pepper"` and `"red-pepper"`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Variety {
    pub id: String,
    pub name: String,
}
