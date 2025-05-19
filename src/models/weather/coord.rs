use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}
