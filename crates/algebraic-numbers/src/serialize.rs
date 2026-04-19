//! Purpose: stable serialization helpers for scalar values.
//! Context: callers persist canonical coefficient vectors and human-readable
//! field metadata instead of depending on ad hoc debug formatting.

use crate::field::OrderedField;
use num_rational::BigRational;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

/// Stable serialized representation of one scalar value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalElement {
    pub field_name: String,
    pub basis_labels: Vec<String>,
    #[serde(
        serialize_with = "serialize_big_rational_vec",
        deserialize_with = "deserialize_big_rational_vec"
    )]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct SerializableRational {
    numer: String,
    denom: String,
}

fn serialize_big_rational_vec<S>(coeffs: &[BigRational], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let encoded: Vec<SerializableRational> = coeffs
        .iter()
        .map(|coeff| SerializableRational {
            numer: coeff.numer().to_string(),
            denom: coeff.denom().to_string(),
        })
        .collect();
    encoded.serialize(serializer)
}

fn deserialize_big_rational_vec<'de, D>(deserializer: D) -> Result<Vec<BigRational>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded = Vec::<SerializableRational>::deserialize(deserializer)?;
    encoded
        .into_iter()
        .map(|coeff| {
            let numer = coeff
                .numer
                .parse()
                .map_err(|_| D::Error::custom("invalid rational numerator"))?;
            let denom = coeff
                .denom
                .parse()
                .map_err(|_| D::Error::custom("invalid rational denominator"))?;
            Ok(BigRational::new(numer, denom))
        })
        .collect()
}

#[cfg(test)]
#[path = "test_serialize.rs"]
mod test_serialize;
