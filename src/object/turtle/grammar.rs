use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Grammar {
    pub angle: f64,
    steps: usize,
    axiom: String,
    rules: HashMap<String, String>,
}

impl Grammar {
    pub fn expand(&self) -> String {
        let mut expanded = self.axiom.clone();

        for _ in 0..self.steps {
            let mut s = String::new();

            for c in expanded.chars() {
                let rule = self.rules.get(&*String::from(c));
                if let Some(rule) = rule {
                    s.push_str(rule);
                } else {
                    s.push(c);
                }
            }

            expanded = s;
        }

        expanded
    }
}

pub fn parse_grammar(path: String) -> Result<Grammar, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let json = serde_json::from_reader(reader)?;

    Ok(json)
}
