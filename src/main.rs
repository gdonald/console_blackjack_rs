use rand::thread_rng;
use rand::seq::SliceRandom;

use regex::Regex;

use std::io;
use std::io::{Read, Write, BufRead, Stdin};

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use std::convert::TryInto;

use std::vec::Vec;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

const MIN_BET: u32 = 500;
const MAX_BET: u32 = 10000000;
const MAX_PLAYER_HANDS: u8 = 7;
const CARDS_PER_DECK: u16 = 52;
const SAVE_FILE: &str = "bj.txt";

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

#[derive(Clone, Copy)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>
}

#[derive(Clone)]
pub enum HandStatus {
    Unknown = 0,
    Won,
    Lost,
    Push,
}

#[derive(Clone)]
pub struct PlayerHand {
    pub status: HandStatus,
    pub stood: bool,
    pub played: bool,
    pub payed: bool,
    pub bet: u32,
    pub hand: Hand,
}

pub struct DealerHand {
    pub hide_down_card: bool,
    pub hand: Hand,
}

pub struct Game {
    pub quitting: bool,
    pub num_decks: u16,
    pub money: u32,
    pub current_bet: u32,
    pub current_player_hand: usize,
    pub shuffle_specs: [[u8; 2]; 8],
    pub card_faces: [[&'static str; 4]; 14],
    pub matchers: HashMap<&'static str, Regex>,
    pub term: Termios,
    pub dealer_hand: DealerHand,
    pub player_hands: Vec<PlayerHand>,
    pub shoe: Vec<Card>,
}

fn save_game(game: &Game) {
    let mut f = match File::create(Path::new(SAVE_FILE)) {
        Ok(f) => f,
        Err(e) => panic!("cannot create save file: {}", e),
    };

    let buf: &str = &*format!("{}\n{}\n{}", game.num_decks, game.money, game.current_bet);

    match f.write_all(buf.as_bytes()) {
        Ok(_) => {}
        Err(e) => panic!("cannot write to save file: {}", e),
    }
}

fn load_game(game: &mut Game) {
    let result: Result<String, io::Error> = read_save_file();

    match result {
        Ok(s) => {
            let vec: Vec<&str> = s.split("\n").collect();
            if vec.len() >= 3 {
                (*game).num_decks = vec[0].parse::<u16>().unwrap();
                (*game).money = vec[1].parse::<u32>().unwrap();
                (*game).current_bet = vec[2].parse::<u32>().unwrap();
            }
        }
        _ => {}
    }
}

fn read_save_file() -> Result<String, io::Error> {
    let f = File::open(SAVE_FILE);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn play_more_hands(game: &mut Game) {
    (*game).current_player_hand += 1;

    deal_card(&mut (*game).shoe, &mut (*game).player_hands[game.current_player_hand].hand);

    if player_is_done(game) {
        process(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
}

fn play_dealer_hand(game: &mut Game) {
    if is_blackjack(&(*game).dealer_hand.hand) {
        (*game).dealer_hand.hide_down_card = false;
    }

    if !need_to_play_dealer_hand(game) {
        pay_hands(game);
        return;
    }

    (*game).dealer_hand.hide_down_card = false;

    let mut soft_count: u8 = dealer_get_value(&(*game).dealer_hand, CountMethod::Soft);
    let mut hard_count: u8 = dealer_get_value(&(*game).dealer_hand, CountMethod::Hard);

    while soft_count < 18 && hard_count < 17 {
        deal_card(&mut (*game).shoe, &mut (*game).dealer_hand.hand);
        soft_count = dealer_get_value(&(*game).dealer_hand, CountMethod::Soft);
        hard_count = dealer_get_value(&(*game).dealer_hand, CountMethod::Hard);
    }

    pay_hands(game);
}

fn new_shoe(game: &mut Game, values: &Vec<u8>) {
    let total_cards: usize = (CARDS_PER_DECK * game.num_decks).into();

    game.shoe.clear();

    while game.shoe.len() < total_cards {
        for suit in 0..4 {
            if game.shoe.len() >= total_cards { break }

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

fn new_aces(game: &mut Game) {
    new_shoe(game, &vec![0u8])
}

fn new_jacks(game: &mut Game) {
    new_shoe(game, &vec![10u8])
}

fn new_aces_jacks(game: &mut Game) {
    new_shoe(game, &vec![0u8, 10u8])
}

fn new_sevens(game: &mut Game) {
    new_shoe(game, &vec![6u8])
}

fn new_eights(game: &mut Game) {
    new_shoe(game, &vec![7u8])
}

fn dbl(game: &mut Game) {
    deal_card(&mut (*game).shoe, &mut (*game).player_hands[game.current_player_hand].hand);

    let player_hand: &mut PlayerHand = &mut (*game).player_hands[game.current_player_hand];
    player_hand.played = true;
    player_hand.bet *= 2;

    if player_is_done(game) {
        process(game);
    }
}

fn split(game: &mut Game) {
    if !can_split(game) {
        draw_hands(game);
        hand_option(game);
        return;
    }

    let new_hand: PlayerHand = PlayerHand {
        hand: Hand { cards: vec![] },
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        payed: false,
        bet: game.current_bet,
    };
    let mut hand_count: usize = game.player_hands.len();

    game.player_hands.push(new_hand);

    while hand_count > game.current_player_hand {
        let ph: PlayerHand = game.player_hands[hand_count - 1].clone();
        (*game).player_hands[hand_count] = ph;
        hand_count -= 1;
    }

    let cards: Vec<Card> = game.player_hands[game.current_player_hand].hand.cards.clone();
    game.player_hands[game.current_player_hand].hand.cards = vec![cards[0]];
    game.player_hands[game.current_player_hand + 1].hand.cards = vec![cards[1]];

    deal_card(&mut game.shoe, &mut game.player_hands[game.current_player_hand].hand);

    if player_is_done(game) {
        process(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
}

fn more_hands_to_play(game: &Game) -> bool {
    let curr: usize = game.current_player_hand;
    let len = &game.player_hands.len() - 1;

    curr < len
}

fn need_to_play_dealer_hand(game: &Game) -> bool {
    let mut player_hand: &PlayerHand;

    for x in 0..game.player_hands.len() {
        player_hand = &game.player_hands[x];

        if !(player_is_busted(player_hand) || is_blackjack(&player_hand.hand)) {
            return true;
        }
    }

    false
}

fn stand(game: &mut Game) {
    let player_hand: &mut PlayerHand = &mut game.player_hands[game.current_player_hand];
    player_hand.stood = true;
    player_hand.played = true;

    if more_hands_to_play(game) {
        play_more_hands(game);
        return;
    }

    play_dealer_hand(game);
    draw_hands(game);
    bet_options(game);
}

fn hit(game: &mut Game) {
    deal_card(&mut (*game).shoe, &mut (*game).player_hands[game.current_player_hand].hand);

    if player_is_done(game) {
        process(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
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

fn get_current_player_hand(game: &Game) -> &PlayerHand {
    &game.player_hands[game.current_player_hand]
}

fn can_hit(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    !(player_hand.played
        || player_hand.stood
        || 21 == player_get_value(player_hand, CountMethod::Hard)
        || is_blackjack(&player_hand.hand)
        || player_is_busted(player_hand))
}

fn can_stand(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    !(player_hand.stood
        || player_is_busted(player_hand)
        || is_blackjack(&player_hand.hand))
}

fn can_split(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    if player_hand.stood || game.player_hands.len() as u8 >= MAX_PLAYER_HANDS {
        return false;
    }

    if game.money < all_bets(game) + player_hand.bet {
        return false;
    }

    player_hand.hand.cards.len() == 2 && player_hand.hand.cards[0].value == player_hand.hand.cards[1].value
}

fn can_dbl(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    if game.money < all_bets(game) + player_hand.bet {
        return false;
    }

    if player_hand.stood
        || player_hand.hand.cards.len() != 2
        || player_is_busted(&player_hand)
        || is_blackjack(&player_hand.hand) {
        return false;
    }

    true
}

fn normalize_bet(game: &mut Game) {
    if game.current_bet < MIN_BET {
        (*game).current_bet = MIN_BET;
    } else if game.current_bet > MAX_BET {
        (*game).current_bet = MAX_BET;
    }

    if game.current_bet > game.money {
        (*game).current_bet = game.money;
    }
}

fn game_options(game: &mut Game) {
    clear();
    draw_hands(game);

    println!(" (N) Number of Decks  (T) Deck Type  (B) Back");

    let c: char;

    loop {
        c = read_one_char(&game.matchers.get("GameOptions").unwrap());

        match c {
            'n' => {
                get_new_num_decks(game);
                break;
            }
            't' => {
                get_new_deck_type(game);
                break;
            }
            'b' => {
                clear();
                draw_hands(game);
                bet_options(game);
                break;
            }
            _ => {
                game_options(game);
                break;
            }
        }
    }
}

fn get_new_bet(game: &mut Game) {
    clear();
    draw_hands(game);

    buffer_on(&game.term);

    print!("  Current Bet: ${}  Enter New Bet: $", game.current_bet / 100);
    io::stdout().flush().expect("Cannot flush stdout");

    let stdin: Stdin = io::stdin();
    let tmp: String = stdin.lock().lines().next().unwrap().unwrap();

    buffer_off(&game.term);

    (*game).current_bet = tmp.parse::<u32>().unwrap() * 100;
    normalize_bet(game);
    deal_new_hand(game);
}

fn get_new_num_decks(game: &mut Game) {
    clear();
    draw_hands(game);

    buffer_on(&game.term);

    print!("  Number Of Decks: {}  Enter New Number Of Decks (1-8): ", game.num_decks);
    io::stdout().flush().expect("Cannot flush stdout");

    let stdin: Stdin = io::stdin();
    let tmp: String = stdin.lock().lines().next().unwrap().unwrap();

    buffer_off(&game.term);

    let mut num_decks: u16 = tmp.parse::<u16>().unwrap();

    if num_decks < 1 { num_decks = 1 }
    if num_decks > 8 { num_decks = 8 }

    (*game).num_decks = num_decks;

    game_options(game);
}

fn get_new_deck_type(game: &mut Game) {
    clear();
    draw_hands(game);
    println!(" (1) Regular  (2) Aces  (3) Jacks  (4) Aces & Jacks  (5) Sevens  (6) Eights");

    let c: char;

    loop {
        c = read_one_char(&game.matchers.get("DeckTypeOptions").unwrap());

        match c {
            '1' => {
                new_regular(game);
                break;
            }
            '2' => {
                new_aces(game);
                break;
            }
            '3' => {
                new_jacks(game);
                break;
            }
            '4' => {
                new_aces_jacks(game);
                break;
            }
            '5' => {
                new_sevens(game);
                break;
            }
            '6' => {
                new_eights(game);
                break;
            }
            _ => {
                get_new_deck_type(game);
                break;
            }
        }
    }

    draw_hands(game);
    bet_options(game);
}

fn bet_options(game: &mut Game) {
    println!(" (D) Deal Hand  (B) Change Bet  (O) Options  (Q) Quit");

    let mut c: char;

    loop {
        c = read_one_char(&game.matchers.get("BetOptions").unwrap());

        match c {
            'd' => {
                deal_new_hand(game);
                break;
            }
            'b' => {
                get_new_bet(game);
                break;
            }
            'o' => {
                game_options(game);
                break;
            }
            'q' => {
                (*game).quitting = true;
                clear();
                break;
            }
            _ => {
                clear();
                draw_hands(game);
                bet_options(game);
            }
        }
    }
}

fn hand_option(game: &mut Game) {
    print!(" ");
    if can_hit(game) { print!("(H) Hit  ") }
    if can_stand(game) { print!("(S) Stand  ") }
    if can_split(game) { print!("(P) Split  ") }
    if can_dbl(game) { print!("(D) Double  ") }
    println!();

    let mut c: char;

    loop {
        c = read_one_char(&game.matchers.get("HandOption").unwrap());

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

fn player_is_done(game: &mut Game) -> bool {
    let player_hand: &mut PlayerHand = &mut game.player_hands[game.current_player_hand];

    if player_hand.played ||
        player_hand.stood ||
        is_blackjack(&player_hand.hand) ||
        player_is_busted(player_hand) ||
        21 == player_get_value(player_hand, CountMethod::Soft) ||
        21 == player_get_value(player_hand, CountMethod::Hard) {
        player_hand.played = true;

        if !player_hand.payed && player_is_busted(player_hand) {
            player_hand.payed = true;
            player_hand.status = HandStatus::Lost;
            (*game).money -= player_hand.bet;
        }

        return true;
    }

    false
}

fn dealer_upcard_is_ace(dealer_hand: &DealerHand) -> bool {
    is_ace(&dealer_hand.hand.cards[0])
}

fn deal_card(shoe: &mut Vec<Card>, hand: &mut Hand) {
    let c: Card = shoe.pop().unwrap();
    hand.cards.push(c);
}

fn need_to_shuffle(game: &Game) -> bool {
    let num_cards: u16 = game.num_decks * CARDS_PER_DECK;
    let current_card: u16 = num_cards - game.shoe.len() as u16;
    let used = current_card as f64 / num_cards as f64 * 100.0;
    for x in 0..game.shuffle_specs.len() {
        let spec: [u8; 2] = game.shuffle_specs[x];
        if game.num_decks == spec[1] as u16 && used > spec[0] as f64 {
            return true;
        }
    }

    false
}

fn deal_new_hand(game: &mut Game) {
    if need_to_shuffle(game) {
        new_regular(game);
    }

    (*game).player_hands = vec![PlayerHand {
        hand: Hand { cards: vec![] },
        status: HandStatus::Unknown,
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

    if player_is_done(game) {
        game.dealer_hand.hide_down_card = false;
        pay_hands(game);
        draw_hands(game);
        bet_options(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
    save_game(game);
}

fn insure_hand(game: &mut Game) {
    let player_hand: &mut PlayerHand = &mut game.player_hands[game.current_player_hand];

    player_hand.bet /= 2;
    player_hand.played = true;
    player_hand.payed = true;
    player_hand.status = HandStatus::Lost;
    (*game).money -= player_hand.bet;

    draw_hands(game);
    bet_options(game);
}

fn no_insurance(game: &mut Game) {
    let dealer_hand: &mut DealerHand = &mut game.dealer_hand;

    if is_blackjack(&dealer_hand.hand) {
        (*dealer_hand).hide_down_card = false;

        pay_hands(game);
        draw_hands(game);
        bet_options(game);
        return;
    }

    if player_is_done(game) {
        play_dealer_hand(game);
        draw_hands(game);
        bet_options(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
}

fn ask_insurance(game: &mut Game) {
    clear();
    draw_hands(game);

    println!(" Insurance?  (Y) Yes  (N) No");

    let c: char;

    loop {
        c = read_one_char(&game.matchers.get("AskInsurance").unwrap());

        match c {
            'y' => {
                insure_hand(game);
                break;
            }
            'n' => {
                no_insurance(game);
                break;
            }
            _ => {
                ask_insurance(game);
                break;
            }
        }
    }
}

fn dealer_is_busted(dealer_hand: &DealerHand) -> bool {
    dealer_get_value(dealer_hand, CountMethod::Soft) > 21
}

fn pay_hands(game: &mut Game) {
    let dealer_hand: &DealerHand = &game.dealer_hand;
    let dhv: u8 = dealer_get_value(dealer_hand, CountMethod::Soft);
    let dhb: bool = dealer_is_busted(dealer_hand);

    let mut player_hand: &mut PlayerHand;
    let mut phv: u8;

    for x in 0..game.player_hands.len() {
        player_hand = &mut game.player_hands[x];

        if player_hand.payed {
            continue;
        }

        (*player_hand).payed = true;

        phv = player_get_value(player_hand, CountMethod::Soft);

        if dhb || phv > dhv {
            if is_blackjack(&player_hand.hand) {
                (*player_hand).bet = (player_hand.bet as f64 * 1.5) as u32;
            }

            (*game).money += player_hand.bet;
            player_hand.status = HandStatus::Won;
        } else if phv < dhv {
            game.money -= player_hand.bet;
            player_hand.status = HandStatus::Lost;
        } else {
            player_hand.status = HandStatus::Push;
        }
    }

    normalize_bet(game);
    save_game(game);
}

fn player_is_busted(player_hand: &PlayerHand) -> bool {
    player_get_value(player_hand, CountMethod::Soft) > 21
}

fn clear() {
    let esc: char = 27 as char;
    print!("{}[2J{}[1;1H", esc, esc);
}

fn dealer_get_value(dealer_hand: &DealerHand, count_method: CountMethod) -> u8 {
    let mut total: u8 = 0;

    for i in 0..dealer_hand.hand.cards.len() {
        if i == 1 && dealer_hand.hide_down_card { continue }

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
            if total > 21 { return dealer_get_value(dealer_hand, CountMethod::Hard) }
        }
        _ => {}
    }

    total
}

fn dealer_draw_hand(game: &Game) -> String {
    let dealer_hand: &DealerHand = &game.dealer_hand;
    let mut result: String = " ".to_owned();

    for i in 0..dealer_hand.hand.cards.len() {
        let card: &Card = &dealer_hand.hand.cards[i];
        let c: String = if i == 1 && dealer_hand.hide_down_card { format!("{}", CARD_FACES[13][0]) } else { draw_card(game, card) };
        result.push_str(&format!("{} ", c));
    }

    result.push_str(" ⇒  ");
    result.push_str(&format!("{}", dealer_get_value(dealer_hand, CountMethod::Soft)));

    result
}

fn player_get_value(player_hand: &PlayerHand, count_method: CountMethod) -> u8 {
    let mut total: u8 = 0;

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
            if total > 21 { return player_get_value(player_hand, CountMethod::Hard) }
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
    if hand.cards.len() != 2 { return false }
    if is_ace(&hand.cards[0]) && is_ten(&hand.cards[1]) { return true }

    is_ace(&hand.cards[1]) && is_ten(&hand.cards[0])
}

fn draw_card(game: &Game, card: &Card) -> String {
    format!("{}", game.card_faces[card.value as usize][card.suit as usize])
}

fn player_draw_hand(game: &Game, index: usize) -> String {
    let player_hand: &PlayerHand = &game.player_hands[index];

    let mut result = " ".to_owned();

    for i in 0..player_hand.hand.cards.len() {
        result.push_str(&format!("{} ", &draw_card(game, &player_hand.hand.cards[i])));
    }

    result.push_str(" ⇒  ");
    result.push_str(&format!("{}  ", player_get_value(&player_hand, CountMethod::Soft)));

    result.push_str(
        match player_hand.status {
            HandStatus::Lost => { "-" }
            HandStatus::Won => { "+" }
            _ => { "" }
        });

    result.push_str(&format!("${:.2}", player_hand.bet as f64 / 100.0));

    if !player_hand.played && index == game.current_player_hand {
        result.push_str(" ⇐");
    }

    result.push_str(" ");

    result.push_str(
        match player_hand.status {
            HandStatus::Lost => { if player_is_busted(&player_hand) { "Busted!" } else { "Lose!" } }
            HandStatus::Won => { if is_blackjack(&player_hand.hand) { "Blackjack!" } else { "Won!" } }
            HandStatus::Push => { "Push" }
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
    println!(" Player ${:.2}:", game.money as f64 / 100.0);

    for i in 0..game.player_hands.len() {
        println!("{}", player_draw_hand(&game, i.try_into().unwrap()));
        println!();
        println!();
    }
}

fn read_one_char(re: &Regex) -> char {
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0; 1];
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();

    if !re.is_match(&format!("{}", buffer[0] as char)) {
        return read_one_char(re);
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
    let mut matchers = HashMap::new();
    matchers.insert("DeckTypeOptions", Regex::new("[1-6]").unwrap());
    matchers.insert("AskInsurance", Regex::new("[yn]").unwrap());
    matchers.insert("GameOptions", Regex::new("[ntb]").unwrap());
    matchers.insert("HandOption", Regex::new("[hspd]").unwrap());
    matchers.insert("BetOptions", Regex::new("[dboq]").unwrap());
    matchers.insert("NewBet", Regex::new("[0-9]").unwrap());

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
        matchers,
        term: Termios::from_fd(0).unwrap(),
        quitting: false,
    };

    load_game(&mut game);
    new_regular(&mut game);

    buffer_off(&game.term);
    loop {
        if game.quitting { break }
        deal_new_hand(&mut game);
    }
    buffer_on(&game.term);
}
