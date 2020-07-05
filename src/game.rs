use crate::dealer_hand::DealerHand;
use crate::player_hand::PlayerHand;
use crate::shoe::Shoe;

pub struct Game {
    pub shoe: Shoe,
    dealer_hand: DealerHand,
    player_hands: Vec<PlayerHand>,
    pub num_decks: u16,
    money: u32,
    current_bet: u32,
    current_player_hand: u8
}

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

    fn deal_new_hand(&mut self) {
        self.player_hands = vec![PlayerHand::new(self.current_bet)];
        self.current_player_hand = 0;
        self.dealer_hand = DealerHand::new();

        self.dealer_hand.hand.deal_card(&mut self.shoe);
        self.player_hands[0].hand.deal_card(&mut self.shoe);
        self.dealer_hand.hand.deal_card(&mut self.shoe);
        self.player_hands[0].hand.deal_card(&mut self.shoe);
    }
}
