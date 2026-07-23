use crate::MetricSpec;

pub mod nearest_neighbor_rms;
pub mod ordered_rms;

pub fn all() -> Vec<MetricSpec> {
    vec![nearest_neighbor_rms::SPEC, ordered_rms::SPEC]
}
