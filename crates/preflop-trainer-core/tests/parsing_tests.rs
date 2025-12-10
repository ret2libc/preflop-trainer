use preflop_trainer_core::{HandNotation, HandType, Rank, parse_range_str};
use std::str::FromStr;

// Helper to create a HandNotation for tests
fn hn(s: &str) -> HandNotation {
    HandNotation::from_str(s).unwrap()
}

#[test]
fn test_parse_range_str_simple() {
    let range_str = "AA,KQs,T9o";
    let expected_len = 3;
    let range_map = parse_range_str(range_str).unwrap();

    assert_eq!(range_map.len(), expected_len);
    assert_eq!(range_map.get(&hn("AA")), Some(&1.0));
    assert_eq!(range_map.get(&hn("KQs")), Some(&1.0));
    assert_eq!(range_map.get(&hn("T9o")), Some(&1.0));
}

#[test]
fn test_parse_range_str_with_frequencies() {
    let range_str = "AA,KQs:0.5,T9o:0.25";
    let range_map = parse_range_str(range_str).unwrap();

    assert_eq!(range_map.len(), 3);
    assert_eq!(range_map.get(&hn("AA")), Some(&1.0));
    assert_eq!(range_map.get(&hn("KQs")), Some(&0.5));
    assert_eq!(range_map.get(&hn("T9o")), Some(&0.25));
}

#[test]
fn test_parse_range_str_with_whitespace() {
    let range_str = "  AA  , KQs:0.5 ,   T9o ";
    let range_map = parse_range_str(range_str).unwrap();

    assert_eq!(range_map.len(), 3);
    assert!(range_map.contains_key(&hn("AA")));
    assert!(range_map.contains_key(&hn("KQs")));
    assert!(range_map.contains_key(&hn("T9o")));
}

#[test]
fn test_parse_range_str_empty() {
    let range_str = "";
    let range_map = parse_range_str(range_str).unwrap();
    assert!(range_map.is_empty());
}

#[test]
fn test_parse_range_str_invalid_hand() {
    let range_str = "AA,InvalidHand,KK";
    let result = parse_range_str(range_str);
    assert!(result.is_err());
}

#[test]
fn test_parse_range_str_invalid_frequency() {
    let range_str = "AA,KQs:abc";
    let result = parse_range_str(range_str);
    assert!(result.is_err());
}

#[test]
fn test_hand_notation_from_str() {
    // Pairs
    assert_eq!(
        HandNotation::from_str("AA").unwrap(),
        HandNotation {
            rank1: Rank::Ace,
            rank2: Rank::Ace,
            hand_type: HandType::Pair
        }
    );
    // Suited
    assert_eq!(
        HandNotation::from_str("AKs").unwrap(),
        HandNotation {
            rank1: Rank::Ace,
            rank2: Rank::King,
            hand_type: HandType::Suited
        }
    );
    // Offsuit
    assert_eq!(
        HandNotation::from_str("T2o").unwrap(),
        HandNotation {
            rank1: Rank::Ten,
            rank2: Rank::Two,
            hand_type: HandType::Offsuit
        }
    );
    // Order doesn't matter for non-pairs
    assert_eq!(
        HandNotation::from_str("KAs").unwrap(),
        HandNotation {
            rank1: Rank::Ace,
            rank2: Rank::King,
            hand_type: HandType::Suited
        }
    );
}

#[test]
fn test_hand_notation_from_str_invalid() {
    assert!(HandNotation::from_str("AXs").is_err());
    assert!(HandNotation::from_str("AAs").is_err());
    assert!(HandNotation::from_str("AKx").is_err());
    assert!(HandNotation::from_str("AKA").is_err());
    assert!(HandNotation::from_str("AK").is_err());
}

#[test]
fn test_parse_range_str_plus_notation_pairs() {
    let range_str = "22+";
    let range_map = parse_range_str(range_str).unwrap();

    // Should be in range
    assert!(range_map.contains_key(&hn("22")));
    assert!(range_map.contains_key(&hn("44")));
    assert!(range_map.contains_key(&hn("55")));
    assert!(range_map.contains_key(&hn("AA")));

    // Should NOT be in range
    assert!(!range_map.contains_key(&hn("23s")));
    assert!(!range_map.contains_key(&hn("23o")));
    assert!(!range_map.contains_key(&hn("A2s")));
    assert!(!range_map.contains_key(&hn("A2o"))); // Assuming "A2" meant A2s or A2o
}

#[test]
fn test_parse_range_str_plus_notation_suited() {
    let range_str = "A3s+";
    let range_map = parse_range_str(range_str).unwrap();

    // Should be in range
    assert!(range_map.contains_key(&hn("A3s")));
    assert!(range_map.contains_key(&hn("A4s")));
    assert!(range_map.contains_key(&hn("AKs")));

    // Should NOT be in range
    assert!(!range_map.contains_key(&hn("A2s")));
    assert!(!range_map.contains_key(&hn("AA")));
    assert!(!range_map.contains_key(&hn("A3o")));
}

#[test]
fn test_parse_range_str_plus_notation_offsuit() {
    let range_str = "KTo+";
    let range_map = parse_range_str(range_str).unwrap();

    // Should be in range
    assert!(range_map.contains_key(&hn("KTo")));
    assert!(range_map.contains_key(&hn("KJo")));
    assert!(range_map.contains_key(&hn("KQo")));

    // Should NOT be in range
    assert!(!range_map.contains_key(&hn("K9o")));
    assert!(!range_map.contains_key(&hn("K2o")));
    assert!(!range_map.contains_key(&hn("KTs")));
}
