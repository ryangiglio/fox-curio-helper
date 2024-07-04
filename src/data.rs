use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct GameData {
    pub weather: WeatherData,
    pub customers: CustomerData,
}

impl GameData {
  pub fn load(filename: &str) -> Self {
    let json_string = fs::read_to_string(filename).expect("Unable to read game data file");

    let data: GameData = serde_json::from_str(&json_string)
        .expect("Unable to parse json from data file");

    data
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDescription {
    pub text: String,
    pub customer_forecast: String,
    pub weather_event: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherDuration {
    pub text: String,
    pub all_day: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Season {
    pub description: HashMap<String, WeatherDescription>,
    pub duration: HashMap<String, WeatherDuration>,
}

pub type WeatherData = HashMap<String, Season>;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerData {
    pub description: HashMap<String, String>,
    pub book_genre: HashMap<String, String>,
}