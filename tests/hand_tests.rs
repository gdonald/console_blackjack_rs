use console_blackjack_rs::{Card, Hand};

#[test]
fn test_new_hand_is_empty() {
    let hand = Hand { cards: Vec::new() };
    assert!(hand.cards.is_empty(), "New hand should be empty.");
}

#[test]
fn test_cloned_hand_has_same_cards() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let original_hand = Hand {
        cards: vec![card1, card2],
    };
    let cloned_hand = original_hand.clone();

    assert_eq!(original_hand, cloned_hand, "Hands should be the same.");
}

#[test]
fn test_hand_has_different_card_count() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let original_hand = Hand {
        cards: vec![card1, card2],
    };
    let other_hand = Hand {
        cards: vec![card1],
    };

    assert_ne!(original_hand, other_hand, "Hands should not be the same.");
}

#[test]
fn test_hand_has_different_cards() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let card3 = Card { value: 1, suit: 0 };
    let original_hand = Hand {
        cards: vec![card1, card2],
    };
    let other_hand = Hand {
        cards: vec![card2, card3],
    };

    assert_ne!(original_hand, other_hand, "Hands should not be the same.");
}
