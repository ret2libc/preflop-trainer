use preflop_trainer_core::{
    Game, GameConfig, HandNotation, HandType, Position, Rank, SpotType, parse_range_str,
};
use std::collections::HashMap;

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
fn test_game_new_deck_is_full() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let mut game = Game::new(config);
    assert!(game.generate_random_spot().is_some());
}

#[test]
fn test_generate_random_spot_depletes_deck() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let mut game = Game::new(config);

    // Deal 26 hands (deplete the deck)
    for i in 0..26 {
        assert!(
            game.generate_random_spot().is_some(),
            "Should be able to deal hand {} from the first deck",
            i + 1
        );
    }
    // After 26 hands, the deck should be exhausted within generate_random_spot's logic,
    // and the next call should trigger a reshuffle and deal a new hand.
    let next_spot = game.generate_random_spot();
    assert!(
        next_spot.is_some(),
        "Game did not reshuffle and deal a new hand after deck was exhausted."
    );

    // And we should be able to deal more hands from the reshuffled deck
    for i in 0..26 {
        assert!(
            game.generate_random_spot().is_some(),
            "Should be able to deal hand {} from the reshuffled deck",
            i + 1
        );
    }
}

#[test]
fn test_deck_reshuffles_and_continues() {
    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA".to_string());
    let config = create_full_test_game_config(Some(ur_map), None, None, None);
    let mut game = Game::new(config);

    // Exhaust the first deck
    for _ in 0..26 {
        game.generate_random_spot();
    }

    // The 27th hand should be valid after a reshuffle
    let spot = game.generate_random_spot();
    assert!(spot.is_some());

    // We can't easily check that the deck is *different*, but we can check that we can keep dealing
    let mut successful_deals = 0;
    for _ in 0..26 {
        if let Some((_, _, _)) = game.generate_random_spot() {
            successful_deals += 1;
        }
    }
    assert_eq!(
        successful_deals, 26,
        "Failed to deal a full new deck after reshuffling"
    );
}

#[test]

fn test_weighted_random_hand_selection() {
    // Define a very specific range for UTG: only AA

    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA".to_string());

    let config = create_full_test_game_config(Some(ur_map), None, None, None);

    let mut game = Game::new(config);

    let mut aa_count = 0;

    let iterations = 10000; // Increased iterations for better statistical significance

    for _ in 0..iterations {
        if let Some((
            SpotType::Open {
                position: Position::UTG,
            },
            hand,
            _,
        )) = game.generate_random_spot()
        {
            // Only count AA if it's an Open spot from UTG, as configured.
            let hn = HandNotation::from_hand(hand);

            let aa_notation = HandNotation {
                rank1: Rank::Ace,
                rank2: Rank::Ace,
                hand_type: HandType::Pair,
            };

            if hn == aa_notation {
                aa_count += 1;
            }
        }
    }

    // Recalculate expected percentage more accurately based on the weights.

    // AA has weight 50 (for 1.0 freq). Other 168 hands have weight 20.

    // Total weighted "units" for any hand being drawn: 50 (for AA) + (168 * 20) = 3410.

    // Probability of drawing AA in an Open spot from UTG (where it's the only 1.0 freq hand): 50 / 3410 = ~0.0146.

    // Since generate_random_spot has a 50% chance of being an Open spot,

    // and there are 5 possible opening positions, the probability of an Open spot from UTG is 0.5 * (1/5) = 0.1.

    // So, the expected AA count in 10000 iterations from UTG Open spots is:

    // 10000 (iterations) * (50 / 3410) (prob of AA in weighted list) * 0.1 (prob of UTG Open spot) = ~14.6

    // Let's set a conservative lower bound for actual_aa_percentage.

    let min_expected_aa_percentage = (50.0 / 3410.0) * (1.0 / 5.0) * 0.5 * 0.5; // (Prob AA in list) * (Prob Open from UTG) * safety margin (50%)

    let actual_aa_percentage = aa_count as f32 / iterations as f32;

    assert!(
        actual_aa_percentage >= min_expected_aa_percentage,
        "Expected AA percentage to be at least {:.2}%, but got {:.2}%",
        min_expected_aa_percentage * 100.0,
        actual_aa_percentage * 100.0
    );
}

#[test]
fn test_weighted_random_hand_selection_with_adjusted_weights() {
    // This test verifies the new weighting system for hand selection,
    // where non-in-range hands have an increased weight.

    let mut ur_map = HashMap::new();
    ur_map.insert(Position::UTG, "AA".to_string()); // Only AA is in range

    // We only allow one spot type to make the calculation simpler.
    let config = create_full_test_game_config(
        Some(ur_map),
        None,
        None,
        Some(vec![SpotType::Open {
            position: Position::UTG,
        }]),
    );

    let mut game = Game::new(config);

    let mut aa_count = 0;
    let mut other_count = 0;
    let iterations = 20000; // More iterations for statistical reliability

    for _ in 0..iterations {
        if let Some((_, hand, _)) = game.generate_random_spot() {
            let hn = HandNotation::from_hand(hand);
            let aa_notation = HandNotation {
                rank1: Rank::Ace,
                rank2: Rank::Ace,
                hand_type: HandType::Pair,
            };

            if hn == aa_notation {
                aa_count += 1;
            } else {
                other_count += 1;
            }
        }
    }

    // With the new weights:
    // Weight of AA (in-range) = 50
    // Weight of any other hand (out-of-range) = 20
    // Total hands = 169. 1 is in-range, 168 are out-of-range.
    // Total weight = 50 * 1 + 20 * 168 = 50 + 3360 = 3410.
    // Probability of AA = 50 / 3410 = ~0.01466
    // Probability of not AA = 3360 / 3410 = ~0.98534

    // Expected ratio of other hands to AA hands:
    let expected_ratio = (168.0 * 20.0) / 50.0; // Expected to be 67.2

    let actual_ratio = if aa_count > 0 {
        other_count as f32 / aa_count as f32
    } else {
        0.0
    };

    // We allow for a 35% tolerance margin for the randomness.
    let lower_bound = expected_ratio * 0.65;
    let upper_bound = expected_ratio * 1.35;

    assert!(
        actual_ratio >= lower_bound && actual_ratio <= upper_bound,
        "The ratio of other hands to AA hands is out of the expected range. Expected ratio: {:.2}, Actual: {:.2}, Bounds: [{:.2}, {:.2}]",
        expected_ratio,
        actual_ratio,
        lower_bound,
        upper_bound
    );
}
