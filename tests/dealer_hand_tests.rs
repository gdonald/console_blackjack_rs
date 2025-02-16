use console_blackjack_rs::{Card, DealerHand, Hand};

#[test]
fn test_new_dealer_hand_initialization() {
    let hand = Hand { cards: Vec::new() };
    let dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand.clone(),
    };

    assert!(
        dealer_hand.hide_down_card,
        "hide_down_card should be true initially."
    );
    assert_eq!(
        dealer_hand.hand, hand,
        "Initial hand should match the provided hand."
    );
}

#[test]
fn test_dealer_hands_are_equal_when_cloned() {
    let card = Card { value: 0, suit: 0 };
    let hand = Hand {
        cards: vec![card],
    };
    let dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand.clone(),
    };
    let other_dealer_hand = dealer_hand.clone();

    assert_eq!(
        dealer_hand, other_dealer_hand,
        "Dealer hands should be equal"
    );
}

#[test]
fn test_hand_has_different_card_count() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let hand1 = Hand {
        cards: vec![card1, card2],
    };
    let hand2 = Hand {
        cards: vec![card1],
    };
    let dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand1.clone(),
    };
    let other_dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand2.clone(),
    };

    assert_ne!(
        dealer_hand, other_dealer_hand,
        "Dealer hands should not be the same."
    );
}

#[test]
fn test_hand_has_different_cards() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let card3 = Card { value: 1, suit: 0 };
    let hand1 = Hand {
        cards: vec![card1, card2],
    };
    let hand2 = Hand {
        cards: vec![card2, card3],
    };
    let dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand1.clone(),
    };
    let other_dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand2.clone(),
    };

    assert_ne!(
        dealer_hand, other_dealer_hand,
        "Dealer hands should not be the same."
    );
}

#[test]
fn test_hand_has_different_hide_down_card_value() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let hand1 = Hand {
        cards: vec![card1, card2],
    };
    let dealer_hand = DealerHand {
        hide_down_card: true,
        hand: hand1.clone(),
    };
    let other_dealer_hand = DealerHand {
        hide_down_card: false,
        hand: hand1.clone(),
    };

    assert_ne!(
        dealer_hand, other_dealer_hand,
        "Dealer hands should not be the same."
    );
}
