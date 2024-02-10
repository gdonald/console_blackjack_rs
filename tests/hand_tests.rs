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
        cards: vec![card1.clone(), card2.clone()],
    };

    let cloned_hand = original_hand.clone();

    assert_eq!(
        original_hand.cards.len(),
        cloned_hand.cards.len(),
        "Number of cards should be the same."
    );

    for (original_card, cloned_card) in original_hand.cards.iter().zip(cloned_hand.cards.iter()) {
        assert_eq!(
            original_card, cloned_card,
            "Cards in original and cloned hands should be the same."
        );
    }
}
