use std::{fmt::Display, str::FromStr};

#[derive(Default)]
pub enum WeatherDescription {
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

impl Display for WeatherDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Thunderstorm => write!(f, "Thunderstorm"),
            Self::Drizzle => write!(f, "Drizzle"),
            Self::Rain => write!(f, "Rain"),
            Self::Snow => write!(f, "Snow"),
            Self::Atmosphere => write!(f, "Atmosphere"),
            Self::Clear => write!(f, "Clear"),
            Self::Clouds => write!(f, "Clouds"),
            Self::Other => write!(f, "Other"),
        }
    }
}

impl FromStr for WeatherDescription {
    type Err = ();

    fn from_str(input: &str) -> Result<WeatherDescription, Self::Err> {
        match input {
            "Thunderstorm" => Ok(Self::Thunderstorm),
            "Drizzle" => Ok(Self::Drizzle),
            "Rain" => Ok(Self::Rain),
            "Snow" => Ok(Self::Snow),
            "Atmosphere" => Ok(Self::Atmosphere),
            "Clear" => Ok(Self::Clear),
            "Clouds" => Ok(Self::Clouds),
            _ => Ok(Self::Other),
        }
    }
}

impl WeatherDescription {
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
