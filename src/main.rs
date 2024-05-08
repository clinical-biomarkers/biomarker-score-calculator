use glob::glob;
use models::{Biomarker, BiomarkerScore};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::{fs, io, process};

pub mod models;

fn main() {
    let glob_pattern = "./src/data/*.json";
    let mut score_map = HashMap::new();
    let weights = get_user_weights();

    let start_time = Instant::now();

    for file in glob(glob_pattern).expect("Failed to read glob pattern.") {
        match file {
            Ok(path) => {
                let contents = fs::read_to_string(path).expect("Could not read file.");
                let biomarkers: Vec<Biomarker> =
                    serde_json::from_str(&contents).expect("Error parsing JSON.");
                for biomarker in biomarkers {
                    let score = calculate_score(&biomarker, &weights);
                    let biomarker_score = BiomarkerScore {
                        biomarker_id: biomarker.biomarker_id.clone(),
                        biomarker_score: score,
                    };
                    score_map.insert(biomarker.biomarker_id, biomarker_score);
                }
            }
            Err(e) => println!("Error processing file: {:?}", e),
        }
    }
    if score_map.is_empty() {
        println!("Unexpected error, no scores were computed. Check JSON data and glob pattern.");
        process::exit(1);
    }
    let duration = start_time.elapsed();
    println!(
        "Scores computed for {} biomarkers. Took {:?} seconds.",
        score_map.len(),
        duration.as_secs_f64()
    );

    let output_file = "score_outputs.json";
    let biomarker_scores: Vec<_> = score_map.values().collect();
    let serialized_data =
        serde_json::to_string_pretty(&biomarker_scores).expect("Error serializing output data.");
    fs::write(output_file, serialized_data).expect("Error writing to output file.");
}

fn get_user_weights() -> Weights {
    let mut weights = Weights::default();
    println!("Enter weight/configuration override or press Enter to use default value:");

    println!("Clinical use (default = {}):", weights.clinical_use);
    weights.clinical_use = read_input().unwrap_or(weights.clinical_use);

    println!("First PMID (default = {}):", weights.first_pmid);
    weights.first_pmid = read_input().unwrap_or(weights.first_pmid);

    println!("Other PMID (default = {}):", weights.other_pmid);
    weights.other_pmid = read_input().unwrap_or(weights.other_pmid);

    println!("PMID Limit (default = {}):", weights.other_pmid);
    weights.other_pmid = read_input().unwrap_or(weights.other_pmid);

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

struct Weights {
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

fn calculate_score(biomarker: &Biomarker, weights: &Weights) -> f64 {
    let mut score = 0.0;
    let mut unique_pmids = HashSet::new();

    // handle top level evidence sources
    for evidence in &biomarker.evidence_source {
        if evidence.database.to_lowercase() == "pubmed" {
            if unique_pmids.insert(&evidence.id) {
                if unique_pmids.len() == 1 {
                    score += weights.first_pmid as f64;
                } else if unique_pmids.len() < weights.pmid_limit {
                    score += weights.other_pmid;
                }
            }
        }
    }

    // check for generic condition penalty
    if weights.generic_conditions.contains(&biomarker.condition.id) {
        score += weights.generic_condition_pen as f64;
    }

    // handle biomarker component scoring criteria
    for component in &biomarker.biomarker_component {
        // handle component evidence sources
        for evidence in &component.evidence_source {
            if evidence.database.to_lowercase() == "pubmed" {
                if unique_pmids.insert(&evidence.id) {
                    if unique_pmids.len() == 1 {
                        score += weights.first_pmid as f64;
                    } else if unique_pmids.len() < weights.pmid_limit {
                        score += weights.other_pmid;
                    }
                }
            }
        }
        // handle loinc code
        for specimen in &component.specimen {
            if !specimen.loinc_code.is_empty() {
                score += weights.loinc as f64;
                break;
            }
        }
    }

    // round negative score back up to zero
    if score < 0.0 {
        score = 0.0
    }

    Decimal::from_f64_retain(score)
        .unwrap()
        .round_dp(2)
        .to_f64()
        .unwrap()
}
