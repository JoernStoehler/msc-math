use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::evaluator::Evaluation;
use crate::manifest::CmaScaleMode;
use crate::quotient::{add_flat_direction, l2_norm, quotient_basis, QuotientBasis};
use crate::schema::AlgorithmStateRow;
use nalgebra::{DMatrix, DVector, SymmetricEigen};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde_json::json;

const MIN_COVARIANCE_EIGENVALUE: f64 = 1.0e-12;
const MAX_COVARIANCE_EIGENVALUE: f64 = 1.0e12;

#[derive(Clone, Debug)]
struct Candidate {
    x: DVector<f64>,
    y: DVector<f64>,
}

pub struct CmaEs {
    anchor: Evaluation,
    anchor_norm: f64,
    quotient: QuotientBasis,
    rng: ChaCha8Rng,
    mean: DVector<f64>,
    covariance: DMatrix<f64>,
    evolution_covariance: DVector<f64>,
    evolution_sigma: DVector<f64>,
    sigma: f64,
    scale_mode: CmaScaleMode,
    population_size: usize,
    minimum_sigma: f64,
    maximum_sigma: f64,
    generation: usize,
    pending: Vec<Candidate>,
    done: Option<String>,
}

impl CmaEs {
    pub fn new(
        seed: u64,
        initial: &Evaluation,
        sigma: f64,
        population_size: usize,
        minimum_sigma: f64,
        maximum_sigma: f64,
        scale_mode: CmaScaleMode,
    ) -> Result<Self, String> {
        if !initial.row.usable_by_optimizer {
            return Err("CMA-ES requires a usable initial point".to_string());
        }
        let quotient = quotient_basis(&initial.duals)?;
        let dimension = quotient.slice_basis.len();
        if dimension == 0 {
            return Err("CMA-ES quotient slice has zero dimension".to_string());
        }
        let anchor_norm = l2_norm(&initial.duals);
        if !anchor_norm.is_finite() || anchor_norm <= 0.0 {
            return Err("CMA-ES initial coordinate norm is invalid".to_string());
        }
        let scale_divisor = match scale_mode {
            CmaScaleMode::PerCoordinate => 1.0,
            CmaScaleMode::NormalizedRmsDistance => (dimension as f64).sqrt(),
        };
        Ok(Self {
            anchor: initial.clone(),
            anchor_norm,
            quotient,
            rng: ChaCha8Rng::seed_from_u64(seed),
            mean: DVector::zeros(dimension),
            covariance: DMatrix::identity(dimension, dimension),
            evolution_covariance: DVector::zeros(dimension),
            evolution_sigma: DVector::zeros(dimension),
            sigma: sigma / scale_divisor,
            scale_mode,
            population_size,
            minimum_sigma: minimum_sigma / scale_divisor,
            maximum_sigma: maximum_sigma / scale_divisor,
            generation: 0,
            pending: Vec::new(),
            done: None,
        })
    }

    fn coordinate_to_duals(&self, coordinate: &DVector<f64>) -> Vec<nalgebra::Vector4<f64>> {
        let mut ambient = DVector::zeros(self.anchor.duals.len() * 4);
        for (axis, coefficient) in self.quotient.slice_basis.iter().zip(coordinate.iter()) {
            ambient += axis * *coefficient;
        }
        add_flat_direction(&self.anchor.duals, &ambient, self.anchor_norm)
    }

    fn covariance_factors(&mut self) -> (DMatrix<f64>, DMatrix<f64>, f64, f64) {
        let symmetric = (&self.covariance + self.covariance.transpose()) * 0.5;
        let eigen = SymmetricEigen::new(symmetric);
        let clamped = eigen
            .eigenvalues
            .map(|value| value.clamp(MIN_COVARIANCE_EIGENVALUE, MAX_COVARIANCE_EIGENVALUE));
        let square_root = clamped.map(f64::sqrt);
        let inverse_square_root = square_root.map(|value| 1.0 / value);
        let transform = &eigen.eigenvectors * DMatrix::from_diagonal(&square_root);
        let inverse_transform = &eigen.eigenvectors
            * DMatrix::from_diagonal(&inverse_square_root)
            * eigen.eigenvectors.transpose();
        self.covariance = &transform * transform.transpose();
        (transform, inverse_transform, clamped.min(), clamped.max())
    }
}

impl Optimizer for CmaEs {
    fn ask(&mut self, remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() || remaining_budget == 0 {
            return Ok(Vec::new());
        }
        let count = self.population_size.min(remaining_budget);
        let (transform, _, minimum_eigenvalue, maximum_eigenvalue) = self.covariance_factors();
        let mean_duals = self.coordinate_to_duals(&self.mean);
        self.pending.clear();
        let mut proposals = Vec::with_capacity(count);
        for population_index in 0..count {
            let standard =
                DVector::from_fn(self.mean.len(), |_, _| StandardNormal.sample(&mut self.rng));
            let y = &transform * standard;
            let x = &self.mean + self.sigma * &y;
            let duals = self.coordinate_to_duals(&x);
            proposals.push(Proposal {
                duals,
                baseline_evaluation_id: None,
                geometric_reference_kind: Some("cma_mean".to_string()),
                geometric_reference_duals: Some(mean_duals.clone()),
                fields: json!({
                    "generation": self.generation,
                    "population_index": population_index,
                    "population_count": count,
                    "configured_population_size": self.population_size,
                    "sigma": self.sigma,
                    "scale_mode": self.scale_mode,
                    "normalized_rms_distance": self.sigma * (self.mean.len() as f64).sqrt(),
                    "mean_coordinate_norm": self.mean.norm(),
                    "sample_coordinate_norm": x.norm(),
                    "sample_standardized_norm": y.norm(),
                    "covariance_min_eigenvalue": minimum_eigenvalue,
                    "covariance_max_eigenvalue": maximum_eigenvalue,
                }),
            });
            self.pending.push(Candidate { x, y });
        }
        Ok(proposals)
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        if observations.is_empty() || observations.len() > self.pending.len() {
            return Err(format!(
                "CMA-ES received {} observations for {} pending candidates",
                observations.len(),
                self.pending.len()
            ));
        }
        // A measured-compute budget can end a population after only a prefix
        // was evaluated. The pending candidates and observations share order,
        // so updating from that prefix is well-defined and avoids charging a
        // complete generation after the budget was exhausted.
        self.pending.truncate(observations.len());
        let sigma_before = self.sigma;
        let mut ranked = observations
            .iter()
            .enumerate()
            .filter_map(|(index, observation)| {
                observation
                    .evaluation
                    .row
                    .usable_by_optimizer
                    .then_some((index, observation.evaluation.row.sys?))
            })
            .collect::<Vec<_>>();
        ranked.sort_by(|left, right| right.1.total_cmp(&left.1));
        let selected_count = (self.population_size / 2).min(ranked.len());
        if selected_count < 2 {
            self.sigma *= 0.5;
            self.generation += 1;
            if self.sigma < self.minimum_sigma {
                self.done = Some("minimum_sigma".to_string());
            }
            return Ok(TellOutcome {
                selected: Vec::new(),
                stop_reason: self.done.clone(),
                fields: json!({
                    "generation": self.generation - 1,
                    "valid_count": ranked.len(),
                    "update_status": "insufficient_valid_candidates",
                    "sigma_before": sigma_before,
                    "sigma_after": self.sigma,
                    "scale_mode": self.scale_mode,
                    "normalized_rms_distance_after": self.sigma * (self.mean.len() as f64).sqrt(),
                }),
            });
        }
        let mut weights = (1..=selected_count)
            .map(|rank| ((selected_count as f64) + 0.5).ln() - (rank as f64).ln())
            .collect::<Vec<_>>();
        let weight_sum = weights.iter().sum::<f64>();
        for weight in &mut weights {
            *weight /= weight_sum;
        }
        let mu_eff = 1.0 / weights.iter().map(|weight| weight * weight).sum::<f64>();
        let dimension = self.mean.len() as f64;
        let cc = (4.0 + mu_eff / dimension) / (dimension + 4.0 + 2.0 * mu_eff / dimension);
        let cs = (mu_eff + 2.0) / (dimension + mu_eff + 5.0);
        let c1 = 2.0 / ((dimension + 1.3).powi(2) + mu_eff);
        let cmu = (1.0 - c1)
            .min(2.0 * (mu_eff - 2.0 + 1.0 / mu_eff) / ((dimension + 2.0).powi(2) + mu_eff));
        let damping =
            1.0 + 2.0 * (0.0f64.max(((mu_eff - 1.0) / (dimension + 1.0)).sqrt() - 1.0)) + cs;
        let old_mean = self.mean.clone();
        self.mean.fill(0.0);
        for ((index, _), weight) in ranked.iter().take(selected_count).zip(&weights) {
            self.mean += &self.pending[*index].x * *weight;
        }
        let weighted_step = (&self.mean - &old_mean) / self.sigma;
        let (_, inverse_sqrt_covariance, _, _) = self.covariance_factors();
        self.evolution_sigma = (1.0 - cs) * &self.evolution_sigma
            + (cs * (2.0 - cs) * mu_eff).sqrt() * (&inverse_sqrt_covariance * &weighted_step);
        let chi_dimension =
            dimension.sqrt() * (1.0 - 1.0 / (4.0 * dimension) + 1.0 / (21.0 * dimension.powi(2)));
        let generation_factor = (1.0 - (1.0 - cs).powi(2 * (self.generation as i32 + 1))).sqrt();
        let h_sigma = (self.evolution_sigma.norm() / generation_factor / chi_dimension)
            < 1.4 + 2.0 / (dimension + 1.0);
        self.evolution_covariance = (1.0 - cc) * &self.evolution_covariance
            + if h_sigma {
                (cc * (2.0 - cc) * mu_eff).sqrt() * &weighted_step
            } else {
                DVector::zeros(self.mean.len())
            };
        let old_covariance = self.covariance.clone();
        let rank_one = &self.evolution_covariance * self.evolution_covariance.transpose()
            + if h_sigma {
                DMatrix::zeros(self.mean.len(), self.mean.len())
            } else {
                cc * (2.0 - cc) * &old_covariance
            };
        let mut rank_mu = DMatrix::zeros(self.mean.len(), self.mean.len());
        for ((index, _), weight) in ranked.iter().take(selected_count).zip(&weights) {
            rank_mu += *weight * (&self.pending[*index].y * self.pending[*index].y.transpose());
        }
        self.covariance = (1.0 - c1 - cmu) * old_covariance + c1 * rank_one + cmu * rank_mu;
        self.sigma *= ((cs / damping) * (self.evolution_sigma.norm() / chi_dimension - 1.0)).exp();
        self.sigma = self
            .sigma
            .clamp(self.minimum_sigma * 0.5, self.maximum_sigma);
        self.generation += 1;
        if self.sigma < self.minimum_sigma {
            self.done = Some("minimum_sigma".to_string());
        } else if self.sigma >= self.maximum_sigma {
            self.done = Some("maximum_sigma".to_string());
        }
        let selected = ranked
            .iter()
            .take(selected_count)
            .zip(weights)
            .map(|((index, _), weight)| (*index, weight))
            .collect::<Vec<_>>();
        Ok(TellOutcome {
            selected,
            stop_reason: self.done.clone(),
            fields: json!({
                "generation": self.generation - 1,
                "valid_count": ranked.len(),
                "selected_count": selected_count,
                "update_status": "updated",
                "sigma_before": sigma_before,
                "sigma_after": self.sigma,
                "scale_mode": self.scale_mode,
                "normalized_rms_distance_after": self.sigma * (self.mean.len() as f64).sqrt(),
                "mean_coordinate_norm": self.mean.norm(),
                "evolution_sigma_norm": self.evolution_sigma.norm(),
                "evolution_covariance_norm": self.evolution_covariance.norm(),
                "h_sigma": h_sigma,
            }),
        })
    }

    fn is_done(&self) -> Option<String> {
        self.done.clone()
    }

    fn algorithm_state(&self) -> AlgorithmStateRow {
        AlgorithmStateRow::UnevaluatedModelOrDistribution
    }
}
