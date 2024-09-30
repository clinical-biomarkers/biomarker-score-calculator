use crate::models::traits::{BiomarkerData, ComponentData, EvidenceData, SpecimenData};
use crate::prelude::*;
use std::collections::HashSet;

pub fn calculate_score<B>(biomarker: &B, weights: &Weights) -> (f64, ScoreInfo)
where
    B: BiomarkerData,
    B::Evidence: AsRef<B::Evidence>,
    B::Component: AsRef<B::Component>,
{
    let mut score = 0.0;
    let mut unique_pmids = HashSet::new();
    let mut unique_sources = HashSet::new();
    let mut contributions = Vec::new();

    // Chain the evidence iterators together for reduced redundancy
    let all_evidence = biomarker.evidence_sources().iter().chain(
        biomarker
            .biomarker_components()
            .iter()
            .flat_map(|component| component.evidence_source().iter()),
    );

    let mut first_pmid_count = 0;
    let mut other_pmid_count = 0;
    let mut first_source_count = 0;
    let mut other_source_count = 0;

    for evidence in all_evidence {
        let is_pubmed = evidence.database().to_lowercase().trim() == "pubmed";
        let unique_set = if is_pubmed {
            &mut unique_pmids
        } else {
            &mut unique_sources
        };

        let evidence_id = evidence.id().to_owned();
        if unique_set.insert(evidence_id) {
            if is_pubmed {
                if unique_pmids.len() == 1 {
                    score += weights.first_pmid.unwrap_or(FIRST_PMID) as f64;
                    first_pmid_count += 1;
                } else if unique_pmids.len() <= weights.pmid_limit.unwrap_or(PMID_LIMIT) {
                    score += weights.other_pmid.unwrap_or(OTHER_PMID);
                    other_pmid_count += 1;
                }
            } else {
                if unique_sources.len() == 1 {
                    score += weights.first_source.unwrap_or(FIRST_SOURCE) as f64;
                    first_source_count += 1;
                } else {
                    score += weights.other_source.unwrap_or(OTHER_SOURCE);
                    other_source_count += 1;
                }
            }
        }
    }

    contributions.push(ScoreContribution {
        c: "first_pmid".to_string(),
        w: weights.first_pmid.unwrap_or(FIRST_PMID) as f64,
        f: first_pmid_count as f64,
    });
    contributions.push(ScoreContribution {
        c: "other_pmid".to_string(),
        w: weights.other_pmid.unwrap_or(OTHER_PMID),
        f: other_pmid_count as f64,
    });
    contributions.push(ScoreContribution {
        c: "first_source".to_string(),
        w: weights.first_source.unwrap_or(FIRST_SOURCE) as f64,
        f: first_source_count as f64,
    });
    contributions.push(ScoreContribution {
        c: "other_source".to_string(),
        w: weights.other_source.unwrap_or(OTHER_SOURCE),
        f: other_source_count as f64,
    });

    // Check for generic condition penalty
    let mut generic_condition_count = 0;
    if weights
        .generic_conditions
        .clone()
        .unwrap_or(GENERIC_CONDITIONS.iter().map(|&s| s.to_owned()).collect())
        .contains(biomarker.condition_id())
    {
        score += weights
            .generic_condition_pen
            .unwrap_or(GENERIC_CONDITION_PEN) as f64;
        generic_condition_count += 1;
    }
    contributions.push(ScoreContribution {
        c: "generic_condition_pen".to_string(),
        w: weights
            .generic_condition_pen
            .unwrap_or(GENERIC_CONDITION_PEN) as f64,
        f: generic_condition_count as f64,
    });

    // Handle LOINC scoring criteria
    let mut loinc_count = 0;
    if biomarker.biomarker_components().iter().any(|component| {
        component
            .as_ref()
            .specimen()
            .iter()
            .any(|specimen| !specimen.loinc_code().is_empty())
    }) {
        score += weights.loinc.unwrap_or(LOINC) as f64;
        loinc_count += 1;
    }
    contributions.push(ScoreContribution {
        c: "loinc".to_string(),
        w: weights.loinc.unwrap_or(LOINC) as f64,
        f: loinc_count as f64,
    });

    // Round negative score back up to zero
    score = score.max(0.0);

    let score = (score * 100.0).round() / 100.0; // Round to 2 decimal places

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
