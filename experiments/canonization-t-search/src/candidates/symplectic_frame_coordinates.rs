use crate::{analytic_center, translate_duals, CandidateOutput, CandidateSpec};
use nalgebra::{Matrix4, Vector4};
use std::cmp::Ordering;

pub const SPEC: CandidateSpec = CandidateSpec {
    label: "symplectic_frame_coordinates",
    canonicalize,
};

const BASIS_DET_EPS: f64 = 1e-10;
const SCORE_SCALE: f64 = 1e12;
const TOP_FRAME_SETS: usize = 128;

#[derive(Clone)]
struct FrameSet {
    indices: [usize; 4],
    abs_det: f64,
}

pub fn canonicalize(duals: &[Vector4<f64>]) -> CandidateOutput {
    let (center, center_status) = analytic_center(duals);
    let shifted = if center_status == "ok" {
        match translate_duals(duals, &center) {
            Ok(translated) => translated,
            Err(_) => {
                return CandidateOutput {
                    duals: duals.to_vec(),
                    status: "translation_failed",
                };
            }
        }
    } else {
        return CandidateOutput {
            duals: duals.to_vec(),
            status: center_status,
        };
    };

    let Some(coordinates) = best_coordinate_representation(&shifted) else {
        return CandidateOutput {
            duals: shifted,
            status: "no_frame",
        };
    };

    CandidateOutput {
        duals: coordinates,
        status: "ok",
    }
}

fn best_coordinate_representation(duals: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let frame_sets = top_frame_sets(duals);
    let mut best: Option<Vec<Vector4<f64>>> = None;
    for frame_set in frame_sets {
        for permutation in permutations4(frame_set.indices) {
            let frame = Matrix4::from_columns(&[
                duals[permutation[0]],
                duals[permutation[1]],
                duals[permutation[2]],
                duals[permutation[3]],
            ]);
            let Some(frame_inverse) = frame.try_inverse() else {
                continue;
            };
            let mut coordinates = duals
                .iter()
                .map(|dual| frame_inverse * dual)
                .collect::<Vec<_>>();
            coordinates.sort_by(compare_vectors_lexicographically);
            if best
                .as_ref()
                .map(|current| compare_representations(&coordinates, current) == Ordering::Less)
                .unwrap_or(true)
            {
                best = Some(coordinates);
            }
        }
    }
    best
}

fn top_frame_sets(duals: &[Vector4<f64>]) -> Vec<FrameSet> {
    let mut frames = Vec::new();
    if duals.len() < 4 {
        return frames;
    }
    for i in 0..duals.len() {
        for j in (i + 1)..duals.len() {
            for k in (j + 1)..duals.len() {
                for l in (k + 1)..duals.len() {
                    let matrix = Matrix4::from_columns(&[duals[i], duals[j], duals[k], duals[l]]);
                    let abs_det = matrix.determinant().abs();
                    if abs_det > BASIS_DET_EPS {
                        frames.push(FrameSet {
                            indices: [i, j, k, l],
                            abs_det,
                        });
                    }
                }
            }
        }
    }
    frames.sort_by(|left, right| right.abs_det.total_cmp(&left.abs_det));
    frames.truncate(TOP_FRAME_SETS);
    frames
}

fn permutations4(indices: [usize; 4]) -> Vec<[usize; 4]> {
    let mut out = Vec::with_capacity(24);
    for a in 0..4 {
        for b in 0..4 {
            if b == a {
                continue;
            }
            for c in 0..4 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..4 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    out.push([indices[a], indices[b], indices[c], indices[d]]);
                }
            }
        }
    }
    out
}

fn compare_representations(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> Ordering {
    for (left_row, right_row) in left.iter().zip(right.iter()) {
        match compare_vectors_lexicographically(left_row, right_row) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    left.len().cmp(&right.len())
}

fn compare_vectors_lexicographically(left: &Vector4<f64>, right: &Vector4<f64>) -> Ordering {
    for index in 0..4 {
        let left_rounded = quantize(left[index]);
        let right_rounded = quantize(right[index]);
        match left_rounded.cmp(&right_rounded) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    Ordering::Equal
}

fn quantize(value: f64) -> i128 {
    (value * SCORE_SCALE).round() as i128
}
