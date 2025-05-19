use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
}
