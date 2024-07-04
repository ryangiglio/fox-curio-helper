use crate::data::GameData;
use cardpack::{Pack, Pile};
use rand::Rng;
use rand::rngs::ThreadRng;
use unic_langid::{langid, LanguageIdentifier};
use strum_macros::{EnumString, Display};

#[derive(EnumString, Display)]
pub enum Mode {
    Manual,
    Automatic,
}

pub struct Context {
    pub game_data: GameData,
    pub deck: Pile,
    pub mode: Mode,
    pub li: LanguageIdentifier,
    pub rng: ThreadRng,
}

impl Context {
    pub fn setup() -> Self {
        let game_data = GameData::load("data/gameData.json");

        let deck: Pile = Pack::french_deck().cards().shuffle();

        let mode = Mode::Automatic;

        let li = langid!("en-US");

        let rng = rand::thread_rng();

        Context {
            game_data,
            deck,
            mode,
            li,
            rng,
        }
    }

    pub fn roll_d(&mut self, sides: u8) -> u8 {
      self.rng.gen_range(1..=sides)
    }
}
