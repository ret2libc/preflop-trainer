// Unix implementation (uses termion)
#[cfg(unix)]
#[deny(clippy::all)]
mod unix_cli {
    use clap::{Parser, Subcommand};
    use colored::*;
    use preflop_trainer_core::{AnswerResult, Game, UserAction, check_answer, load_config};
    use std::io::{Write, stdin, stdout};
    use std::str::FromStr;
    use termion::{input::TermRead, raw::IntoRawMode};

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Cli {
        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand, Default)]
    enum Commands {
        CheckRange {
            #[arg(short = 'r', long)]
            range_str: String,
            #[arg(short = 's', long)]
            hand_str: String,
        },
        #[default]
        Game,
    }

    pub fn run() {
        let cli = Cli::parse();

        match cli.command.unwrap_or_default() {
            Commands::CheckRange {
                range_str,
                hand_str,
            } => handle_check_range_command(&range_str, &hand_str),
            Commands::Game => run_game_loop(),
        }
    }

    fn run_game_loop() {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        write!(stdout, "--- Poker Preflop Trainer ---\r\n").unwrap();
        stdout.flush().unwrap();

        let game_config = match load_config() {
            Ok(config) => config,
            Err(e) => {
                write!(
                    stdout,
                    "{}\r\n{}",
                    termion::cursor::Show,
                    format!("Error loading configuration: {}", e).red()
                )
                .unwrap();
                stdout.flush().unwrap();
                return;
            }
        };

        write!(
            stdout,
            "Configuration loaded successfully. Starting game...\r\n\r\n"
        )
        .unwrap();
        stdout.flush().unwrap();

        let mut game = Game::new(game_config.clone());
        let mut correct_answers = 0.0_f32;
        let mut total_questions = 0;
        let mut current_question_answered = true;
        let mut current_spot_details: Option<(
            preflop_trainer_core::SpotType,
            preflop_trainer_core::Hand,
            u8,
        )> = None;

        loop {
            if current_question_answered {
                total_questions += 1;
                match game.generate_random_spot() {
                    Some((spot_type, hand, mixed_strategy_rng_value)) => {
                        write!(stdout, "Question {}:\r\n", total_questions).unwrap();
                        write!(stdout, "Position: {}\r\n", format!("{}", spot_type).cyan())
                            .unwrap();
                        write!(stdout, "Hole Cards: {}\r\n", format!("{}", hand).yellow()).unwrap();
                        write!(stdout, "RNG: {}\r\n", mixed_strategy_rng_value).unwrap();

                        let actions_prompt = match spot_type {
                            preflop_trainer_core::SpotType::Open { .. } => "(R)aise or (F)old? ",
                            preflop_trainer_core::SpotType::BBDefense { .. } => {
                                "(R)aise, (C)all, or (F)old? "
                            }
                        };
                        write!(stdout, "{}", actions_prompt).unwrap();

                        stdout.flush().unwrap();
                        current_spot_details = Some((spot_type, hand, mixed_strategy_rng_value));
                        current_question_answered = false;
                    }
                    None => {
                        write!(stdout, "Reshuffling deck...\r\n").unwrap();
                        stdout.flush().unwrap();
                        total_questions -= 1;
                        continue;
                    }
                }
            }

            if let Some(Ok(key)) = stdin.lock().keys().next() {
                let user_action = match key {
                    termion::event::Key::Char('r') | termion::event::Key::Char('R') => {
                        Some(UserAction::Raise)
                    }
                    termion::event::Key::Char('f') | termion::event::Key::Char('F') => {
                        Some(UserAction::Fold)
                    }
                    termion::event::Key::Char('c') | termion::event::Key::Char('C') => {
                        Some(UserAction::Call)
                    }
                    termion::event::Key::Char('q') | termion::event::Key::Char('Q') => {
                        write!(stdout, "\r\nQuitting game.\r\n").unwrap();
                        if !current_question_answered {
                            total_questions -= 1;
                        }
                        break;
                    }
                    termion::event::Key::Ctrl('c') | termion::event::Key::Ctrl('d') => {
                        write!(stdout, "\r\nQuitting game.\r\n").unwrap();
                        if !current_question_answered {
                            total_questions -= 1;
                        }
                        break;
                    }
                    _ => None,
                };

                if let Some(action) = user_action
                    && !current_question_answered
                    && let Some((spot_type, hand, mixed_strategy_rng_value)) = current_spot_details
                {
                    let result = check_answer(
                        &game_config,
                        spot_type,
                        hand,
                        action,
                        mixed_strategy_rng_value,
                    );

                    match result {
                        AnswerResult::Correct => {
                            correct_answers += 1.0;
                            write!(stdout, "{}\r\n", "Correct!".green()).unwrap();
                        }
                        AnswerResult::Wrong => {
                            write!(stdout, "{}\r\n", "Wrong.".red()).unwrap();
                        }
                        AnswerResult::FrequencyMistake => {
                            correct_answers += 0.5;
                            write!(stdout, "{}\r\n", "Frequency mistake.".yellow()).unwrap();
                        }
                    }

                    let percentage = if total_questions > 0 {
                        (correct_answers / total_questions as f32) * 100.0
                    } else {
                        0.0
                    };
                    write!(
                        stdout,
                        "Score: {}/{} ({:.2}%)\r\n\r\n",
                        correct_answers, total_questions, percentage
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    current_question_answered = true;
                    current_spot_details = None;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        write!(stdout, "--- Game Over ---\r\n").unwrap();
        write!(
            stdout,
            "Final Score: {}/{} ({:.2}%)\r\n",
            correct_answers,
            total_questions,
            if total_questions > 0 {
                (correct_answers / total_questions as f32) * 100.0
            } else {
                0.0
            }
        )
        .unwrap();
        write!(stdout, "{}", termion::cursor::Show).unwrap();
        stdout.flush().unwrap();
    }

    fn handle_check_range_command(range_str: &str, hand_str: &str) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let _stdin = stdin();

        let _game_config = match load_config() {
            Ok(config) => config,
            Err(e) => {
                write!(
                    stdout,
                    "{}\r\n{}",
                    termion::cursor::Show,
                    format!("Error loading configuration: {}", e).red()
                )
                .unwrap();
                stdout.flush().unwrap();
                return;
            }
        };

        let range_map = match preflop_trainer_core::parse_range_str(range_str) {
            Ok(map) => map,
            Err(e) => {
                write!(
                    stdout,
                    "{}\r\n",
                    format!("Error parsing range string: {}", e).red()
                )
                .unwrap();
                stdout.flush().unwrap();
                return;
            }
        };

        let hand_notation = match preflop_trainer_core::HandNotation::from_str(hand_str) {
            Ok(hn) => hn,
            Err(e) => {
                write!(
                    stdout,
                    "{}\r\n",
                    format!("Error parsing hand string: {}", e).red()
                )
                .unwrap();
                stdout.flush().unwrap();
                return;
            }
        };

        match range_map.get(&hand_notation) {
            Some(&frequency) => write!(
                stdout,
                "Hand {} is in range with frequency: {:.2}%\r\n",
                hand_str.yellow(),
                frequency * 100.0
            )
            .unwrap(),
            None => write!(
                stdout,
                "Hand {} is {} in range.\r\n",
                hand_str.yellow(),
                "NOT".red()
            )
            .unwrap(),
        }
        write!(stdout, "{}", termion::cursor::Show).unwrap();
        stdout.flush().unwrap();
    }
}

// Non-Unix stub so the crate builds on Windows for workspace checks
#[cfg(not(unix))]
fn main() {
    println!("preflop-trainer-cli: CLI is only supported on Unix-like systems. Skipping CLI.");
}

// On Unix, call the unix runner
#[cfg(unix)]
fn main() {
    unix_cli::run();
}
