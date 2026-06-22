use crate::ScanCase;
use euclidean_polytopes::sample_random_dual_vertices_f64;
use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::{Vector2, Vector4};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const H_MIN: f64 = 0.6;
const H_MAX: f64 = 1.8;
const MAX_RAW_RANDOM_ATTEMPTS_PER_CASE: u64 = 100_000;
const MIN_PLANAR_GAP: f64 = 0.05;
const MAX_PLANAR_GAP: f64 = std::f64::consts::PI - 0.05;

pub fn generated_f64_cases(samples_per_facet: usize, seed: u64) -> Vec<ScanCase> {
    generated_f64_cases_with_source_filter(samples_per_facet, seed, &[])
}

pub fn generated_f64_cases_with_source_filter(
    samples_per_facet: usize,
    seed: u64,
    source_id_filter: &[String],
) -> Vec<ScanCase> {
    if !source_id_filter.is_empty() {
        return generated_f64_cases_for_source_ids(seed, source_id_filter);
    }
    let mut cases = Vec::new();
    for facet_count in 5..=12 {
        for sample in 0..samples_per_facet {
            cases.push(generated_random_case(facet_count, sample, seed));
        }
    }
    for q_facets in 3..=6 {
        for p_facets in 3..=6 {
            for sample in 0..samples_per_facet {
                let attempt = generated_attempt(q_facets * 100 + p_facets, sample, 0);
                cases.push(generated_product_case(q_facets, p_facets, attempt, seed));
            }
        }
    }
    cases
}

fn generated_f64_cases_for_source_ids(seed: u64, source_id_filter: &[String]) -> Vec<ScanCase> {
    let mut cases = Vec::new();
    for source_id in source_id_filter {
        let Some(spec) = GeneratedSourceId::parse(source_id, seed) else {
            continue;
        };
        let case = match spec {
            GeneratedSourceId::Random {
                facet_count,
                sample,
            } => generated_random_case(facet_count, sample, seed),
            GeneratedSourceId::Product {
                q_facets,
                p_facets,
                attempt,
            } => generated_product_case(q_facets, p_facets, attempt, seed),
        };
        if case.source_id == *source_id {
            cases.push(case);
        }
    }
    cases
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GeneratedSourceId {
    Random {
        facet_count: usize,
        sample: usize,
    },
    Product {
        q_facets: usize,
        p_facets: usize,
        attempt: u64,
    },
}

impl GeneratedSourceId {
    fn parse(source_id: &str, seed: u64) -> Option<Self> {
        let rest = source_id.strip_prefix(&format!("seed{seed}:"))?;
        if let Some(rest) = rest.strip_prefix('F') {
            let (facet_count, rest) = split_usize(rest, ":sample")?;
            let (sample, _attempt) = split_usize(rest, ":attempt")?;
            return Some(Self::Random {
                facet_count,
                sample,
            });
        }
        let rest = rest.strip_prefix('q')?;
        let (q_facets, rest) = split_usize(rest, ":p")?;
        let (p_facets, rest) = split_usize(rest, ":attempt")?;
        let attempt = rest.parse().ok()?;
        Some(Self::Product {
            q_facets,
            p_facets,
            attempt,
        })
    }
}

fn split_usize<'a>(text: &'a str, delimiter: &str) -> Option<(usize, &'a str)> {
    let (value, rest) = text.split_once(delimiter)?;
    Some((value.parse().ok()?, rest))
}

fn generated_random_case(facet_count: usize, sample: usize, seed: u64) -> ScanCase {
    let (accepted_attempt, dual_vertices) =
        sample_valid_random_dual_vertices(facet_count, sample, seed);
    ScanCase {
        family: "generated_random_f64".to_string(),
        source_id: format!("seed{seed}:F{facet_count}:sample{sample}:attempt{accepted_attempt}"),
        input_source: "generated_f64".to_string(),
        generated_attempt: Some(accepted_attempt),
        generator_seed: Some(seed),
        requested_facet_count: Some(facet_count),
        dual_vertices,
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

fn sample_valid_random_dual_vertices(
    facet_count: usize,
    sample: usize,
    seed: u64,
) -> (u64, Vec<Vector4<f64>>) {
    for rejection_index in 0..MAX_RAW_RANDOM_ATTEMPTS_PER_CASE {
        let attempt = generated_attempt(facet_count, sample, rejection_index);
        let mut rng = seeded_rng(seed, "generated_random_f64", attempt);
        let dual_vertices = sample_random_dual_vertices_f64(facet_count, H_MIN, H_MAX, &mut rng);
        if SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices.clone()).is_some() {
            return (attempt, dual_vertices);
        }
    }
    panic!(
        "failed to sample exact-valid generated_random_f64 case for F={facet_count}, sample={sample}, seed={seed}"
    );
}

fn generated_product_case(q_facets: usize, p_facets: usize, attempt: u64, seed: u64) -> ScanCase {
    let mut rng = seeded_rng(seed, "generated_product_f64", attempt);
    let q_normals = random_bounded_planar_normals(q_facets, &mut rng);
    let p_normals = random_bounded_planar_normals(p_facets, &mut rng);
    let q_height = rng.gen_range(H_MIN..H_MAX);
    let p_height = rng.gen_range(H_MIN..H_MAX);
    let mut dual_vertices = Vec::with_capacity(q_facets + p_facets);
    for normal in &q_normals {
        dual_vertices.push(Vector4::new(
            normal[0] / q_height,
            normal[1] / q_height,
            0.0,
            0.0,
        ));
    }
    for normal in &p_normals {
        dual_vertices.push(Vector4::new(
            0.0,
            0.0,
            normal[0] / p_height,
            normal[1] / p_height,
        ));
    }

    ScanCase {
        family: "generated_product_f64".to_string(),
        source_id: format!("seed{seed}:q{q_facets}:p{p_facets}:attempt{attempt}"),
        input_source: "generated_f64".to_string(),
        generated_attempt: Some(attempt),
        generator_seed: Some(seed),
        requested_facet_count: Some(q_facets + p_facets),
        dual_vertices,
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

fn random_bounded_planar_normals(count: usize, rng: &mut ChaCha8Rng) -> Vec<Vector2<f64>> {
    for _ in 0..10_000 {
        let mut angles = (0..count)
            .map(|_| rng.gen_range(0.0..std::f64::consts::TAU))
            .collect::<Vec<_>>();
        angles.sort_by(f64::total_cmp);
        if planar_gaps_are_well_spread(&angles) {
            return angles
                .into_iter()
                .map(|angle| Vector2::new(angle.cos(), angle.sin()))
                .collect();
        }
    }
    panic!("failed to sample bounded planar normals for {count} facets");
}

fn planar_gaps_are_well_spread(angles: &[f64]) -> bool {
    angles.iter().enumerate().all(|(i, angle)| {
        let next = angles
            .get(i + 1)
            .copied()
            .unwrap_or_else(|| angles[0] + std::f64::consts::TAU);
        let gap = next - angle;
        (MIN_PLANAR_GAP..=MAX_PLANAR_GAP).contains(&gap)
    })
}

fn seeded_rng(seed: u64, family: &str, attempt: u64) -> ChaCha8Rng {
    let mut mixed = seed ^ attempt.rotate_left(17);
    for byte in family.as_bytes() {
        mixed = mixed.rotate_left(5) ^ u64::from(*byte);
    }
    ChaCha8Rng::seed_from_u64(mixed)
}

fn generated_attempt(facet_key: usize, sample: usize, rejection_index: u64) -> u64 {
    (facet_key as u64) * 1_000_000_000
        + (sample as u64) * MAX_RAW_RANDOM_ATTEMPTS_PER_CASE
        + rejection_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_product_factors_are_well_spread() {
        let mut rng = seeded_rng(99599604, "test", 0);
        for count in 3..=6 {
            for _ in 0..100 {
                let normals = random_bounded_planar_normals(count, &mut rng);
                let angles = normals
                    .iter()
                    .map(|normal| normal[1].atan2(normal[0]).rem_euclid(std::f64::consts::TAU))
                    .collect::<Vec<_>>();
                assert!(planar_gaps_are_well_spread(&angles));
            }
        }
    }

    #[test]
    fn generated_random_cases_are_exact_valid() {
        let case = generated_random_case(8, 0, 99599604);
        assert_eq!(case.dual_vertices.len(), 8);
        assert!(SysLandscapePolytopeCache::from_f64_dual_vertices(case.dual_vertices).is_some());
    }

    #[test]
    fn generated_source_id_filter_avoids_full_bank() {
        let source_ids = vec![
            "seed99540836:F5:sample0:attempt5000000008".to_string(),
            "seed99540836:q4:p5:attempt405000000000".to_string(),
        ];
        let cases = generated_f64_cases_with_source_filter(1, 99540836, &source_ids);
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].source_id, source_ids[0]);
        assert_eq!(cases[1].source_id, source_ids[1]);
    }
}
