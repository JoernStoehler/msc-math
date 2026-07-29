use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DistanceScheduleSpec {
    Adaptive {
        initial_distance: f64,
        expansion: f64,
        contraction: f64,
        minimum_distance: f64,
    },
    Geometric {
        initial_distance: f64,
        multiplier: f64,
        minimum_distance: f64,
    },
    FixedSequence {
        distances: Vec<f64>,
        #[serde(default)]
        repeat_last: bool,
    },
}

impl DistanceScheduleSpec {
    pub fn validate(&self, id: &str) -> Result<(), String> {
        match self {
            Self::Adaptive {
                initial_distance,
                expansion,
                contraction,
                minimum_distance,
            } => {
                validate_initial_minimum(id, *initial_distance, *minimum_distance)?;
                if !expansion.is_finite() || *expansion <= 1.0 {
                    return Err(format!("{id}: expansion must exceed one"));
                }
                if !contraction.is_finite() || !(0.0..1.0).contains(contraction) {
                    return Err(format!("{id}: contraction must lie in (0,1)"));
                }
            }
            Self::Geometric {
                initial_distance,
                multiplier,
                minimum_distance,
            } => {
                validate_initial_minimum(id, *initial_distance, *minimum_distance)?;
                if !multiplier.is_finite() || !(0.0..1.0).contains(multiplier) {
                    return Err(format!("{id}: geometric multiplier must lie in (0,1)"));
                }
            }
            Self::FixedSequence {
                distances,
                repeat_last: _,
            } => {
                if distances.is_empty()
                    || distances
                        .iter()
                        .any(|distance| !distance.is_finite() || *distance <= 0.0)
                {
                    return Err(format!(
                        "{id}: fixed distances must be nonempty, positive, and finite"
                    ));
                }
            }
        }
        Ok(())
    }
}

fn validate_initial_minimum(
    id: &str,
    initial_distance: f64,
    minimum_distance: f64,
) -> Result<(), String> {
    if !initial_distance.is_finite() || initial_distance <= 0.0 {
        return Err(format!(
            "{id}: initial distance must be positive and finite"
        ));
    }
    if !minimum_distance.is_finite()
        || minimum_distance <= 0.0
        || minimum_distance >= initial_distance
    {
        return Err(format!(
            "{id}: minimum distance must be positive and below initial distance"
        ));
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct DistanceSchedule {
    spec: DistanceScheduleSpec,
    current: f64,
    fixed_index: usize,
    done: bool,
}

impl DistanceSchedule {
    pub fn new(spec: DistanceScheduleSpec) -> Self {
        let current = match &spec {
            DistanceScheduleSpec::Adaptive {
                initial_distance, ..
            }
            | DistanceScheduleSpec::Geometric {
                initial_distance, ..
            } => *initial_distance,
            DistanceScheduleSpec::FixedSequence { distances, .. } => distances[0],
        };
        Self {
            spec,
            current,
            fixed_index: 0,
            done: false,
        }
    }

    pub fn current(&self) -> f64 {
        self.current
    }

    pub fn observe(&mut self, accepted: bool) {
        match &self.spec {
            DistanceScheduleSpec::Adaptive {
                expansion,
                contraction,
                minimum_distance,
                ..
            } => {
                self.current *= if accepted { *expansion } else { *contraction };
                self.done = self.current < *minimum_distance;
            }
            DistanceScheduleSpec::Geometric {
                multiplier,
                minimum_distance,
                ..
            } => {
                self.current *= *multiplier;
                self.done = self.current < *minimum_distance;
            }
            DistanceScheduleSpec::FixedSequence {
                distances,
                repeat_last,
            } => {
                if self.fixed_index + 1 < distances.len() {
                    self.fixed_index += 1;
                    self.current = distances[self.fixed_index];
                } else if !repeat_last {
                    self.done = true;
                }
            }
        }
    }

    pub fn contract_without_evaluation(&mut self) -> bool {
        match &self.spec {
            DistanceScheduleSpec::Adaptive {
                contraction,
                minimum_distance,
                ..
            } => {
                self.current *= *contraction;
                self.done = self.current < *minimum_distance;
                !self.done
            }
            DistanceScheduleSpec::Geometric {
                multiplier,
                minimum_distance,
                ..
            } => {
                self.current *= *multiplier;
                self.done = self.current < *minimum_distance;
                !self.done
            }
            DistanceScheduleSpec::FixedSequence {
                distances,
                repeat_last,
            } => {
                if self.fixed_index + 1 < distances.len() {
                    self.fixed_index += 1;
                    self.current = distances[self.fixed_index];
                    true
                } else {
                    self.done = !*repeat_last;
                    false
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn kind(&self) -> &'static str {
        match self.spec {
            DistanceScheduleSpec::Adaptive { .. } => "adaptive",
            DistanceScheduleSpec::Geometric { .. } => "geometric",
            DistanceScheduleSpec::FixedSequence { .. } => "fixed_sequence",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_sequence_does_not_depend_on_acceptance() {
        let spec = DistanceScheduleSpec::FixedSequence {
            distances: vec![0.1, 0.03, 0.01],
            repeat_last: false,
        };
        let mut accepted = DistanceSchedule::new(spec.clone());
        let mut rejected = DistanceSchedule::new(spec);
        for _ in 0..2 {
            accepted.observe(true);
            rejected.observe(false);
            assert_eq!(accepted.current(), rejected.current());
        }
        accepted.observe(true);
        assert!(accepted.is_done());
    }

    #[test]
    fn geometric_schedule_can_skip_a_nonpositive_model_distance() {
        let mut schedule = DistanceSchedule::new(DistanceScheduleSpec::Geometric {
            initial_distance: 0.1,
            multiplier: 0.5,
            minimum_distance: 0.01,
        });
        assert!(schedule.contract_without_evaluation());
        assert_eq!(schedule.current(), 0.05);
    }
}
