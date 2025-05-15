mod enums;
mod models;
mod services;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No city provided as argument!");
    }

    let city = &args[1];

    let _ = services::weather_service::get_weather(city);

    Ok(())
}
