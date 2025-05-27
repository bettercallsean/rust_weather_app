use clouds::Clouds;
use coord::Coord;
use serde::{Deserialize, Serialize};
use sys::Sys;
use temperature::Temperature;
use weather_synopsis::WeatherSynopsis;
use wind::Wind;

pub mod clouds;
pub mod coord;
pub mod sys;
pub mod temperature;
pub mod weather_synopsis;
pub mod wind;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub coord: Coord,
    #[serde(rename = "weather")]
    pub synopsis: Vec<WeatherSynopsis>,
    #[serde(rename = "main")]
    pub temperature: Temperature,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    #[serde(rename = "dt")]
    pub forecast_date: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    #[serde(rename = "name")]
    pub city: String,
}
