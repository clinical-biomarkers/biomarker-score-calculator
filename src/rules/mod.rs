use std::fs;

pub mod schema;
pub mod engine;

pub fn parse_rules(rules_file: Option<&String>) -> Option<schema::CustomRules> {
    if let Some(path) = rules_file {
        let file_contents = fs::read_to_string(path).expect("Could not read rules file.");
        let rules = serde_json::from_str(&file_contents).expect("Error parsing rules file.");
        Some(rules)
    } else {
        None
    }
}
