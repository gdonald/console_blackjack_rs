pub const CARD_FACES: [[&str; 4]; 14] = [
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
    ["🂠", "", "", ""]
];

pub(crate) const CARDS_PER_DECK: u16 = 52;

pub struct Card {
    pub value: u8,
    pub suit: u8
}

impl Card {
    pub(crate) fn new(value: u8, suit: u8) -> Card {
        Card { value, suit }
    }

    pub fn is_ace(&self) -> bool {
        self.value == 0
    }

    pub fn is_ten(&self) -> bool {
        self.value > 8
    }

    pub fn draw(&self) -> String {
        format!("{}", CARD_FACES[self.value as usize][self.suit as usize])
    }
}
