use console_blackjack_rs::{Card, is_ace};

#[test]
fn test_is_ace_true() {
    let card = Card { value: 0, suit: 0 };
    assert!(is_ace(&card), "Card with value 0 should be an Ace.");
}

#[test]
fn test_is_ace_false() {
    let card = Card { value: 1, suit: 0 };
    assert!(!is_ace(&card), "Card with value not equal to 0 should not be an Ace.");
}
