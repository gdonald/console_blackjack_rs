
use console_blackjack::*;

#[test]
fn card_faces_contains_14_card_values() {
    assert_eq!(CARD_FACES.len(), 14);
    assert_eq!(CARD_FACES_2.len(), 14);
}

#[test]
fn each_card_value_contains_4_faces() {
    for face in CARD_FACES.iter() {
        assert_eq!(face.len(), 4);
    }
    for face in CARD_FACES_2.iter() {
        assert_eq!(face.len(), 4);
    }
}

#[test]
fn ace_of_spades_is_correct() {
    assert_eq!(CARD_FACES[0][0], "A♠");
    assert_eq!(CARD_FACES_2[0][0], "🂡");
}

#[test]
fn king_of_diamonds_is_correct() {
    assert_eq!(CARD_FACES[12][3], "K♦");
    assert_eq!(CARD_FACES_2[12][3], "🃞");
}

#[test]
fn unknown_card_is_correct() {
    assert_eq!(CARD_FACES[13], ["??", "", "", ""]);
    assert_eq!(CARD_FACES_2[13], ["🂠", "", "", ""]);
}
