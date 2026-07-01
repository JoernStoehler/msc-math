//! Exact exhaustive flow-graph search over transition-pruned closed words.

use crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use crate::algorithms::flow_graph::exact_tube::{
    resolve_closed_word_exact_with_action_cutoff, validate_exact_input, ExactClosedTubeError,
    ExactClosedWordOutcome, ExactFlatTubeInput,
};
use crate::algorithms::hk2017::for_each_sigma_pruned_by_transition;
use algebraic_numbers::rank;
use nalgebra::DMatrix;
use num_rational::BigRational;
use num_traits::Signed;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactActionCutoffPolicy {
    Disabled,
    Enabled,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactFlowGraphSearchResult {
    pub capacity_action: BigRational,
    pub action_threshold: BigRational,
    pub orbits: Vec<ExactFlowGraphOrbit>,
    pub checked_word_count: usize,
    pub empty_or_no_orbit_count: usize,
    pub action_cutoff_word_count: usize,
    /// Execution metric only: how often the exact tube resolver intersected
    /// a word domain with the active action cutoff.
    pub action_cutoff_intersection_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactFlowGraphOrbit {
    pub facets: Vec<usize>,
    pub action: BigRational,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExactFlowGraphSearchError {
    InvalidActionThreshold,
    InvalidInput {
        error: ExactClosedTubeError,
    },
    UnsupportedZeroOmegaTransition {
        first: usize,
        second: usize,
    },
    WordResolution {
        sigma: Vec<usize>,
        error: ExactClosedTubeError,
    },
    UnsupportedPositiveSingular {
        sigma: Vec<usize>,
        singular_status: &'static str,
        min_action: Option<BigRational>,
        max_action: Option<BigRational>,
    },
    UnsupportedLinearlyDependentFacetPresentation {
        facets: Vec<usize>,
    },
    UnsupportedNonLengthThreeSingular {
        sigma: Vec<usize>,
        singular_status: &'static str,
    },
    NoPositiveOrbit {
        checked_word_count: usize,
        empty_or_no_orbit_count: usize,
    },
}

pub fn search_closed_orbits_exact(
    input: &ExactFlatTubeInput<'_>,
    action_threshold: BigRational,
    action_cutoff_policy: ExactActionCutoffPolicy,
) -> Result<ExactFlowGraphSearchResult, ExactFlowGraphSearchError> {
    if action_threshold.is_negative() {
        return Err(ExactFlowGraphSearchError::InvalidActionThreshold);
    }
    validate_exact_input(input)
        .map_err(|error| ExactFlowGraphSearchError::InvalidInput { error })?;
    // Zero omega on a nonempty facet pair is a structural unsupported case for
    // the current exact FG route, not a numerical tolerance issue.  Lagrangian
    // products and HKO-style product degeneracies are expected to fail here.
    validate_no_geometric_zero_omega_transition(input)?;
    validate_linear_independence_up_to_four(input)?;

    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        input.facet_intersection_is_nonempty,
        input.omega_signs,
    );
    let mut positive_orbits = Vec::new();
    let mut confirmed_best_action: Option<BigRational> = None;
    let mut empty_or_no_orbit_count = 0usize;
    let mut checked_word_count = 0usize;
    let mut action_cutoff_word_count = 0usize;
    let mut action_cutoff_intersection_count = 0u64;
    let mut search_error = None;

    for_each_sigma_pruned_by_transition(&transition_is_allowed, |sigma| {
        if search_error.is_some() {
            return;
        }
        checked_word_count += 1;
        // The theorem-facing exact search uses the HK2017 simple-minimizer
        // route plus the flow-graph linear-independence input contract. Under
        // that contract, simple words of length at most four cannot carry a
        // strict positive-time closed trajectory. Keeping this skip at the
        // search boundary avoids making capacity output depend on singular
        // zero-time closed-tube diagnostics such as length-three fixed lines.
        if sigma.len() <= 4 {
            empty_or_no_orbit_count += 1;
            return;
        }
        // The cutoff policy is an exact speed-up, not a separate certificate:
        // once a positive exact action is known, words whose whole closed-tube
        // domain lies above best+threshold cannot contribute retained output.
        // Tests compare the enabled policy against the disabled baseline.
        let action_cutoff = match action_cutoff_policy {
            ExactActionCutoffPolicy::Disabled => None,
            ExactActionCutoffPolicy::Enabled => confirmed_best_action
                .as_ref()
                .map(|best_action| best_action + &action_threshold),
        };
        if action_cutoff.is_some() {
            action_cutoff_word_count += 1;
        }
        let (result, metrics) = match resolve_closed_word_exact_with_action_cutoff(
            input,
            sigma,
            action_cutoff.as_ref(),
        ) {
            Ok((result, metrics)) => (result, metrics),
            Err(error) => {
                search_error = Some(ExactFlowGraphSearchError::WordResolution {
                    sigma: sigma.to_vec(),
                    error,
                });
                return;
            }
        };
        action_cutoff_intersection_count += metrics.action_cutoff_intersections;
        let outcome = result.outcome;
        match outcome {
            ExactClosedWordOutcome::EmptyTube | ExactClosedWordOutcome::NonStrictNoOrbit { .. } => {
                // These are exact no-orbit outcomes for the displayed strict
                // word.  They are counted for diagnostics but do not weaken an
                // earlier positive candidate.
                empty_or_no_orbit_count += 1;
            }
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some("length_three_zero_time"),
                ..
            }
            | ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: None,
                ..
            } => {
                empty_or_no_orbit_count += 1;
            }
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some(singular_status),
                ..
            } => {
                search_error = Some(
                    ExactFlowGraphSearchError::UnsupportedNonLengthThreeSingular {
                        sigma: sigma.to_vec(),
                        singular_status,
                    },
                );
            }
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                if confirmed_best_action
                    .as_ref()
                    .map(|best_action| action < *best_action)
                    .unwrap_or(true)
                {
                    confirmed_best_action = Some(action.clone());
                }
                positive_orbits.push(ExactFlowGraphOrbit {
                    facets: sigma.to_vec(),
                    action,
                });
            }
            ExactClosedWordOutcome::UnsupportedPositiveSingular {
                singular_status,
                min_action,
                max_action,
            } => {
                // Positive-action singular fixed sets are deliberately typed
                // non-success.  The exact slow path may diagnose them, but the
                // search wrapper must not turn them into capacity values.
                search_error = Some(ExactFlowGraphSearchError::UnsupportedPositiveSingular {
                    sigma: sigma.to_vec(),
                    singular_status,
                    min_action,
                    max_action,
                });
            }
        }
    });

    if let Some(error) = search_error {
        return Err(error);
    }
    let Some(capacity_action) = positive_orbits
        .iter()
        .map(|orbit| orbit.action.clone())
        .min()
    else {
        return Err(ExactFlowGraphSearchError::NoPositiveOrbit {
            checked_word_count,
            empty_or_no_orbit_count,
        });
    };
    let action_cutoff = &capacity_action + &action_threshold;
    positive_orbits.retain(|orbit| orbit.action <= action_cutoff);
    positive_orbits.sort_by(|left, right| left.action.cmp(&right.action));

    Ok(ExactFlowGraphSearchResult {
        capacity_action,
        action_threshold,
        orbits: positive_orbits,
        checked_word_count,
        empty_or_no_orbit_count,
        action_cutoff_word_count,
        action_cutoff_intersection_count,
    })
}

pub fn capacity_exact(
    input: &ExactFlatTubeInput<'_>,
    action_threshold: BigRational,
) -> Result<ExactFlowGraphSearchResult, ExactFlowGraphSearchError> {
    search_closed_orbits_exact(input, action_threshold, ExactActionCutoffPolicy::Enabled)
}

fn validate_no_geometric_zero_omega_transition(
    input: &ExactFlatTubeInput<'_>,
) -> Result<(), ExactFlowGraphSearchError> {
    for first in 0..input.facet_count() {
        for second in 0..input.facet_count() {
            if input.facet_intersection_is_nonempty[(first, second)]
                && input.omega_signs[(first, second)] == 0
            {
                return Err(ExactFlowGraphSearchError::UnsupportedZeroOmegaTransition {
                    first,
                    second,
                });
            }
        }
    }
    Ok(())
}

fn validate_linear_independence_up_to_four(
    input: &ExactFlatTubeInput<'_>,
) -> Result<(), ExactFlowGraphSearchError> {
    let facet_count = input.facet_count();
    for size in 1..=facet_count.min(4) {
        let mut subset = Vec::with_capacity(size);
        if let Some(dependent) = first_dependent_subset(input, 0, size, &mut subset) {
            return Err(
                ExactFlowGraphSearchError::UnsupportedLinearlyDependentFacetPresentation {
                    facets: dependent,
                },
            );
        }
    }
    Ok(())
}

fn first_dependent_subset(
    input: &ExactFlatTubeInput<'_>,
    start: usize,
    target_size: usize,
    subset: &mut Vec<usize>,
) -> Option<Vec<usize>> {
    if subset.len() == target_size {
        return (subset_rank(input, subset) < target_size).then(|| subset.clone());
    }

    let remaining_slots = target_size - subset.len();
    for facet in start..=input.facet_count() - remaining_slots {
        subset.push(facet);
        if let Some(dependent) = first_dependent_subset(input, facet + 1, target_size, subset) {
            return Some(dependent);
        }
        subset.pop();
    }
    None
}

fn subset_rank(input: &ExactFlatTubeInput<'_>, subset: &[usize]) -> usize {
    let matrix = DMatrix::from_fn(subset.len(), 4, |row, col| {
        input.dual_vertices[subset[row]][col].clone()
    });
    rank(&matrix)
}

#[cfg(test)]
fn exact_search_accepts_no_orbit_outcome(outcome: &ExactClosedWordOutcome) -> bool {
    matches!(
        outcome,
        ExactClosedWordOutcome::EmptyTube
            | ExactClosedWordOutcome::NonStrictNoOrbit { .. }
            | ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: None,
                ..
            }
            | ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some("length_three_zero_time"),
                ..
            }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::flow_graph::exact_tube::ExactFlatTubeInput;
    use crate::geom::known_polytopes;
    use num_traits::Zero;

    fn q(n: i64) -> BigRational {
        BigRational::from_integer(n.into())
    }

    #[test]
    fn exact_search_accepts_only_proved_singular_no_orbit_outcomes() {
        assert!(exact_search_accepts_no_orbit_outcome(
            &ExactClosedWordOutcome::ZeroActionNoOrbit {
                action: Some(q(0)),
                start_coords: None,
                singular_status: Some("length_three_zero_time"),
            }
        ));
        assert!(!exact_search_accepts_no_orbit_outcome(
            &ExactClosedWordOutcome::ZeroActionNoOrbit {
                action: Some(q(0)),
                start_coords: None,
                singular_status: Some("singular_fixed_line"),
            }
        ));
    }

    #[test]
    fn exact_search_rejects_when_short_word_skip_hypothesis_fails() {
        let dual_vertices = vec![
            [q(1), q(0), q(0), q(0)],
            [q(0), q(1), q(0), q(0)],
            [q(0), q(0), q(1), q(0)],
            [q(1), q(1), q(0), q(0)],
        ];
        let facet_intersection_is_nonempty = nalgebra::DMatrix::from_element(4, 4, false);
        let omega_signs = nalgebra::DMatrix::from_element(4, 4, 1);
        let input = ExactFlatTubeInput {
            dual_vertices: &dual_vertices,
            facet_intersection_is_nonempty: &facet_intersection_is_nonempty,
            omega_signs: &omega_signs,
        };

        assert_eq!(
            search_closed_orbits_exact(&input, q(0), ExactActionCutoffPolicy::Disabled),
            Err(
                ExactFlowGraphSearchError::UnsupportedLinearlyDependentFacetPresentation {
                    facets: vec![0, 1, 3],
                }
            )
        );
    }

    #[test]
    fn exact_search_skips_all_short_words_under_linear_independence_contract() {
        let dual_vertices = vec![
            [q(1), q(0), q(0), q(0)],
            [q(0), q(1), q(0), q(0)],
            [q(0), q(0), q(1), q(0)],
            [q(0), q(0), q(0), q(1)],
        ];
        let facet_intersection_is_nonempty = nalgebra::DMatrix::from_element(4, 4, true);
        let omega_signs = nalgebra::DMatrix::from_element(4, 4, 1);
        let input = ExactFlatTubeInput {
            dual_vertices: &dual_vertices,
            facet_intersection_is_nonempty: &facet_intersection_is_nonempty,
            omega_signs: &omega_signs,
        };

        let Err(ExactFlowGraphSearchError::NoPositiveOrbit {
            checked_word_count,
            empty_or_no_orbit_count,
        }) = search_closed_orbits_exact(&input, q(0), ExactActionCutoffPolicy::Disabled)
        else {
            panic!("short-word-only input should not report a positive orbit");
        };

        assert!(checked_word_count > 0);
        assert_eq!(checked_word_count, empty_or_no_orbit_count);
    }

    #[test]
    fn exact_search_rejects_negative_action_threshold() {
        let fixture = known_polytopes::simplex();
        let input = ExactFlatTubeInput {
            dual_vertices: &fixture.dual_vertices,
            facet_intersection_is_nonempty: &fixture.facet_intersection_is_nonempty,
            omega_signs: &fixture.omega_signs,
        };
        assert_eq!(
            search_closed_orbits_exact(&input, q(-1), ExactActionCutoffPolicy::Disabled),
            Err(ExactFlowGraphSearchError::InvalidActionThreshold)
        );
    }

    #[test]
    fn exact_search_rejects_invalid_input_shape() {
        let dual_vertices = vec![[q(1), q(0), q(0), q(0)], [q(0), q(1), q(0), q(0)]];
        let facet_intersection_is_nonempty = nalgebra::DMatrix::from_element(2, 3, true);
        let omega_signs = nalgebra::DMatrix::from_element(2, 2, 1);
        let input = ExactFlatTubeInput {
            dual_vertices: &dual_vertices,
            facet_intersection_is_nonempty: &facet_intersection_is_nonempty,
            omega_signs: &omega_signs,
        };

        assert_eq!(
            search_closed_orbits_exact(&input, q(0), ExactActionCutoffPolicy::Disabled),
            Err(ExactFlowGraphSearchError::InvalidInput {
                error: ExactClosedTubeError::InvalidInput
            })
        );
    }

    #[test]
    fn exact_search_rejects_geometric_zero_omega_transition() {
        let fixture = known_polytopes::hko_pentagon();
        assert_exact_search_rejects_zero_omega_fixture(fixture);
    }

    #[test]
    fn exact_search_rejects_lagrangian_triangle_product_zero_omega_transition() {
        let fixture = known_polytopes::lagrangian_triangle_product();
        assert_exact_search_rejects_zero_omega_fixture(fixture);
    }

    #[test]
    fn exact_search_rejects_lagrangian_triangle_square_zero_omega_transition() {
        let fixture = known_polytopes::lagrangian_triangle_square();
        assert_exact_search_rejects_zero_omega_fixture(fixture);
    }

    #[test]
    fn exact_search_rejects_when_exhaustive_search_finds_no_positive_orbit() {
        let dual_vertices = vec![[q(1), q(0), q(0), q(0)], [q(0), q(1), q(0), q(0)]];
        let facet_intersection_is_nonempty = nalgebra::DMatrix::from_element(2, 2, false);
        let omega_signs = nalgebra::DMatrix::from_element(2, 2, 1);
        let input = ExactFlatTubeInput {
            dual_vertices: &dual_vertices,
            facet_intersection_is_nonempty: &facet_intersection_is_nonempty,
            omega_signs: &omega_signs,
        };

        assert!(matches!(
            search_closed_orbits_exact(&input, q(0), ExactActionCutoffPolicy::Disabled),
            Err(ExactFlowGraphSearchError::NoPositiveOrbit { .. })
        ));
    }

    fn assert_exact_search_rejects_zero_omega_fixture(fixture: &known_polytopes::KnownPolytope) {
        let input = ExactFlatTubeInput {
            dual_vertices: &fixture.dual_vertices,
            facet_intersection_is_nonempty: &fixture.facet_intersection_is_nonempty,
            omega_signs: &fixture.omega_signs,
        };

        assert!(matches!(
            search_closed_orbits_exact(
                &input,
                BigRational::zero(),
                ExactActionCutoffPolicy::Disabled
            ),
            Err(ExactFlowGraphSearchError::UnsupportedZeroOmegaTransition { .. })
        ));
    }
}
