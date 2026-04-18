//! Purpose: stable serialization helpers for scalar values.
//! Context: callers persist canonical coefficient vectors and human-readable
//! field metadata instead of depending on ad hoc debug formatting.

use crate::field::OrderedField;
use num_rational::BigRational;
use serde::{Deserialize, Serialize};

/// Stable serialized representation of one scalar value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalElement {
    pub field_name: String,
    pub basis_labels: Vec<String>,
    pub coeffs: Vec<BigRational>,
}

/// Build the stable serialized representation of one scalar.
pub fn canonical_element<F: OrderedField>(value: &F) -> CanonicalElement {
    CanonicalElement {
        field_name: F::field_name().to_string(),
        basis_labels: F::basis_labels(),
        coeffs: value.canonical_coeffs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::OrderedField;
    use num_rational::BigRational;

    #[test]
    fn rational_serialization_is_single_coefficient() {
        let value = BigRational::from_frac(3, 7);
        let encoded = canonical_element(&value);
        assert_eq!(encoded.field_name, "Q");
        assert_eq!(encoded.basis_labels, vec!["1"]);
        assert_eq!(encoded.coeffs, vec![BigRational::from_frac(3, 7)]);
    }
}
