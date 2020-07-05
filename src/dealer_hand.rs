use crate::hand::Hand;

pub struct DealerHand {
    pub hand: Hand,
    pub hide_down_card: bool
}

impl DealerHand {
    pub(crate) fn new() -> DealerHand {
        DealerHand { hand: Hand { cards: vec![] }, hide_down_card: false }
    }
}
