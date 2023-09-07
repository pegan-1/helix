//!
//! Helix Programming Language Interpreter
//!
//! Given a valid Helix Programming File (hpl), the
//! interpreter will run the program.
//!

// Standard Library Imports
use std::env;
use std::process;

// Helix imports
use helix::Config;

fn main() {
    eprintln!("Helix Interpreter: v0.0.1.");
    // Read in the program arguments
    let args: Vec<String> = env::args().collect();

    // And build the configuration
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Configuration Error: {err}");
        process::exit(1);
    });

    // With the configuration obtained, run the program
    if let Err(e) = helix::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
