# Preflop Trainer

A command-line tool to help poker players train and memorize their preflop opening ranges in 6-max No-Limit Texas Hold'em. It helps users learn by putting them in different preflop spots where they must decide to raise or fold, based on ranges they configure themselves in `ranges.toml`.

## Development Workflow

This project follows a Test-Driven Development (TDD) approach. All new functionalities should be implemented following these steps:

1. **Write Extensive Tests:** Before writing any implementation code, create comprehensive tests for the new functionality in the `tests/` directory. These tests should cover all the expected behaviors and edge cases.
2. **Create Skeletons:** Write the minimal amount of code required for the tests to compile. This includes creating basic structs, functions, and methods with empty or placeholder implementations.
3. **Implement and Pass:** Write the actual implementation code with the goal of making all the tests pass.
4. **Maintain Code Quality:** Always ensure that all code quality checks pass. Before committing changes, run the following commands:
    - `cargo fmt --all` to format the code.
    - `cargo clippy --workspace --all-targets --all-features -- -D warnings` to run linting checks and fix any warnings or errors.
    - `cargo test --workspace --locked --verbose` to execute all tests.
5. **Commit Progressively:** Use Git to commit changes frequently and progressively. Each commit should represent a small, logical, and self-contained change, making it easier to track development and revert if necessary.

Keep track of what needs to be done in TODOs.txt.

## Building the Project

To build the project, navigate to the project's root directory and run:

```bash
cargo build
```

## Running the Project

To run the main executable, use:

```bash
cargo run
```

## Running Tests

To execute the tests for the project, run:

```bash
cargo test
```

## Configuration

The project uses `ranges.toml` for configuration.
