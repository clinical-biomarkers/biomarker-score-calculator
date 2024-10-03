use crate::models::traits::{BiomarkerData, ComponentData, EvidenceData, SpecimenData};
use crate::models::{CustomCondition, CustomRuleApplication};
use crate::rules::schema::{Action, Condition, CustomRules, Field};

pub fn apply_custom_rules<B: BiomarkerData>(
    biomarker: &B,
    rules: &CustomRules,
    current_score: f64,
) -> (f64, Vec<CustomRuleApplication>) {
    let mut score = current_score;
    let mut applied_rules = Vec::new();

    let mut sorted_rules = rules.rules.clone();
    sorted_rules.sort_by_key(|r| r.priority);

    for rule in sorted_rules.iter() {
        if evaluate_condition(biomarker, &rule.condition) {
            let (new_score, effect) = apply_action(score, &rule.action);
            score = new_score;
            applied_rules.push(CustomRuleApplication {
                rule_name: rule.name.clone(),
                condition: condition_to_custom_condition(&rule.condition),
                action: format!("{:?}", rule.action),
                effect,
            })
        }
    }
    (score, applied_rules)
}

fn evaluate_condition<B: BiomarkerData>(biomarker: &B, condition: &Condition) -> bool {
    match condition {
        Condition::NonPubmedEvidenceSourceMatch(value) => biomarker
            .evidence_sources()
            .iter()
            .filter(|e| e.database().to_lowercase() != "pubmed")
            .all(|e| e.database() == value),
        Condition::FieldEquals { field, value } => {
            match_field(biomarker, field).iter().all(|f| f == value)
        }
        Condition::FieldAllContains { field, value } => {
            !match_field(biomarker, field).is_empty()
                && match_field(biomarker, field)
                    .iter()
                    .all(|f| f.contains(value))
        }
        Condition::FieldSomeContains { field, value } => match_field(biomarker, field)
            .iter()
            .any(|f| f.contains(value)),
        Condition::FieldLenGreaterThan { field, value } => {
            match_field(biomarker, field).len() as f64 > *value
        }
        Condition::FieldLenLessThan { field, value } => {
            (match_field(biomarker, field).len() as f64) < *value
        }
        Condition::FieldLenEqual { field, value } => {
            match_field(biomarker, field).len() as f64 == *value
        }
        Condition::And { conditions } => {
            conditions.iter().all(|c| evaluate_condition(biomarker, c))
        }
        Condition::Or { conditions } => conditions.iter().any(|c| evaluate_condition(biomarker, c)),
    }
}

fn apply_action(score: f64, action: &Action) -> (f64, f64) {
    match action {
        Action::SetScore(value) => (*value, *value - score),
        Action::AddToScore(value) => (score + value, *value),
        Action::MultiplyScore(value) => (score * value, score * (value - 1.0)),
        Action::SubtractScore(value) => (score - value, -*value),
        Action::DivideScore(value) => (score / value, score * (1.0 / value - 1.0)),
    }
}

fn match_field<B: BiomarkerData>(biomarker: &B, field: &Field) -> Vec<String> {
    match field {
        Field::BiomarkerID => vec![biomarker.biomarker_id().to_owned()],
        Field::ComponentEvidenceSourceDatabase => biomarker
            .biomarker_components()
            .iter()
            .flat_map(|c| c.evidence_source())
            .map(|e| e.database().to_owned())
            .collect(),
        Field::ConditionID => vec![biomarker.condition_id().to_owned()],
        Field::TopEvidenceSourceDatabase => biomarker
            .evidence_sources()
            .iter()
            .map(|e| e.database().to_owned())
            .collect(),
        Field::LoincCode => biomarker
            .biomarker_components()
            .iter()
            .flat_map(|s| s.specimen())
            .map(|l| l.loinc_code().to_owned())
            .collect(),
    }
}

fn condition_to_custom_condition(condition: &Condition) -> CustomCondition {
    match condition {
        Condition::And { conditions } => CustomCondition::And(
            conditions
                .iter()
                .map(condition_to_custom_condition)
                .collect(),
        ),
        Condition::Or { conditions } => CustomCondition::Or(
            conditions
                .iter()
                .map(condition_to_custom_condition)
                .collect(),
        ),
        _ => CustomCondition::Simple(format!("{:?}", condition)),
    }
}
