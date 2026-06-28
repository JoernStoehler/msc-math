use crate::{nearest_neighbor_rms, MetricSpec};
use nalgebra::Vector4;

pub const SPEC: MetricSpec = MetricSpec {
    label: "nearest_neighbor_rms",
    distance,
};

pub fn distance(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> f64 {
    nearest_neighbor_rms(left, right)
}
