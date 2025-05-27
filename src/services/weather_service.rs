use crate::models::location::{Location, Locations};
use crate::models::weather::Weather;
use chrono::{DateTime, Local};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::{env, process};

const WEATHER_JSON_FILE: &str = "weather.json";

pub async fn get_weather_forecast(city: &str) -> Result<(), Box<dyn Error>> {
    let weather = match get_stored_weather_forecast() {
        Ok(weather) => {
            let current_time = Local::now().timestamp();

            if weather.city != *city || ((current_time - weather.forecast_date) / 60) >= 10 {
                get_latest_weather_forecast(city).await?
            } else {
                weather
            }
        }
        Err(_) => get_latest_weather_forecast(city).await?,
    };

    print_weather(&weather);

    store_weather_forecast(&weather)?;

    Ok(())
}

async fn get_latest_weather_forecast(city: &str) -> Result<Weather, Box<dyn Error>> {
    let api_key = get_weather_api_key()?;
    let location = get_city_location(&api_key, city).await?;
    let weather = get_new_weather_forecast(&api_key, &location).await?;

    Ok(weather)
}

fn get_weather_api_key() -> Result<String, Box<dyn Error>> {
    dotenvy::dotenv()?;

    match env::var("OPEN_WEATHER_API_KEY") {
        Ok(val) => Ok(val),
        Err(e) => Err(Box::new(e)),
    }
}

async fn get_new_weather_forecast(
    api_key: &str,
    location: &Location,
) -> Result<Weather, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lon={}&lat={}&appid={}&units=metric",
        location.lon, location.lat, api_key
    );

    let response = reqwest::get(&url).await?;

    match response.error_for_status() {
        Ok(response) => {
            let json = response.text().await?;
            let weather = parse_weather_forecast_json(json.as_str())?;
            Ok(weather)
        }
        Err(error) => Err(Box::new(error)),
    }
}

async fn get_city_location(api_key: &str, city: &str) -> Result<Location, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&&appid={}",
        city, api_key
    );

    let response = reqwest::get(&url).await?;

    match response.error_for_status() {
        Ok(response) => {
            let json = response.text().await?;
            let location = parse_location_json(json.as_str())?;
            Ok(location)
        }
        Err(error) => Err(Box::new(error)),
    }
}

fn parse_location_json(location_json: &str) -> Result<Location, Box<dyn Error>> {
    let locations: Locations = serde_json::from_str(location_json)?;

    match locations.first() {
        Some(location) => Ok(location.to_owned()),
        None => {
            eprintln!("Location not found by Geolocation API!");
            process::exit(1);
        }
    }
}

fn parse_weather_forecast_json(weather_json: &str) -> Result<Weather, Box<dyn Error>> {
    let weather: Weather = serde_json::from_str(weather_json)?;

    Ok(weather)
}

fn print_weather(weather: &Weather) {
    println!("Weather for {}", weather.city);
    let forecast_time = match DateTime::from_timestamp(weather.forecast_date, 0) {
        Some(time) => <DateTime<Local>>::from(time),
        None => {
            eprintln!("Failed to parse stored forecast time");
            Local::now()
        }
    };
    println!("Forecast date: {}", forecast_time.format("%d/%m/%y %H:%M"));

    if let Some(weather_synopsis) = weather.synopsis.first() {
        println!(
            "{}: {} {}",
            weather_synopsis.synopsis,
            weather_synopsis.description,
            weather_synopsis.synopsis.get_emoji()
        );
    }

    println!("Current temp: {:.0}ºC", weather.temperature.temp);
    println!("Feels like: {:.0}ºC", weather.temperature.feels_like);
}

fn store_weather_forecast(weather: &Weather) -> std::io::Result<()> {
    let weather_json = serde_json::to_string(weather)?;
    let mut file = File::create(WEATHER_JSON_FILE)?;
    write!(file, "{}", weather_json)?;

    Ok(())
}

fn get_stored_weather_forecast() -> Result<Weather, Box<dyn Error>> {
    let weather_json = fs::read_to_string(WEATHER_JSON_FILE)?;
    let weather: Weather = serde_json::from_str(&weather_json)?;

    Ok(weather)
}
