use crate::hand::Hand;

enum Status {
    Unknown = 0,
    Won,
    Lost,
    Push,
}

pub struct PlayerHand {
    pub hand: Hand,
    status: Status,
    stood: bool,
    played: bool,
    payed: bool,
    bet: u32,
}

impl PlayerHand {
    pub(crate) fn new(bet: u32) -> PlayerHand {
        PlayerHand {
            hand: Hand { cards: vec![] },
            status: Status::Unknown,
            stood: false,
            played: false,
            payed: false,
            bet
        }
    }
}