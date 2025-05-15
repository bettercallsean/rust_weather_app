mod location;
mod weather;
mod weather_description;

use chrono::Local;
use location::{Location, Locations};
use std::env;
use std::error::Error;
use std::str::FromStr;
use weather::Weather;
use weather_description::WeatherDescription;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = get_weather_api_key()?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No city provided as argument!");
    }

    let city = &args[1];

    let location_json = get_city_location_json(&api_key, city)?;

    let location = parse_location_json(location_json)?;

    let weather_json = get_weather_json(&api_key, &location)?;

    let weather = parse_weather_json(weather_json)?;

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

fn get_weather_json(api_key: &String, location: &Location) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lon={}&lat={}&appid={}&units=metric",
        location.lon, location.lat, api_key
    );

    let response = reqwest::blocking::get(&url)?;

    match response.error_for_status() {
        Ok(response) => {
            let json = response.text()?;
            Ok(json)
        }
        Err(error) => Err(error),
    }
}

fn get_city_location_json(api_key: &String, city: &String) -> Result<String, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&&appid={}",
        city, api_key
    );

    let response = reqwest::blocking::get(&url)?;

    match response.error_for_status() {
        Ok(response) => {
            let json = response.text()?;
            Ok(json)
        }
        Err(error) => Err(error),
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
