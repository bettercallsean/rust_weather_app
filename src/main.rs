mod models;
mod services;

use services::weather_service;
use std::error::Error;
use std::{env, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No city provided as argument!");
        process::exit(1)
    }

    let city = &args[1];

    if let Err(e) = weather_service::get_weather_forecast(city).await {
        eprintln!("Failed to get weather");
        Err(e)
    } else {
        Ok(())
    }
}
