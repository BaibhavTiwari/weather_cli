use std::io;

use serde::Deserialize; 

use colored::*; 
#[derive(Deserialize,Debug)]

struct WeatherResponse {
    weather:Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize,Debug)]

struct Weather{
    description:String,
}

#[derive(Deserialize,Debug)]

struct Main{
    temp:f64,
    humidity:f64,
    pressure:f64
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64, // Wind speed in meters per second
}

fn get_weather_info(city:&str,country_code: &str, api_key: &str) -> 
Result<WeatherResponse,reqwest::Error>{
    let url
}