extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card::Card;
use crate::card::CARDS_PER_DECK;
use crate::game::Game;

pub(crate) const SHUFFLE_SPECS: [[u8; 2]; 8] = [
    [95, 8],
    [92, 7],
    [89, 6],
    [86, 5],
    [84, 4],
    [82, 3],
    [81, 2],
    [80, 1]
];

pub struct Shoe {
    pub cards: Vec<Card>
}

impl Shoe {
    pub(crate) fn new_regular(mut game: &mut Game) -> Shoe {
        new_shoe(&mut game, &(0..13).map(u8::from).collect::<Vec<_>>())
    }

    pub(crate) fn new_aces(mut game: &mut Game) -> Shoe {
        new_shoe(&mut game, &vec![0u8])
    }

    pub(crate) fn new_jacks(mut game: &mut Game) -> Shoe {
        new_shoe(&mut game, &vec![10u8])
    }

    pub(crate) fn new_aces_jacks(mut game: &mut Game) -> Shoe {
        new_shoe(&mut game, &vec![0u8, 10u8])
    }

    pub(crate) fn new_sevens(mut game: &mut Game) -> Shoe {
        new_shoe(&mut game, &vec![6u8])
    }

    pub(crate) fn new_eights(mut game: &mut Game) -> Shoe {
        new_shoe(&mut game, &vec![7u8])
    }

    pub fn get_next_card(&mut self) -> Card {
        self.cards.remove(0)
    }
}

fn new_shoe(game: &mut Game, values: &Vec<u8>) -> Shoe {
    let mut shoe = Shoe { cards: vec![] };
    let total_cards = (CARDS_PER_DECK * game.num_decks).into();

    while shoe.cards.len() < total_cards {
        for suit in 0..4 {
            if shoe.cards.len() >= total_cards { break; }

            for value in values.iter() {
                shoe.cards.push(Card::new(*value, suit));
            }
        }
    }

    shoe.cards.shuffle(&mut thread_rng());
    shoe
}
