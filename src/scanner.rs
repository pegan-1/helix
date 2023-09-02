/**
* Helix Programming Language Scanner
*
* @author  Peter Egan
* @since   2023-08-31
* @lastUpdated 2023-09-02
*/

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
    pub fn whoami(&self) {
        println!("{}", self.name);
    }
}
