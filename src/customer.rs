use crate::context::{Context, Mode};
use crate::util::pause;
use cardpack::Named;
use inquire::{validator::Validation, CustomType, Text};

pub fn draw_customer(context: &mut Context) {
    let customer_card_index: String;
    let genre_roll: String;

    if context.mode == Mode::Manual {
        println!("Draw a card to reveal a customer.");
        customer_card_index = Text::new("What card did you draw?").prompt().unwrap();
        println!("Roll a d20 for what type of book the customer is looking for.");
        genre_roll = CustomType::<u8>::new("What number did you roll?")
            .with_validator(|val: &u8| {
                if *val < 1 || *val > 20 {
                    Ok(Validation::Invalid(
                        "Please enter a number between 1 and 20".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .unwrap()
            .to_string();
    } else {
        let customer_pile = context.deck.draw(1).unwrap();

        let customer_card = customer_pile.get(0).unwrap();

        customer_card_index = customer_card.index_default();

        genre_roll = context.roll_d(20).to_string();

        println!(
            "Card drawn: {} d20 roll: {}",
            customer_card.symbol_colorized(&context.li),
            genre_roll
        );
    }

    let customer_description = context
        .game_data
        .customers
        .description
        .get(&customer_card_index)
        .expect("Can't find customer for card")
        .to_string();

    let genre = context
        .game_data
        .customers
        .book_genre
        .get(&genre_roll)
        .expect("Can't find genre for roll")
        .to_string();

    println!("{customer_description}");
    println!("Looking for {}", genre);

    pause();
}
