use console_blackjack_rs::{save_game, DealerHand, Game, Hand, SAVE_FILE};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use termios::Termios;

#[test]
fn test_save_game_writes_correct_data() {
    let game = Game {
        quitting: false,
        num_decks: 4,
        deck_type: 1,
        face_type: 1,
        money: 1000,
        current_bet: 50,
        current_player_hand: 0,
        shuffle_specs: [[0, 0]; 8],
        matchers: HashMap::new(),
        term: Termios::from_fd(0).unwrap(),
        dealer_hand: DealerHand {
            hide_down_card: true,
            hand: Hand { cards: Vec::new() },
        },
        player_hands: Vec::new(),
        shoe: Vec::new(),
    };

    let expected_content = "4\n1000\n50\n1\n1".to_string();

    save_game(&game);

    let mut file_content = String::new();
    let mut file = File::open(SAVE_FILE).expect("cannot open save file");
    file.read_to_string(&mut file_content)
        .expect("cannot read save file");

    assert_eq!(
        file_content, expected_content,
        "File content does not match expected content."
    );
}
