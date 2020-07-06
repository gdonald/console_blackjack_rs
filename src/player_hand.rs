use crate::hand::Hand;
use crate::hand::CountMethod;

pub enum Status {
    Unknown = 0,
    Won,
    Lost,
    Push,
}

pub struct PlayerHand {
    pub hand: Hand,
    pub status: Status,
    stood: bool,
    pub played: bool,
    pub payed: bool,
    pub bet: u32,
}

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

    pub fn get_action(&mut self) {}

    fn get_value(&self, count_method: CountMethod) -> u8 {
        let mut total= 0;

        for card in &self.hand.cards {
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

    pub fn draw(&self, current: bool) -> String {
        let mut result = " ".to_owned();

        for i in 0..self.hand.cards.len() {
            result.push_str(&format!("{} ", &self.hand.cards[i].draw()));
        }

        result.push_str(" ⇒  ");
        result.push_str(&format!("{}  ", self.get_value(CountMethod::Soft)));

        result.push_str(
            match self.status {
                Status::Lost => { "-" }
                Status::Won => { "+" }
                _ => {""}
            });

        result.push_str(&format!("${}", self.bet as f64 / 100.0));

        if !self.played && current {
            result.push_str(" ⇐");
        }

        result.push_str(" ");

        result.push_str(
            match self.status {
                Status::Lost => { if self.is_busted() { "Busted!" } else { "Lose!" } }
                Status::Won => { if self.hand.is_blackjack() { "Blackjack!" } else { "Won!" } }
                Status::Push => { "Push" }
                _ => {""}
            });

        result
    }

    pub fn is_busted(&self) -> bool {
        self.get_value(CountMethod::Soft) > 21
    }

    pub fn is_done(&self) -> bool {
        self.played ||
            self.stood ||
            self.hand.is_blackjack() ||
            self.is_busted() ||
            21 == self.get_value(CountMethod::Soft) ||
            21 == self.get_value(CountMethod::Hard)
    }
}