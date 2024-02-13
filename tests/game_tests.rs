use console_blackjack_rs::{
    build_game, initialize_matchers, Card, DealerHand, Game, Hand, HandStatus, PlayerHand,
    SHUFFLE_SPECS,
};
use termios::Termios;

#[test]
fn test_new_game_initialization() {
    let dealer_hand = DealerHand {
        hide_down_card: true,
        hand: Hand { cards: Vec::new() },
    };

    let player_hands = vec![PlayerHand {
        status: HandStatus::Unknown,
        stood: false,
        played: false,
        paid: false,
        bet: 0,
        hand: Hand { cards: Vec::new() },
    }];

    let game = Game {
        quitting: false,
        num_decks: 1,
        deck_type: 0,
        face_type: 0,
        money: 1000,
        current_bet: 0,
        current_player_hand: 0,
        shuffle_specs: SHUFFLE_SPECS,
        matchers: initialize_matchers(),
        term: Termios::from_fd(0).unwrap(),
        dealer_hand,
        player_hands,
        shoe: Vec::new(),
    };

    assert!(!game.quitting, "quitting should be false initially.");
    assert_eq!(game.num_decks, 1, "num_decks should be 1.");
    assert_eq!(game.deck_type, 0, "deck_type should be 0.");
    assert_eq!(game.face_type, 0, "face_type should be 0.");
    assert_eq!(game.money, 1000, "money should be 1000.");
    assert_eq!(game.current_bet, 0, "current_bet should be 0.");
    assert_eq!(
        game.current_player_hand, 0,
        "current_player_hand should be 0."
    );
    assert_eq!(
        game.shuffle_specs, SHUFFLE_SPECS,
        "shuffle_specs should be [[0, 0]; 8]."
    );
    assert_eq!(game.matchers.len(), 6, "Matchers should be initialized.");
}

#[test]
fn test_build_game() {
    let game = build_game();

    assert_eq!(game.shoe, Vec::<Card>::new(), "Shoe should be empty.");

    let expected_dealer_hand = DealerHand {
        hand: Hand {
            cards: Vec::<Card>::new(),
        },
        hide_down_card: true,
    };
    assert_eq!(
        game.dealer_hand, expected_dealer_hand,
        "Dealer hand should match the expected value."
    );

    assert_eq!(
        game.player_hands,
        Vec::<PlayerHand>::new(),
        "Player hands should be empty."
    );
    assert_eq!(game.num_decks, 8, "num_decks should be 8.");
    assert_eq!(game.deck_type, 1, "deck_type should be 1.");
    assert_eq!(game.face_type, 1, "face_type should be 1.");
    assert_eq!(game.money, 10000, "money should be 10000.");
    assert_eq!(game.current_bet, 500, "current_bet should be 500.");
    assert_eq!(
        game.current_player_hand, 0,
        "current_player_hand should be 0."
    );
}
