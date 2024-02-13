use console_blackjack_rs::{build_game, save_game, MockTermiosWrapper, SAVE_FILE};
use std::fs::File;
use std::io::Read;

#[test]
fn test_save_game_writes_correct_data() {
    let game = build_game::<MockTermiosWrapper>();
    let expected_content = "8\n10000\n500\n1\n1".to_string();

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
