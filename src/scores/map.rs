use crate::prelude::*;
use crate::CHECKPOINT;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

pub async fn generate_score_map(
    glob_pattern: &str,
    weights: &Weights,
    custom_rules: Option<CustomRules>,
) -> Result<HashMap<String, HashMap<String, BiomarkerScore>>, Box<dyn std::error::Error>> {
    let mut score_map = HashMap::new();
    let files = glob::glob(glob_pattern)?;

    for (idx, file) in files.enumerate() {
        let path = file?;
        if idx % CHECKPOINT == 0 {
            println!("Checkpoint reached at file index: {}", idx + 1);
        }
        process_file(&path, weights, &mut score_map, custom_rules.as_ref()).await?;
    }

    Ok(score_map)
}

async fn process_file(
    path: &Path,
    weights: &Weights,
    score_map: &mut HashMap<String, HashMap<String, BiomarkerScore>>,
    custom_rules: Option<&CustomRules>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = path.file_name().unwrap().to_string_lossy().into_owned();
    let contents = fs::read_to_string(path).await?;
    let biomarkers: Vec<MinBiomarker> = serde_json::from_str(&contents)?;

    let file_scores = score_map.entry(filename).or_insert_with(HashMap::new);
    for biomarker in biomarkers {
        let (score, score_info) = calculate_score(&biomarker, weights, custom_rules);
        file_scores.insert(
            biomarker.biomarker_id.clone(),
            BiomarkerScore { score, score_info },
        );
    }

    Ok(())
}
