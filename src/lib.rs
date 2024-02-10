pub const MIN_BET: u32 = 500;
pub const MAX_BET: u32 = 10000000;
pub const MAX_PLAYER_HANDS: u8 = 7;
pub const CARDS_PER_DECK: u16 = 52;
pub const SAVE_FILE: &str = "bj.txt";

pub const CARD_FACES: [[&str; 4]; 14] = [
    ["A♠", "A♥", "A♣", "A♦"],
    ["2♠", "2♥", "2♣", "2♦"],
    ["3♠", "3♥", "3♣", "3♦"],
    ["4♠", "4♥", "4♣", "4♦"],
    ["5♠", "5♥", "5♣", "5♦"],
    ["6♠", "6♥", "6♣", "6♦"],
    ["7♠", "7♥", "7♣", "7♦"],
    ["8♠", "8♥", "8♣", "8♦"],
    ["9♠", "9♥", "9♣", "9♦"],
    ["T♠", "T♥", "T♣", "T♦"],
    ["J♠", "J♥", "J♣", "J♦"],
    ["Q♠", "Q♥", "Q♣", "Q♦"],
    ["K♠", "K♥", "K♣", "K♦"],
    ["??", "", "", ""],
];

pub const CARD_FACES_2: [[&str; 4]; 14] = [
    ["🂡", "🂱", "🃁", "🃑"],
    ["🂢", "🂲", "🃂", "🃒"],
    ["🂣", "🂳", "🃃", "🃓"],
    ["🂤", "🂴", "🃄", "🃔"],
    ["🂥", "🂵", "🃅", "🃕"],
    ["🂦", "🂶", "🃆", "🃖"],
    ["🂧", "🂷", "🃇", "🃗"],
    ["🂨", "🂸", "🃈", "🃘"],
    ["🂩", "🂹", "🃉", "🃙"],
    ["🂪", "🂺", "🃊", "🃚"],
    ["🂫", "🂻", "🃋", "🃛"],
    ["🂭", "🂽", "🃍", "🃝"],
    ["🂮", "🂾", "🃎", "🃞"],
    ["🂠", "", "", ""],
];

pub const SHUFFLE_SPECS: [[u8; 2]; 8] = [
    [95, 8],
    [92, 7],
    [89, 6],
    [86, 5],
    [84, 4],
    [82, 3],
    [81, 2],
    [80, 1],
];

pub enum CountMethod {
    Soft,
    Hard,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.suit == other.suit
    }
}

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

pub fn is_ace(card: &Card) -> bool {
    card.value == 0
}

pub fn is_ten(card: &Card) -> bool {
    card.value > 8
}
