//! Experiment-owned exact catalog schema for the algebraic exactness spike.
//!
//! This deliberately stays separate from `crates/symplectic/src/database.rs`.
//! The spike persists exact field tags plus canonical basis coefficients
//! without changing the shared rational cache format.

use super::field::ExactOrderedField;
use super::named_field::NamedFieldTag;
use serde::Serialize;

/// One exact field element serialized by canonical basis coefficients.
#[derive(Clone, Debug, Serialize)]
pub struct ElementRecord {
    pub coeffs: Vec<String>,
}

impl ElementRecord {
    /// Serialize one exact field element by its canonical coefficient vector.
    pub fn from_field<F: ExactOrderedField>(value: &F) -> Self {
        Self {
            coeffs: value
                .canonical_coeffs()
                .into_iter()
                .map(|coeff| format!("{}/{}", coeff.numer(), coeff.denom()))
                .collect(),
        }
    }
}

/// One exact polytope row in the experiment-owned exact catalog.
#[derive(Clone, Debug, Serialize)]
pub struct ExactPolytopeCatalogRow {
    pub name: String,
    pub field: NamedFieldTag,
    pub field_description: String,
    pub basis: Vec<String>,
    pub facet_count: usize,
    pub vertex_count: usize,
    pub dual_vertices: Vec<[ElementRecord; 4]>,
    pub vertices: Vec<[ElementRecord; 4]>,
    pub has_zero_omega: bool,
}

/// One selected exact-KKT comparison row.
#[derive(Clone, Debug, Serialize)]
pub struct ExactKktComparisonRow {
    pub name: String,
    pub field: NamedFieldTag,
    pub sigma_label: String,
    pub sigma: Vec<usize>,
    pub q_exact: ElementRecord,
    pub q_exact_f64: f64,
    pub action_exact_f64: f64,
    pub beta_f64: Vec<f64>,
    pub reference_source: String,
    pub reference_q_f64: Option<f64>,
    pub abs_diff_vs_reference: Option<f64>,
}
