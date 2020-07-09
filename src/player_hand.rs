use crate::hand::Hand;
use crate::hand::CountMethod;
use crate::game::Game;

impl PlayerHand {
    pub(crate) fn new(bet: u32) -> PlayerHand {
        PlayerHand {
            hand: Hand::new(),
            status: Status::Unknown,
            stood: false,
            played: false,
            payed: false,
            bet,
        }
    }
}