use console_blackjack_rs::CountMethod;

#[test]
fn test_count_method_enum_variants() {
    match CountMethod::Soft {
        CountMethod::Soft => {}
        _ => panic!("CountMethod::Soft variant is missing."),
    };

    match CountMethod::Hard {
        CountMethod::Hard => {}
        _ => panic!("CountMethod::Hard variant is missing."),
    };
}
