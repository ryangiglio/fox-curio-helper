mod weather;
mod util;

use cardpack::{Pack, Pile};
use inquire::Select;
use weather::draw_weather;
use crate::util::pause;

fn main() {
    let mut deck: Pile = Pack::french_deck().cards().shuffle();

    loop {
        let menu_options = vec!["Draw weather for the day", "Shuffle the deck", "Quit"];
        let selection = Select::new("What do you want to do", menu_options).prompt();

        match selection {
            Ok(selection) => {
                match selection {
                    "Draw weather for the day" => draw_weather(&mut deck),
                    "Shuffle the deck" => shuffle(&mut deck),
                    "Quit" => break,
                    &_ => break,
                }
            }
            Err(_) => println!("There was an error, somehow"),
        }
    }
}

fn shuffle(deck: &mut Pile) {
    *deck = Pack::french_deck().cards().shuffle();

    println!("Deck shuffled.");

    pause();
}