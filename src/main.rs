enum CountMethod {
    Soft,
    Hard,
}

enum HandStatus {
    Won = 1,
    Lost,
    Push,
}

struct Card {
    value: u8,
    suit: u8,
}

struct DealerHand {
    cards: Vec<Card>,
    hide_down_card: bool,
}

struct PlayerHand {
    cards: Vec<Card>,
    bet: u32,
    stood: bool,
    played: bool,
    payed: bool,
    hand_status: HandStatus,
}

struct Game {
    shoe: Vec<Card>,
    dealer_hand: DealerHand,
    player_hands: Vec<PlayerHand>,
    num_decks: u8,
    money: u32,
    current_bet: u32,
    current_player_hand: u8,
    shuffle_specs: [[u8; 2]; 8],
    card_faces: [[&'static str; 4]; 14],
}

fn main() {
    let mut game: Game = Game {
        shoe: vec![],
        dealer_hand: DealerHand { cards: vec![], hide_down_card: false },
        player_hands: vec![],
        num_decks: 8,
        money: 10000,
        current_bet: 500,
        current_player_hand: 0,
        shuffle_specs: [
            [95, 8],
            [92, 7],
            [89, 6],
            [86, 5],
            [84, 4],
            [82, 3],
            [81, 2],
            [80, 1]
        ],
        card_faces: [
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
        ],
    };


}
