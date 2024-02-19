use console_blackjack_rs::{Card, Hand, HandStatus, PlayerHand};

#[test]
fn test_new_player_hand_initialization() {
    let hand = Hand { cards: Vec::new() };
    let player_hand = PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: hand.clone(),
    };

    assert_eq!(
        player_hand.status,
        HandStatus::Unknown,
        "Initial status should be Unknown."
    );
    assert!(!player_hand.stood, "Initial stood should be false.");
    assert!(!player_hand.played, "Initial played should be false.");
    assert!(!player_hand.paid, "Initial paid should be false.");
    assert_eq!(player_hand.bet, 0, "Initial bet should be 0.");
    assert_eq!(
        player_hand.hand, hand,
        "Initial hand should match the provided hand."
    );
}

#[test]
fn test_player_hands_are_equal_when_cloned() {
    let card = Card { value: 0, suit: 0 };
    let hand = Hand {
        cards: vec![card.clone()],
    };
    let player_hand = PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: hand.clone(),
    };
    let other_player_hand = player_hand.clone();

    assert_eq!(
        player_hand, other_player_hand,
        "Player hands should be equal"
    );
}

#[test]
fn test_hand_has_different_card_count() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let hand1 = Hand {
        cards: vec![card1.clone(), card2.clone()],
    };
    let hand2 = Hand {
        cards: vec![card1.clone()],
    };
    let player_hand = PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: hand1.clone(),
    };
    let other_player_hand = PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: hand2.clone(),
    };

    assert_ne!(
        player_hand, other_player_hand,
        "Player hands should not be the same."
    );
}

#[test]
fn test_hand_has_different_cards() {
    let card1 = Card { value: 0, suit: 0 };
    let card2 = Card { value: 1, suit: 1 };
    let card3 = Card { value: 1, suit: 0 };
    let hand1 = Hand {
        cards: vec![card1.clone(), card2.clone()],
    };
    let hand2 = Hand {
        cards: vec![card2.clone(), card3.clone()],
    };
    let player_hand = PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: hand1.clone(),
    };
    let other_player_hand = PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: hand2.clone(),
    };

    assert_ne!(
        player_hand, other_player_hand,
        "Player hands should not be the same."
    );
}
