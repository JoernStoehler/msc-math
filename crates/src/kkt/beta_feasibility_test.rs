//! Tests for beta_feasibility: max-margin LP search for beta > 0.
//!
//! Proposition: find_max_margin returns the certified global optimum of
//! max_alpha min_j (beta0 + V * alpha)_j, with margin = min(beta) exactly.
//! Reference: Chebyshev center LP formulation.
//!
//! Strategy: fixture-based covering k=0 (trivial), k=1 (analytic), k>=2 (LP).

use super::beta_feasibility::*;
use nalgebra::{DMatrix, DVector};

// ── Helpers ──

/// Verify the critical invariant: margin = min(beta).
fn assert_margin_is_tight(result: &MarginResult) {
    let min_beta = result
        .beta
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    assert!(
        (result.margin - min_beta).abs() < 1e-12,
        "margin ({:.2e}) != min(beta) ({:.2e}), diff = {:.2e}",
        result.margin,
        min_beta,
        (result.margin - min_beta).abs()
    );
}

/// Verify beta = beta0 + V * alpha reconstruction.
fn assert_beta_reconstruction(beta0: &DVector<f64>, null_basis: &DMatrix<f64>, result: &MarginResult) {
    let expected = beta0 + null_basis * &result.alpha;
    let diff = (&result.beta - &expected).norm();
    assert!(
        diff < 1e-12,
        "beta reconstruction error: ||beta - (beta0 + V * alpha)|| = {:.2e}",
        diff
    );
}

// ── k = 0: trivial cases ──

#[test]
fn trivial_feasible() {
    let beta0 = DVector::from_vec(vec![1.0, 1.0, 1.0]);
    let v = DMatrix::zeros(3, 0);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - 1.0).abs() < 1e-14);
    assert_eq!(result.alpha.len(), 0);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn trivial_infeasible() {
    let beta0 = DVector::from_vec(vec![-1.0, 1.0, 1.0]);
    let v = DMatrix::zeros(3, 0);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - (-1.0)).abs() < 1e-14);
    assert_margin_is_tight(&result);
}

#[test]
fn trivial_single_component() {
    let beta0 = DVector::from_vec(vec![3.5]);
    let v = DMatrix::zeros(1, 0);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - 3.5).abs() < 1e-14);
    assert_margin_is_tight(&result);
}

// ── k = 1: analytic cases ──

#[test]
fn one_dim_feasible() {
    let beta0 = DVector::from_vec(vec![-1.0, 2.0]);
    let v = DMatrix::from_vec(2, 1, vec![1.0, 0.0]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn one_dim_infeasible() {
    let beta0 = DVector::from_vec(vec![-1.0, -2.0]);
    let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin < 0.0, "expected infeasible, got margin = {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn one_dim_midpoint() {
    let beta0 = DVector::from_vec(vec![0.0, 0.0]);
    let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin.abs() < 1e-14, "expected margin ~ 0, got {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn one_dim_bounded_interval() {
    let beta0 = DVector::from_vec(vec![2.0, 4.0]);
    let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - 3.0).abs() < 1e-12, "expected margin = 3, got {}", result.margin);
    assert!((result.alpha[0] - 1.0).abs() < 1e-12, "expected alpha = 1, got {}", result.alpha[0]);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn one_dim_all_directions_zero() {
    let beta0 = DVector::from_vec(vec![1.0, 2.0, 3.0]);
    let v = DMatrix::from_vec(3, 1, vec![1e-16, 0.0, 1e-17]);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - 1.0).abs() < 1e-10, "expected margin ~ 1, got {}", result.margin);
    assert_margin_is_tight(&result);
}

// ── k >= 2: multi-dimensional cases ──

#[test]
fn two_dim_feasible() {
    let beta0 = DVector::from_vec(vec![-1.0, -1.0, 3.0]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(3, 2, &[
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
    ]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
    for j in 0..3 {
        assert!(result.beta[j] > 0.0, "beta[{}] = {} should be > 0", j, result.beta[j]);
    }
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn two_dim_infeasible() {
    // sum(beta) = -15 constant, so at least one component <= -5.
    let beta0 = DVector::from_vec(vec![-5.0, -5.0, -5.0]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(3, 2, &[
        1.0,  0.0,
        0.0,  1.0,
       -1.0, -1.0,
    ]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin < 0.0, "expected infeasible, got margin = {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn two_dim_symmetric() {
    // beta = [alpha1, alpha2, -alpha1, 6 - alpha2].
    // Optimal: alpha1 = 0, alpha2 = 3 -> beta = [0, 3, 0, 3], margin = 0.
    let beta0 = DVector::from_vec(vec![0.0, 0.0, 0.0, 6.0]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(4, 2, &[
         1.0,  0.0,
         0.0,  1.0,
        -1.0,  0.0,
         0.0, -1.0,
    ]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin.abs() < 0.1, "expected margin ~ 0, got {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

// ── Cross-cutting properties ──

#[test]
fn margin_is_tight_k0() {
    let beta0 = DVector::from_vec(vec![3.0, 1.5, 7.2, 0.1]);
    let v = DMatrix::zeros(4, 0);
    let result = find_max_margin(&beta0, &v);
    assert_margin_is_tight(&result);
}

#[test]
fn margin_is_tight_k1() {
    let beta0 = DVector::from_vec(vec![1.0, 3.0, 0.5, 2.0]);
    let v = DMatrix::from_vec(4, 1, vec![0.5, -0.3, 0.8, -0.1]);
    let result = find_max_margin(&beta0, &v);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn margin_is_tight_k2() {
    let beta0 = DVector::from_vec(vec![1.0, -0.5, 2.0, 0.1, -0.3]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(5, 2, &[
        0.3,  0.1,
       -0.2,  0.5,
        0.1, -0.3,
        0.4,  0.2,
       -0.1,  0.6,
    ]);
    let result = find_max_margin(&beta0, &v);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn null_basis_empty_equals_k0() {
    let beta0 = DVector::from_vec(vec![2.0, 0.5, 1.0]);
    let v = DMatrix::zeros(3, 0);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - 0.5).abs() < 1e-14);
    assert_eq!(result.alpha.len(), 0);
    assert_margin_is_tight(&result);
}

#[test]
fn k1_optimal_is_midpoint() {
    // lo = -1, hi = 5, midpoint = 2. beta = [3, 3], margin = 3.
    let beta0 = DVector::from_vec(vec![1.0, 5.0]);
    let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
    let result = find_max_margin(&beta0, &v);

    assert!((result.margin - 3.0).abs() < 1e-12);
    assert!((result.alpha[0] - 2.0).abs() < 1e-12);
}

#[test]
fn larger_k3_feasible() {
    let beta0 = DVector::from_vec(vec![-1.0, -2.0, -1.0, 10.0, 10.0, 10.0]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(6, 3, &[
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
        0.0, 0.0, 0.0,
    ]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

#[test]
fn k1_half_bounded_lower_only() {
    let beta0 = DVector::from_vec(vec![2.0, 0.0]);
    let v = DMatrix::from_vec(2, 1, vec![0.0, 1.0]);
    let result = find_max_margin(&beta0, &v);

    assert!(result.margin > 0.0, "expected positive margin, got {}", result.margin);
    assert_margin_is_tight(&result);
    assert_beta_reconstruction(&beta0, &v, &result);
}

// ── Convenience wrapper tests ──

#[test]
fn find_feasible_beta_returns_some_when_feasible() {
    let beta0 = DVector::from_vec(vec![-1.0, -1.0, 3.0]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(3, 2, &[
        1.0, 0.0,
        0.0, 1.0,
        0.0, 0.0,
    ]);
    let result = find_feasible_beta(&beta0, &v);
    assert!(result.is_some(), "should find feasible beta");
    let beta = result.unwrap();
    assert!(beta.iter().all(|&b| b > 0.0), "all components should be positive");
}

#[test]
fn find_feasible_beta_returns_none_when_infeasible() {
    let beta0 = DVector::from_vec(vec![-5.0, -5.0, -5.0]);
    #[rustfmt::skip]
    let v = DMatrix::from_row_slice(3, 2, &[
        1.0,  0.0,
        0.0,  1.0,
       -1.0, -1.0,
    ]);
    let result = find_feasible_beta(&beta0, &v);
    assert!(result.is_none(), "should not find feasible beta");
}
