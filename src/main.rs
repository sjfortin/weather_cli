extern crate reqwest;
extern crate serde;

use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct WeatherInfo {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    main: String,
    description: String,
}

fn main() {
    let city = "Minneapolis"; // Todo: use std::env to get this from command line arguments.
    match fetch_weather(city) {
        Ok(weather_info) => {
            println!(
                "Current weather in {}: {}°F, {} - {}",
                city,
                weather_info.main.temp,
                weather_info.weather[0].main,
                weather_info.weather[0].description
            );
        }
        Err(e) => println!("Error fetching weather data: {}", e),
    }
}

fn fetch_weather(city: &str) -> Result<WeatherInfo, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
        city, api_key
    );

    let response = reqwest::blocking::get(url)?.json::<WeatherInfo>()?;

    Ok(response)
}
