#[derive(Clone, Debug)]
pub struct F64CapacityReport {
    pub outcome: F64CapacityOutcome,
    pub sigma_count: u64,
    pub admissible_f64_count: usize,
    pub indeterminate_f64_count: usize,
    pub inadmissible_count: usize,
    pub numerical_failure_count: usize,
    pub vertex_count: usize,
    pub facets_with_definite_vertex_count: usize,
    pub facets_with_possible_vertex_count: usize,
    pub vertex_indeterminate_count: usize,
    pub near_singular_vertex_count: usize,
    pub bounded_near_singular_vertex_count: usize,
    pub ambiguous_vertex_incidence_count: usize,
    pub facet_intersection_true_count: usize,
    pub facet_intersection_false_count: usize,
    pub facet_intersection_indeterminate_count: usize,
    pub omega_indeterminate_count: usize,
    pub min_action_gap: Option<f64>,
    pub indeterminate_overlaps_best_interval: bool,
}

#[derive(Clone, Debug)]
pub enum F64CapacityOutcome {
    Success { capacity: f64, sigma: Vec<usize> },
    Failure { reason: F64FailureReason },
}

#[derive(Clone, Debug)]
pub enum F64FailureReason {
    InvalidInput,
    NoVertices,
    NoAdmissibleF64Orbit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F64CapacityMethod {
    TransitionPrunedHk,
    ProductBilliardOrHk,
}

impl F64CapacityMethod {
    pub fn label(&self) -> &'static str {
        match self {
            Self::TransitionPrunedHk => "transition_pruned_hk",
            Self::ProductBilliardOrHk => "product_billiard_or_hk",
        }
    }
}

impl F64FailureReason {
    pub(crate) fn label(&self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::NoVertices => "no_vertices",
            Self::NoAdmissibleF64Orbit => "no_admissible_f64_orbit",
        }
    }
}

impl F64CapacityOutcome {
    pub(crate) fn outcome_label(&self) -> &'static str {
        match self {
            Self::Success { .. } => "success",
            Self::Failure { .. } => "failure",
        }
    }

    pub(crate) fn failure_reason(&self) -> Option<String> {
        match self {
            Self::Success { .. } => None,
            Self::Failure { reason } => Some(reason.label().to_string()),
        }
    }

    pub(crate) fn capacity(&self) -> Option<f64> {
        match self {
            Self::Success { capacity, .. } => Some(*capacity),
            Self::Failure { .. } => None,
        }
    }

    pub(crate) fn sigma(&self) -> Option<Vec<usize>> {
        match self {
            Self::Success { sigma, .. } => Some(sigma.clone()),
            Self::Failure { .. } => None,
        }
    }
}
