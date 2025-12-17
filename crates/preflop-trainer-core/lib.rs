#![deny(clippy::all)]
// src/lib.rs

#[macro_use]
extern crate lazy_static;

use rand::Rng;
use rand::prelude::IndexedRandom; // Needed for .choose() method
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap; // Add HashMap for uniqueness checks in tests
use std::fmt;
use std::fs;
use std::str::FromStr;
use dirs;

lazy_static! {
    static ref EMPTY_HAND_RANGE: HashMap<HandNotation, f32> = HashMap::new();
}

// --- Data Structures for Poker Concepts ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub const VALUES: [Self; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' => Ok(Rank::Ten),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            'A' => Ok(Rank::Ace),
            _ => Err(format!("Invalid rank character: {}", c)),
        }
    }

    pub fn to_char_lower(&self) -> char {
        match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 't',
            Rank::Jack => 'j',
            Rank::Queen => 'q',
            Rank::King => 'k',
            Rank::Ace => 'a',
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub const VALUES: [Self; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

    pub fn to_char_lower(&self) -> char {
        match self {
            Suit::Spades => 's',
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
        }
    }

    pub fn to_asset_string(&self) -> String {
        format!("suit_{}", self.to_char_lower())
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::Spades => "s",
            Suit::Hearts => "h",
            Suit::Diamonds => "d",
            Suit::Clubs => "c",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    pub card1: Card,
    pub card2: Card,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.card1, self.card2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandType {
    Pair,
    Suited,
    Offsuit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandNotation {
    pub rank1: Rank,
    pub rank2: Rank,
    pub hand_type: HandType,
}

impl HandNotation {
    pub fn from_hand(hand: Hand) -> Self {
        let rank1 = std::cmp::max(hand.card1.rank, hand.card2.rank);
        let rank2 = std::cmp::min(hand.card1.rank, hand.card2.rank);
        let hand_type = if hand.card1.rank == hand.card2.rank {
            HandType::Pair
        } else if hand.card1.suit == hand.card2.suit {
            HandType::Suited
        } else {
            HandType::Offsuit
        };
        HandNotation {
            rank1,
            rank2,
            hand_type,
        }
    }
}

impl FromStr for HandNotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() < 2 || chars.len() > 3 {
            return Err(format!("Invalid hand notation length: {}", s));
        }

        let rank1 = Rank::from_char(chars[0])?;
        let rank2 = Rank::from_char(chars[1])?;

        if chars.len() == 2 {
            // Pocket pair
            if rank1 == rank2 {
                Ok(HandNotation {
                    rank1,
                    rank2,
                    hand_type: HandType::Pair,
                })
            } else {
                Err(format!("Invalid pair notation: {}", s))
            }
        } else {
            // Suited or offsuit (chars.len() == 3)
            if rank1 == rank2 {
                return Err(format!("Invalid suited/offsuit notation for a pair: {}", s));
            }
            let hand_type = match chars[2] {
                's' => Ok(HandType::Suited),
                'o' => Ok(HandType::Offsuit),
                _ => Err(format!("Invalid hand type char: {}", chars[2])),
            }?;
            let (r1, r2) = if rank1 > rank2 {
                (rank1, rank2)
            } else {
                (rank2, rank1)
            };
            Ok(HandNotation {
                rank1: r1,
                rank2: r2,
                hand_type,
            })
        }
    }
}

// Helper function to generate all 169 unique HandNotations
pub fn get_all_possible_hand_notations() -> Vec<HandNotation> {
    let mut hand_notations = Vec::new();
    let ranks = &Rank::VALUES;

    // Pairs
    for &rank in ranks.iter() {
        hand_notations.push(HandNotation {
            rank1: rank,
            rank2: rank,
            hand_type: HandType::Pair,
        });
    }

    // Offsuit and Suited hands
    for i in (0..ranks.len()).rev() {
        for j in (0..ranks.len()).rev() {
            if ranks[i] > ranks[j] {
                // Suited
                hand_notations.push(HandNotation {
                    rank1: ranks[i],
                    rank2: ranks[j],
                    hand_type: HandType::Suited,
                });
                // Offsuit
                hand_notations.push(HandNotation {
                    rank1: ranks[i],
                    rank2: ranks[j],
                    hand_type: HandType::Offsuit,
                });
            }
        }
    }
    hand_notations
}

// --- Configuration Structures ---

// New struct for BBDefense ranges
#[derive(Debug, Deserialize)]
pub struct BBDefensePositionDetail {
    pub call_range: String,
    pub raise_range: String,
}

#[derive(Debug, Deserialize)]
pub struct GenericConfig {
    pub allowed_spot_types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    #[serde(rename = "unopened_raise")]
    pub unopened_raise: HashMap<String, PositionDetail>,
    #[serde(rename = "bb_defense")]
    pub bb_defense: Option<HashMap<String, BBDefensePositionDetail>>, // Use new struct here
    pub generic: Option<GenericConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PositionDetail {
    pub range: String, // Keep this for unopened_raise
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Position {
    UTG,
    MP,
    CO,
    BTN,
    SB,
    BB,
}

impl Position {
    pub const VALUES: [Self; 6] = [
        Position::UTG,
        Position::MP,
        Position::CO,
        Position::BTN,
        Position::SB,
        Position::BB,
    ];

    pub fn is_opener(&self) -> bool {
        matches!(
            self,
            Position::UTG | Position::MP | Position::CO | Position::BTN | Position::SB
        )
    }
}

impl FromStr for Position {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "UTG" => Ok(Position::UTG),
            "MP" => Ok(Position::MP),
            "CO" => Ok(Position::CO),
            "BTN" => Ok(Position::BTN),
            "SB" => Ok(Position::SB),
            "BB" => Ok(Position::BB),
            _ => Err(format!("Invalid position: {}", s)),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Position::UTG => "UTG",
            Position::MP => "MP",
            Position::CO => "CO",
            Position::BTN => "Button",
            Position::SB => "Small Blind",
            Position::BB => "Big Blind",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotType {
    Open { position: Position },
    BBDefense { opener_position: Position },
}

impl fmt::Display for SpotType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpotType::Open { position } => write!(f, "Open from {}", position),
            SpotType::BBDefense { opener_position } => write!(f, "BB vs {} Open", opener_position),
        }
    }
}

impl FromStr for SpotType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('_').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid SpotType string format: {}", s));
        }

        let type_str = parts[0];
        let pos_str = parts[1];

        match type_str {
            "Open" => Ok(SpotType::Open {
                position: Position::from_str(pos_str)?,
            }),
            "BBDefense" => Ok(SpotType::BBDefense {
                opener_position: Position::from_str(pos_str)?,
            }),
            _ => Err(format!("Unknown SpotType: {}", type_str)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserAction {
    Raise,
    Call,
    Fold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnswerResult {
    Correct,
    Wrong,
    FrequencyMistake,
}

#[derive(Debug, Clone, Default)]
pub struct GameConfig {
    pub unopened_raise_ranges: HashMap<Position, HashMap<HandNotation, f32>>,
    pub bb_defense_call_ranges: HashMap<Position, HashMap<HandNotation, f32>>, // New
    pub bb_defense_raise_ranges: HashMap<Position, HashMap<HandNotation, f32>>, // New
    pub allowed_spot_types: Vec<SpotType>,
}

use std::path::PathBuf;

pub fn find_or_create_config() -> Result<PathBuf, std::io::Error> {
    // 1. Check current working directory
    let cwd_candidate = PathBuf::from("ranges.toml");
    if cwd_candidate.exists() {
        return Ok(cwd_candidate);
    }

    // 2. Check executable directory
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_candidate = exe_dir.join("ranges.toml");
            if exe_candidate.exists() {
                return Ok(exe_candidate);
            }
        }
    }

    // 3. Check platform-specific config directory
    if let Some(config_dir) = dirs::config_dir() {
        let app_config_dir = config_dir.join("preflop-trainer");
        if !app_config_dir.exists() {
            fs::create_dir_all(&app_config_dir)?;
        }
        let config_path = app_config_dir.join("ranges.toml");
        if config_path.exists() {
            return Ok(config_path);
        } else {
            // 4. Create config from embedded example
            let example_content = include_str!("../../ranges.toml.example");
            fs::write(&config_path, example_content)?;
            return Ok(config_path);
        }
    }

    // 5. Fallback to a temporary file if all else fails
    let tmp = std::env::temp_dir().join(format!(
        "preflop_trainer_ranges_{}.toml",
        std::process::id()
    ));
    let example_content = include_str!("../../ranges.toml.example");
    fs::write(&tmp, example_content)?;
    Ok(tmp)
}

pub fn load_config() -> Result<GameConfig, Box<dyn std::error::Error>> {
    let config_path = find_or_create_config()?;
    let contents = fs::read_to_string(config_path)?;
    let toml_config: TomlConfig = toml::from_str(&contents)?;

    let mut unopened_raise_ranges = HashMap::new();
    for (pos_str, detail) in toml_config.unopened_raise {
        let position = Position::from_str(&pos_str)?;
        let range_map = parse_range_str(&detail.range)?;
        unopened_raise_ranges.insert(position, range_map);
    }

    let mut bb_defense_call_ranges = HashMap::new();
    let mut bb_defense_raise_ranges = HashMap::new();
    if let Some(bb_defense_toml) = toml_config.bb_defense {
        for (pos_str, detail) in bb_defense_toml {
            let position = Position::from_str(&pos_str)?;
            let call_range_map = parse_range_str(&detail.call_range)?;
            let raise_range_map = parse_range_str(&detail.raise_range)?;
            bb_defense_call_ranges.insert(position, call_range_map);
            bb_defense_raise_ranges.insert(position, raise_range_map);
        }
    }

    Ok(GameConfig {
        unopened_raise_ranges,
        bb_defense_call_ranges,
        bb_defense_raise_ranges,
        allowed_spot_types: if let Some(generic_config) = toml_config.generic {
            if let Some(toml_spot_types) = generic_config.allowed_spot_types {
                toml_spot_types
                    .into_iter()
                    .map(|s| SpotType::from_str(&s))
                    .collect::<Result<Vec<SpotType>, String>>()?
            } else {
                vec![
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
                ]
            }
        } else {
            vec![
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
            ]
        },
    })
}

pub fn parse_range_str(range_str: &str) -> Result<HashMap<HandNotation, f32>, String> {
    let mut range_map = HashMap::new();
    if range_str.is_empty() {
        return Ok(range_map);
    }
    for hand_part in range_str.split(',') {
        let parts: Vec<&str> = hand_part.trim().split(':').collect();
        let hand_notation_str_raw = parts[0];

        let frequency = if parts.len() == 2 {
            parts[1].parse::<f32>().map_err(|e| e.to_string())?
        } else {
            1.0
        };

        if hand_notation_str_raw.ends_with('+') {
            let base_hand_str = &hand_notation_str_raw[0..hand_notation_str_raw.len() - 1];
            let base_hand_notation = HandNotation::from_str(base_hand_str)?;

            if base_hand_notation.hand_type == HandType::Pair {
                let base_rank = base_hand_notation.rank1;
                for &rank in Rank::VALUES.iter().rev() {
                    // Iterate from Ace down to Two
                    if rank >= base_rank {
                        let notation = HandNotation {
                            rank1: rank,
                            rank2: rank,
                            hand_type: HandType::Pair,
                        };
                        range_map.insert(notation, frequency);
                    } else {
                        break;
                    }
                }
            } else {
                // Handle suited and offsuit '+' notation
                let base_rank1 = base_hand_notation.rank1;
                let base_rank2 = base_hand_notation.rank2;
                let hand_type = base_hand_notation.hand_type;

                // For XYs+ or XYo+, fix the higher rank (rank1) and iterate the lower rank (rank2) upwards
                // Example: A2s+ means A2s, A3s, ..., AKs (all suited Aces with lower card >= 2)
                for &rank2_iter in Rank::VALUES.iter() {
                    if rank2_iter >= base_rank2 && rank2_iter < base_rank1 {
                        // Lower rank must be less than higher rank
                        let notation = HandNotation {
                            rank1: base_rank1,
                            rank2: rank2_iter,
                            hand_type,
                        };
                        range_map.insert(notation, frequency);
                    } else if rank2_iter >= base_rank1 {
                        break; // Stop if lower rank becomes higher than or equal to base_rank1
                    }
                }
            }
        } else {
            let hand_notation = HandNotation::from_str(hand_notation_str_raw)?;
            range_map.insert(hand_notation, frequency);
        }
    }
    Ok(range_map)
}

// Helper function to calculate weighted hand notations
fn calculate_weighted_hand_notations(
    target_range: &HashMap<HandNotation, f32>,
    all_notations: &[HandNotation],
) -> Vec<(HandNotation, u32)> {
    let mut weighted_notations = Vec::new();

    for &hand_notation in all_notations {
        let mut weight = 20; // Default weight for hands not in any range

        if let Some(&frequency) = target_range.get(&hand_notation) {
            if frequency < 1.0 && frequency > 0.0 {
                weight = 5000; // High weight for mixed strategy hands
            } else if frequency == 1.0 {
                weight = 50; // Reduced weight for solid in-range hands
            }
        }
        weighted_notations.push((hand_notation, weight));
    }
    weighted_notations
}

// --- Deck Structure ---
#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for &suit in &Suit::VALUES {
            for &rank in &Rank::VALUES {
                cards.push(Card { rank, suit });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = ThreadRng::default();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal_hand(&mut self) -> Option<Hand> {
        if self.cards.len() < 2 {
            return None;
        }
        let card1 = self.cards.pop()?;
        let card2 = self.cards.pop()?;
        Some(Hand { card1, card2 })
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

// --- Game State ---
#[derive(Debug, Clone)]
pub struct Game {
    deck: Deck,
    config: GameConfig,
    all_possible_hand_notations: Vec<HandNotation>,
}

impl Game {
    pub fn new(config: GameConfig) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let all_possible_hand_notations = get_all_possible_hand_notations();
        Game {
            deck,
            config,
            all_possible_hand_notations,
        }
    }

    pub fn generate_random_spot(&mut self) -> Option<(SpotType, Hand, u8)> {
        let mut rng = ThreadRng::default();

        loop {
            // Reshuffle if deck is empty or too few cards
            if self.deck.cards.len() < 2 {
                self.deck = Deck::new();
                self.deck.shuffle();
            }

            let spot_type: SpotType;
            let target_hand_range: HashMap<HandNotation, f32>; // This will be owned

            // If no allowed spot types are configured, panic as no spots can be generated
            if self.config.allowed_spot_types.is_empty() {
                panic!(
                    "No valid spot types configured or able to be generated. Please configure 'allowed_spot_types' in GameConfig."
                );
            }

            // Randomly select one of the allowed spot types
            let chosen_allowed_spot_type = self.config.allowed_spot_types.choose(&mut rng).expect(
                "Should always be able to choose from a non-empty list of allowed spot types",
            );

            match chosen_allowed_spot_type {
                SpotType::Open {
                    position: chosen_position,
                } => {
                    spot_type = SpotType::Open {
                        position: *chosen_position,
                    };
                    target_hand_range = self
                        .config
                        .unopened_raise_ranges
                        .get(chosen_position)
                        .cloned() // Clone the HashMap to own it
                        .unwrap_or_else(|| EMPTY_HAND_RANGE.clone()); // Or use EMPTY_HAND_RANGE
                }
                SpotType::BBDefense {
                    opener_position: chosen_opener_position,
                } => {
                    spot_type = SpotType::BBDefense {
                        opener_position: *chosen_opener_position,
                    };

                    let mut combined_bb_defense_range = HashMap::new();
                    if let Some(call_map) = self
                        .config
                        .bb_defense_call_ranges
                        .get(chosen_opener_position)
                    {
                        combined_bb_defense_range.extend(call_map.iter().map(|(&k, &v)| (k, v)));
                    }
                    if let Some(raise_map) = self
                        .config
                        .bb_defense_raise_ranges
                        .get(chosen_opener_position)
                    {
                        // Raise frequencies take precedence if hand is in both
                        combined_bb_defense_range.extend(raise_map.iter().map(|(&k, &v)| (k, v)));
                    }
                    target_hand_range = combined_bb_defense_range;
                }
            }

            let weighted_hand_notations = calculate_weighted_hand_notations(
                &target_hand_range, // Now `target_hand_range` is owned
                &self.all_possible_hand_notations,
            );

            // 1. Manual weighted selection of a HandNotation
            let total_weight: u32 = weighted_hand_notations
                .iter()
                .map(|&(_, weight)| weight)
                .sum();
            if total_weight == 0 {
                // If the selected range is empty or has no weighted hands,
                // reshuffle and try to get a new spot and hand.
                self.deck = Deck::new();
                self.deck.shuffle();
                continue;
            }

            let mut rand_weight = rng.random_range(0..total_weight);
            let chosen_hand_notation = weighted_hand_notations
                .iter()
                .find_map(|&(hn, weight)| {
                    if rand_weight < weight {
                        Some(hn)
                    } else {
                        rand_weight -= weight;
                        None
                    }
                })
                .expect("Weighted selection failed to find a hand");

            // 3. Attempt to deal the concrete hand
            if let Some(hand) = self.try_deal_specific_hand(&chosen_hand_notation) {
                // 4. Generate RNG value for mixed strategies
                let mixed_strategy_rng_value: u8 = rng.random_range(0..100);
                return Some((spot_type, hand, mixed_strategy_rng_value));
            }
            // If try_deal_specific_hand returns None, we reshuffle and try again.
            self.deck = Deck::new();
            self.deck.shuffle();
        }
    }

    // Another helper function: tries to deal a specific hand from the current deck without reshuffling
    fn try_deal_specific_hand(&mut self, target_notation: &HandNotation) -> Option<Hand> {
        let mut matching_card_indices = Vec::new();

        // Iterate through all cards in the deck to find pairs that match the target_notation
        for i in 0..self.deck.cards.len() {
            for j in (i + 1)..self.deck.cards.len() {
                let card1 = self.deck.cards[i];
                let card2 = self.deck.cards[j];

                // Create a temporary Hand and its HandNotation to compare
                let current_hand_notation = HandNotation::from_hand(Hand { card1, card2 });

                if current_hand_notation == *target_notation {
                    matching_card_indices.push((i, j));
                }
            }
        }

        if matching_card_indices.is_empty() {
            return None; // No matching hand found in current deck
        }

        // Pick a random matching hand from the found ones
        let mut rng = ThreadRng::default();
        let (idx1, idx2) = matching_card_indices.choose(&mut rng)?.to_owned();

        // Get the cards before removing them
        let card1 = self.deck.cards[idx1];
        let card2 = self.deck.cards[idx2];
        let hand_to_deal = Hand { card1, card2 };

        // Remove the chosen cards from the deck
        // Remove higher index first to avoid issues with shifting indices
        self.deck.cards.remove(std::cmp::max(idx1, idx2));
        self.deck.cards.remove(std::cmp::min(idx1, idx2));

        Some(hand_to_deal)
    }
}

pub fn check_answer(
    config: &GameConfig,
    spot_type: SpotType,
    hand: Hand,
    user_action: UserAction,
    mixed_strategy_rng_value: u8,
) -> AnswerResult {
    let hand_notation = HandNotation::from_hand(hand);

    match spot_type {
        SpotType::Open { position } => {
            // For Open spots, only Raise and Fold are considered valid actions based on range
            if user_action == UserAction::Call {
                return AnswerResult::Wrong; // Cannot call an unopened pot
            }

            let position_range = config
                .unopened_raise_ranges
                .get(&position)
                .unwrap_or(&EMPTY_HAND_RANGE);
            let expected_to_raise_freq = position_range.get(&hand_notation).copied().unwrap_or(0.0);

            if expected_to_raise_freq == 1.0 {
                // 100% Raise
                if user_action == UserAction::Raise {
                    AnswerResult::Correct
                } else {
                    AnswerResult::Wrong
                }
            } else if expected_to_raise_freq == 0.0 {
                // 100% Fold
                if user_action == UserAction::Fold {
                    AnswerResult::Correct
                } else {
                    AnswerResult::Wrong
                }
            } else {
                // Mixed strategy for Raise/Fold
                let correct_action =
                    if (expected_to_raise_freq * 100.0) as u8 > mixed_strategy_rng_value {
                        UserAction::Raise
                    } else {
                        UserAction::Fold
                    };
                if user_action == correct_action {
                    AnswerResult::Correct
                } else {
                    AnswerResult::FrequencyMistake
                }
            }
        }
        SpotType::BBDefense { opener_position } => {
            let call_range = config
                .bb_defense_call_ranges
                .get(&opener_position)
                .unwrap_or(&EMPTY_HAND_RANGE);
            let raise_range = config
                .bb_defense_raise_ranges
                .get(&opener_position)
                .unwrap_or(&EMPTY_HAND_RANGE);

            let call_freq = call_range.get(&hand_notation).copied().unwrap_or(0.0);
            let raise_freq = raise_range.get(&hand_notation).copied().unwrap_or(0.0);

            // Determine the correct action based on stacked frequencies
            let raise_threshold = (raise_freq * 100.0) as u8;
            let call_threshold = raise_threshold.saturating_add((call_freq * 100.0) as u8);

            let correct_action = if mixed_strategy_rng_value < raise_threshold {
                UserAction::Raise
            } else if mixed_strategy_rng_value < call_threshold {
                UserAction::Call
            } else {
                UserAction::Fold
            };

            if user_action == correct_action {
                AnswerResult::Correct
            } else {
                // The user's action did not match the action dictated by the RNG.
                // We return `FrequencyMistake` if the user's action is *any* valid part of the
                // hand's overall strategy (even if it's not correct for this specific RNG).
                // Otherwise, it's just plain `Wrong`.
                let is_raise_possible = raise_freq > 0.0;
                let is_call_possible = call_freq > 0.0;
                let is_fold_possible = (raise_freq + call_freq) < 1.0;

                let is_user_action_part_of_strategy = (user_action == UserAction::Raise
                    && is_raise_possible)
                    || (user_action == UserAction::Call && is_call_possible)
                    || (user_action == UserAction::Fold && is_fold_possible);

                if is_user_action_part_of_strategy {
                    AnswerResult::FrequencyMistake
                } else {
                    AnswerResult::Wrong
                }
            }
        }
    }
}

pub fn get_action_frequencies(
    config: &GameConfig,
    spot_type: SpotType,
    hand: Hand,
) -> (f32, f32, f32) {
    // (raise, call, fold)
    let hand_notation = HandNotation::from_hand(hand);
    match spot_type {
        SpotType::Open { position } => {
            let range = config
                .unopened_raise_ranges
                .get(&position)
                .unwrap_or(&EMPTY_HAND_RANGE);
            let raise_freq = range.get(&hand_notation).copied().unwrap_or(0.0);
            (raise_freq, 0.0, 1.0 - raise_freq)
        }
        SpotType::BBDefense { opener_position } => {
            let call_range = config
                .bb_defense_call_ranges
                .get(&opener_position)
                .unwrap_or(&EMPTY_HAND_RANGE);
            let raise_range = config
                .bb_defense_raise_ranges
                .get(&opener_position)
                .unwrap_or(&EMPTY_HAND_RANGE);
            let call_freq = call_range.get(&hand_notation).copied().unwrap_or(0.0);
            let raise_freq = raise_range.get(&hand_notation).copied().unwrap_or(0.0);
            let total_play_freq = call_freq + raise_freq;
            (raise_freq, call_freq, 1.0 - total_play_freq.min(1.0))
        }
    }
}
