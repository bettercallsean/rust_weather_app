use crate::enums::weather_description::WeatherDescription;
use crate::models::location::{Location, Locations};
use crate::models::weather::Weather;
use chrono::Local;
use std::env;
use std::error::Error;
use std::str::FromStr;

pub async fn get_weather_report(city: &String) -> Result<(), Box<dyn Error>> {
    let api_key = get_weather_api_key()?;

    let location = get_city_location(&api_key, city).await?;

    let weather = get_weather(&api_key, &location).await?;

    print_weather(weather);

    Ok(())
}

fn get_weather_api_key() -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;

    match env::var("OPEN_WEATHER_API_KEY") {
        Ok(val) => Ok(val),
        Err(e) => Err(Box::new(e)),
    }
}

async fn get_weather(api_key: &String, location: &Location) -> Result<Weather, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lon={}&lat={}&appid={}&units=metric",
        location.lon, location.lat, api_key
    );

    let response = reqwest::get(&url).await?;

    match response.error_for_status() {
        Ok(response) => {
            let json = response.text().await?;
            let weather = parse_weather_json(json)?;
            Ok(weather)
        }
        Err(error) => Err(Box::new(error)),
    }
}

async fn get_city_location(api_key: &String, city: &String) -> Result<Location, Box<dyn Error>> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&&appid={}",
        city, api_key
    );

    let response = reqwest::get(&url).await?;

    match response.error_for_status() {
        Ok(response) => {
            let json = response.text().await?;
            let location = parse_location_json(json)?;
            Ok(location)
        }
        Err(error) => Err(Box::new(error)),
    }
}

fn parse_location_json(location_json: String) -> Result<Location, Box<dyn Error>> {
    let locations: Locations = serde_json::from_str(&location_json)?;

    match locations.first() {
        Some(location) => Ok(location.to_owned()),
        None => panic!("Location not found by Geolocation API!"),
    }
}

fn parse_weather_json(weather_json: String) -> Result<Weather, Box<dyn Error>> {
    let weather: Weather = serde_json::from_str(&weather_json)?;

    Ok(weather)
}

fn print_weather(weather: Weather) {
    println!("Weather for {}", weather.name);
    println!("{}", Local::now().format("%d/%m/%y %H:%M"));

    if let Some(weather_synopsis) = weather.weather.first() {
        println!(
            "{}: {} {}",
            weather_synopsis.main,
            weather_synopsis.description,
            get_weather_emoji(
                WeatherDescription::from_str(&weather_synopsis.main).unwrap_or_default()
            )
        );
    }

    println!("Current temp: {:.0}ºC", weather.main.temp);
    println!("Feels like: {:.0}ºC", weather.main.feels_like);
}

fn get_weather_emoji<'a>(weather_description: WeatherDescription) -> &'a str {
    match weather_description {
        WeatherDescription::Thunderstorm => "⚡️",
        WeatherDescription::Drizzle => "🌧️",
        WeatherDescription::Rain => "☔️",
        WeatherDescription::Snow => "❄️",
        WeatherDescription::Atmosphere => "🌫️",
        WeatherDescription::Clear => "☀️",
        WeatherDescription::Clouds => "🌥️",
        WeatherDescription::Other => "❓",
    }
}
