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

#[test]
fn rational_serialization_json_shape_is_stable() {
    let value = BigRational::from_frac(3, 7);
    let encoded = canonical_element(&value);
    let json = serde_json::to_string(&encoded).expect("serialize canonical element");
    assert_eq!(
        json,
        r#"{"field_name":"Q","basis_labels":["1"],"coeffs":[{"numer":"3","denom":"7"}]}"#
    );
}
