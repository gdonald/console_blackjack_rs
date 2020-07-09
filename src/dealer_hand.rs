use crate::hand::{Hand, CountMethod};
use crate::card::CARD_FACES;

impl DealerHand {
    pub(crate) fn new() -> DealerHand {
        DealerHand { hand: Hand { cards: vec![] }, hide_down_card: true }
    }
}
