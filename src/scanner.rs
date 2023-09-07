//!
//! Helix Programming Language Scanner
//!
//! Given a valid Helix Programming File (hpl), will tokenize the input.
//!

pub struct Scanner {
    pub name: String,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            name: "Helix Scanner".to_string(),
        }
    }
}

impl Scanner {
    pub fn scan(&self, hpl_contents: String) -> String {
        hpl_contents
    }

    /// Returns a string with the name of the service
    pub fn whoami(&self) {
        println!("{}", self.name);
    }
}
