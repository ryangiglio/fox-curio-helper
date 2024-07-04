use crate::context::Context;
use crate::data::Season;
use crate::util::pause;
use inquire::Select;

pub fn draw_weather(context: &mut Context) {
    let menu_options = vec!["Bloom", "Burn", "Brimming", "Brink", "Brisk"];
    let season_name = Select::new("What season is it?", menu_options)
        .prompt()
        .unwrap();

    let season: &Season = context
        .game_data
        .weather
        .get(season_name)
        .expect("Can't find season");

    let weather_pile1 = context.deck.draw(2).unwrap();

    let description_card1 = weather_pile1.get(0).unwrap();
    let duration_card = weather_pile1.get(1).unwrap();

    let description = season
        .description
        .get(&description_card1.rank.to_string())
        .expect("Can't find weather for rank");
    let duration = season
        .duration
        .get(&duration_card.rank.to_string())
        .expect("Can't find duration for rank");

    println!(
        "Cards drawn: Duration {}, Description {}",
        duration_card.symbol_colorized(&context.li),
        description_card1.symbol_colorized(&context.li)
    );

    println!("{}: {}", duration.text, description.text);

    if !duration.all_day {
        let weather_pile2 = context.deck.draw(1).unwrap();

        let description_card2 = weather_pile2.get(0).unwrap();

        println!(
            "Card drawn: {}",
            description_card2.symbol_colorized(&context.li)
        );

        let remaining_weather = season
            .description
            .get(&description_card2.rank.to_string())
            .expect("Can't find weather for rank");

        println!("The rest of the day: {}", remaining_weather.text);
    }

    println!("Customer forecast: {}", description.customer_forecast);

    pause();
}
