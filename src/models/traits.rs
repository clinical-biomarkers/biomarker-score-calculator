pub trait BiomarkerData {
    // Associated types, means that whatever type is used for `Component`
    // must implement the `ComponentData` trait, and its `Evidence` type
    // must be the same as the `Evidence` type used in the struct
    type Component: ComponentData<Evidence = Self::Evidence>;
    type Evidence: EvidenceData;

    fn biomarker_id(&self) -> &str;
    fn biomarker_components(&self) -> &[Self::Component];
    fn condition_id(&self) -> &str;
    fn evidence_sources(&self) -> &[Self::Evidence];
}

pub trait ComponentData {
    type Evidence: EvidenceData;
    type Specimen: SpecimenData;

    fn evidence_source(&self) -> &[Self::Evidence];
    fn specimen(&self) -> &[Self::Specimen];
}

pub trait EvidenceData {
    fn id(&self) -> &str;
    fn database(&self) -> &str;
}

pub trait SpecimenData {
    fn loinc_code(&self) -> &str;
}
