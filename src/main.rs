use crate::runner::Runner;

mod cli_tools;
mod runner;

/// Starts patching the game.
fn main() {
    Runner::new().patch_game()
}
