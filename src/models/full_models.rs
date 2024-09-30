//! Full Models Module
//!
//! Full data models for overwriting source files. Essentially the
//! same as the minimum models but includes the `other: Value` field
//! so no data is lost when re-dumping the output data.

use super::traits::{BiomarkerData, ComponentData, EvidenceData, SpecimenData};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Biomarker {
    pub biomarker_id: String,
    pub biomarker_component: Vec<Component>,
    pub condition: Condition,
    pub evidence_source: Vec<Evidence>,
    #[serde(flatten)]
    pub other: Value,
}

impl BiomarkerData for Biomarker {
    type Component = Component;
    type Evidence = Evidence;

    fn biomarker_id(&self) -> &str {
        &self.biomarker_id
    }
    fn biomarker_components(&self) -> &[Self::Component] {
        &self.biomarker_component
    }
    fn condition_id(&self) -> &str {
        &self.condition.id
    }
    fn evidence_sources(&self) -> &[Self::Evidence] {
        &self.evidence_source
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Component {
    pub specimen: Vec<Specimen>,
    pub evidence_source: Vec<Evidence>,
    #[serde(flatten)]
    pub other: Value,
}

impl ComponentData for Component {
    type Evidence = Evidence;
    type Specimen = Specimen;

    fn evidence_source(&self) -> &[Self::Evidence] {
        &self.evidence_source
    }
    fn specimen(&self) -> &[Self::Specimen] {
        &self.specimen
    }
}

impl AsRef<Component> for Component {
    fn as_ref(&self) -> &Component {
        self
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Condition {
    pub id: String,
    #[serde(flatten)]
    pub other: Value,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Evidence {
    pub id: String,
    pub database: String,
    #[serde(flatten)]
    pub other: Value,
}

impl EvidenceData for Evidence {
    fn id(&self) -> &str {
        &self.id
    }
    fn database(&self) -> &str {
        &self.database
    }
}

impl AsRef<Evidence> for Evidence {
    fn as_ref(&self) -> &Evidence {
        self
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Specimen {
    pub id: String,
    pub loinc_code: String,
    #[serde(flatten)]
    pub other: Value,
}

impl SpecimenData for Specimen {
    fn loinc_code(&self) -> &str {
        &self.loinc_code
    }
}
