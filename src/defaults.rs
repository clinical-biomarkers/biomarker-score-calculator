//! Scoring condition defaults

pub const CLINICAL_USE: i32 = 5;
pub const FIRST_PMID: i32 = 1;
pub const OTHER_PMID: f64 = 0.2;
pub const PMID_LIMIT: usize = 10;
pub const FIRST_SOURCE: i32 = 1;
pub const OTHER_SOURCE: f64 = 0.1;
pub const LOINC: i32 = 1;
pub const GENERIC_CONDITION_PEN: i32 = -4;
pub const GENERIC_CONDITIONS: [&str; 1] = ["DOID:162"];
