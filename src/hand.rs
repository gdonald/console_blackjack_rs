use crate::card::Card;
use crate::shoe::Shoe;

pub enum CountMethod {
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

    pub fn is_blackjack(&self) -> bool {
        if self.cards.len() != 2 { return false; }
        if self.cards[0].is_ace() && self.cards[1].is_ten() { return true; }

        self.cards[1].is_ace() && self.cards[0].is_ten()
    }
}
