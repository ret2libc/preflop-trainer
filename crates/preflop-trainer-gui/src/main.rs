use iced::{
    Application, Background, Color, Command, Element, Length, Theme,
    alignment::{self, Horizontal},
    border::Border,
    executor, theme,
    widget::{Button, Svg, column, container, row, text},
};
// Embed the `assets/cards` directory so the binary can render cards without external assets.

// `include_dir!` paths are relative to the crate root (where Cargo.toml is),
// the repository `assets` directory lives two levels up from the crate root.
// Embed the four suit SVGs so the binary can render cards without external assets.
// Embed the four suit SVGs so the binary can render cards without external assets.
static SUIT_C_SVG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/cards/suit_c.svg"
));
static SUIT_D_SVG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/cards/suit_d.svg"
));
static SUIT_H_SVG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/cards/suit_h.svg"
));
static SUIT_S_SVG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/cards/suit_s.svg"
));

pub fn main() -> iced::Result {
    PreflopTrainerGui::run(iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(600.0, 720.0), // Increased height for feedback
            resizable: false,
            ..Default::default()
        },
        ..iced::Settings::default()
    })
}

#[derive(Debug, Clone)]
struct PreflopTrainerGui {
    game: preflop_trainer_core::Game,
    current_spot_type: preflop_trainer_core::SpotType,
    current_hand: preflop_trainer_core::Hand,
    mixed_strategy_rng_value: u8,
    config: preflop_trainer_core::GameConfig,
    previous_hand_info: Option<PreviousHandInfo>,
    correct_answers: f32,
    total_questions: u32,
    game_ended: bool,
}

#[derive(Debug, Clone, Copy)]
struct PreviousHandInfo {
    hand: preflop_trainer_core::Hand,
    spot_type: preflop_trainer_core::SpotType,
    user_action: preflop_trainer_core::UserAction,
    rng_value: u8,
    result: preflop_trainer_core::AnswerResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Message {
    Raise,
    Fold,
    Call,
    EndGame,
}

impl Application for PreflopTrainerGui {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let config =
            preflop_trainer_core::load_config().expect("Failed to load or parse ranges.toml");

        let mut game = preflop_trainer_core::Game::new(config.clone());
        let (spot_type, hand, rng_value) = game
            .generate_random_spot()
            .expect("Failed to generate initial spot");

        (
            Self {
                game,
                current_spot_type: spot_type,
                current_hand: hand,
                mixed_strategy_rng_value: rng_value,
                config,
                previous_hand_info: None,
                correct_answers: 0.0,
                total_questions: 0,
                game_ended: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Preflop Trainer GUI")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        if self.game_ended && message != Message::EndGame {
            return Command::none();
        }

        match message {
            Message::Raise | Message::Fold | Message::Call => {
                let user_action = match message {
                    Message::Raise => preflop_trainer_core::UserAction::Raise,
                    Message::Fold => preflop_trainer_core::UserAction::Fold,
                    Message::Call => preflop_trainer_core::UserAction::Call,
                    _ => unreachable!(),
                };

                let result = preflop_trainer_core::check_answer(
                    &self.config,
                    self.current_spot_type,
                    self.current_hand,
                    user_action,
                    self.mixed_strategy_rng_value,
                );

                self.previous_hand_info = Some(PreviousHandInfo {
                    hand: self.current_hand,
                    spot_type: self.current_spot_type,
                    user_action,
                    rng_value: self.mixed_strategy_rng_value,
                    result,
                });

                self.total_questions += 1;
                match result {
                    preflop_trainer_core::AnswerResult::Correct => self.correct_answers += 1.0,
                    preflop_trainer_core::AnswerResult::FrequencyMistake => {
                        self.correct_answers += 0.5
                    }
                    preflop_trainer_core::AnswerResult::Wrong => {}
                }

                // Immediately generate the NEXT hand
                let (spot_type, hand, rng_value) = self
                    .game
                    .generate_random_spot()
                    .expect("Failed to generate next spot");
                self.current_spot_type = spot_type;
                self.current_hand = hand;
                self.mixed_strategy_rng_value = rng_value;
            }

            Message::EndGame => {
                if self.game_ended {
                    // Restart the game
                    self.game_ended = false;
                    self.total_questions = 0;
                    self.correct_answers = 0.0;
                    let (spot_type, hand, rng_value) = self
                        .game
                        .generate_random_spot()
                        .expect("Failed to generate next spot");
                    self.current_spot_type = spot_type;
                    self.current_hand = hand;
                    self.mixed_strategy_rng_value = rng_value;
                    self.previous_hand_info = None;
                } else {
                    // End the game
                    self.game_ended = true;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        if self.game_ended {
            let percentage = if self.total_questions > 0 {
                (self.correct_answers / self.total_questions as f32) * 100.0
            } else {
                0.0
            };

            return column![
                text("Game Over!").size(50),
                text(format!("Total Questions: {}", self.total_questions)).size(30),
                text(format!("Correct Answers: {}", self.correct_answers)).size(30),
                text(format!("Score: {:.2}%", percentage)).size(30),
                Button::new(text("Play Again").size(25)).on_press(Message::EndGame),
            ]
            .spacing(20)
            .align_items(alignment::Horizontal::Center.into())
            .into();
        }

        let render_card =
            |card: &preflop_trainer_core::Card, size_multiplier: f32| -> Element<'_, Message> {
                let rank_size = (50.0 * size_multiplier) as u16;
                let suit_svg_width = 30.0 * size_multiplier;
                let suit_svg_height = 30.0 * size_multiplier;
                let card_width = 80.0 * size_multiplier;
                let card_height = 100.0 * size_multiplier;
                let padding_val = (5.0 * size_multiplier) as u16;

                let suit_color = match card.suit {
                    preflop_trainer_core::Suit::Clubs => Color::from_rgb(0.0, 0.5, 0.0),
                    preflop_trainer_core::Suit::Diamonds => Color::from_rgb(0.0, 0.0, 1.0),
                    preflop_trainer_core::Suit::Hearts => Color::from_rgb(1.0, 0.0, 0.0),
                    preflop_trainer_core::Suit::Spades => Color::from_rgb(0.0, 0.0, 0.0),
                };

                container(
                    column![
                        text(format!("{}", card.rank))
                            .size(rank_size)
                            .horizontal_alignment(Horizontal::Center)
                            .style(theme::Text::Color(suit_color)),
                        {
                            // Match directly on the suit enum so we reliably use embedded bytes.
                            let handle = match card.suit {
                                preflop_trainer_core::Suit::Clubs => {
                                    iced::widget::svg::Handle::from_memory(SUIT_C_SVG.to_vec())
                                }
                                preflop_trainer_core::Suit::Diamonds => {
                                    iced::widget::svg::Handle::from_memory(SUIT_D_SVG.to_vec())
                                }
                                preflop_trainer_core::Suit::Hearts => {
                                    iced::widget::svg::Handle::from_memory(SUIT_H_SVG.to_vec())
                                }
                                preflop_trainer_core::Suit::Spades => {
                                    iced::widget::svg::Handle::from_memory(SUIT_S_SVG.to_vec())
                                }
                            };
                            Svg::new(handle)
                        }
                        .width(Length::Fixed(suit_svg_width))
                        .height(Length::Fixed(suit_svg_height)),
                    ]
                    .align_items(alignment::Horizontal::Center.into())
                    .padding(padding_val),
                )
                .width(Length::Fixed(card_width))
                .height(Length::Fixed(card_height))
                .center_x()
                .center_y()
                .style(theme::Container::Custom(Box::new(MyContainerStyle::new(
                    ContainerStyleType::Card,
                ))))
                .into()
            };

        let position_labels = ["UTG", "MP", "CO", "Button", "Small Blind", "Big Blind"];
        let mut positions_layout = row![].spacing(10).width(Length::Fill);

        let (user_pos_str, opener_pos_str_option) = match &self.current_spot_type {
            preflop_trainer_core::SpotType::Open { position } => (format!("{}", position), None),
            preflop_trainer_core::SpotType::BBDefense { opener_position } => (
                "Big Blind".to_string(),
                Some(format!("{}", opener_position)),
            ),
        };

        for &pos_label in position_labels.iter() {
            let style_type = if pos_label == user_pos_str.as_str() {
                ContainerStyleType::SeatUser
            } else if let Some(opener_str) = &opener_pos_str_option {
                if pos_label == opener_str.as_str() {
                    ContainerStyleType::SeatOpener
                } else {
                    ContainerStyleType::SeatNormal
                }
            } else {
                ContainerStyleType::SeatNormal
            };

            let seat_content = container(text(pos_label))
                .width(Length::Fixed(80.0))
                .height(Length::Fixed(40.0))
                .center_x()
                .center_y()
                .style(theme::Container::Custom(Box::new(MyContainerStyle::new(
                    style_type,
                ))));
            positions_layout = positions_layout.push(seat_content);
        }

        let poker_table = container(
            column![
                positions_layout,
                row![
                    render_card(&self.current_hand.card1, 1.0),
                    render_card(&self.current_hand.card2, 1.0),
                ]
                .spacing(10)
                .align_items(alignment::Vertical::Center.into()),
                text(format!("RNG: {}", self.mixed_strategy_rng_value)).size(20),
            ]
            .spacing(20)
            .align_items(alignment::Horizontal::Center.into()),
        )
        .width(Length::Fixed(600.0))
        .height(Length::Fixed(300.0))
        .center_x()
        .center_y()
        .style(theme::Container::Custom(Box::new(MyContainerStyle::new(
            ContainerStyleType::Table,
        ))));

        let raise_button = Button::new(
            text("Raise")
                .size(25)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::Raise)
        .width(Length::Fixed(120.0))
        .padding(10);
        let fold_button = Button::new(
            text("Fold")
                .size(25)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::Fold)
        .width(Length::Fixed(120.0))
        .padding(10);
        let call_button = Button::new(
            text("Call")
                .size(25)
                .horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::Call)
        .width(Length::Fixed(120.0))
        .padding(10);

        let mut action_buttons = row![]
            .spacing(10)
            .align_items(alignment::Vertical::Center.into());
        match self.current_spot_type {
            preflop_trainer_core::SpotType::Open { .. } => {
                action_buttons = action_buttons.push(raise_button).push(fold_button);
            }
            preflop_trainer_core::SpotType::BBDefense { .. } => {
                action_buttons = action_buttons
                    .push(raise_button)
                    .push(call_button)
                    .push(fold_button);
            }
        }

        let mut main_content = column![poker_table, action_buttons]
            .spacing(20)
            .align_items(alignment::Horizontal::Center.into());

        if let Some(info) = &self.previous_hand_info {
            let (raise_freq, call_freq, fold_freq) = preflop_trainer_core::get_action_frequencies(
                &self.config,
                info.spot_type,
                info.hand,
            );

            let raise_threshold = (raise_freq * 100.0) as u8;
            let call_threshold = raise_threshold.saturating_add((call_freq * 100.0) as u8);

            let correct_action_for_rng = if info.rng_value < raise_threshold {
                preflop_trainer_core::UserAction::Raise
            } else if info.rng_value < call_threshold {
                preflop_trainer_core::UserAction::Call
            } else {
                preflop_trainer_core::UserAction::Fold
            };

            let render_feedback_button =
                |action: preflop_trainer_core::UserAction, percentage: f32| {
                    let mut style =
                        MyContainerStyle::new(ContainerStyleType::Feedback(FeedbackStyle::Neutral));

                    if info.user_action == action {
                        style.style = match info.result {
                            preflop_trainer_core::AnswerResult::Correct => {
                                ContainerStyleType::Feedback(FeedbackStyle::Correct)
                            }
                            preflop_trainer_core::AnswerResult::Wrong => {
                                ContainerStyleType::Feedback(FeedbackStyle::Wrong)
                            }
                            preflop_trainer_core::AnswerResult::FrequencyMistake => {
                                ContainerStyleType::Feedback(FeedbackStyle::Ok)
                            }
                        };
                    }

                    if correct_action_for_rng == action
                        && info.user_action != correct_action_for_rng
                    {
                        style.border_color = Color::from_rgb(0.0, 0.6, 0.0);
                        style.border_width = 2.0;
                    }

                    let action_text = match action {
                        preflop_trainer_core::UserAction::Raise => "Raise",
                        preflop_trainer_core::UserAction::Call => "Call",
                        preflop_trainer_core::UserAction::Fold => "Fold",
                    };

                    container(
                        column![
                            text(action_text).size(20),
                            text(format!("{:.0}%", percentage * 100.0)).size(18),
                        ]
                        .align_items(alignment::Horizontal::Center.into())
                        .spacing(5),
                    )
                    .width(Length::Fixed(100.0))
                    .padding(10)
                    .center_x()
                    .style(theme::Container::Custom(Box::new(style)))
                };

            let separator = container(text(""))
                .width(Length::Fill)
                .height(Length::Fixed(1.0))
                .style(theme::Container::Custom(Box::new(MyContainerStyle::new(
                    ContainerStyleType::Separator,
                ))));

            let previous_hand_summary = row![
                text("Previous Hand:").size(18),
                text(format!("{}", info.spot_type)).size(18),
                render_card(&info.hand.card1, 0.7),
                render_card(&info.hand.card2, 0.7),
            ]
            .spacing(10)
            .align_items(alignment::Vertical::Center.into());

            let feedback_row = row![
                render_feedback_button(preflop_trainer_core::UserAction::Raise, raise_freq),
                render_feedback_button(preflop_trainer_core::UserAction::Call, call_freq),
                render_feedback_button(preflop_trainer_core::UserAction::Fold, fold_freq),
            ]
            .spacing(10);

            main_content = main_content.push(
                column![separator, previous_hand_summary, feedback_row]
                    .spacing(10)
                    .align_items(alignment::Horizontal::Center.into()),
            );
        }

        let control_buttons =
            row![Button::new(text("End Game").size(20)).on_press(Message::EndGame),].spacing(20);

        main_content = main_content.push(control_buttons);

        main_content.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FeedbackStyle {
    Correct,
    Wrong,
    Ok,
    Neutral,
}

#[derive(Clone, Copy, Debug)]
enum ContainerStyleType {
    SeatNormal,
    SeatUser,
    SeatOpener,
    Card,
    Table,
    Feedback(FeedbackStyle),
    Separator,
}

#[derive(Clone, Copy, Debug)]
struct MyContainerStyle {
    style: ContainerStyleType,
    border_color: Color,
    border_width: f32,
}

impl MyContainerStyle {
    fn new(style: ContainerStyleType) -> Self {
        Self {
            style,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        }
    }
}

impl container::StyleSheet for MyContainerStyle {
    type Style = Theme;

    fn appearance(&self, _theme: &Self::Style) -> container::Appearance {
        let mut appearance = container::Appearance {
            border: Border {
                color: self.border_color,
                width: self.border_width,
                radius: 5.0.into(),
            },
            ..Default::default()
        };

        let background = match self.style {
            ContainerStyleType::SeatNormal => Some(Color::from_rgb(0.4, 0.4, 0.4)),
            ContainerStyleType::SeatUser => Some(Color::from_rgb(1.0, 1.0, 0.0)),
            ContainerStyleType::SeatOpener => Some(Color::from_rgb(1.0, 0.65, 0.0)),
            ContainerStyleType::Card => Some(Color::WHITE),
            ContainerStyleType::Table => {
                appearance.border.radius = 20.0.into();
                Some(Color::from_rgb(0.2, 0.5, 0.3))
            }
            ContainerStyleType::Feedback(feedback_style) => match feedback_style {
                FeedbackStyle::Correct => Some(Color::from_rgb(0.7, 1.0, 0.7)),
                FeedbackStyle::Wrong => Some(Color::from_rgb(1.0, 0.7, 0.7)),
                FeedbackStyle::Ok => Some(Color::from_rgb(1.0, 0.9, 0.7)),
                FeedbackStyle::Neutral => Some(Color::from_rgb(0.9, 0.9, 0.9)),
            },
            ContainerStyleType::Separator => Some(Color::from_rgb(0.5, 0.5, 0.5)),
        };

        appearance.background = background.map(Background::Color);
        appearance
    }
}
