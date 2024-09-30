//! Minimum Models Module
//!
//! The minimum viable models for calculating scores. Used for 
//! generating the external score maps in a synchronous fashion. 
//! Has a reduced memory footprint.

use super::traits::{BiomarkerData, ComponentData, EvidenceData, SpecimenData};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Biomarker {
    pub biomarker_id: String,
    pub biomarker_component: Vec<Component>,
    pub condition: Condition,
    pub evidence_source: Vec<Evidence>,
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

#[derive(Deserialize, Debug)]
pub struct Component {
    pub specimen: Vec<Specimen>,
    pub evidence_source: Vec<Evidence>,
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

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Evidence {
    pub id: String,
    pub database: String,
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

#[derive(Deserialize, Debug)]
pub struct Specimen {
    pub id: String,
    pub loinc_code: String,
}

impl SpecimenData for Specimen {
    fn loinc_code(&self) -> &str {
        &self.loinc_code
    }
}
