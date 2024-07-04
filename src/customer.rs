use crate::context::Context;
use crate::util::pause;
use cardpack::Named;

pub fn draw_customer(context: &mut Context) {
    let customer_pile = context.deck.draw(1).unwrap();

    let customer_card = customer_pile.get(0).unwrap();

    let customer_description = context
        .game_data
        .customers
        .description
        .get(&customer_card.index_default())
        .expect("Can't find customer for card")
        .to_string();

    let genre_roll = context.roll_d(20).to_string();

    let genre = context
        .game_data
        .customers
        .book_genre
        .get(&genre_roll)
        .expect("Can't find genre for roll")
        .to_string();

    println!(
        "Card drawn: {} d20 roll: {}",
        customer_card.symbol_colorized(&context.li),
        genre_roll
    );

    println!("{customer_description}");
    println!("Looking for {}", genre);

    pause();
}
