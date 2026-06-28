use crate::{rms_dual_norm, MetricSpec};
use nalgebra::Vector4;

pub const SPEC: MetricSpec = MetricSpec {
    label: "ordered_rms",
    distance,
};

pub fn distance(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> f64 {
    if left.len() != right.len() {
        return f64::INFINITY;
    }
    let mean_square = left
        .iter()
        .zip(right.iter())
        .map(|(left_row, right_row)| (left_row - right_row).norm_squared())
        .sum::<f64>()
        / left.len() as f64;
    let scale = 1.0_f64.max(rms_dual_norm(left)).max(rms_dual_norm(right));
    mean_square.sqrt() / scale
}
