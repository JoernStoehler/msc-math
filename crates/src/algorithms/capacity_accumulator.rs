//! Certified/uncertain capacity candidate tracking across permutation enumeration.
//!
//! Extracts the shared pattern from `ehz_capacity_unpruned`, `ehz_capacity`, and
//! `billiard_capacity`: iterate over (permutation, solver) pairs, classify each
//! solution into certified/uncertain tiers by verdict, track the best (minimum
//! action) candidate in each tier, and finalize with gap/sanity assertions.
//!
//! Mathematical correspondence: [alg:ehz], [thm:billiard-characterization]
//!
//! ## Two-tier tracking
//!
//! - **Certified**: `Verdict::True` — all beta_k > +EPS. Trustworthy.
//! - **Uncertain**: `Verdict::True` or `Verdict::Indeterminate` — all beta_k > -EPS.
//!   Might be valid solutions with floating-point sign ambiguity.
//!
//! The gap invariant (certified action - uncertain action <= GAP_TOLERANCE) is
//! asserted at finalization: if an uncertain candidate achieves significantly lower
//! action than any certified candidate, the capacity cannot be resolved at f64 precision.

use crate::kkt::{Solution, Verdict};

/// Tolerance for the gap invariant: certified_action - uncertain_action.
///
/// Capacity values are O(1)--O(10), so 1e-10 is ~10 orders below typical values.
/// Near-degenerate Lagrangian products (e.g. (4,4) at theta ~ 0 degrees) produce
/// gaps up to ~2.4e-11 from f64 rounding noise.
const GAP_TOLERANCE: f64 = 1e-10;

/// Minimum Q value for a solution to be tracked. Solutions with Q below this
/// threshold yield enormous action (0.5/Q) and are never competitive.
///
/// This guards against division by near-zero Q, not against invalid solutions
/// (which are already filtered by verdict).
const EPS_Q_FLOOR: f64 = 1e-15;

/// Result of a capacity computation across all enumerated permutations.
///
/// Returned by [`CapacityAccumulator::finalize`]. Contains the best certified
/// capacity (minimum action over all certified candidates) plus diagnostic fields.
#[derive(Clone, Debug)]
pub struct CapacityResult {
    /// The EHZ capacity: minimum action over all certified candidates.
    /// action = 0.5 / Q(beta).
    pub capacity: f64,
    /// Capacity from uncertain tier (certified + indeterminate candidates).
    /// Always <= capacity. If strictly less, borderline candidates exist with lower
    /// action than any certified candidate.
    pub capacity_uncertain: f64,
    /// Cyclic permutation achieving the minimum certified action.
    pub best_permutation: Vec<usize>,
    /// Beta vector at the certified optimum.
    pub best_beta: Vec<f64>,
    /// Total number of solutions submitted to the accumulator.
    pub iterations: u64,
}

impl CapacityResult {
    /// Gap between certified and uncertain capacity.
    ///
    /// Zero means high confidence. Positive means borderline candidates exist.
    pub fn numerical_gap(&self) -> f64 {
        self.capacity - self.capacity_uncertain
    }
}

/// Internal candidate representation: (action, permutation, beta).
#[derive(Clone, Debug)]
struct Candidate {
    /// action = 0.5 / Q(beta). Smaller is better (capacity = minimum action).
    action: f64,
    /// Cyclic permutation of facet indices.
    permutation: Vec<usize>,
    /// Optimal beta vector.
    beta: Vec<f64>,
}

/// Accumulates capacity candidates across a permutation enumeration.
///
/// Two tiers:
/// - Certified: `Verdict::True` (all beta_k > +EPS). Trustworthy.
/// - Uncertain: `Verdict::True` or `Verdict::Indeterminate` (all beta_k > -EPS).
///   Might be valid solutions with floating-point sign ambiguity.
///
/// `finalize()` asserts the gap invariant (certified - uncertain <= GAP_TOLERANCE)
/// and returns the best certified result.
///
/// # Usage
///
/// ```ignore
/// let mut acc = CapacityAccumulator::new();
/// for perm in permutations {
///     if let Some(solution) = solver(&perm) {
///         acc.submit(&perm, &solution);
///     }
/// }
/// let result = acc.finalize();
/// ```
pub struct CapacityAccumulator {
    /// Best candidate in the certified tier (Verdict::True only).
    best_certified: Option<Candidate>,
    /// Best candidate in the uncertain tier (Verdict::True or Indeterminate).
    best_uncertain: Option<Candidate>,
    /// Total number of solutions submitted.
    iterations: u64,
}

impl CapacityAccumulator {
    /// Create a new empty accumulator.
    pub fn new() -> Self {
        Self {
            best_certified: None,
            best_uncertain: None,
            iterations: 0,
        }
    }

    /// Submit a KKT solution for a given permutation.
    ///
    /// The solution is classified by its verdict and tracked in the appropriate
    /// tier(s). Solutions with `Verdict::False` or Q <= EPS_Q_FLOOR are ignored.
    pub fn submit(&mut self, perm: &[usize], solution: &Solution) {
        self.iterations += 1;

        // Skip infeasible solutions.
        if solution.verdict == Verdict::False {
            return;
        }

        // Skip near-zero Q: action = 0.5/Q would be enormous or undefined.
        if solution.q <= EPS_Q_FLOOR {
            return;
        }

        let action = 0.5 / solution.q;

        // Uncertain tier: True or Indeterminate (all beta_k > -EPS).
        let dominated_uncertain = self
            .best_uncertain
            .as_ref()
            .is_some_and(|b| action >= b.action);
        if !dominated_uncertain {
            self.best_uncertain = Some(Candidate {
                action,
                permutation: perm.to_vec(),
                beta: solution.beta.clone(),
            });
        }

        // Certified tier: True only (all beta_k > +EPS).
        if solution.verdict == Verdict::True {
            let dominated_certified = self
                .best_certified
                .as_ref()
                .is_some_and(|b| action >= b.action);
            if !dominated_certified {
                self.best_certified = Some(Candidate {
                    action,
                    permutation: perm.to_vec(),
                    beta: solution.beta.clone(),
                });
            }
        }
    }

    /// Finalize the accumulation and return the best certified capacity.
    ///
    /// Returns `None` if no certified candidate was found.
    ///
    /// # Panics
    ///
    /// Panics if the gap invariant is violated: the uncertain capacity is more
    /// than `GAP_TOLERANCE` below the certified capacity. This indicates an
    /// ambiguous candidate that cannot be resolved at f64 precision.
    ///
    /// Panics if the certified capacity is non-positive or non-finite.
    pub fn finalize(self) -> Option<CapacityResult> {
        let certified = self.best_certified?;
        let uncertain_action = self
            .best_uncertain
            .as_ref()
            .map_or(certified.action, |u| u.action);

        // Gap invariant: certified action >= uncertain action (since uncertain
        // is a superset), and the gap must be small.
        let gap = certified.action - uncertain_action;
        assert!(
            gap <= GAP_TOLERANCE,
            "Numerical gap: certified capacity {:.6e} > uncertain capacity {:.6e} (gap = {:.6e}). \
             An UNKNOWN candidate achieves lower action than the best certified candidate. \
             Cannot resolve at f64 precision.",
            certified.action,
            uncertain_action,
            gap,
        );

        // Sanity: capacity must be positive and finite.
        assert!(
            certified.action > 0.0,
            "capacity must be positive, got {:.2e}",
            certified.action
        );
        assert!(
            certified.action.is_finite(),
            "capacity must be finite, got {:.2e}",
            certified.action
        );

        Some(CapacityResult {
            capacity: certified.action,
            capacity_uncertain: uncertain_action,
            best_permutation: certified.permutation,
            best_beta: certified.beta,
            iterations: self.iterations,
        })
    }

    /// Number of solutions submitted so far.
    pub fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Whether any certified candidate has been found.
    pub fn has_certified(&self) -> bool {
        self.best_certified.is_some()
    }

    /// Whether any uncertain candidate has been found.
    pub fn has_uncertain(&self) -> bool {
        self.best_uncertain.is_some()
    }
}

impl Default for CapacityAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
