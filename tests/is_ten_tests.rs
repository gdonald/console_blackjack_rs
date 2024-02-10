use console_blackjack_rs::{is_ten, Card};

#[test]
fn test_is_ten_true() {
    let card = Card { value: 9, suit: 0 };
    assert!(
        is_ten(&card),
        "Card with value greater than 8 should be a ten."
    );
}

#[test]
fn test_is_ten_false() {
    let card = Card { value: 8, suit: 0 };
    assert!(
        !is_ten(&card),
        "Card with value 8 or less should not be a ten."
    );
}
