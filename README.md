# rust_weather_app

A terribly basic weather app that I've built in Rust to have a practice with :)

## How to run

The app uses OpenWeatherMap's [current weather API](https://openweathermap.org/current), so you'll need to generate an API key for it first.

Next, create a `.env` file in the root of the repository, with the key `OPEN_WEATHER_API_KEY` in it (see `.env.sample` in the repo for an example), then run `cargo run {city}` in the terminal, replacing `{city}` with the name of the city you want to get a weather report for.

![](https://www.icegif.com/wp-content/uploads/2023/07/icegif-767.gif)
