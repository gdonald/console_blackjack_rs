use console_blackjack_rs::HandStatus;

#[test]
fn test_hand_status_variants() {
    assert_eq!(
        HandStatus::Unknown as u32,
        0,
        "Unknown variant should have value 0."
    );
    assert_eq!(
        HandStatus::Won as u32,
        1,
        "Won variant should have value 1."
    );
    assert_eq!(
        HandStatus::Lost as u32,
        2,
        "Lost variant should have value 2."
    );
    assert_eq!(
        HandStatus::Push as u32,
        3,
        "Push variant should have value 3."
    );
}

#[test]
fn test_hand_status_clone() {
    let original_status = HandStatus::Unknown;
    let cloned_status = original_status.clone();
    assert_eq!(
        original_status, cloned_status,
        "Cloned HandStatus should be equal to the original."
    );
}
