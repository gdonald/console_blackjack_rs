use console_blackjack_rs::{read_save_file, SAVE_FILE};

#[test]
fn test_read_save_file() {
    let save_content = "4\n1000\n50\n1\n1";
    std::fs::write(SAVE_FILE, save_content).expect("Failed to create save file for testing");

    let result = read_save_file();

    match result {
        Ok(content) => {
            assert_eq!(
                content, save_content,
                "Read content does not match expected content."
            );
        }
        Err(e) => {
            panic!("Error reading save file: {}", e);
        }
    }
}
