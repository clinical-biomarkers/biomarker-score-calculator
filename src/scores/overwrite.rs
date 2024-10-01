use super::calculate::calculate_score;
use crate::prelude::*;
use serde_json::json;
use std::path::Path;
use tokio::fs;

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
    let mut biomarkers: Vec<FullBiomarker> = serde_json::from_str(&contents)?;

    for biomarker in &mut biomarkers {
        let (score, score_info) = calculate_score(biomarker, weights, custom_rules);

        // Insert score information into the existing structure
        biomarker.other["score"] = json!(score);
        biomarker.other["score_info"] = json!(score_info);
    }

    let serialized_data = serde_json::to_string_pretty(&biomarkers)?;
    fs::write(path, serialized_data).await?;

    Ok(())
}
