use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

const THUNDERSTORM: &str = "Thunderstorm";
const DRIZZLE: &str = "Drizzle";
const RAIN: &str = "Rain";
const SNOW: &str = "Snow";
const ATMOSPHERE: &str = "Atmosphere";
const CLEAR: &str = "Clear";
const CLOUDS: &str = "Clouds";
const OTHER: &str = "Other";

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
