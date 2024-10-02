use super::calculate::calculate_score;
use crate::prelude::*;
use serde_json::json;
use std::path::Path;
use tokio::fs;

/// Handles whether the source data is a JSON array of biomarkers or a singular biomarker record
enum SourceType {
    Single(FullBiomarker),
    Multiple(Vec<FullBiomarker>),
}

pub async fn overwrite_source_files(
    glob_pattern: &str,
    weights: &Weights,
    custom_rules: Option<CustomRules>,
) -> Result<(), Box<dyn std::error::Error>> {
    let files = glob::glob(glob_pattern)?;

    for file in files {
        let path = file?;
        process_file(&path, weights, custom_rules.as_ref()).await?;
    }

    println!("All files have been processed and overwritten.");
    Ok(())
}

async fn process_file(
    path: &Path,
    weights: &Weights,
    custom_rules: Option<&CustomRules>,
) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path).await?;
    let mut biomarker_data = deserialize_biomarker_data(&contents)?;

    match &mut biomarker_data {
        SourceType::Single(biomarker) => {
            let (score, score_info) = calculate_score(biomarker, weights, custom_rules);
            biomarker.other["score"] = json!(score);
            biomarker.other["score_info"] = json!(score_info);
        }
        SourceType::Multiple(biomarkers) => {
            for biomarker in biomarkers {
                let (score, score_info) = calculate_score(biomarker, weights, custom_rules);
                biomarker.other["score"] = json!(score);
                biomarker.other["score_info"] = json!(score_info);
            }
        }
    }

    let serialized_data = match biomarker_data {
        SourceType::Single(biomarker) => serde_json::to_string_pretty(&biomarker)?,
        SourceType::Multiple(biomarkers) => serde_json::to_string_pretty(&biomarkers)?,
    };
    fs::write(path, serialized_data).await?;

    Ok(())
}

fn deserialize_biomarker_data(contents: &str) -> Result<SourceType, Box<dyn std::error::Error>> {
    if let Ok(biomarkers) = serde_json::from_str::<Vec<FullBiomarker>>(contents) {
        Ok(SourceType::Multiple(biomarkers))
    } else {
        let biomarker = serde_json::from_str::<FullBiomarker>(contents)?;
        Ok(SourceType::Single(biomarker))
    }
}
