use preflop_trainer_core::{
    AnswerResult, Card, GameConfig, Hand, Position, Rank, SpotType, Suit, UserAction, check_answer,
    parse_range_str,
};
use std::collections::HashMap;

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

// --- Tests for BB vs BTN with J8s ---
// Strategy: raise 50%, call 50%

#[test]
fn test_bb_vs_btn_j8s_raise_correct_with_low_rng() {
    let mut call_map = HashMap::new();
    call_map.insert(Position::BTN, "J8s:0.5".to_string());
    let mut raise_map = HashMap::new();
    raise_map.insert(Position::BTN, "J8s:0.5".to_string());
    let config = create_full_test_game_config(None, Some(call_map), Some(raise_map), None);

    let hand = Hand {
        card1: c('J', 's'),
        card2: c('8', 's'),
    }; // J8s
    let spot_type = SpotType::BBDefense {
        opener_position: Position::BTN,
    };
    let user_action = UserAction::Raise;
    let rng_value = 49; // < 50, should be a raise

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be Correct to raise with low RNG"
    );
}

#[test]
fn test_bb_vs_btn_j8s_call_correct_with_high_rng() {
    let mut call_map = HashMap::new();
    call_map.insert(Position::BTN, "J8s:0.5".to_string());
    let mut raise_map = HashMap::new();
    raise_map.insert(Position::BTN, "J8s:0.5".to_string());
    let config = create_full_test_game_config(None, Some(call_map), Some(raise_map), None);

    let hand = Hand {
        card1: c('J', 's'),
        card2: c('8', 's'),
    }; // J8s
    let spot_type = SpotType::BBDefense {
        opener_position: Position::BTN,
    };
    let user_action = UserAction::Call;
    let rng_value = 50; // >= 50 and < 100, should be a call

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::Correct,
        "Should be Correct to call with high RNG"
    );
}

#[test]
fn test_bb_vs_btn_j8s_raise_freq_mistake_with_high_rng() {
    let mut call_map = HashMap::new();
    call_map.insert(Position::BTN, "J8s:0.5".to_string());
    let mut raise_map = HashMap::new();
    raise_map.insert(Position::BTN, "J8s:0.5".to_string());
    let config = create_full_test_game_config(None, Some(call_map), Some(raise_map), None);

    let hand = Hand {
        card1: c('J', 's'),
        card2: c('8', 's'),
    }; // J8s
    let spot_type = SpotType::BBDefense {
        opener_position: Position::BTN,
    };
    let user_action = UserAction::Raise;
    let rng_value = 50; // >= 50, should be a call

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::FrequencyMistake,
        "Should be FrequencyMistake to raise with high RNG"
    );
}

#[test]
fn test_bb_vs_btn_j8s_call_freq_mistake_with_low_rng() {
    let mut call_map = HashMap::new();
    call_map.insert(Position::BTN, "J8s:0.5".to_string());
    let mut raise_map = HashMap::new();
    raise_map.insert(Position::BTN, "J8s:0.5".to_string());
    let config = create_full_test_game_config(None, Some(call_map), Some(raise_map), None);

    let hand = Hand {
        card1: c('J', 's'),
        card2: c('8', 's'),
    }; // J8s
    let spot_type = SpotType::BBDefense {
        opener_position: Position::BTN,
    };
    let user_action = UserAction::Call;
    let rng_value = 49; // < 50, should be a raise

    let result = check_answer(&config, spot_type, hand, user_action, rng_value);
    assert_eq!(
        result,
        AnswerResult::FrequencyMistake,
        "Should be FrequencyMistake to call with low RNG"
    );
}

#[test]
fn test_bb_vs_btn_j8s_fold_is_wrong_with_any_rng() {
    let mut call_map = HashMap::new();
    call_map.insert(Position::BTN, "J8s:0.5".to_string());
    let mut raise_map = HashMap::new();
    raise_map.insert(Position::BTN, "J8s:0.5".to_string());
    let config = create_full_test_game_config(None, Some(call_map), Some(raise_map), None);

    let hand = Hand {
        card1: c('J', 's'),
        card2: c('8', 's'),
    }; // J8s
    let spot_type = SpotType::BBDefense {
        opener_position: Position::BTN,
    };
    let user_action = UserAction::Fold;

    // Test with low RNG
    let rng_value_low = 49;
    let result_low = check_answer(&config, spot_type, hand, user_action, rng_value_low);
    assert_eq!(
        result_low,
        AnswerResult::Wrong,
        "Should be Wrong to fold with low RNG"
    );

    // Test with high RNG
    let rng_value_high = 50;
    let result_high = check_answer(&config, spot_type, hand, user_action, rng_value_high);
    assert_eq!(
        result_high,
        AnswerResult::Wrong,
        "Should be Wrong to fold with high RNG"
    );
}
