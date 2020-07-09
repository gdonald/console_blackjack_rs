// extern crate termios;

use crate::dealer_hand::DealerHand;
use crate::player_hand::PlayerHand;
use crate::shoe::Shoe;
use crate::player_hand::Status;

impl Game {
    pub(crate) fn new() -> Game {
        let mut game: Game = Game {
            shoe: Shoe { cards: vec![] },
            money: 10000,
            current_bet: 500,
            num_decks: 8,
            current_player_hand: 0,
            player_hands: vec![],
            dealer_hand: DealerHand::new()
        };

        let shoe: Shoe = Shoe::new_regular(&mut game);
        game.shoe = shoe;

        game.deal_new_hand();

        game
    }
}
