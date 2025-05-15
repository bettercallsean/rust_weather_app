use std::str::FromStr;

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

impl FromStr for WeatherDescription {
    type Err = ();

    fn from_str(input: &str) -> Result<WeatherDescription, Self::Err> {
        match input {
            "Thunderstorm" => Ok(WeatherDescription::Thunderstorm),
            "Drizzle" => Ok(WeatherDescription::Drizzle),
            "Rain" => Ok(WeatherDescription::Rain),
            "Snow" => Ok(WeatherDescription::Snow),
            "Atmosphere" => Ok(WeatherDescription::Atmosphere),
            "Clear" => Ok(WeatherDescription::Clear),
            "Clouds" => Ok(WeatherDescription::Clouds),
            _ => Ok(WeatherDescription::Other),
        }
    }
}
