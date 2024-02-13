use console_blackjack_rs::{Hand, HandStatus, PlayerHand};

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
