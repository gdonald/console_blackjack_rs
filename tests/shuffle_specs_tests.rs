use console_blackjack_rs::SHUFFLE_SPECS;

#[test]
fn shuffle_specs_length_is_correct() {
    assert_eq!(
        SHUFFLE_SPECS.len(),
        8,
        "SHUFFLE_SPECS should contain exactly 8 elements."
    );
}

#[test]
fn shuffle_specs_values_are_within_expected_ranges() {
    for spec in SHUFFLE_SPECS.iter() {
        assert!(
            spec[0] >= 80 && spec[0] <= 95,
            "The first value of each spec should be between 80 and 95."
        );
        assert!(
            spec[1] >= 1 && spec[1] <= 8,
            "The second value of each spec should be between 1 and 8."
        );
    }
}

#[test]
fn shuffle_specs_follow_expected_pattern() {
    for i in 0..SHUFFLE_SPECS.len() - 1 {
        assert!(
            SHUFFLE_SPECS[i][0] > SHUFFLE_SPECS[i + 1][0],
            "The first value should decrease with each subsequent spec."
        );
        assert!(
            SHUFFLE_SPECS[i][1] > SHUFFLE_SPECS[i + 1][1],
            "The second value should increase with each subsequent spec."
        );
    }
}
