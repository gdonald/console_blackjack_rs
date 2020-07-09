use crate::card::Card;
use crate::shoe::Shoe;

impl Hand {
    pub(crate) fn new() -> Hand {
        Hand { cards: vec![] }
    }
}
