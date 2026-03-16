//! Tests for capacity_accumulator: certified/uncertain candidate tracking.
//!
//! Proposition: The accumulator correctly tracks two-tier candidates, enforces
//! the gap invariant, and returns the minimum-action certified result.
//!
//! Strategy: unit tests with mock Solution values (no real KKT solvers needed).

use crate::algorithms::capacity_accumulator::CapacityAccumulator;
use crate::kkt::{Solution, Verdict};

/// Helper: create a certified (Verdict::True) solution with given Q and beta.
fn certified_solution(q: f64, beta: Vec<f64>) -> Solution {
    let margin = beta.iter().cloned().fold(f64::INFINITY, f64::min);
    Solution {
        verdict: Verdict::True,
        q,
        beta,
        margin,
    }
}

/// Helper: create an indeterminate (Verdict::Indeterminate) solution.
fn indeterminate_solution(q: f64, beta: Vec<f64>) -> Solution {
    let margin = beta.iter().cloned().fold(f64::INFINITY, f64::min);
    Solution {
        verdict: Verdict::Indeterminate,
        q,
        beta,
        margin,
    }
}

/// Helper: create a false (Verdict::False) solution.
fn false_solution() -> Solution {
    Solution {
        verdict: Verdict::False,
        q: 0.0,
        beta: vec![],
        margin: -1.0,
    }
}

// ── Empty accumulator ──

/// Verify an empty accumulator finalizes to None.
#[test]
fn empty_accumulator_returns_none() {
    let acc = CapacityAccumulator::new();
    assert!(acc.finalize().is_none());
}

/// Verify a default-constructed accumulator has no candidates and zero iterations.
#[test]
fn default_accumulator_is_empty() {
    let acc = CapacityAccumulator::default();
    assert!(!acc.has_certified());
    assert!(!acc.has_uncertain());
    assert_eq!(acc.iterations(), 0);
}

// ── Single submission ──

/// Verify a single certified submission produces the correct capacity = 1/(2Q).
#[test]
fn single_certified_submission() {
    let mut acc = CapacityAccumulator::new();
    let q = 2.0; // action = 0.5 / 2.0 = 0.25
    let perm = vec![0, 1, 2];
    let sol = certified_solution(q, vec![0.3, 0.4, 0.3]);
    acc.submit(&perm, &sol);

    assert!(acc.has_certified());
    assert!(acc.has_uncertain());
    assert_eq!(acc.iterations(), 1);

    let result = acc.finalize().unwrap();
    assert!((result.capacity - 0.25).abs() < 1e-14);
    assert!((result.capacity_uncertain - 0.25).abs() < 1e-14);
    assert_eq!(result.best_permutation, vec![0, 1, 2]);
    assert_eq!(result.best_beta, vec![0.3, 0.4, 0.3]);
    assert_eq!(result.iterations, 1);
}

/// Verify a single indeterminate submission does not produce a certified result.
#[test]
fn single_indeterminate_does_not_certify() {
    let mut acc = CapacityAccumulator::new();
    let sol = indeterminate_solution(2.0, vec![0.0001, 0.5, 0.4999]);
    acc.submit(&[0, 1, 2], &sol);

    assert!(!acc.has_certified());
    assert!(acc.has_uncertain());
    assert!(acc.finalize().is_none());
}

// ── Verdict::False is ignored ──

/// Verify Verdict::False submissions are counted but not tracked as candidates.
#[test]
fn false_verdict_ignored() {
    let mut acc = CapacityAccumulator::new();
    acc.submit(&[0, 1], &false_solution());

    assert_eq!(acc.iterations(), 1);
    assert!(!acc.has_certified());
    assert!(!acc.has_uncertain());
}

// ── Near-zero Q is ignored ──

/// Verify submissions with Q near zero are rejected (below EPS_Q_FLOOR).
#[test]
fn near_zero_q_ignored() {
    let mut acc = CapacityAccumulator::new();
    let sol = certified_solution(1e-16, vec![0.5, 0.5]);
    acc.submit(&[0, 1], &sol);

    // Not tracked because Q <= EPS_Q_FLOOR.
    assert!(!acc.has_certified());
}

// ── Multiple submissions: best (minimum action) wins ──

/// Verify the candidate with minimum action (= 1/(2Q)) wins among multiple certified.
#[test]
fn minimum_action_wins() {
    let mut acc = CapacityAccumulator::new();

    // Q = 1.0 -> action = 0.5
    acc.submit(&[0, 1, 2], &certified_solution(1.0, vec![0.4, 0.3, 0.3]));
    // Q = 2.0 -> action = 0.25 (better, lower action)
    acc.submit(&[3, 4, 5], &certified_solution(2.0, vec![0.3, 0.4, 0.3]));
    // Q = 0.5 -> action = 1.0 (worse)
    acc.submit(&[6, 7, 8], &certified_solution(0.5, vec![0.2, 0.5, 0.3]));

    let result = acc.finalize().unwrap();
    assert!((result.capacity - 0.25).abs() < 1e-14);
    assert_eq!(result.best_permutation, vec![3, 4, 5]);
    assert_eq!(result.iterations, 3);
}

// ── Two-tier tracking: indeterminate tracked in uncertain tier ──

/// Verify indeterminate submissions are tracked in the uncertain tier alongside certified.
#[test]
fn indeterminate_tracked_in_uncertain_tier() {
    let mut acc = CapacityAccumulator::new();

    // Certified: Q = 1.0 -> action = 0.5
    acc.submit(&[0, 1], &certified_solution(1.0, vec![0.5, 0.5]));
    // Indeterminate with same Q: goes into uncertain tier but not certified.
    // (same action, so doesn't beat the certified candidate in uncertain tier)
    acc.submit(
        &[2, 3],
        &indeterminate_solution(1.0, vec![0.0001, 0.9999]),
    );

    let result = acc.finalize().unwrap();
    // Both tiers have action = 0.5 (tied).
    assert!((result.capacity - 0.5).abs() < 1e-14);
    assert!((result.capacity_uncertain - 0.5).abs() < 1e-14);
}

/// Verify a large gap between certified and uncertain tiers triggers a panic.
#[test]
fn indeterminate_with_lower_action_tracked_in_uncertain() {
    let mut acc = CapacityAccumulator::new();

    // Certified: Q = 1.0 -> action = 0.5
    acc.submit(&[0, 1], &certified_solution(1.0, vec![0.5, 0.5]));
    // Indeterminate: Q = 2.0 -> action = 0.25 (lower action, better)
    acc.submit(
        &[2, 3],
        &indeterminate_solution(2.0, vec![0.0001, 0.9999]),
    );

    // Certified tier: action = 0.5.
    // Uncertain tier: action = 0.25 (the indeterminate candidate wins).
    // Gap = 0.5 - 0.25 = 0.25 > GAP_TOLERANCE -> should panic.
    let result = std::panic::catch_unwind(|| {
        let acc_inner = {
            let mut a = CapacityAccumulator::new();
            a.submit(&[0, 1], &certified_solution(1.0, vec![0.5, 0.5]));
            a.submit(
                &[2, 3],
                &indeterminate_solution(2.0, vec![0.0001, 0.9999]),
            );
            a
        };
        acc_inner.finalize()
    });
    assert!(result.is_err(), "should panic on large gap");
}

// ── Gap invariant ──

/// Verify a tiny gap (< GAP_TOLERANCE) between tiers does not trigger a panic.
#[test]
fn tiny_gap_passes() {
    let mut acc = CapacityAccumulator::new();

    // Certified: Q = 1.0 -> action = 0.5
    acc.submit(&[0, 1], &certified_solution(1.0, vec![0.5, 0.5]));
    // Indeterminate with very slightly higher Q (slightly lower action).
    // Q = 1.0 + 1e-13 -> action = 0.5 / (1 + 1e-13) ~ 0.5 - 5e-14
    // Gap ~ 5e-14 < GAP_TOLERANCE.
    let q_uncertain = 1.0 + 1e-13;
    acc.submit(
        &[2, 3],
        &indeterminate_solution(q_uncertain, vec![1e-10, 1.0 - 1e-10]),
    );

    let result = acc.finalize().unwrap();
    assert!(result.numerical_gap().abs() < 1e-10);
}

// ── Certified candidate also enters uncertain tier ──

/// Verify certified candidates enter both certified and uncertain tiers.
#[test]
fn certified_enters_both_tiers() {
    let mut acc = CapacityAccumulator::new();

    // Submit only certified solutions.
    acc.submit(&[0, 1], &certified_solution(2.0, vec![0.5, 0.5]));
    acc.submit(&[2, 3], &certified_solution(4.0, vec![0.3, 0.7]));

    let result = acc.finalize().unwrap();
    // Best certified: Q=4 -> action=0.125
    // Best uncertain: Q=4 -> action=0.125 (same, since both are certified)
    assert!((result.capacity - 0.125).abs() < 1e-14);
    assert!((result.capacity_uncertain - 0.125).abs() < 1e-14);
    assert_eq!(result.numerical_gap(), 0.0);
}

// ── Iteration counting ──

/// Verify iteration count includes all submissions (even rejected ones).
#[test]
fn iteration_count_includes_all_submissions() {
    let mut acc = CapacityAccumulator::new();

    acc.submit(&[0, 1], &false_solution()); // rejected
    acc.submit(&[0, 1], &certified_solution(1e-16, vec![0.5, 0.5])); // Q too small
    acc.submit(&[0, 1], &certified_solution(1.0, vec![0.5, 0.5])); // accepted

    // All 3 submissions counted, even rejected ones.
    assert_eq!(acc.iterations(), 3);
    let result = acc.finalize().unwrap();
    assert_eq!(result.iterations, 3);
}

// ── Permutation is stored correctly ──

/// Verify the finalized result stores the correct permutation and beta vector.
#[test]
fn stores_correct_permutation_and_beta() {
    let mut acc = CapacityAccumulator::new();
    let perm = vec![7, 3, 1, 5];
    let beta = vec![0.1, 0.2, 0.3, 0.4];
    acc.submit(&perm, &certified_solution(10.0, beta.clone()));

    let result = acc.finalize().unwrap();
    assert_eq!(result.best_permutation, perm);
    assert_eq!(result.best_beta, beta);
}

// ── Later certified candidate replaces earlier one ──

/// Verify a later certified candidate with lower action replaces an earlier one.
#[test]
fn later_better_candidate_replaces_earlier() {
    let mut acc = CapacityAccumulator::new();

    // First: Q=1 -> action=0.5
    acc.submit(&[0, 1], &certified_solution(1.0, vec![0.5, 0.5]));
    // Second: Q=5 -> action=0.1 (better)
    acc.submit(&[2, 3], &certified_solution(5.0, vec![0.3, 0.7]));
    // Third: Q=3 -> action=0.1667 (worse than second)
    acc.submit(&[4, 5], &certified_solution(3.0, vec![0.4, 0.6]));

    let result = acc.finalize().unwrap();
    assert!((result.capacity - 0.1).abs() < 1e-14);
    assert_eq!(result.best_permutation, vec![2, 3]);
}

// ── Edge case: equal actions ──

/// Verify that equal-action candidates keep the first submitted one.
#[test]
fn equal_action_keeps_first() {
    let mut acc = CapacityAccumulator::new();

    acc.submit(&[0, 1], &certified_solution(2.0, vec![0.5, 0.5]));
    acc.submit(&[2, 3], &certified_solution(2.0, vec![0.3, 0.7]));

    let result = acc.finalize().unwrap();
    // First candidate kept (equal action is not strictly less).
    assert_eq!(result.best_permutation, vec![0, 1]);
}
