//! Models Module

use crate::defaults::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;

pub mod full_models;
pub mod minimum_models;
pub mod traits;

#[derive(Deserialize)]
pub struct Weights {
    pub clinical_use: Option<i32>,
    pub first_pmid: Option<i32>,
    pub other_pmid: Option<f64>,
    pub pmid_limit: Option<usize>,
    pub first_source: Option<i32>,
    pub other_source: Option<f64>,
    pub loinc: Option<i32>,
    pub generic_condition_pen: Option<i32>,
    pub generic_conditions: Option<HashSet<String>>,
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            clinical_use: Some(CLINICAL_USE),
            first_pmid: Some(FIRST_PMID),
            other_pmid: Some(OTHER_PMID),
            pmid_limit: Some(PMID_LIMIT),
            first_source: Some(FIRST_SOURCE),
            other_source: Some(OTHER_SOURCE),
            loinc: Some(LOINC),
            generic_condition_pen: Some(GENERIC_CONDITION_PEN),
            generic_conditions: Some(GENERIC_CONDITIONS.iter().map(|&s| s.to_owned()).collect()),
        }
    }
}

impl Weights {
    // Merges the overrides with the default values
    pub fn with_defaults(overrides: Option<&Weights>) -> Self {
        let default_weights = Weights::default();

        Weights {
            clinical_use: overrides
                .and_then(|w| w.clinical_use)
                .or(default_weights.clinical_use),
            first_pmid: overrides
                .and_then(|w| w.first_pmid)
                .or(default_weights.first_pmid),
            other_pmid: overrides
                .and_then(|w| w.other_pmid)
                .or(default_weights.other_pmid),
            pmid_limit: overrides
                .and_then(|w| w.pmid_limit)
                .or(default_weights.pmid_limit),
            first_source: overrides
                .and_then(|w| w.first_source)
                .or(default_weights.first_source),
            other_source: overrides
                .and_then(|w| w.other_source)
                .or(default_weights.other_source),
            loinc: overrides.and_then(|w| w.loinc).or(default_weights.loinc),
            generic_condition_pen: overrides
                .and_then(|w| w.generic_condition_pen)
                .or(default_weights.generic_condition_pen),
            generic_conditions: overrides
                .and_then(|w| w.generic_conditions.clone())
                .or(default_weights.generic_conditions),
        }
    }
}

pub fn get_weights_overrides(overrides_file: Option<&String>) -> Weights {
    if let Some(path) = overrides_file {
        let file_contents = fs::read_to_string(path).expect("Could not read overrides file.");
        let overrides =
            serde_json::from_str(&file_contents).expect("Error parsing overrides file.");
        Weights::with_defaults(Some(&overrides))
    } else {
        Weights::with_defaults(None)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct BiomarkerScore {
    pub score: f64,
    pub score_info: ScoreInfo,
}

#[derive(Serialize, Debug, Clone)]
pub struct ScoreContribution {
    pub c: String,
    pub w: f64,
    pub f: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct ScoreInfo {
    pub contributions: Vec<ScoreContribution>,
    pub formula: String,
    pub variables: HashMap<String, String>,
    pub custom_rules: Option<Vec<CustomRuleApplication>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CustomRuleApplication {
    pub rule_name: String,
    pub condition: CustomCondition,
    pub action: String,
    pub effect: f64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum CustomCondition {
    Simple(String),
    And(Vec<CustomCondition>),
    Or(Vec<CustomCondition>),
}
