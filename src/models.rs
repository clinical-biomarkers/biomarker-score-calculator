use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

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
    pub biomarker_id: String,
    pub biomarker_score: f64,
}

impl PartialEq for BiomarkerScore {
    fn eq(&self, other: &Self) -> bool {
        self.biomarker_id == other.biomarker_id
    }
}

impl Eq for BiomarkerScore {}

impl Hash for BiomarkerScore {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.biomarker_id.hash(state);
    }
}
