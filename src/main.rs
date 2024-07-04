mod context;
mod customer;
mod data;
mod util;
mod weather;

use crate::util::pause;
use cardpack::Pack;
use context::{Context, Mode};
use customer::draw_customer;
use inquire::Select;
use weather::draw_weather;

fn main() {
    let mut context = Context::setup();

    loop {
        println!("Current Mode: {}", context.mode);

        let change_mode_option = if context.mode == Mode::Automatic {
            Mode::Manual
        } else {
            Mode::Automatic
        };

        let change_mode_menu_text = format!("Change to {} Mode", change_mode_option);

        let menu_options = vec![
            "Start a new day",
            "Draw weather for the day",
            "Draw a customer",
            "Roll for extra customers",
            "End of day calculations",
            "Shuffle the deck",
            &change_mode_menu_text,
            "Quit",
        ];

        let selection = Select::new("What do you want to do", menu_options).prompt();

        match selection {
            Ok(selection) => match selection {
                "Start a new day" => {
                    println!("TODO - combine draw weather with roll for daily task")
                }
                "Draw weather for the day" => draw_weather(&mut context),
                "Draw a customer" => draw_customer(&mut context),
                "Roll for extra customers" => println!("TODO - roll on extra customers table"),
                "End of day calculations" => {
                    println!("TODO - calculate total customers, books sold, and earnings")
                }
                "Shuffle the deck" => shuffle(&mut context),
                s if s == change_mode_menu_text => switch_mode(&mut context),
                "Quit" => break,
                &_ => break,
            },
            Err(_) => println!("There was an error, somehow"),
        }
    }
}

fn shuffle(context: &mut Context) {
    context.deck = Pack::french_deck().cards().shuffle();

    println!("Deck shuffled.");

    pause();
}

fn switch_mode(context: &mut Context) {
    if context.mode == Mode::Automatic {
        context.mode = Mode::Manual;
        println!("Changed to Manual mode - use a physical deck/dice and enter the results");
        println!("Enter cards as two characters, like 2H for the Two of Hearts or JD for the Jack of Diamonds. Use T instead of 10, like TS for the Ten of Spades.");
    } else {
        context.mode = Mode::Automatic;
        println!("Changed to Automatic mode - the app will draw/roll for you");
    }

    pause();
}
