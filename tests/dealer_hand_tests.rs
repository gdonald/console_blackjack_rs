use console_blackjack_rs::{DealerHand, Hand};

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
