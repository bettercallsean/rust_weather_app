mod enums;
mod models;
mod services;

use services::weather_service;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No city provided as argument!");
    }

    let city = &args[1];

    let _ = weather_service::get_weather_report(city);

    Ok(())
}
