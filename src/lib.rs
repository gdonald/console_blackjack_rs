use rand::prelude::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write as OtherWrite;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Stdin, Write};
use std::path::Path;
use std::str::FromStr;
use termios::{tcsetattr, Termios, ICANON, TCSANOW};

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

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.cards.len() != other.cards.len() {
            return false;
        }

        for (index, card) in self.cards.iter().enumerate() {
            if card != &other.cards[index] {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
pub enum HandStatus {
    Unknown = 0,
    Won,
    Lost,
    Push,
}

impl PartialEq for HandStatus {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Clone, Debug)]
pub struct PlayerHand {
    pub status: HandStatus,
    pub stood: bool,
    pub played: bool,
    pub paid: bool,
    pub bet: u32,
    pub hand: Hand,
}

impl PartialEq for PlayerHand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand.cards.len() != other.hand.cards.len() {
            return false;
        }

        for (index, card) in self.hand.cards.iter().enumerate() {
            if card != &other.hand.cards[index] {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug)]
pub struct DealerHand {
    pub hide_down_card: bool,
    pub hand: Hand,
}

impl PartialEq for DealerHand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand != other.hand {
            return false;
        }

        if self.hide_down_card != other.hide_down_card {
            return false;
        }

        true
    }
}

pub trait TermiosWrapper {
    fn from_fd(fd: i32) -> Result<Self, io::Error>
    where
        Self: Sized;

    fn buffer_off(&mut self);
    fn buffer_on(&mut self);
}

impl TermiosWrapper for Termios {
    fn from_fd(fd: i32) -> Result<Self, io::Error> {
        Termios::from_fd(fd)
    }

    fn buffer_off(&mut self) {
        self.c_lflag &= !ICANON;
        tcsetattr(0, TCSANOW, &self).unwrap();
    }

    fn buffer_on(&mut self) {
        tcsetattr(0, TCSANOW, &self).unwrap();
    }
}

pub struct MockTermiosWrapper;

impl TermiosWrapper for MockTermiosWrapper {
    fn from_fd(_fd: i32) -> Result<Self, io::Error> {
        Ok(MockTermiosWrapper)
    }

    fn buffer_off(&mut self) {
        // noop
    }

    fn buffer_on(&mut self) {
        // noop
    }
}

pub struct Game {
    pub quitting: bool,
    pub num_decks: u16,
    pub deck_type: u8,
    pub face_type: u8,
    pub money: u32,
    pub current_bet: u32,
    pub current_player_hand: usize,
    pub shuffle_specs: [[u8; 2]; 8],
    pub matchers: HashMap<&'static str, Regex>,
    pub term: Box<dyn TermiosWrapper>,
    pub dealer_hand: DealerHand,
    pub player_hands: Vec<PlayerHand>,
    pub shoe: Vec<Card>,
}

pub fn initialize_matchers() -> HashMap<&'static str, Regex> {
    let mut matchers = HashMap::new();
    matchers.insert("DeckTypeOptions", Regex::new("[1-6]").unwrap());
    matchers.insert("FaceTypeOptions", Regex::new("[1-2]").unwrap());
    matchers.insert("AskInsurance", Regex::new("[yn]").unwrap());
    matchers.insert("GameOptions", Regex::new("[ntfb]").unwrap());
    matchers.insert("HandOption", Regex::new("[hspd]").unwrap());
    matchers.insert("BetOptions", Regex::new("[dboq]").unwrap());
    matchers
}

pub fn save_game(game: &Game) {
    let mut f = match File::create(Path::new(SAVE_FILE)) {
        Ok(f) => f,
        Err(e) => panic!("cannot create save file: {}", e),
    };

    let buf: &str = &*format!(
        "{}\n{}\n{}\n{}\n{}",
        game.num_decks, game.money, game.current_bet, game.deck_type, game.face_type
    );

    match f.write_all(buf.as_bytes()) {
        Ok(_) => {}
        Err(e) => panic!("cannot write to save file: {}", e),
    }
}

pub fn read_save_file() -> Result<String, io::Error> {
    let f = File::open(SAVE_FILE);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn load_game(game: &mut Game) {
    let result: Result<String, io::Error> = read_save_file();

    if let Ok(s) = result {
        let vec: Vec<&str> = s.split('\n').collect();
        if vec.len() >= 5 {
            (*game).num_decks = vec[0].parse::<u16>().unwrap();
            (*game).money = vec[1].parse::<u32>().unwrap();
            (*game).current_bet = vec[2].parse::<u32>().unwrap();
            (*game).deck_type = vec[3].parse::<u8>().unwrap();
            (*game).face_type = vec[4].parse::<u8>().unwrap();
        }
    }
}

pub fn build_game<T: TermiosWrapper + 'static>() -> Game {
    let term = T::from_fd(0).unwrap();

    Game {
        shoe: vec![],
        dealer_hand: DealerHand {
            hand: Hand { cards: vec![] },
            hide_down_card: true,
        },
        player_hands: vec![],
        num_decks: 8,
        deck_type: 1,
        face_type: 1,
        money: 10000,
        current_bet: 500,
        current_player_hand: 0,
        shuffle_specs: SHUFFLE_SPECS,
        matchers: initialize_matchers(),
        term: Box::new(term),
        quitting: false,
    }
}

pub fn buffer_off(term: &mut dyn TermiosWrapper) {
    term.buffer_off();
}

pub fn buffer_on(term: &mut dyn TermiosWrapper) {
    term.buffer_on();
}

pub fn deal_new_hand(game: &mut Game) {
    if need_to_shuffle(game) {
        build_new_shoe(game);
    }

    (*game).player_hands = vec![PlayerHand {
        hand: Hand { cards: vec![] },
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: game.current_bet,
    }];
    (*game).current_player_hand = 0;
    (*game).dealer_hand = DealerHand {
        hand: Hand { cards: vec![] },
        hide_down_card: true,
    };

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

pub fn play_more_hands(game: &mut Game) {
    (*game).current_player_hand += 1;

    deal_card(
        &mut (*game).shoe,
        &mut (*game).player_hands[game.current_player_hand].hand,
    );

    if player_is_done(game) {
        process(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
}

pub fn play_dealer_hand(game: &mut Game) {
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

pub fn new_shoe(game: &mut Game, values: &[u8]) {
    let total_cards: usize = (CARDS_PER_DECK * game.num_decks).into();

    game.shoe.clear();

    while game.shoe.len() < total_cards {
        for suit in 0..4 {
            if game.shoe.len() >= total_cards {
                break;
            }

            for value in values.iter() {
                let c: Card = Card {
                    value: *value,
                    suit,
                };
                game.shoe.push(c);
            }
        }
    }

    game.shoe.shuffle(&mut thread_rng());
}

pub fn new_regular(game: &mut Game) {
    new_shoe(game, &(0..13).map(u8::from).collect::<Vec<_>>())
}

pub fn new_aces(game: &mut Game) {
    new_shoe(game, &[0u8])
}

pub fn new_jacks(game: &mut Game) {
    new_shoe(game, &[10u8])
}

pub fn new_aces_jacks(game: &mut Game) {
    new_shoe(game, &[0u8, 10u8])
}

pub fn new_sevens(game: &mut Game) {
    new_shoe(game, &[6u8])
}

pub fn new_eights(game: &mut Game) {
    new_shoe(game, &[7u8])
}

pub fn build_new_shoe(game: &mut Game) {
    match game.deck_type {
        2 => {
            new_aces(game);
        }
        3 => {
            new_jacks(game);
        }
        4 => {
            new_aces_jacks(game);
        }
        5 => {
            new_sevens(game);
        }
        6 => {
            new_eights(game);
        }
        _ => {
            new_regular(game);
        }
    }
}

pub fn dbl(game: &mut Game) {
    deal_card(
        &mut (*game).shoe,
        &mut (*game).player_hands[game.current_player_hand].hand,
    );

    let player_hand: &mut PlayerHand = &mut (*game).player_hands[game.current_player_hand];
    player_hand.played = true;
    player_hand.bet *= 2;

    if player_is_done(game) {
        process(game);
    }
}

pub fn split(game: &mut Game) {
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
        paid: false,
        bet: game.current_bet,
    };
    let mut hand_count: usize = game.player_hands.len();

    game.player_hands.push(new_hand);

    while hand_count > game.current_player_hand {
        let ph: PlayerHand = game.player_hands[hand_count - 1].clone();
        (*game).player_hands[hand_count] = ph;
        hand_count -= 1;
    }

    let cards: Vec<Card> = game.player_hands[game.current_player_hand]
        .hand
        .cards
        .clone();
    game.player_hands[game.current_player_hand].hand.cards = vec![cards[0]];
    game.player_hands[game.current_player_hand + 1].hand.cards = vec![cards[1]];

    deal_card(
        &mut game.shoe,
        &mut game.player_hands[game.current_player_hand].hand,
    );

    if player_is_done(game) {
        process(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
}

pub fn more_hands_to_play(game: &Game) -> bool {
    let curr: usize = game.current_player_hand;
    let len = &game.player_hands.len() - 1;

    curr < len
}

pub fn need_to_play_dealer_hand(game: &Game) -> bool {
    game.player_hands
        .iter()
        .any(|hand| !player_is_busted(hand) && !is_blackjack(&hand.hand))
}

pub fn stand(game: &mut Game) {
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

pub fn hit(game: &mut Game) {
    deal_card(
        &mut (*game).shoe,
        &mut (*game).player_hands[game.current_player_hand].hand,
    );

    if player_is_done(game) {
        process(game);
        return;
    }

    draw_hands(game);
    hand_option(game);
}

pub fn process(game: &mut Game) {
    if more_hands_to_play(game) {
        play_more_hands(game);
        return;
    }

    play_dealer_hand(game);
    draw_hands(game);
    bet_options(game);
}

pub fn get_current_player_hand(game: &Game) -> &PlayerHand {
    &game.player_hands[game.current_player_hand]
}

pub fn can_hit(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    !(player_hand.played
        || player_hand.stood
        || 21 == player_get_value(player_hand, CountMethod::Hard)
        || is_blackjack(&player_hand.hand)
        || player_is_busted(player_hand))
}

pub fn can_stand(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    !(player_hand.stood || player_is_busted(player_hand) || is_blackjack(&player_hand.hand))
}

pub fn can_split(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    if player_hand.stood || game.player_hands.len() as u8 >= MAX_PLAYER_HANDS {
        return false;
    }

    if game.money < all_bets(game) + player_hand.bet {
        return false;
    }

    player_hand.hand.cards.len() == 2
        && player_hand.hand.cards[0].value == player_hand.hand.cards[1].value
}

pub fn can_dbl(game: &Game) -> bool {
    let player_hand: &PlayerHand = get_current_player_hand(game);

    if game.money < all_bets(game) + player_hand.bet {
        return false;
    }

    if player_hand.stood
        || player_hand.hand.cards.len() != 2
        || player_is_busted(player_hand)
        || is_blackjack(&player_hand.hand)
    {
        return false;
    }

    true
}

pub fn normalize_bet(game: &mut Game) {
    if game.current_bet < MIN_BET {
        (*game).current_bet = MIN_BET;
    } else if game.current_bet > MAX_BET {
        (*game).current_bet = MAX_BET;
    }

    if game.current_bet > game.money {
        (*game).current_bet = game.money;
    }
}

pub fn game_options(game: &mut Game) {
    clear();
    draw_hands(game);

    println!(" (N) Number of Decks  (T) Deck Type  (F) Face Type  (B) Back");

    let c: char = read_one_char(game.matchers.get("GameOptions").unwrap());

    match c {
        'n' => {
            get_new_num_decks(game);
        }
        't' => {
            get_new_deck_type(game);
        }
        'f' => {
            get_new_face_type(game);
        }
        'b' => {
            clear();
            draw_hands(game);
            bet_options(game);
        }
        _ => {
            game_options(game);
        }
    }
}

pub fn get_new_bet(game: &mut Game) {
    clear();
    draw_hands(game);

    buffer_on(&mut *game.term);

    print!(
        "  Current Bet: ${}  Enter New Bet: $",
        game.current_bet / 100
    );
    io::stdout().flush().expect("Cannot flush stdout");

    let stdin: Stdin = io::stdin();
    let tmp: String = stdin.lock().lines().next().unwrap().unwrap();

    match u32::from_str(&tmp) {
        Ok(bet) => {
            (*game).current_bet = bet * 100;
            normalize_bet(game);
            deal_new_hand(game);
        }
        Err(_) => {
            get_new_bet(game);
        }
    }

    buffer_off(&mut *game.term);
}

pub fn get_new_num_decks(game: &mut Game) {
    clear();
    draw_hands(game);

    buffer_on(&mut *game.term);

    print!(
        "  Number Of Decks: {}  Enter New Number Of Decks (1-8): ",
        game.num_decks
    );
    io::stdout().flush().expect("Cannot flush stdout");

    let stdin: Stdin = io::stdin();
    let tmp: String = stdin.lock().lines().next().unwrap().unwrap();

    match u16::from_str(&tmp) {
        Ok(num_decks) => {
            (*game).num_decks = num_decks.clone().clamp(1, 8);
            game_options(game);
        }
        Err(_) => {
            get_new_num_decks(game);
        }
    }

    buffer_off(&mut *game.term);
}

pub fn get_new_deck_type(game: &mut Game) {
    clear();
    draw_hands(game);
    println!(" (1) Regular  (2) Aces  (3) Jacks  (4) Aces & Jacks  (5) Sevens  (6) Eights");

    let c: char = read_one_char(game.matchers.get("DeckTypeOptions").unwrap());
    let c_val: u8 = c.to_digit(10).unwrap().try_into().unwrap();

    match c_val {
        1..=6 => {
            game.deck_type = c_val;
            if c_val > 1 {
                game.num_decks = 8;
            }
            build_new_shoe(game);
        }
        _ => {
            get_new_deck_type(game);
        }
    }

    draw_hands(game);
    bet_options(game);
}

pub fn get_new_face_type(game: &mut Game) {
    clear();
    draw_hands(game);
    println!(" (1) A♠ (2) 🂡");

    let c: char = read_one_char(game.matchers.get("FaceTypeOptions").unwrap());
    let c_val: u8 = c.to_digit(10).unwrap().try_into().unwrap();

    match c_val {
        1..=2 => {
            game.face_type = c_val;
        }
        _ => {
            get_new_face_type(game);
        }
    }

    draw_hands(game);
    bet_options(game);
}

pub fn bet_options(game: &mut Game) {
    println!(" (D) Deal Hand  (B) Change Bet  (O) Options  (Q) Quit");

    let mut c: char;

    loop {
        c = read_one_char(game.matchers.get("BetOptions").unwrap());

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

pub fn hand_option(game: &mut Game) {
    print!(" ");
    if can_hit(game) {
        print!("(H) Hit  ")
    }
    if can_stand(game) {
        print!("(S) Stand  ")
    }
    if can_split(game) {
        print!("(P) Split  ")
    }
    if can_dbl(game) {
        print!("(D) Double  ")
    }
    println!();

    let mut c: char;

    loop {
        c = read_one_char(game.matchers.get("HandOption").unwrap());

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

pub fn all_bets(game: &Game) -> u32 {
    let mut bets: u32 = 0;

    for x in 0..game.player_hands.len() {
        bets += game.player_hands[x].bet;
    }

    bets
}

pub fn player_is_done(game: &mut Game) -> bool {
    let player_hand: &mut PlayerHand = &mut game.player_hands[game.current_player_hand];

    if player_hand.played
        || player_hand.stood
        || is_blackjack(&player_hand.hand)
        || player_is_busted(player_hand)
        || 21 == player_get_value(player_hand, CountMethod::Soft)
        || 21 == player_get_value(player_hand, CountMethod::Hard)
    {
        player_hand.played = true;

        if !player_hand.paid && player_is_busted(player_hand) {
            player_hand.paid = true;
            player_hand.status = HandStatus::Lost;
            (*game).money -= player_hand.bet;
        }

        return true;
    }

    false
}

pub fn dealer_upcard_is_ace(dealer_hand: &DealerHand) -> bool {
    is_ace(&dealer_hand.hand.cards[0])
}

pub fn deal_card(shoe: &mut Vec<Card>, hand: &mut Hand) {
    let c: Card = shoe.pop().unwrap();
    hand.cards.push(c);
}

pub fn need_to_shuffle(game: &Game) -> bool {
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

pub fn insure_hand(game: &mut Game) {
    let player_hand: &mut PlayerHand = &mut game.player_hands[game.current_player_hand];

    player_hand.bet /= 2;
    player_hand.played = true;
    player_hand.paid = true;
    player_hand.status = HandStatus::Lost;
    (*game).money -= player_hand.bet;

    draw_hands(game);
    bet_options(game);
}

pub fn no_insurance(game: &mut Game) {
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

pub fn ask_insurance(game: &mut Game) {
    clear();
    draw_hands(game);

    println!(" Insurance?  (Y) Yes  (N) No");

    let c: char = read_one_char(game.matchers.get("AskInsurance").unwrap());

    match c {
        'y' => {
            insure_hand(game);
        }
        'n' => {
            no_insurance(game);
        }
        _ => {
            ask_insurance(game);
        }
    }
}

pub fn dealer_is_busted(dealer_hand: &DealerHand) -> bool {
    dealer_get_value(dealer_hand, CountMethod::Soft) > 21
}

pub fn pay_hands(game: &mut Game) {
    let dealer_hand: &DealerHand = &game.dealer_hand;
    let dhv: u8 = dealer_get_value(dealer_hand, CountMethod::Soft);
    let dhb: bool = dealer_is_busted(dealer_hand);

    let mut player_hand: &mut PlayerHand;
    let mut phv: u8;

    for x in 0..game.player_hands.len() {
        player_hand = &mut game.player_hands[x];

        if player_hand.paid {
            continue;
        }

        (*player_hand).paid = true;

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

pub fn player_is_busted(player_hand: &PlayerHand) -> bool {
    player_get_value(player_hand, CountMethod::Soft) > 21
}

pub fn clear() {
    let esc: char = 27 as char;
    print!("{}[2J{}[1;1H", esc, esc);
}

pub fn dealer_get_value(dealer_hand: &DealerHand, count_method: CountMethod) -> u8 {
    let mut total: u8 = 0;

    for i in 0..dealer_hand.hand.cards.len() {
        if i == 1 && dealer_hand.hide_down_card {
            continue;
        }

        let card = &dealer_hand.hand.cards[i];
        let tmp_v = card.value + 1;
        let mut v = if tmp_v > 9 { 10 } else { tmp_v };

        if let CountMethod::Soft = count_method {
            if v == 1 && total < 11 {
                v = 11
            }
        }

        total += v;
    }

    if let CountMethod::Soft = count_method {
        if total > 21 {
            return dealer_get_value(dealer_hand, CountMethod::Hard);
        }
    }

    total
}

pub fn dealer_draw_hand(game: &Game) -> String {
    let dealer_hand: &DealerHand = &game.dealer_hand;
    let mut result: String = " ".to_owned();

    for i in 0..dealer_hand.hand.cards.len() {
        let card: &Card = &dealer_hand.hand.cards[i];
        let c: String = if i == 1 && dealer_hand.hide_down_card {
            draw_card(game, &Card { value: 13, suit: 0 })
        } else {
            draw_card(game, card)
        };
        write!(result, "{} ", c).unwrap();
    }

    result.push_str(" ⇒  ");
    write!(
        result,
        "{}",
        dealer_get_value(dealer_hand, CountMethod::Soft)
    )
    .unwrap();

    result
}

pub fn player_get_value(player_hand: &PlayerHand, count_method: CountMethod) -> u8 {
    let mut total: u8 = 0;

    for card in &player_hand.hand.cards {
        let tmp_v = card.value + 1;
        let mut v = if tmp_v > 9 { 10 } else { tmp_v };

        if let CountMethod::Soft = count_method {
            if v == 1 && total < 11 {
                v = 11
            }
        }

        total += v;
    }

    if let CountMethod::Soft = count_method {
        if total > 21 {
            return player_get_value(player_hand, CountMethod::Hard);
        }
    }

    total
}

pub fn is_blackjack(hand: &Hand) -> bool {
    if hand.cards.len() != 2 {
        return false;
    }
    if is_ace(&hand.cards[0]) && is_ten(&hand.cards[1]) {
        return true;
    }

    is_ace(&hand.cards[1]) && is_ten(&hand.cards[0])
}

pub fn draw_card(game: &Game, card: &Card) -> String {
    if game.face_type == 2 {
        return CARD_FACES_2[card.value as usize][card.suit as usize].to_string();
    }

    CARD_FACES[card.value as usize][card.suit as usize].to_string()
}

pub fn player_draw_hand(game: &Game, index: usize) -> String {
    let player_hand: &PlayerHand = &game.player_hands[index];

    let mut result = " ".to_owned();

    for i in 0..player_hand.hand.cards.len() {
        write!(result, "{} ", &draw_card(game, &player_hand.hand.cards[i])).unwrap();
    }

    result.push_str(" ⇒  ");
    write!(
        result,
        "{}  ",
        player_get_value(player_hand, CountMethod::Soft)
    )
    .unwrap();

    result.push_str(match player_hand.status {
        HandStatus::Lost => "-",
        HandStatus::Won => "+",
        _ => "",
    });

    write!(result, "${:.2}", player_hand.bet as f64 / 100.0).unwrap();

    if !player_hand.played && index == game.current_player_hand {
        result.push_str(" ⇐");
    }

    result.push(' ');

    result.push_str(match player_hand.status {
        HandStatus::Lost => {
            if player_is_busted(player_hand) {
                "Busted!"
            } else {
                "Lose!"
            }
        }
        HandStatus::Won => {
            if is_blackjack(&player_hand.hand) {
                "Blackjack!"
            } else {
                "Won!"
            }
        }
        HandStatus::Push => "Push",
        _ => "",
    });

    result
}

pub fn draw_hands(game: &Game) {
    clear();

    println!();
    println!(" Dealer:");
    println!("{}", dealer_draw_hand(game));

    println!();
    println!(" Player ${:.2}:", game.money as f64 / 100.0);

    for i in 0..game.player_hands.len() {
        println!("{}", player_draw_hand(game, i));
        println!();
    }
}

pub fn read_one_char(re: &Regex) -> char {
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

pub fn run_game() {
    let mut game = build_game::<Termios>();
    load_game(&mut game);

    buffer_off(&mut *game.term);
    loop {
        if game.quitting {
            break;
        }
        deal_new_hand(&mut game);
    }
    buffer_on(&mut *game.term);
}

pub fn is_ace(card: &Card) -> bool {
    card.value == 0
}

pub fn is_ten(card: &Card) -> bool {
    card.value > 8
}
