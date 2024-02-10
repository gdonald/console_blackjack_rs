use console_blackjack_rs::{CARDS_PER_DECK, MAX_BET, MAX_PLAYER_HANDS, MIN_BET, SAVE_FILE};

#[test]
fn min_bet_is_500() {
    assert_eq!(MIN_BET, 500);
}

#[test]
fn max_bet_is_10000000() {
    assert_eq!(MAX_BET, 10000000);
}

#[test]
fn max_bet_is_greater_than_min_bet() {
    assert!(MAX_BET > MIN_BET);
}

#[test]
fn max_player_hands_is_7() {
    assert_eq!(MAX_PLAYER_HANDS, 7);
}

#[test]
fn cards_per_deck_is_52() {
    assert_eq!(CARDS_PER_DECK, 52);
}

#[test]
fn save_file_name_is_correct() {
    assert_eq!(
        SAVE_FILE, "bj.txt",
        "The SAVE_FILE constant does not match the expected value."
    );
}
