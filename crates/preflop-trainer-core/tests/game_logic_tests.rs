use preflop_trainer_core::{
    AnswerResult, Card, Deck, GameConfig, Hand, Position, Rank, SpotType, Suit, UserAction,
    check_answer, parse_range_str,
};
use std::collections::{HashMap, HashSet};

// Helper to create a Card for tests
fn c(rank_char: char, suit_char: char) -> Card {
    Card {
        rank: Rank::from_char(rank_char).unwrap(),
        suit: match suit_char {
            's' => Suit::Spades,
            'h' => Suit::Hearts,
            'd' => Suit::Diamonds,
            'c' => Suit::Clubs,
            _ => panic!("Invalid suit char"),
        },
    }
}

// Helper to create a GameConfig for testing
fn create_full_test_game_config(
    unopened_raise_ranges: Option<HashMap<Position, String>>,
    bb_defense_call_ranges: Option<HashMap<Position, String>>,
    bb_defense_raise_ranges: Option<HashMap<Position, String>>,
    allowed_spot_types: Option<Vec<SpotType>>,
) -> GameConfig {
    let mut game_config_unopened_raise = HashMap::new();
    if let Some(ur_map) = unopened_raise_ranges {
        for (pos, range_str) in ur_map {
            game_config_unopened_raise.insert(pos, parse_range_str(&range_str).unwrap());
        }
    }

    let mut game_config_bb_call = HashMap::new();
    if let Some(bb_call_map) = bb_defense_call_ranges {
        for (pos, range_str) in bb_call_map {
            game_config_bb_call.insert(pos, parse_range_str(&range_str).unwrap());
        }
    }

    let mut game_config_bb_raise = HashMap::new();
    if let Some(bb_raise_map) = bb_defense_raise_ranges {
        for (pos, range_str) in bb_raise_map {
            game_config_bb_raise.insert(pos, parse_range_str(&range_str).unwrap());
        }
    }

    let default_allowed_spot_types = vec![
        SpotType::Open {
            position: Position::UTG,
        },
        SpotType::Open {
            position: Position::MP,
        },
        SpotType::Open {
            position: Position::CO,
        },
        SpotType::Open {
            position: Position::BTN,
        },
        SpotType::Open {
            position: Position::SB,
        },
        SpotType::BBDefense {
            opener_position: Position::UTG,
        },
        SpotType::BBDefense {
            opener_position: Position::MP,
        },
        SpotType::BBDefense {
            opener_position: Position::CO,
        },
        SpotType::BBDefense {
            opener_position: Position::BTN,
        },
        SpotType::BBDefense {
            opener_position: Position::SB,
        },
    ];

    GameConfig {
        unopened_raise_ranges: game_config_unopened_raise,
        bb_defense_call_ranges: game_config_bb_call,
        bb_defense_raise_ranges: game_config_bb_raise,
        allowed_spot_types: allowed_spot_types.unwrap_or(default_allowed_spot_types),
    }
}

#[test]
fn test_new_deck_has_52_unique_cards() {
    let deck = Deck::new();
    assert_eq!(deck.cards.len(), 52);

    let mut unique_cards = HashSet::new();
    for card in deck.cards {
        unique_cards.insert(card);
    }
    assert_eq!(unique_cards.len(), 52);
}

#[test]
fn test_shuffled_deck_retains_52_unique_cards() {
    let mut deck = Deck::new();
    deck.shuffle();
    assert_eq!(deck.cards.len(), 52);

    let mut unique_cards = HashSet::new();
    for card in deck.cards {
        unique_cards.insert(card);
    }
    assert_eq!(unique_cards.len(), 52);
}

#[test]
fn test_deal_hand_removes_cards() {
    let mut deck = Deck::new();
    let initial_len = deck.cards.len();
    let _hand = deck.deal_hand().expect("Should be able to deal a hand");
    assert_eq!(deck.cards.len(), initial_len - 2);
}

#[test]
fn test_deal_hand_returns_two_distinct_cards() {
    let mut deck = Deck::new();
    let hand = deck.deal_hand().expect("Should be able to deal a hand");
    assert_ne!(hand.card1, hand.card2); // Cards should be distinct
}

#[test]
fn test_deal_hand_empty_deck() {
    let mut deck = Deck::new();
    // Deal all 26 hands (52 cards)
    for _ in 0..26 {
        deck.deal_hand()
            .expect("Should be able to deal a hand until deck is empty");
    }
    assert!(deck.deal_hand().is_none()); // Should return None when deck is empty
}

#[test]
fn test_check_answer_correct_raise_in_range_1_0_freq() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA,AKs".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('A', 's'),
        card2: c('A', 'c'),
    }; // AA
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Raise;
    let rng_value = 0; // Dummy value, not relevant for 1.0 frequency

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for raising AA in range"
    );
}

#[test]
fn test_check_answer_correct_raise_in_range_0_5_freq() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "K6s:0.5".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('K', 's'),
        card2: c('6', 's'),
    }; // K6s
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Raise;
    let rng_value = 20; // Will result in a raise

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for raising K6s (0.5 freq) in range"
    );
}

#[test]
fn test_check_answer_correct_fold_not_in_range() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA,AKs".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('2', 's'),
        card2: c('7', 'd'),
    }; // 72o
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Fold;
    let rng_value = 0; // Dummy value

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for folding 72o not in range"
    );
}

#[test]
fn test_check_answer_incorrect_fold_in_range() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA,AKs".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('A', 's'),
        card2: c('K', 's'),
    }; // AKs
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Fold;
    let rng_value = 0; // Dummy value

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Wrong,
        "Should be incorrect for folding AKs in range"
    );
}

#[test]
fn test_check_answer_incorrect_raise_not_in_range() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA,AKs".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('2', 's'),
        card2: c('2', 'c'),
    }; // 22
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Raise;
    let rng_value = 0; // Dummy value

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Wrong,
        "Should be incorrect for raising 22 not in range"
    );
}

#[test]
fn test_check_answer_mixed_strategy_raise() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "K6s:0.5".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('K', 's'),
        card2: c('6', 's'),
    }; // K6s
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Raise;
    let rng_value = 20; // < 50, so should be a raise

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for raising K6s (0.5 freq) with RNG < 50"
    );
}

#[test]
fn test_check_answer_mixed_strategy_fold() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "K6s:0.5".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('K', 's'),
        card2: c('6', 's'),
    }; // K6s
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Fold;
    let rng_value = 70; // >= 50, so should be a fold

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for folding K6s (0.5 freq) with RNG >= 50"
    );
}

#[test]
fn test_check_answer_mixed_strategy_zero_freq() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "K6s:0.0".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let hand = Hand {
        card1: c('K', 's'),
        card2: c('6', 's'),
    }; // K6s
    let spot_type = SpotType::Open {
        position: Position::UTG,
    };
    let user_action = UserAction::Fold;
    let rng_value = 10; // Irrelevant, should always be fold

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for folding K6s (0.0 freq)"
    );
}

// --- New BBDefense tests for QJs (JdQd) vs SB Open ---

#[test]
fn test_check_answer_bb_sb_open_qjs_raise_mixed_correct() {
    let mut bb_raise_map = HashMap::new();
    bb_raise_map.insert(Position::SB, "QJs:0.5".to_string());
    let config = create_full_test_game_config(None, None, Some(bb_raise_map), None);

    let hand = Hand {
        card1: c('J', 'd'),
        card2: c('Q', 'd'),
    }; // Jd Qd is QJs
    let spot_type = SpotType::BBDefense {
        opener_position: Position::SB,
    };
    let user_action = UserAction::Raise;
    let rng_value = 20; // < 50, so it should hit the raise frequency

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for raising QJs (0.5 freq) with RNG < 50 in BB vs SB"
    );
}

#[test]
fn test_check_answer_bb_sb_open_qjs_raise_mixed_freq_mistake() {
    let mut bb_raise_map = HashMap::new();
    bb_raise_map.insert(Position::SB, "QJs:0.5".to_string());
    let config = create_full_test_game_config(None, None, Some(bb_raise_map), None);

    let hand = Hand {
        card1: c('J', 'd'),
        card2: c('Q', 'd'),
    }; // Jd Qd is QJs
    let spot_type = SpotType::BBDefense {
        opener_position: Position::SB,
    };
    let user_action = UserAction::Raise;
    let rng_value = 70; // >= 50, so it should miss the raise frequency and expect a fold

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::FrequencyMistake,
        "Should be FrequencyMistake for raising QJs (0.5 freq) with RNG >= 50 in BB vs SB"
    );
}

#[test]
fn test_check_answer_bb_sb_open_qjs_fold_mixed_correct() {
    let mut bb_raise_map = HashMap::new();
    bb_raise_map.insert(Position::SB, "QJs:0.5".to_string());
    let config = create_full_test_game_config(None, None, Some(bb_raise_map), None);

    let hand = Hand {
        card1: c('J', 'd'),
        card2: c('Q', 'd'),
    }; // Jd Qd is QJs
    let spot_type = SpotType::BBDefense {
        opener_position: Position::SB,
    };
    let user_action = UserAction::Fold;
    let rng_value = 70; // >= 50, so it should miss the raise frequency and expect a fold

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be correct for folding QJs (0.5 freq) with RNG >= 50 in BB vs SB"
    );
}

#[test]
fn test_check_answer_bb_sb_open_qjs_call_when_raise_freq_non_zero() {
    let mut bb_raise_map = HashMap::new();
    bb_raise_map.insert(Position::SB, "QJs:0.5".to_string());
    let config = create_full_test_game_config(None, None, Some(bb_raise_map), None);

    let hand = Hand {
        card1: c('J', 'd'),
        card2: c('Q', 'd'),
    }; // Jd Qd is QJs
    let spot_type = SpotType::BBDefense {
        opener_position: Position::SB,
    };
    let user_action = UserAction::Call;
    let rng_value = 20; // < 50, so should hit raise frequency

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Wrong, // Not a frequency mistake, Call is never a valid action here.
        "Should be Wrong for calling QJs when it should be a mixed-strategy Raise/Fold"
    );
}

#[test]
fn test_check_answer_bb_sb_open_qjs_call_when_raise_freq_zero() {
    let mut bb_call_map = HashMap::new();
    bb_call_map.insert(Position::SB, "QJs:0.5".to_string());
    let config = create_full_test_game_config(None, Some(bb_call_map), None, None);

    let hand = Hand {
        card1: c('J', 'd'),
        card2: c('Q', 'd'),
    }; // Jd Qd is QJs
    let spot_type = SpotType::BBDefense {
        opener_position: Position::SB,
    };
    let user_action = UserAction::Call;
    let rng_value = 20; // < 50, so should hit call frequency

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be Correct for calling QJs (0.5 call freq) with RNG < 50 in BB vs SB"
    );
}

#[test]
fn test_check_answer_bb_sb_open_qjs_fold_when_call_freq_zero_mixed_freq_mistake() {
    let mut bb_call_map = HashMap::new();
    bb_call_map.insert(Position::SB, "QJs:0.5".to_string());
    let config = create_full_test_game_config(None, Some(bb_call_map), None, None);

    let hand = Hand {
        card1: c('J', 'd'),
        card2: c('Q', 'd'),
    }; // Jd Qd is QJs
    let spot_type = SpotType::BBDefense {
        opener_position: Position::SB,
    };
    let user_action = UserAction::Fold;
    let rng_value = 20; // < 50, so should hit call frequency, expect call

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::FrequencyMistake,
        "Should be FrequencyMistake for folding QJs (0.5 call freq) with RNG < 50 in BB vs SB"
    );
}
