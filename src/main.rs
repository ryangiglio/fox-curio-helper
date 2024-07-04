mod context;
mod customer;
mod data;
mod util;
mod weather;

use crate::util::pause;
use cardpack::{Pack, Pile};
use context::{Context, Mode};
use customer::draw_customer;
use inquire::Select;
use weather::draw_weather;

fn main() {
    let mut context = Context::setup();

    loop {
        let menu_options = vec![
            "Draw weather for the day",
            "Draw a customer",
            "Shuffle the deck",
            "Change modes",
            "Quit",
        ];
        let selection = Select::new("What do you want to do", menu_options).prompt();

        match selection {
            Ok(selection) => match selection {
                "Draw weather for the day" => draw_weather(&mut context),
                "Draw a customer" => draw_customer(&mut context),
                "Shuffle the deck" => shuffle(&mut context),
                "Change modes" => switch_mode(&mut context),
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
    match context.mode {
        Mode::Manual => context.mode = Mode::Automatic,
        Mode::Automatic => context.mode = Mode::Manual,
    }

    println!("Mode changed to {}", context.mode);

    pause();
}
