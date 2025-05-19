use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

const THUNDERSTORM: &str = "Thunderstorm";
const DRIZZLE: &str = "Drizzle";
const RAIN: &str = "Rain";
const SNOW: &str = "Snow";
const ATMOSPHERE: &str = "Atmosphere";
const CLEAR: &str = "Clear";
const CLOUDS: &str = "Clouds";
const OTHER: &str = "Other";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    pub coord: Coord,
    #[serde(rename = "weather")]
    pub synopsis: Vec<WeatherSynopsis>,
    pub base: String,
    pub main: Main,
    pub visibility: i64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeatherSynopsis {
    #[serde(rename = "main")]
    pub synopsis: Synopsis,
    pub description: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Synopsis {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Atmosphere,
    Clear,
    Clouds,
    #[default]
    Other,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i64,
    pub humidity: i64,
    pub sea_level: i64,
    pub grnd_level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: i64,
    pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clouds {
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

impl Display for Synopsis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thunderstorm => write!(f, "{}", THUNDERSTORM),
            Self::Drizzle => write!(f, "{}", DRIZZLE),
            Self::Rain => write!(f, "{}", RAIN),
            Self::Snow => write!(f, "{}", SNOW),
            Self::Atmosphere => write!(f, "{}", ATMOSPHERE),
            Self::Clear => write!(f, "{}", CLEAR),
            Self::Clouds => write!(f, "{}", CLOUDS),
            Self::Other => write!(f, "{}", OTHER),
        }
    }
}

impl FromStr for Synopsis {
    type Err = ();

    fn from_str(input: &str) -> Result<Synopsis, Self::Err> {
        match input {
            THUNDERSTORM => Ok(Self::Thunderstorm),
            DRIZZLE => Ok(Self::Drizzle),
            RAIN => Ok(Self::Rain),
            SNOW => Ok(Self::Snow),
            ATMOSPHERE => Ok(Self::Atmosphere),
            CLEAR => Ok(Self::Clear),
            CLOUDS => Ok(Self::Clouds),
            _ => Ok(Self::Other),
        }
    }
}

impl Synopsis {
    pub fn get_emoji(&self) -> &str {
        match self {
            Self::Thunderstorm => "⚡️",
            Self::Drizzle => "🌧️",
            Self::Rain => "☔️",
            Self::Snow => "❄️",
            Self::Atmosphere => "🌫️",
            Self::Clear => "☀️",
            Self::Clouds => "🌥️",
            Self::Other => "❓",
        }
    }
}
