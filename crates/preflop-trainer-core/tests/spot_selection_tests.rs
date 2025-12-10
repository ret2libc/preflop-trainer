use preflop_trainer_core::{
    Game, GameConfig, HandNotation, Position, SpotType, get_all_possible_hand_notations,
};
use std::collections::HashMap;

// Helper to create a GameConfig for tests
fn create_test_config(allowed_spot_types: Vec<SpotType>) -> GameConfig {
    let mut unopened_raise_ranges = HashMap::new();
    let mut bb_defense_call_ranges = HashMap::new();
    let mut bb_defense_raise_ranges = HashMap::new();

    // Populate with some dummy data to ensure ranges are not empty
    let all_notations = get_all_possible_hand_notations();
    let dummy_range: HashMap<HandNotation, f32> =
        all_notations.iter().take(5).map(|&hn| (hn, 1.0)).collect();

    unopened_raise_ranges.insert(Position::UTG, dummy_range.clone());
    bb_defense_call_ranges.insert(Position::UTG, dummy_range.clone());
    bb_defense_raise_ranges.insert(Position::UTG, dummy_range.clone());

    GameConfig {
        unopened_raise_ranges,
        bb_defense_call_ranges,
        bb_defense_raise_ranges,
        allowed_spot_types,
    }
}

#[test]
fn test_generate_random_spot_only_open() {
    let config = create_test_config(vec![SpotType::Open {
        position: Position::UTG,
    }]);
    let mut game = Game::new(config);

    for _ in 0..100 {
        // Generate many spots to ensure consistency
        let (spot_type, _, _) = game.generate_random_spot().expect("Should generate a spot");
        assert!(
            matches!(spot_type, SpotType::Open { .. }),
            "Expected only Open spot, got {:?}",
            spot_type
        );
    }
}

#[test]
fn test_generate_random_spot_only_bb_defense() {
    let config = create_test_config(vec![SpotType::BBDefense {
        opener_position: Position::UTG,
    }]);
    let mut game = Game::new(config);

    for _ in 0..100 {
        // Generate many spots to ensure consistency
        let (spot_type, _, _) = game.generate_random_spot().expect("Should generate a spot");
        assert!(
            matches!(spot_type, SpotType::BBDefense { .. }),
            "Expected only BBDefense spot, got {:?}",
            spot_type
        );
    }
}

#[test]
fn test_generate_random_spot_all_allowed() {
    let config = create_test_config(vec![
        SpotType::Open {
            position: Position::UTG,
        },
        SpotType::BBDefense {
            opener_position: Position::UTG,
        },
    ]);
    let mut game = Game::new(config);

    let mut open_count = 0;
    let mut bb_defense_count = 0;

    for _ in 0..200 {
        // Generate enough spots to get a mix
        let (spot_type, _, _) = game.generate_random_spot().expect("Should generate a spot");
        match spot_type {
            SpotType::Open { .. } => open_count += 1,
            SpotType::BBDefense { .. } => bb_defense_count += 1,
        }
    }

    assert!(open_count > 0, "Expected some Open spots");
    assert!(bb_defense_count > 0, "Expected some BBDefense spots");
    assert_eq!(open_count + bb_defense_count, 200);
}

#[test]
#[should_panic(expected = "No valid spot types configured or able to be generated")]
fn test_generate_random_spot_empty_allowed_list() {
    let config = create_test_config(vec![]); // Empty allowed list
    let mut game = Game::new(config);

    // This should panic because no spots can be generated
    game.generate_random_spot();
}
