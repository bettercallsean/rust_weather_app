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
