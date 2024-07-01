use crate::util::pause;
use cardpack::Pile;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use unic_langid::langid;

#[derive(Serialize, Deserialize, Debug)]
struct WeatherDescription {
    text: String,
    customer_forecast: String,
    weather_event: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherDuration {
    text: String,
    all_day: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Season {
    weather: HashMap<String, WeatherDescription>,
    duration: HashMap<String, WeatherDuration>,
}

type WeatherData = HashMap<String, Season>;

pub fn draw_weather(deck: &mut Pile) {
    let li = langid!("en-US");

    let weather_json =
        fs::read_to_string("data/weather.json").expect("Unable to read weather data file");
    let weather_data: WeatherData =
        serde_json::from_str(&weather_json).expect("Unable to parse json from weather data file");

    let menu_options = vec!["Bloom", "Burn", "Brimming", "Brink", "Brisk"];
    let season_name = Select::new("What season is it?", menu_options)
        .prompt()
        .unwrap();

    let season: &Season = weather_data.get(season_name).expect("Can't find season");

    let weather_pile1 = deck.draw(2).unwrap();

    let description_card1 = weather_pile1.get(0).unwrap();
    let duration_card = weather_pile1.get(1).unwrap();

    let description = season
        .weather
        .get(&description_card1.rank.to_string())
        .expect("Can't find weather for rank");
    let duration = season
        .duration
        .get(&duration_card.rank.to_string())
        .expect("Can't find duration for rank");

    println!(
        "Cards drawn: Duration {}, Description {}",
        duration_card.symbol_colorized(&li),
        description_card1.symbol_colorized(&li)
    );

    println!("{}: {}", duration.text, description.text);

    if !duration.all_day {
        let weather_pile2 = deck.draw(1).unwrap();

        let description_card2 = weather_pile2.get(0).unwrap();

        println!("Card drawn: {}", description_card2.symbol_colorized(&li));

        let remaining_weather = season
            .weather
            .get(&description_card2.rank.to_string())
            .expect("Can't find weather for rank");

        println!("The rest of the day: {}", remaining_weather.text);
    }

    println!("Customer forecast: {}", description.customer_forecast);

    pause();
}
