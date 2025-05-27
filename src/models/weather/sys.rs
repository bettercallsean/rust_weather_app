use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
