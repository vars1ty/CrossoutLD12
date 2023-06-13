use std::io::{Read, Write};

/// Basic CLI Tools.
pub struct CLITools;

impl CLITools {
    /// Prints a message and pauses the application until the user has pressed a key.
    pub fn print_pause(message: &str) {
        println!("{message}");
        // Sigh, Rust and not having a pause function for CLI applications.
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        stdout.flush().unwrap();
        stdin.read_exact(&mut [0u8]).unwrap();
    }
}
