use rand::thread_rng;
use rand::seq::SliceRandom;

use regex::Regex;

use std::io;
use std::io::Read;
use std::io::Write;
// use std::process::Command;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::convert::TryInto;

use std::vec::Vec;

const CARDS_PER_DECK: u16 = 52;

const CARD_FACES: [[&str; 4]; 14] = [
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

const SHUFFLE_SPECS: [[u8; 2]; 8] = [
    [95, 8],
    [92, 7],
    [89, 6],
    [86, 5],
    [84, 4],
    [82, 3],
    [81, 2],
    [80, 1]
];

pub enum CountMethod {
    Soft,
    Hard,
}

pub enum Status {
    Unknown = 0,
    Won,
    Lost,
    Push,
}

pub struct Card {
    pub value: u8,
    pub suit: u8,
}

pub struct Hand {
    pub cards: Vec<Card>
}

pub struct DealerHand {
    pub hand: Hand,
    pub hide_down_card: bool,
}

pub struct PlayerHand {
    pub hand: Hand,
    pub status: Status,
    pub stood: bool,
    pub played: bool,
    pub payed: bool,
    pub bet: u32,
}

pub struct Game {
    pub shoe: Vec<Card>,
    pub dealer_hand: DealerHand,
    pub player_hands: Vec<PlayerHand>,
    pub num_decks: u16,
    pub money: u32,
    pub current_bet: u32,
    pub current_player_hand: u8,
    pub shuffle_specs: [[u8; 2]; 8],
    pub card_faces: [[&'static str; 4]; 14],
}

fn load_game(_game: &Game) {
    // FILE *fp = fopen(SAVE_FILE, "r");
    //
    // if (fp != NULL) {
    //     char buffer[32];
    //
    //     fgets(buffer, sizeof(buffer), fp);
    //     game->num_decks = strtoul(buffer, NULL, 0);
    //
    //     fgets(buffer, sizeof(buffer), fp);
    //     game->money = strtoul(buffer, NULL, 0);
    //
    //     fgets(buffer, sizeof(buffer), fp);
    //     game->current_bet = strtoul(buffer, NULL, 0);
    //
    //     fclose(fp);
    // }
}

fn new_shoe(game: &mut Game, values: &Vec<u8>) {
    let total_cards = (CARDS_PER_DECK * game.num_decks).into();

    while game.shoe.len() < total_cards {
        for suit in 0..4 {
            if game.shoe.len() >= total_cards { break; }

            for value in values.iter() {
                let c: Card = Card { value: *value, suit };
                game.shoe.push(c);
            }
        }
    }

    game.shoe.shuffle(&mut thread_rng());
}

fn new_regular(game: &mut Game) {
    new_shoe(game, &(0..13).map(u8::from).collect::<Vec<_>>())
}

// pub(crate) fn new_aces(mut game: &mut Game) -> Shoe {
//     new_shoe(&mut game, &vec![0u8])
// }
//
// pub(crate) fn new_jacks(mut game: &mut Game) -> Shoe {
//     new_shoe(&mut game, &vec![10u8])
// }
//
// pub(crate) fn new_aces_jacks(mut game: &mut Game) -> Shoe {
//     new_shoe(&mut game, &vec![0u8, 10u8])
// }
//
// pub(crate) fn new_sevens(mut game: &mut Game) -> Shoe {
//     new_shoe(&mut game, &vec![6u8])
// }
//
// pub(crate) fn new_eights(mut game: &mut Game) -> Shoe {
//     new_shoe(&mut game, &vec![7u8])
// }

fn dbl(game: &Game) {}

fn split(game: &Game) {}

fn stand(game: &Game) {}

fn hit(game: &Game) {}

fn can_hit(game: &Game) -> bool {
    true
}

fn can_stand(game: &Game) -> bool {
    true
}

fn can_split(game: &Game) -> bool {
    true
}

fn can_dbl(game: &Game) -> bool {
    true
}

fn player_get_action(game: &Game) {
    print!(" ");
    if can_hit(game) { print!("(H) Hit  ") }
    if can_stand(game) { print!("(S) Stand  ") }
    if can_split(game) { print!("(P) Split  ") }
    if can_dbl(game) { print!("(D) Double  ") }
    println!();

    let mut c: char;

    loop {
        c = read_one_char("[hspd]");

        match c {
            'h' => {
                hit(game);
            }
            's' => {
                stand(game);
            }
            'p' => {
                split(game);
            }
            'd' => {
                dbl(game);
            }
            _ => {}
        }
    }
}

fn player_is_done(player_hand: &PlayerHand) -> bool {
    player_hand.played ||
        player_hand.stood ||
        is_blackjack(&player_hand.hand) ||
        player_is_busted(player_hand) ||
        21 == player_get_value(player_hand, CountMethod::Soft) ||
        21 == player_get_value(player_hand, CountMethod::Hard)
}

fn dealer_upcard_is_ace(dealer_hand: &DealerHand) -> bool {
    is_ace(&dealer_hand.hand.cards[0])
}

fn deal_card(shoe: &mut Vec<Card>, hand: &mut Hand) {
    let c = shoe.pop().unwrap();
    hand.cards.push(c);
}

fn deal_new_hand(game: &mut Game) {
    game.player_hands = vec![PlayerHand {
        hand: Hand { cards: vec![] },
        status: Status::Unknown,
        stood: false,
        played: false,
        payed: false,
        bet: game.current_bet,
    }];
    game.current_player_hand = 0;
    game.dealer_hand = DealerHand { hand: Hand { cards: vec![] }, hide_down_card: true };

    deal_card(&mut game.shoe, &mut game.player_hands[0].hand);
    deal_card(&mut game.shoe, &mut game.dealer_hand.hand);
    deal_card(&mut game.shoe, &mut game.player_hands[0].hand);
    deal_card(&mut game.shoe, &mut game.dealer_hand.hand);

    if dealer_upcard_is_ace(&game.dealer_hand) && !is_blackjack(&game.player_hands[0].hand) {
        draw_hands(game);
        ask_insurance(game);
        return;
    }

    if player_is_done(&game.player_hands[0]) {
        game.player_hands[0].played = true;

        if !game.player_hands[0].payed {
            if player_is_busted(&game.player_hands[0]) {
                game.player_hands[0].payed = true;
                game.player_hands[0].status = Status::Lost;
                game.money -= game.player_hands[0].bet;
            }
        }

        game.dealer_hand.hide_down_card = false;
        pay_hands(game);
        draw_hands(game);
        bet_options(game);
        return;
    }

    draw_hands(game);
    player_get_action(game);
    save_game();
}

fn split_hand(player_hand: &mut PlayerHand) {}

fn ask_insurance(game: &Game) {}

fn bet_options(game: &Game) {}

fn pay_hands(game: &Game) {}

fn player_is_busted(player_hand: &PlayerHand) -> bool {
    false
}

fn clear() {
    // Command::new("sh")
    //     .arg("-c")
    //     .arg("TERM=linux clear")
    //     .spawn()
    //     .expect("failed to execute process");
}

fn dealer_get_value(dealer_hand: &DealerHand, count_method: CountMethod) -> u8 {
    let mut total = 0;

    for i in 0..dealer_hand.hand.cards.len() {
        if i == 1 && dealer_hand.hide_down_card { continue; }

        let card = &dealer_hand.hand.cards[i];
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
            if total > 21 { return dealer_get_value(dealer_hand, CountMethod::Hard); }
        }
        _ => {}
    }

    total
}

fn dealer_draw_hand(game: &Game) -> String {
    let dealer_hand: &DealerHand = &game.dealer_hand;
    let mut result = " ".to_owned();

    for i in 0..dealer_hand.hand.cards.len() {
        let card = &dealer_hand.hand.cards[i];
        let c = if i == 1 && dealer_hand.hide_down_card { format!("{}", CARD_FACES[13][0]) } else { draw_card(game, card) };
        result.push_str(&format!("{} ", c));
    }

    result.push_str(" ⇒  ");
    result.push_str(&format!("{}", dealer_get_value(dealer_hand, CountMethod::Soft)));

    result
}

fn player_get_value(player_hand: &PlayerHand, count_method: CountMethod) -> u8 {
    let mut total = 0;

    for card in &player_hand.hand.cards {
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
            if total > 21 { return player_get_value(player_hand, CountMethod::Hard); }
        }
        _ => {}
    }

    total
}

fn is_ace(card: &Card) -> bool {
    card.value == 0
}

fn is_ten(card: &Card) -> bool {
    card.value > 8
}

fn is_blackjack(hand: &Hand) -> bool {
    if hand.cards.len() != 2 { return false; }
    if is_ace(&hand.cards[0]) && is_ten(&hand.cards[1]) { return true; }

    is_ace(&hand.cards[1]) && is_ten(&hand.cards[0])
}

fn draw_card(game: &Game, card: &Card) -> String {
    format!("{}", game.card_faces[card.value as usize][card.suit as usize])
}

fn player_draw_hand(game: &Game, index: u8) -> String {
    let player_hand: &PlayerHand = &game.player_hands[index as usize];

    let mut result = " ".to_owned();

    for i in 0..player_hand.hand.cards.len() {
        result.push_str(&format!("{} ", &draw_card(game, &player_hand.hand.cards[i])));
    }

    result.push_str(" ⇒  ");
    result.push_str(&format!("{}  ", player_get_value(&player_hand, CountMethod::Soft)));

    result.push_str(
        match player_hand.status {
            Status::Lost => { "-" }
            Status::Won => { "+" }
            _ => { "" }
        });

    result.push_str(&format!("${}", player_hand.bet as f64 / 100.0));

    if !player_hand.played && index == game.current_player_hand {
        result.push_str(" ⇐");
    }

    result.push_str(" ");

    result.push_str(
        match player_hand.status {
            Status::Lost => { if player_is_busted(&player_hand) { "Busted!" } else { "Lose!" } }
            Status::Won => { if is_blackjack(&player_hand.hand) { "Blackjack!" } else { "Won!" } }
            Status::Push => { "Push" }
            _ => { "" }
        });

    result
}

fn draw_hands(game: &Game) {
    clear();

    println!();
    println!(" Dealer:");
    println!("{}", dealer_draw_hand(&game));

    println!();
    println!(" Player ${}:", game.money as f64 / 100.0);

    for i in 0..game.player_hands.len() {
        println!("{}", player_draw_hand(&game, i.try_into().unwrap()));
        println!();
        println!();
    }
}

fn save_game() {}

fn read_one_char(matcher: &str) -> char {
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();

    let re = Regex::new(&matcher).unwrap();
    if !re.is_match(&format!("{}", buffer[0] as char)) {
        return read_one_char(matcher);
    }

    buffer[0] as char
}

fn buffer_off(term: &Termios) {
    let mut new_term = term.clone();
    new_term.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut new_term).unwrap();
}

fn buffer_on(term: &Termios) {
    tcsetattr(0, TCSANOW, &term).unwrap();
}

fn main() {
    let mut game: Game = Game {
        shoe: vec![],
        dealer_hand: DealerHand { hand: Hand { cards: vec![] }, hide_down_card: true },
        player_hands: vec![],
        num_decks: 8,
        money: 10000,
        current_bet: 500,
        current_player_hand: 0,
        shuffle_specs: SHUFFLE_SPECS,
        card_faces: CARD_FACES,
    };

    load_game(&game);
    new_regular(&mut game);

    let term = Termios::from_fd(0).unwrap();

    buffer_off(&term);
    deal_new_hand(&mut game);
    buffer_on(&term);
}
