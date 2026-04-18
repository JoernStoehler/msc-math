//! Consumer-style API tests for the real-algebraic crate.
//!
//! These tests describe the intended external API first. They are expected to
//! fail until the crate surface is implemented.

use num_rational::BigRational;
use real_algebraic::{
    canonical_element, cmp_field, dot, max_field, min_field, solve_square, Algebraic, OrderedField,
    Sign, SolveResult, StaticFieldSpec, TanPiFifth,
};

struct SqrtTwo;

impl StaticFieldSpec for SqrtTwo {
    fn name() -> &'static str {
        "Q(sqrt(2))"
    }

    fn generator_name() -> &'static str {
        "s"
    }

    fn minimal_polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-2).into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(1.into()),
        ]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (
            BigRational::from_integer(1.into()),
            BigRational::from_integer(2.into()),
        )
    }
}

type SqrtTwoField = Algebraic<SqrtTwo>;
type TanPiFifthField = Algebraic<TanPiFifth>;

#[test]
fn consumer_defined_field_spec_supports_operator_arithmetic() {
    let s = SqrtTwoField::generator();
    let lhs = (SqrtTwoField::one() + s.clone()) * (SqrtTwoField::one() + s.clone());
    let rhs = SqrtTwoField::from_i64(3) + SqrtTwoField::from_i64(2) * s;
    assert_eq!(lhs, rhs);
}

#[test]
fn borrowed_operators_work_in_formula_style_code() {
    let s = SqrtTwoField::generator();
    let one = SqrtTwoField::one();
    let lhs = &one + &s;
    let rhs = &lhs * &lhs;
    let expected = SqrtTwoField::from_i64(3) + SqrtTwoField::from_i64(2) * s;
    assert_eq!(rhs, expected);
}

#[test]
fn comparison_and_sign_feel_like_ordinary_scalars() {
    let s = SqrtTwoField::generator();
    assert_eq!(s.sign(), Sign::Positive);
    assert!(SqrtTwoField::from_i64(0).is_zero());
    assert!(cmp_field(&s, &SqrtTwoField::one()).is_gt());
    assert_eq!(min_field(s.clone(), SqrtTwoField::from_i64(2)), s.clone());
    assert_eq!(max_field(SqrtTwoField::one(), s.clone()), s);
}

#[test]
fn dot_product_and_small_linear_solve_use_same_scalar_api() {
    let s = SqrtTwoField::generator();
    let dot_value = dot(
        &[SqrtTwoField::one(), s.clone()],
        &[SqrtTwoField::from_i64(3), SqrtTwoField::from_i64(2)],
    );
    assert_eq!(
        dot_value,
        SqrtTwoField::from_i64(3) + SqrtTwoField::from_i64(2) * s.clone()
    );

    let matrix = [
        [SqrtTwoField::one(), s.clone()],
        [s.clone(), SqrtTwoField::from_i64(3)],
    ];
    let rhs = [SqrtTwoField::from_i64(1), SqrtTwoField::from_i64(0)];
    let result = solve_square(&matrix, &rhs);
    let SolveResult::Unique(solution) = result else {
        panic!("expected unique solution");
    };
    let residual0 =
        matrix[0][0].clone() * solution[0].clone() + matrix[0][1].clone() * solution[1].clone();
    let residual1 =
        matrix[1][0].clone() * solution[0].clone() + matrix[1][1].clone() * solution[1].clone();
    assert_eq!(residual0, rhs[0]);
    assert_eq!(residual1, rhs[1]);
}

#[test]
fn canonical_serialization_is_stable_and_readable() {
    let s = SqrtTwoField::generator();
    let value = SqrtTwoField::from_frac(3, 2) + SqrtTwoField::from_frac(1, 3) * s;
    let encoded = canonical_element(&value);
    assert_eq!(encoded.field_name, "Q(sqrt(2))");
    assert_eq!(encoded.basis_labels, vec!["1", "s"]);
    assert_eq!(encoded.coeffs.len(), 2);

    let json = serde_json::to_string(&encoded).expect("serialize canonical element");
    let decoded = serde_json::from_str::<real_algebraic::CanonicalElement>(&json)
        .expect("deserialize canonical element");
    assert_eq!(decoded, encoded);
}

#[test]
fn provided_tan_pi_fifth_field_supports_the_hko_case_shape() {
    let t = TanPiFifthField::generator();
    let t2 = t.clone() * t.clone();
    let sec36 = (TanPiFifthField::from_i64(3) - t2) / TanPiFifthField::from_i64(2);
    assert!(sec36.to_f64() > 1.0);
}
