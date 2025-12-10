# Preflop Trainer

A command-line tool designed to help poker players train their preflop opening ranges in 6-max No-Limit Texas Hold'em.

## Features

*   **Configurable Ranges:** Define your opening ranges for various positions and scenarios using a simple `ranges.toml` file.
*   **Random Spot Generation:** The application generates random preflop scenarios, including your position and hole cards.
*   **Interactive Training:** Decide whether to Raise or Fold based on the presented spot.
*   **Instant Feedback:** Get immediate validation of your decision against your configured ranges.
*   **Scoring:** Track your performance with a running score and accuracy percentage.
*   **Unopened Pots Focus:** Currently, the trainer focuses exclusively on unopened pot scenarios.

## Getting Started

### Prerequisites

To build and run this application, you need to have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed on your system. `rustup` is the recommended way to manage Rust installations.

### Installation

1.  Navigate to the project directory (where `Cargo.toml` is located).
2.  Build the application:
    ```bash
    cargo build
    ```

## Usage

To start the preflop trainer, run the following command from the project root:

```bash
cargo run
```

The application will:
1.  Load your configured ranges from `ranges.toml`.
2.  Present you with a preflop scenario (e.g., "Position: UTG", "Hole Cards: As Kd").
3.  Prompt you to enter your action:
    *   Type `r` for **Raise**.
    *   Type `f` for **Fold**.
    *   Type `q` to **Quit** the current training session.
4.  Provide immediate feedback on whether your decision was correct or incorrect, along with the correct action if you were wrong.
5.  Display your current score and accuracy.
6.  Ask you to press Enter to continue to the next hand or `q` to quit.

## Configuration (`ranges.toml`)

The preflop trainer uses `ranges.toml` for its configuration. This file defines your preflop opening ranges and other game settings.

### Using the Example Configuration

A template file, `ranges.toml.example`, is provided in the project root. To get started, copy this file and rename it to `ranges.toml`:

```bash
cp ranges.toml.example ranges.toml
```

You can then edit `ranges.toml` to customize your training experience.

### Structure

Ranges are defined under the `[unopened_raise.<POSITION>]` section, where `<POSITION>` is one of `UTG`, `MP`, `CO`, `BTN`, `SB`.

Each position section contains a `range` string.

### Range String Format

The `range` string is a comma-separated list of hand notations.

In addition to explicit hand notations, the trainer supports shorthand for ranges:

*   **"Plus" Notation (`+`):** This indicates a range of hands from the specified hand upwards.
    *   `22+`: Includes `22`, `33`, `44`, ..., `AA`.
    *   `A3s+`: Includes `A3s`, `A4s`, `A5s`, ..., `AKs`.
    *   `KTo+`: Includes `KTo`, `KJo`, `KQo`, `KAo`.

*   **Pocket Pairs:** `AA`, `KK`, `QQ`, `JJ`, `TT`, `99`, `88`, `77`, `66`, `55`, `44`, `33`, `22`
*   **Suited Hands:** `AKs`, `AQs`, `AJs`, `ATs`, `A9s`, `A8s`, `A7s`, `A6s`, `A5s`, `A4s`, `A3s`, `A2s`, `KQs`, `KJs`, `KTs`, `K9s`, `K8s`, `K7s`, `K6s`, `K5s`, `K4s`, `K3s`, `K2s`, `QJs`, `QTs`, `Q9s`, `Q8s`, `Q7s`, `Q6s`, `Q5s`, `Q4s`, `Q3s`, `Q2s`, `JTs`, `J9s`, `J8s`, `J7s`, `J6s`, `J5s`, `J4s`, `J3s`, `J2s`, `T9s`, `T8s`, `T7s`, `T6s`, `T5s`, `T4s`, `98s`, `97s`, `96s`, `95s`, `87s`, `86s`, `85s`, `76s`, `75s`, `65s`, `64s`, `54s`
*   **Offsuit Hands:** `AKo`, `AQo`, `AJo`, `ATo`, `A9o`, `A8o`, `A7o`, `A6o`, `A5o`, `A4o`, `A3o`, `A2o`, `KQo`, `KJo`, `KTo`, `K9o`, `QJo`, `QTo`, `Q9o`, `JTo`, `J9o`, `T9o`, `98o`, `87o`, `76o`, `65o`

### Mixed Strategies

For hands played with a mixed frequency (e.g., sometimes raise, sometimes fold), you can append `: <frequency>` to the hand notation.
Example: `K6s:0.5` means King-Six suited is played 50% of the time. The trainer currently treats any hand with a frequency greater than `0.0` as a "Raise" action.

### Example `ranges.toml` snippet

```toml
[unopened_raise.UTG]
range = "AA,KK,QQ,JJ,TT,99,88,77,AKs,AQs,AJs,ATs,A9s,A8s,A7s,A6s,A5s,A4s,A3s,A2s,KQs,KJs,KTs,K9s,QJs,QTs,JTs,AKo,AQo,AJo,KQo,K6s:0.5"

[unopened_raise.BTN]
range = "22,A2s,K2s,Q2s,J2s,T2s,95s,85s,74s,64s,53s,43s,32s,A2o,K9o,Q9o,J9o,T9o,98o"
```

## Future Enhancements

*   Support for 3-bet pots and other preflop scenarios.
*   More complex actions (e.g., calling, 4-betting).

*   Detailed statistics and progress tracking.
