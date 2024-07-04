use crate::context::{Context, Mode};
use crate::data::Season;
use crate::util::pause;
use inquire::{Select, Text};

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

    let description_rank1: String;
    let duration_rank: String;

    if context.mode == Mode::Manual {
        println!("Enter card ranks as a single digit/letter, using T instead of 10. Possible ranks are A, 2, 3, 4, 5, 6, 7, 8, 9, T, J, Q, K");
        description_rank1 = Text::new("What card rank did you draw for the weather?")
            .prompt()
            .unwrap();
        duration_rank = Text::new("What card rank did you draw for the duration of the weather?")
            .prompt()
            .unwrap();
    } else {
        let weather_pile1 = context.deck.draw(2).unwrap();

        let description_card1 = weather_pile1.get(0).unwrap();
        let duration_card = weather_pile1.get(1).unwrap();

        description_rank1 = description_card1.rank.to_string();
        duration_rank = duration_card.rank.to_string();

        println!(
            "Cards drawn: Duration {}, Description {}",
            duration_card.symbol_colorized(&context.li),
            description_card1.symbol_colorized(&context.li)
        );
    }

    let description = season
        .description
        .get(&description_rank1)
        .expect("Can't find weather for rank");
    let duration = season
        .duration
        .get(&duration_rank)
        .expect("Can't find duration for rank");

    println!("{}: {}", duration.text, description.text);

    if !duration.all_day {
        let description_rank2: String;

        if context.mode == Mode::Manual {
            println!("Draw another card to get the weather for the rest of the day.");

            pause();

            description_rank2 =
                Text::new("What card rank did you draw for the rest of the day's weather?")
                    .prompt()
                    .unwrap();
        } else {
            let weather_pile2: cardpack::Pile = context.deck.draw(1).unwrap();

            let description_card2 = weather_pile2.get(0).unwrap();

            description_rank2 = description_card2.rank.to_string();

            println!(
                "Card drawn: {}",
                description_card2.symbol_colorized(&context.li)
            );
        }

        let remaining_weather = season
            .description
            .get(&description_rank2)
            .expect("Can't find weather for rank");

        println!("The rest of the day: {}", remaining_weather.text);
    }

    println!("Customer forecast: {}", description.customer_forecast);

    pause();
}
