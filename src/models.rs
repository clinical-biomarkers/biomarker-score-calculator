use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Biomarker {
    pub biomarker_id: String,
    pub biomarker_component: Vec<Component>,
    pub condition: Condition,
    pub evidence_source: Vec<Evidence>,
}

#[derive(Deserialize, Debug)]
pub struct Component {
    pub specimen: Vec<Specimen>,
    pub evidence_source: Vec<Evidence>,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Evidence {
    pub id: String,
    pub database: String,
}

#[derive(Deserialize, Debug)]
pub struct Specimen {
    pub loinc_code: String,
}

#[derive(Serialize, Debug)]
pub struct BiomarkerScore {
    biomarker_id: String,
    biomarker_score: f64,
}
