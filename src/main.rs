use glob::glob;
use models::{get_user_weights, Biomarker, BiomarkerScore, ScoreContribution, ScoreInfo, Weights};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::env;
use std::time::Instant;
use std::{fs, process};

pub mod models;

fn main() {
    let args: Vec<String> = env::args().collect();
    let glob_pattern = args
        .get(1)
        .unwrap_or(&"./src/data/*.json".to_string())
        .clone();
    let mut score_map: HashMap<String, HashMap<String, BiomarkerScore>> = HashMap::new();
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
                    let (score, score_info) = calculate_score(&biomarker, &weights);
                    file_scores.insert(
                        biomarker.biomarker_id.clone(),
                        BiomarkerScore { score, score_info },
                    );
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

fn calculate_score(biomarker: &Biomarker, weights: &Weights) -> (f64, ScoreInfo) {
    let mut score = 0.0;
    let mut unique_pmids = HashSet::new();
    let mut unique_sources = HashSet::new();
    let mut contributions = Vec::new();

    // chain the evidence iterators together for reduced redundancy
    let all_evidence = biomarker.evidence_source.iter().chain(
        biomarker
            .biomarker_component
            .iter()
            .flat_map(|component| component.evidence_source.iter()),
    );

    let mut first_pmid_count = 0;
    let mut other_pmid_count = 0;
    let mut first_source_count = 0;
    let mut other_source_count = 0;

    for evidence in all_evidence {
        let is_pubmed = evidence.database.to_lowercase().trim() == "pubmed";
        let unique_set = if is_pubmed {
            &mut unique_pmids
        } else {
            &mut unique_sources
        };

        if unique_set.insert(&evidence.id) {
            if is_pubmed {
                if unique_pmids.len() == 1 {
                    score += weights.first_pmid as f64;
                    first_pmid_count += 1;
                } else if unique_pmids.len() <= weights.pmid_limit {
                    score += weights.other_pmid;
                    other_pmid_count += 1;
                }
            } else {
                if unique_sources.len() == 1 {
                    score += weights.first_source as f64;
                    first_source_count += 1;
                } else {
                    score += weights.other_source;
                    other_source_count += 1;
                }
            }
        }
    }

    contributions.push(ScoreContribution {
        c: "first_pmid".to_string(),
        w: weights.first_pmid as f64,
        f: first_pmid_count as f64,
    });
    contributions.push(ScoreContribution {
        c: "other_pmid".to_string(),
        w: weights.other_pmid,
        f: other_pmid_count as f64,
    });
    contributions.push(ScoreContribution {
        c: "first_source".to_string(),
        w: weights.first_source as f64,
        f: first_source_count as f64,
    });
    contributions.push(ScoreContribution {
        c: "other_source".to_string(),
        w: weights.other_source,
        f: other_source_count as f64,
    });

    // check for generic condition penalty
    let mut generic_condition_count = 0;
    if weights.generic_conditions.contains(&biomarker.condition.id) {
        score += weights.generic_condition_pen as f64;
        generic_condition_count += 1;
    }
    contributions.push(ScoreContribution {
        c: "generic_condition_pen".to_string(),
        w: weights.generic_condition_pen as f64,
        f: generic_condition_count as f64,
    });

    // handle loinc scoring criteria
    let mut loinc_count = 0;
    if biomarker.biomarker_component.iter().any(|component| {
        component
            .specimen
            .iter()
            .any(|specimen| !specimen.loinc_code.is_empty())
    }) {
        score += weights.loinc as f64;
        loinc_count += 1;
    }
    contributions.push(ScoreContribution {
        c: "loinc".to_string(),
        w: weights.loinc as f64,
        f: loinc_count as f64,
    });

    // round negative score back up to zero
    // now support negative scores
    // score = score.max(0.0);

    let score = Decimal::from_f64_retain(score)
        .unwrap()
        .round_dp(2)
        .to_f64()
        .unwrap();

    let score_info = ScoreInfo {
        contributions,
        formula: "sum(w*f)".to_string(),
        variables: [
            ("c".to_string(), "condition".to_string()),
            ("w".to_string(), "weight".to_string()),
            ("f".to_string(), "frequency".to_string()),
        ]
        .iter()
        .cloned()
        .collect(),
    };

    (score, score_info)
}
