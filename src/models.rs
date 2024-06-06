use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Deserialize, Debug)]
pub struct Biomarker {
    pub biomarker_id: String,
    pub biomarker_component: Vec<Component>,
    pub condition: Condition,
    pub evidence_source: Vec<Evidence>,
}

#[derive(Deserialize, Debug)]
pub struct Component {
    pub specimen: Vec<Specimen>,
    pub evidence_source: Vec<Evidence>,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Evidence {
    pub id: String,
    pub database: String,
}

#[derive(Deserialize, Debug)]
pub struct Specimen {
    pub loinc_code: String,
}

#[derive(Serialize)]
pub struct ScoreContribution {
    pub c: String,
    pub w: f64,
    pub f: f64,
}

#[derive(Serialize)]
pub struct ScoreInfo {
    pub contributions: Vec<ScoreContribution>,
    pub formula: String,
    pub variables: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct BiomarkerScore {
    pub score: f64,
    pub score_info: ScoreInfo,
}

pub struct Weights {
    pub clinical_use: i32,
    pub first_pmid: i32,
    pub other_pmid: f64,
    pub pmid_limit: usize,
    pub first_source: i32,
    pub other_source: f64,
    pub loinc: i32,
    pub generic_condition_pen: i32,
    pub generic_conditions: HashSet<String>,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            clinical_use: 5,
            first_pmid: 1,
            other_pmid: 0.2,
            pmid_limit: 10,
            first_source: 1,
            other_source: 0.1,
            loinc: 1,
            generic_condition_pen: -4,
            generic_conditions: ["DOID:162".to_string()].into_iter().collect(),
        }
    }
}

pub fn get_user_weights() -> Weights {
    let mut weights = Weights::default();
    println!("Enter weight/configuration override or press Enter to use default value:");

    println!("Clinical use (default = {}):", weights.clinical_use);
    weights.clinical_use = read_input().unwrap_or(weights.clinical_use);

    println!("First PMID (default = {}):", weights.first_pmid);
    weights.first_pmid = read_input().unwrap_or(weights.first_pmid);

    println!("Other PMID (default = {}):", weights.other_pmid);
    weights.other_pmid = read_input().unwrap_or(weights.other_pmid);

    println!("PMID Limit (default = {}):", weights.pmid_limit);
    weights.pmid_limit = read_input().unwrap_or(weights.pmid_limit);

    println!("First source (default = {}):", weights.first_source);
    weights.first_source = read_input().unwrap_or(weights.first_source);

    println!("Other source (default = {}):", weights.other_source);
    weights.other_source = read_input().unwrap_or(weights.other_source);

    println!("Loinc (default = {}):", weights.loinc);
    weights.loinc = read_input().unwrap_or(weights.loinc);

    println!(
        "Generic condition penalty (default = {}):",
        weights.generic_condition_pen
    );
    weights.generic_condition_pen = read_input().unwrap_or(weights.generic_condition_pen);

    weights
}

fn read_input<T: std::str::FromStr>() -> Option<T> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input.trim().parse().ok()
    } else {
        None
    }
}
