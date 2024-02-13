use console_blackjack_rs::initialize_matchers;

#[test]
fn test_initialize_matchers() {
    let matchers = initialize_matchers();

    assert_eq!(
        matchers.len(),
        6,
        "Matchers HashMap should contain 6 items."
    );
    assert_eq!(
        matchers["DeckTypeOptions"].as_str(),
        "[1-6]",
        "DeckTypeOptions should have pattern [1-6]"
    );
    assert_eq!(
        matchers["FaceTypeOptions"].as_str(),
        "[1-2]",
        "FaceTypeOptions should have pattern [1-2]"
    );
    assert_eq!(
        matchers["AskInsurance"].as_str(),
        "[yn]",
        "AskInsurance should have pattern [yn]"
    );
    assert_eq!(
        matchers["GameOptions"].as_str(),
        "[ntfb]",
        "GameOptions should have pattern [ntfb]"
    );
    assert_eq!(
        matchers["HandOption"].as_str(),
        "[hspd]",
        "HandOption should have pattern [hspd]"
    );
    assert_eq!(
        matchers["BetOptions"].as_str(),
        "[dboq]",
        "BetOptions should have pattern [dboq]"
    );
}
