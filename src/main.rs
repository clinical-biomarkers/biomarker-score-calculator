use glob::glob;
use models::Biomarker;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::collections::{HashSet, HashMap};
use std::env;
use std::time::Instant;
use std::{fs, io, process};

pub mod models;

fn main() {
    let args: Vec<String> = env::args().collect();
    let glob_pattern = args
        .get(1)
        .unwrap_or(&"./src/data/*.json".to_string())
        .clone();
    let mut score_map: HashMap<String, HashMap<String, f64>> = HashMap::new();
    let weights = get_user_weights();

    let start_time = Instant::now();

    for file in glob(&glob_pattern).expect("Failed to read glob pattern.") {
        match file {
            Ok(path) => {
                let filename = path.file_name().unwrap().to_string_lossy().into_owned();
                let contents = fs::read_to_string(path).expect("Could not read file.");
                let biomarkers: Vec<Biomarker> =
                    serde_json::from_str(&contents).expect("Error parsing JSON.");
                let file_scores = score_map.entry(filename).or_insert_with(HashMap::new);
                for biomarker in biomarkers {
                    let score = calculate_score(&biomarker, &weights);
                    file_scores.insert(biomarker.biomarker_id.clone(), score);
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
        "Scores computed for {} files. Took {:?} seconds.",
        score_map.len(),
        duration.as_secs_f64()
    );

    let output_file = "score_outputs.json";
    let serialized_data =
        serde_json::to_string_pretty(&score_map).expect("Error serializing output data.");
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
    let mut unique_sources = HashSet::new();

    // chain the evidence iterators together for reduced redundancy
    let all_evidence = biomarker.evidence_source.iter().chain(
        biomarker
            .biomarker_component
            .iter()
            .flat_map(|component| component.evidence_source.iter()),
    );

    for evidence in all_evidence {
        let is_pubmed = evidence.database.to_lowercase().trim() == "pubmed";
        let unique_set = if is_pubmed {
            &mut unique_pmids
        } else {
            &mut unique_sources
        };

        if unique_set.insert(&evidence.id) {
            score += if unique_set.len() == 1 {
                if is_pubmed {
                    weights.first_pmid as f64
                } else {
                    weights.first_source as f64
                }
            } else {
                if is_pubmed {
                    if unique_pmids.len() <= weights.pmid_limit {
                        weights.other_pmid
                    } else {
                        0.0
                    }
                } else {
                    weights.other_source
                }
            }
        }
    }

    // check for generic condition penalty
    if weights.generic_conditions.contains(&biomarker.condition.id) {
        score += weights.generic_condition_pen as f64;
    }

    // handle loinc scoring criteria
    if biomarker.biomarker_component.iter().any(|component| {
        component
            .specimen
            .iter()
            .any(|specimen| !specimen.loinc_code.is_empty())
    }) {
        score += weights.loinc as f64;
    }

    // round negative score back up to zero
    score = score.max(0.0);

    Decimal::from_f64_retain(score)
        .unwrap()
        .round_dp(2)
        .to_f64()
        .unwrap()
}
