mod weather;

use chrono::Utc;
use std::env;
use std::error::Error;
use weather::Root;

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = get_weather_api_key()?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No city provided as argument!");
    }

    let city = &args[1];

    let weather_json = get_weather_json(api_key, city)?;

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

fn get_weather_json(api_key: String, city: &String) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response = reqwest::blocking::get(&url)?;

    if response.status().is_success() {
        let json = response.text()?;
        Ok(json)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to fetch weather data",
        )))
    }
}

fn parse_weather_json(weather_json: String) -> Result<Root, Box<dyn Error>> {
    let weather: Root = serde_json::from_str(&weather_json)?;

    Ok(weather)
}

fn print_weather(weather: Root) {
    println!("{}", Utc::now().format("%d/%m/%y %H:%M"));

    if let Some(weather_description) = weather.weather.first() {
        println!(
            "{}: {}",
            weather_description.main, weather_description.description
        );
    }

    println!("Current temp: {:.0}ºC", weather.main.temp);
    println!("Feels like: {:.0}ºC", weather.main.feels_like);
}
