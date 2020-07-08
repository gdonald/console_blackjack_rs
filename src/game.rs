// extern crate termios;

use regex::Regex;

use std::io;
use std::io::Read;
use std::io::Write;
// use std::process::Command;

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};

use crate::dealer_hand::DealerHand;
use crate::player_hand::PlayerHand;
use crate::shoe::Shoe;
use crate::player_hand::Status;

pub struct Game {
    pub shoe: Shoe,
    dealer_hand: DealerHand,
    player_hands: Vec<PlayerHand>,
    pub num_decks: u16,
    pub money: u32,
    pub current_bet: u32,
    pub current_player_hand: u8
}

impl Game {
    pub(crate) fn new() -> Game {
        let mut game: Game = Game {
            shoe: Shoe { cards: vec![] },
            money: 10000,
            current_bet: 500,
            num_decks: 8,
            current_player_hand: 0,
            player_hands: vec![],
            dealer_hand: DealerHand::new()
        };

        let shoe: Shoe = Shoe::new_regular(&mut game);
        game.shoe = shoe;

        game.deal_new_hand();

        game
    }

    pub(crate) fn read_one_char(match_str: &str) -> char {
        let stdin = 0;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone();
        new_termios.c_lflag &= !(ICANON | ECHO);
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
        let stdout = io::stdout();
        let mut reader = io::stdin();
        let mut buffer = [0; 1];
        stdout.lock().flush().unwrap();
        reader.read_exact(&mut buffer).unwrap();
        tcsetattr(stdin, TCSANOW, &termios).unwrap();

        let re = Regex::new(&match_str).unwrap();
        if !re.is_match(&format!("{}", buffer[0] as char)) {
           return Game::read_one_char(match_str);
        }

        buffer[0] as char
    }

    pub(crate) fn split_hand(_player_hand: &mut PlayerHand) {

    }

    fn deal_new_hand(&mut self) {
        self.player_hands = vec![PlayerHand::new(self.current_bet)];
        self.current_player_hand = 0;
        self.dealer_hand = DealerHand::new();

        self.dealer_hand.hand.deal_card(&mut self.shoe);
        self.player_hands[0].hand.deal_card(&mut self.shoe);
        self.dealer_hand.hand.deal_card(&mut self.shoe);
        self.player_hands[0].hand.deal_card(&mut self.shoe);

        if self.dealer_hand.upcard_is_ace() && !self.player_hands[0].hand.is_blackjack() {
            self.draw_hands();
            self.ask_insurance();
            return;
        }

        if self.player_hands[0].is_done() {

            self.player_hands[0].played = true;

            if !self.player_hands[0].payed {
                if self.player_hands[0].is_busted() {
                    self.player_hands[0].payed = true;
                    self.player_hands[0].status = Status::Lost;
                    self.money -= self.player_hands[0].bet;
                }
            }

            self.dealer_hand.hide_down_card = false;
            self.pay_hands();
            self.draw_hands();
            self.bet_options();
            return;
        }

        self.draw_hands();
        self.player_hands[0].get_action();
        self.save_game();
    }

    fn ask_insurance(&self) {

    }

    fn bet_options(&self) {

    }

    fn pay_hands(&self) {

    }

    fn clear(&self) {
        // Command::new("sh")
        //     .arg("-c")
        //     .arg("TERM=linux clear")
        //     .spawn()
        //     .expect("failed to execute process");
    }

    fn draw_hands(&self) {
        self.clear();

        println!();
        println!(" Dealer:");
        println!("{}", self.dealer_hand.draw());

        println!();
        println!(" Player ${}:", self.money as f64 / 100.0);

        for i in 0..self.player_hands.len() {
            let current = i as u8 == self.current_player_hand;
            println!("{}", self.player_hands[i].draw(current));
            println!();
            println!();
        }
    }

    fn save_game(&self) {

    }
}
