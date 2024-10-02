use crate::models::traits::{BiomarkerData, ComponentData, EvidenceData, SpecimenData};
use crate::rules::schema::{Action, Condition, CustomRules, Field};

pub fn apply_custom_rules<B: BiomarkerData>(
    biomarker: &B,
    rules: &CustomRules,
    current_score: f64,
) -> f64 {
    let mut score = current_score;

    let mut sorted_rules = rules.rules.clone();
    sorted_rules.sort_by_key(|r| r.priority);

    for rule in sorted_rules.iter() {
        if evaluate_condition(biomarker, &rule.condition) {
            score = apply_action(score, &rule.action);
        }
    }
    score
}

fn evaluate_condition<B: BiomarkerData>(biomarker: &B, condition: &Condition) -> bool {
    match condition {
        Condition::NonPubmedEvidenceSourceMatch(value) => biomarker
            .evidence_sources()
            .iter()
            .filter(|e| e.database().to_lowercase() != "pubmed")
            .all(|e| e.database() == value),
        Condition::FieldEquals { field, value } => match_field(biomarker, field) == *value,
        Condition::FieldAllContains { field, value } => {
            match_field(biomarker, field).contains(value)
        }
        Condition::FieldSomeContains { field, value } => {
            match_field(biomarker, field).contains(value)
        }
        Condition::FieldLenGreaterThan { field, value } => {
            match_field(biomarker, field).split(',').count() as f64 > *value
        }
        Condition::FieldLenLessThan { field, value } => {
            (match_field(biomarker, field).split(',').count() as f64) < *value
        }
        Condition::FieldLenEqual { field, value } => {
            match_field(biomarker, field).split(',').count() as f64 == *value
        }
        Condition::And { conditions } => {
            conditions.iter().all(|c| evaluate_condition(biomarker, c))
        }
        Condition::Or { conditions } => conditions.iter().any(|c| evaluate_condition(biomarker, c)),
    }
}

fn apply_action(score: f64, action: &Action) -> f64 {
    match action {
        Action::SetScore(value) => *value,
        Action::AddToScore(value) => score + value,
        Action::MultiplyScore(value) => score * value,
        Action::SubtractScore(value) => score - value,
        Action::DivideScore(value) => score / value,
    }
}

fn match_field<B: BiomarkerData>(biomarker: &B, field: &Field) -> String {
    match field {
        Field::BiomarkerID => biomarker.biomarker_id().to_string(),
        Field::ComponentEvidenceSourceDatabase => biomarker
            .biomarker_components()
            .iter()
            .flat_map(|c| c.evidence_source())
            .map(|e| e.database())
            .collect::<Vec<_>>()
            .join(","),
        Field::ConditionID => biomarker.condition_id().to_string(),
        Field::TopEvidenceSourceDatabase => biomarker
            .evidence_sources()
            .iter()
            .map(|e| e.database())
            .collect::<Vec<_>>()
            .join(","),
        Field::LoincCode => biomarker
            .biomarker_components()
            .iter()
            .flat_map(|s| s.specimen())
            .map(|l| l.loinc_code())
            .collect::<Vec<_>>()
            .join(","),
    }
}