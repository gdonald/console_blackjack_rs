use crate::hand::{Hand, CountMethod};
use crate::card::CARD_FACES;

pub struct DealerHand {
    pub hand: Hand,
    pub hide_down_card: bool
}

impl DealerHand {
    pub(crate) fn new() -> DealerHand {
        DealerHand { hand: Hand { cards: vec![] }, hide_down_card: true }
    }

    pub fn upcard_is_ace(&self) -> bool {
        self.hand.cards[0].is_ace()
    }

    fn get_value(&self, count_method: CountMethod) -> u8 {
        let mut total = 0;

        for i in 0..self.hand.cards.len() {
            if i == 1 && self.hide_down_card { continue }

            let card = &self.hand.cards[i];
            let tmp_v = card.value + 1;
            let mut v = if tmp_v > 9 { 10 } else { tmp_v };

            match count_method {
                CountMethod::Soft => { if v == 1 && total < 11 { v = 11 } }
                _ => {}
            }

            total += v;
        }

        match count_method {
            CountMethod::Soft => {
                if total > 21 { return self.get_value(CountMethod::Hard) }
            }
            _ => {}
        }

        total
    }

    pub fn draw(&self) -> String {
        let mut result = " ".to_owned();

        for i in 0..self.hand.cards.len() {
            let card = &self.hand.cards[i];
            let c = if i == 1 && self.hide_down_card { format!("{}", CARD_FACES[13][0]) } else { card.draw() };
            result.push_str(&format!("{} ", c));
        }

        result.push_str(" ⇒  ");
        result.push_str(&format!("{}", self.get_value(CountMethod::Soft)));

        result
    }
}
