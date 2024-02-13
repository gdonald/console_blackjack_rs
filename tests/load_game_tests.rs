use console_blackjack_rs::{build_game, load_game, SAVE_FILE};

#[test]
fn test_load_game() {
    let content = "4\n1000\n50\n1\n1";
    std::fs::write(SAVE_FILE, content).expect("Failed to create save file for testing");

    let mut game = build_game();

    load_game(&mut game);

    assert_eq!(game.num_decks, 4, "num_decks should be updated to 4.");
    assert_eq!(game.money, 1000, "money should be updated to 1000.");
    assert_eq!(game.current_bet, 50, "current_bet should be updated to 50.");
    assert_eq!(game.deck_type, 1, "deck_type should be updated to 1.");
    assert_eq!(game.face_type, 1, "face_type should be updated to 1.");
}
