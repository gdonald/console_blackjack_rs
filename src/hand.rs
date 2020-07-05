use crate::card::Card;
use crate::shoe::Shoe;

enum CountMethod {
    Soft,
    Hard
}

pub struct Hand {
    pub cards: Vec<Card>
}

impl Hand {
    pub(crate) fn new() -> Hand {
        Hand { cards: vec![] }
    }

    pub fn deal_card(&mut self, shoe: &mut Shoe) {
        let c = shoe.get_next_card();
        self.cards.push(c);
    }
}
