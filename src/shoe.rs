

use crate::card::Card;
use crate::card::CARDS_PER_DECK;
use crate::game::Game;

impl Shoe {




    pub fn get_next_card(&mut self) -> Card {
        self.cards.remove(0)
    }
}


