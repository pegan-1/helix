/**
* Helix Programming Language Interpreter
*
* @author  Peter Egan
* @since   2023-09-05
* @lastUpdated 2023-09-05
*/
// Standard Library Imports
use std::error::Error;
use std::fs;

// Helix modules
pub mod scanner;

// Alias structs
use scanner::Scanner;

// Stores configuration parameters.
pub struct Config {
    pub hpl_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Return error if incorrect number of arguments
        if args.len() != 2 {
            return Err("There should be only one argument - path/name of HPL file - in the command arguments.");
        }

        // Correct number of args. Grab the file path.
        let hpl_file_path = args[1].clone();
        Ok(Config { hpl_file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.hpl_file_path)?;

    println!("With text:\n{contents}");

    let scanner = Scanner {
        ..Default::default()
    };
    let tokens = scanner.scan(contents);
    println!("Tokens:\n{tokens}");
    scanner.whoami();

    Ok(())
}
