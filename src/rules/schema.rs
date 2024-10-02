//! Schema Module
//!
//! This module sets the schema for the custom rules format and engine parsing.

use serde::Deserialize;

/// The top level custom rule structure.
#[derive(Deserialize)]
pub struct CustomRules {
    /// The list of rules.
    pub rules: Vec<Rule>,
}

/// The schema for a single rule.
#[derive(Deserialize, Clone)]
pub struct Rule {
    /// The rule name, just a concise and descriptive string.
    pub name: String,
    /// The rule condition for it to be applied.
    pub condition: Condition,
    /// The action the rule should take when the condition is met.
    pub action: Action,
    /// The rule priority (in case of rule conflict).
    pub priority: i32,
}

/// The condition for the rule to be applied.
#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Condition {
    NonPubmedEvidenceSourceMatch(String),
    FieldEquals { field: Field, value: String },
    FieldAllContains { field: Field, value: String },
    FieldSomeContains { field: Field, value: String },
    FieldLenGreaterThan { field: Field, value: f64 },
    FieldLenLessThan { field: Field, value: f64 },
    FieldLenEqual { field: Field, value: f64 },
    And { conditions: Vec<Condition> },
    Or { conditions: Vec<Condition> },
}

/// The fields that custom rules can be applied to.
#[derive(Deserialize, Clone)]
pub enum Field {
    BiomarkerID,
    ComponentEvidenceSourceDatabase,
    ConditionID,
    TopEvidenceSourceDatabase,
    LoincCode,
}

impl Field {
    /// Returns the field path for the condition.
    pub fn as_str(&self) -> &'static str {
        match self {
            Field::BiomarkerID => "biomarker_id",
            Field::ComponentEvidenceSourceDatabase => {
                "biomarker_component.evidence_source.database"
            }
            Field::ConditionID => "condition.id",
            Field::TopEvidenceSourceDatabase => "evidence_source.database",
            Field::LoincCode => "biomarker_component.specimen.loinc_code",
        }
    }
}

/// The action to take when a condition is applied.
#[derive(Deserialize, Clone)]
#[serde(tag = "type", content = "value")]
pub enum Action {
    /// Hardcode the biomarker score.
    SetScore(f64),
    /// Add a value to the biomarker score.
    AddToScore(f64),
    /// Multiply a value to the biomarker score.
    MultiplyScore(f64),
    /// Subtract a value from the biomarker score.
    SubtractScore(f64),
    /// Divide a value from the biomarker score.
    DivideScore(f64),
}
