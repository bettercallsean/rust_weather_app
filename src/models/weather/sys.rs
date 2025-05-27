use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
