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
use std::ops::Add;

const MAX_PLAYER_HANDS: u8 = 7;
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

fn play_more_hands(game: &mut Game) {
    &game.current_player_hand.add(1);
    let player_hand: &mut &PlayerHand = &mut &game.player_hands[game.current_player_hand as usize];

    deal_card(&mut (*game).shoe, &mut (*game).dealer_hand.hand);

    if player_is_done(player_hand) {
        process(game);
        return;
    }

    draw_hands(game);
    player_get_action(game);
}

fn play_dealer_hand(game: &mut Game) {
    let mut soft_count: u8;
    let mut hard_count: u8;

    if is_blackjack(&(*game).dealer_hand.hand) {
        (*game).dealer_hand.hide_down_card = false;
    }

    if !need_to_play_dealer_hand(game) {
        pay_hands(game);
        return;
    }

    (*game).dealer_hand.hide_down_card = false;

    soft_count = dealer_get_value(&(*game).dealer_hand, CountMethod::Soft);
    hard_count = dealer_get_value(&(*game).dealer_hand, CountMethod::Hard);

    while soft_count < 18 && hard_count < 17 {
        deal_card(&mut (*game).shoe, &mut (*game).dealer_hand.hand);
        soft_count = dealer_get_value(&(*game).dealer_hand, CountMethod::Soft);
        hard_count = dealer_get_value(&(*game).dealer_hand, CountMethod::Hard);
    }

    pay_hands(game);
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

fn dbl(game: &mut Game) {
    deal_card(&mut (*game).shoe, &mut (*game).player_hands[game.current_player_hand as usize].hand);

    (*game).player_hands[game.current_player_hand as usize].played = true;
    (*game).player_hands[game.current_player_hand as usize].bet *= 2;

    if player_is_done(&game.player_hands[game.current_player_hand as usize]) {
        process(game);
    }
}

fn split(game: &Game) {
    // struct PlayerHand new_hand = {.bet=game->current_bet};
    // unsigned hand_count = game->total_player_hands;
    // struct PlayerHand *this_hand;
    // struct PlayerHand *split_hand;
    // struct Card card;
    //
    // if (!player_can_split(game)) {
    //     draw_hands(game);
    //     player_get_action(game);
    //     return;
    // }
    //
    // game->player_hands[game->total_player_hands++] = new_hand;
    //
    // while (hand_count > game->current_player_hand) {
    //     game->player_hands[hand_count] = game->player_hands[hand_count - 1];
    //     --hand_count;
    // }
    //
    // this_hand = &game->player_hands[game->current_player_hand];
    // split_hand = &game->player_hands[game->current_player_hand + 1];
    //
    // card = this_hand->hand.cards[1];
    // split_hand->hand.cards[0] = card;
    // split_hand->hand.num_cards = 1;
    // this_hand->hand.num_cards = 1;
    // deal_card(&game->shoe, &this_hand->hand);
    //
    // if (player_is_done(game, this_hand)) {
    //     process(game);
    //     return;
    // }
    //
    // draw_hands(game);
    // player_get_action(game);
}

fn more_hands_to_play(game: &Game) -> bool {
    game.current_player_hand < (&game.player_hands.len() - 1).try_into().unwrap()
}

fn need_to_play_dealer_hand(game: &Game) -> bool {
    let mut player_hand: &PlayerHand;

    for x in 0..game.player_hands.len() {
        player_hand = &game.player_hands[x];

        if !(is_busted(player_hand) || is_blackjack(&player_hand.hand)) {
            return true;
        }
    }

    false
}

fn stand(game: &mut Game) {
    // let mut player_hand: &mut PlayerHand = &mut &game.player_hands[game.current_player_hand as usize];

    (*game).player_hands[game.current_player_hand as usize].stood = true;
    (*game).player_hands[game.current_player_hand as usize].played = true;

    if more_hands_to_play(game) {
        play_more_hands(game);
        return;
    }

    play_dealer_hand(game);
    draw_hands(game);
    bet_options(game);
}

fn hit(game: &mut Game) {
    deal_card(&mut (*game).shoe, &mut (*game).player_hands[game.current_player_hand as usize].hand);

    if player_is_done(&game.player_hands[game.current_player_hand as usize]) {
        process(game);
        return;
    }

    draw_hands(game);
    player_get_action(game);
}

fn process(game: &mut Game) {
    if more_hands_to_play(game) {
        play_more_hands(game);
        return;
    }

    play_dealer_hand(game);
    draw_hands(game);
    bet_options(game);
}

fn can_hit(game: &Game) -> bool {
    let player_hand: &PlayerHand = &game.player_hands[game.current_player_hand as usize];

    player_hand.played
        || player_hand.stood
        || 21 == player_get_value(player_hand, CountMethod::Hard)
        || is_blackjack(&player_hand.hand)
        || is_busted(player_hand)
}

fn can_stand(game: &Game) -> bool {
    let player_hand: &PlayerHand = &game.player_hands[game.current_player_hand as usize];

    player_hand.stood
        || is_busted(player_hand)
        || is_blackjack(&player_hand.hand)
}

fn can_split(game: &Game) -> bool {
    let player_hand: &PlayerHand = &game.player_hands[game.current_player_hand as usize];

    if player_hand.stood || game.player_hands.len() as u8 >= MAX_PLAYER_HANDS {
        return false;
    }

    if game.money < all_bets(game) + player_hand.bet {
        return false;
    }

    player_hand.hand.cards.len() == 2 && player_hand.hand.cards[0].value == player_hand.hand.cards[1].value
}

fn can_dbl(game: &Game) -> bool {
    let player_hand: &PlayerHand = &game.player_hands[game.current_player_hand as usize];

    if game.money < all_bets(game) + player_hand.bet {
        return false;
    }

    if player_hand.stood
        || player_hand.hand.cards.len() != 2
        || is_busted(&player_hand)
        || is_blackjack(&player_hand.hand) {
        return false;
    }

    return true;
}

fn player_get_action(game: &mut Game) {
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
                break;
            }
            's' => {
                stand(game);
                break;
            }
            'p' => {
                split(game);
                break;
            }
            'd' => {
                dbl(game);
                break;
            }
            _ => {}
        }
    }
}

fn all_bets(game: &Game) -> u32 {
    let mut bets: u32 = 0;

    for x in 0..game.player_hands.len() {
        bets += game.player_hands[x].bet;
    }

    bets
}

fn player_is_done(player_hand: &PlayerHand) -> bool {
    player_hand.played ||
        player_hand.stood ||
        is_blackjack(&player_hand.hand) ||
        is_busted(player_hand) ||
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
    (*game).player_hands = vec![PlayerHand {
        hand: Hand { cards: vec![] },
        status: Status::Unknown,
        stood: false,
        played: false,
        payed: false,
        bet: game.current_bet,
    }];
    (*game).current_player_hand = 0;
    (*game).dealer_hand = DealerHand { hand: Hand { cards: vec![] }, hide_down_card: true };

    deal_card(&mut (*game).shoe, &mut (*game).player_hands[0].hand);
    deal_card(&mut (*game).shoe, &mut (*game).dealer_hand.hand);
    deal_card(&mut (*game).shoe, &mut (*game).player_hands[0].hand);
    deal_card(&mut (*game).shoe, &mut (*game).dealer_hand.hand);

    if dealer_upcard_is_ace(&game.dealer_hand) && !is_blackjack(&game.player_hands[0].hand) {
        draw_hands(game);
        ask_insurance(game);
        return;
    }

    if player_is_done(&game.player_hands[0]) {

        // this needs to be just after every player_is_done call site
        game.player_hands[0].played = true;

        if !(*game).player_hands[0].payed && is_busted(&game.player_hands[0]) {
            (*game).player_hands[0].payed = true;
            (*game).player_hands[0].status = Status::Lost;
            (*game).money -= game.player_hands[0].bet;
        }
        // end this

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

fn is_busted(player_hand: &PlayerHand) -> bool {
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
            Status::Lost => { if is_busted(&player_hand) { "Busted!" } else { "Lose!" } }
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
