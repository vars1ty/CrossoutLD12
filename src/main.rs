use crate::runner::Runner;

mod cli_tools;
mod runner;

/// Starts patching and runs the game.
fn main() {
    Runner::new().run_game_launcher()
}
