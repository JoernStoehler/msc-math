// ── Shared route data ────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DecisionKind {
    Accept,
    Reject,
}

#[derive(Clone, Copy, Debug)]
struct Decision {
    kind: DecisionKind,
    // These legacy experiment field names have different status by guard:
    // certified radii for OutwardCertified and BatchedAnalyticEnvelope,
    // unverified estimates for EmpiricalThenExact, and None for exact
    // decisions. Do not expose them through a shared production result type.
    action: Option<f64>,
    beta_radius: Option<f64>,
    q_radius: Option<f64>,
    q_lower: Option<f64>,
    q_upper: Option<f64>,
    exact_fallback: bool,
}

#[derive(Clone, Copy, Debug)]
enum FactorKind {
    Lu,
    Lblt,
}

#[derive(Clone, Copy, Debug)]
enum GuardKind {
    OutwardCertified,
    BatchedAnalyticEnvelope,
    NormwiseAnalyticEnvelope,
    HybridAnalyticEnvelope,
    EmpiricalThenExact,
}

#[derive(Clone, Debug)]
struct FactorData {
    solution: Vec<f64>,
    inverse: DMatrix<f64>,
    positive_inertia: Option<usize>,
}

#[derive(Clone, Debug)]
struct CurvatureProposal {
    direction: Vec<f64>,
}

#[derive(Clone, Debug)]
struct Obstruction {
    labels: Vec<usize>,
    mask: u16,
}

#[derive(Clone, Debug, Default)]
struct RouteStats {
    words: usize,
    inherited_rejections: usize,
    direct_obstructions: usize,
    obstruction_proposals: usize,
    obstruction_unknown: usize,
    lblt_factorizations: usize,
    lu_factorizations: usize,
    guarded_decisions: usize,
    exact_fallbacks: usize,
    short_exact_solves: usize,
    short_interval_rejections: usize,
    accepted: usize,
    rejected: usize,
    max_beta_radius: f64,
    max_q_radius: f64,
    best_action: Option<f64>,
    best_action_lower: Option<f64>,
    best_action_upper: Option<f64>,
    elapsed: Duration,
    direct_by_length: BTreeMap<usize, usize>,
    inherited_by_length: BTreeMap<usize, usize>,
    fallback_by_length: BTreeMap<usize, usize>,
    lookup_time: Duration,
    factor_time: Duration,
    obstruction_time: Duration,
    guard_time: Duration,
    guard_phases: GuardPhaseStats,
    exact_time: Duration,
    short_exact_time: Duration,
}

#[derive(Clone, Debug, Default)]
struct GuardPhaseStats {
    entries_time: Duration,
    residual_time: Duration,
    defect_time: Duration,
    decision_time: Duration,
}

#[derive(Clone, Debug)]
struct RouteResult {
    cutoff: Option<usize>,
    long_factor: FactorKind,
    stats: RouteStats,
    decisions: Vec<DecisionKind>,
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn point(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            lo: next_down(self.lo + rhs.lo),
            hi: next_up(self.hi + rhs.hi),
        }
    }

    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }

    fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let lo = products.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = products.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self {
            lo: next_down(lo),
            hi: next_up(hi),
        }
    }

    fn abs_upper(self) -> f64 {
        next_up(self.lo.abs().max(self.hi.abs()))
    }

    fn is_valid_finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite() && self.lo <= self.hi
    }
}
