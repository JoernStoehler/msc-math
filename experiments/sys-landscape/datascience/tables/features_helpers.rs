//! Small numeric helpers shared by the polytope feature-family modules.

use num_rational::BigRational;
use num_traits::ToPrimitive;

pub fn rational_to_f64(value: &BigRational) -> f64 {
    value
        .to_f64()
        .unwrap_or_else(|| panic!("cannot convert rational {value} to f64"))
}

pub fn parse_rational(token: &str) -> BigRational {
    token.parse()
        .unwrap_or_else(|e| panic!("parse rational {token}: {e}"))
}

pub fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

pub fn stats3_or_zero(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), max)
}

pub fn max_share(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    if total <= 0.0 {
        0.0
    } else {
        values.iter().copied().fold(f64::NEG_INFINITY, f64::max) / total
    }
}

pub fn fraction_at_most(values: &[f64], threshold: f64) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
    }
}
