/**
* Helix Programming Language Interpreter
*
* @author  Peter Egan
* @since   2023-08-31
* @lastUpdated 2023-09-02
*/

// Modules
mod scanner;

// Aliases
use scanner::Scanner;

fn main() {
    println!("Welcome to the Helix Interpreter!");
    let scanner = Scanner {
        ..Default::default()
    };
    scanner.whoami();
}
