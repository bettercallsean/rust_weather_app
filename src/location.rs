use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Locations = Vec<Location>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub local_names: HashMap<String, String>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: String,
}
