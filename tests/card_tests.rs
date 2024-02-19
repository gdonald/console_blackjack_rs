use console_blackjack_rs::Card;

#[test]
fn test_cards_with_different_values_are_not_equal() {
    let card1 = Card { value: 0, suit: 1 };
    let card2 = Card { value: 1, suit: 1 };

    assert_ne!(card1, card2, "Cards should not be the same.");
}

#[test]
fn test_cards_with_different_suits_are_not_equal() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 0, suit: 1 };

    assert_ne!(card1, card2, "Cards should not be the same.");
}
