//! Breadth-first smoke executor for the alternative polygon-generator wishlist.
//!
//! The binary deliberately keeps each law isolated: a failed law records a
//! disposition and the remaining laws continue.  It is a feasibility packet,
//! not a production sampler or a transfer claim.

use exp_sys_landscape::{
    capacity_auto, compute_sys_from_capacity, exact_volume_from_incidence_as_f64,
    SysLandscapePolytopeCache,
};
use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Gamma, Normal};
use serde::Serialize;
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::geom::polygon::{polygon_area, random_polygon_2d};

const DEFAULT_SEED: u64 = 20260714;
const DEFAULT_ATTEMPTS: usize = 128;
const DEFAULT_RUNTIME_CAP_MS: f64 = 2_000.0;
const PAIRS: &[(usize, usize)] = &[(3, 3), (4, 6), (6, 6)];

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
    attempts: usize,
    runtime_cap_ms: f64,
    rows_per_law: usize,
    target_backend: bool,
}

#[derive(Clone, Debug)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
}

#[derive(Clone, Serialize)]
struct SmokeRow {
    schema: &'static str,
    sample_id: String,
    law: String,
    wishlist_item: u8,
    law_version: &'static str,
    seed: u64,
    attempt: usize,
    attempts: usize,
    rejections: usize,
    parameter: String,
    pair_bucket: String,
    facet_count: usize,
    accepted: bool,
    validation_status: String,
    rejection_reason: Option<String>,
    factor_q_area: Option<f64>,
    factor_p_area: Option<f64>,
    volume: Option<f64>,
    capacity: Option<f64>,
    sys: Option<f64>,
    iterations: Option<u64>,
    generation_ms: f64,
    validation_ms: f64,
    target_ms: f64,
}

#[derive(Clone, Serialize)]
struct Disposition {
    wishlist_item: u8,
    law: &'static str,
    disposition: &'static str,
    evidence: &'static str,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    law_version: &'static str,
    seed: u64,
    max_attempts_per_row: usize,
    runtime_cap_ms: f64,
    pairs: Vec<String>,
    rows: usize,
    command: String,
    source_revision: String,
    status_counts: BTreeMap<String, usize>,
    per_law: Vec<LawSummary>,
    dispositions: Vec<Disposition>,
    interpretation_boundary: &'static str,
}

#[derive(Serialize)]
struct LawSummary {
    law: String,
    rows: usize,
    accepted_rows: usize,
    survived_rows: usize,
    total_generation_ms: f64,
    total_validation_ms: f64,
    total_target_ms: f64,
    max_attempts_observed: usize,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut args = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/alternative-generator-smoke/artifacts",
        ),
        seed: DEFAULT_SEED,
        attempts: DEFAULT_ATTEMPTS,
        runtime_cap_ms: DEFAULT_RUNTIME_CAP_MS,
        rows_per_law: 1,
        target_backend: false,
    };
    let mut i = 1;
    while i < argv.len() {
        let value = |flag: &str| {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match argv[i].as_str() {
            "--out-dir" => {
                args.out_dir = PathBuf::from(value("--out-dir"));
                i += 2;
            }
            "--seed" => {
                args.seed = value("--seed").parse().expect("--seed must be u64");
                i += 2;
            }
            "--attempts" => {
                args.attempts = value("--attempts")
                    .parse()
                    .expect("--attempts must be usize");
                i += 2;
            }
            "--runtime-cap-ms" => {
                args.runtime_cap_ms = value("--runtime-cap-ms")
                    .parse()
                    .expect("--runtime-cap-ms must be f64");
                i += 2;
            }
            "--rows-per-law" => {
                args.rows_per_law = value("--rows-per-law")
                    .parse()
                    .expect("--rows-per-law must be usize");
                i += 2;
            }
            "--target" => {
                args.target_backend = true;
                i += 1;
            }
            "--help" | "-h" => {
                println!(
                    "--out-dir DIR --seed N --attempts N --runtime-cap-ms MS --rows-per-law N"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    assert!(args.attempts > 0 && args.runtime_cap_ms.is_finite() && args.runtime_cap_ms > 0.0);
    args
}

fn law_seed(seed: u64, law: &str, parameter: &str, attempt: usize) -> [u8; 32] {
    let mut key = Vec::new();
    key.extend_from_slice(&seed.to_le_bytes());
    key.extend_from_slice(law.as_bytes());
    key.push(0);
    key.extend_from_slice(parameter.as_bytes());
    key.push(0);
    key.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&key).as_bytes()
}

fn area_normalize(mut f: Factor) -> Option<Factor> {
    if !all_facets_active(&f) {
        return None;
    }
    let area = polygon_area(&f.normals, &f.heights)?;
    if !area.is_finite() || area <= 0.0 {
        return None;
    }
    let scale = area.sqrt().recip();
    for h in &mut f.heights {
        *h *= scale;
    }
    Some(f)
}

/// Cheap H-representation witness used before the exact 4D boundary.  Each
/// adjacent edge intersection must satisfy every half-plane; this rejects
/// inactive facets without spending the much slower rational reconstruction.
fn all_facets_active(f: &Factor) -> bool {
    let n = f.normals.len();
    if n < 3 || f.heights.len() != n {
        return false;
    }
    for i in 0..n {
        let j = (i + 1) % n;
        let a = f.normals[i];
        let b = f.normals[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return false;
        }
        let x = (f.heights[i] * b[1] - f.heights[j] * a[1]) / det;
        let y = (a[0] * f.heights[j] - b[0] * f.heights[i]) / det;
        for (normal, height) in f.normals.iter().zip(&f.heights) {
            if normal[0] * x + normal[1] * y > *height + 1e-9 {
                return false;
            }
        }
    }
    true
}

fn random_angles<R: Rng>(n: usize, rng: &mut R, period: f64) -> Vec<f64> {
    let mut a: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * period).collect();
    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    a
}

fn from_angles(angles: &[f64], heights: Vec<f64>) -> Factor {
    Factor {
        normals: angles
            .iter()
            .map(|a| Vector2::new(a.cos(), a.sin()))
            .collect(),
        heights,
    }
}

fn baseline(n: usize, rng: &mut ChaCha8Rng) -> Factor {
    let (normals, heights) = random_polygon_2d(n, 0.8, 1.2, rng);
    Factor { normals, heights }
}

fn equal_support(n: usize, rng: &mut ChaCha8Rng) -> Factor {
    let angles = random_angles(n, rng, TAU);
    from_angles(&angles, vec![1.0; n])
}

fn log_support(n: usize, sigma: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let normal = Normal::new(0.0, 1.0).ok()?;
    let mut z: Vec<f64> = (0..n)
        .map(|_| (normal.sample(rng) as f64).clamp(-2.0, 2.0))
        .collect();
    let mean = z.iter().sum::<f64>() / n as f64;
    for x in &mut z {
        *x = sigma * (*x - mean);
    }
    let angles = random_angles(n, rng, TAU);
    Some(from_angles(&angles, z.into_iter().map(f64::exp).collect()))
}

fn dirichlet(n: usize, alpha: f64, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let gamma = Gamma::new(alpha, 1.0).ok()?;
    let mut g: Vec<f64> = (0..n).map(|_| gamma.sample(rng)).collect();
    let sum: f64 = g.iter().sum();
    for x in &mut g {
        *x = TAU * *x / sum;
    }
    if g.iter().any(|x| *x >= PI) {
        return None;
    }
    let rotation = rng.gen::<f64>() * TAU;
    let mut angles = Vec::with_capacity(n);
    let mut t = rotation;
    for gap in g {
        angles.push(t);
        t += gap;
    }
    Some(from_angles(&angles, vec![1.0; n]))
}

fn jittered_regular(n: usize, jitter: f64, rng: &mut ChaCha8Rng) -> Factor {
    let base = PI / 2.0;
    let step = TAU / n as f64;
    let mut angles: Vec<f64> = (0..n)
        .map(|i| base + step * i as f64 + (rng.gen::<f64>() - 0.5) * jitter * step)
        .collect();
    angles.sort_by(|x, y| x.partial_cmp(y).unwrap());
    from_angles(&angles, vec![1.0; n])
}

fn strips(n: usize, broken: bool, rng: &mut ChaCha8Rng) -> Option<Factor> {
    if n % 2 != 0 {
        return None;
    }
    let r = n / 2;
    let lines = random_angles(r, rng, PI);
    let mut normals = Vec::with_capacity(n);
    let mut heights = Vec::with_capacity(n);
    for a in lines {
        let u = Vector2::new(a.cos(), a.sin());
        let (plus, minus) = if broken {
            (0.8 + 0.4 * rng.gen::<f64>(), 0.8 + 0.4 * rng.gen::<f64>())
        } else {
            let w = 0.8 + 0.4 * rng.gen::<f64>();
            (w / 2.0, w / 2.0)
        };
        normals.push(u);
        heights.push(plus);
        normals.push(-u);
        heights.push(minus);
    }
    // Antipodal normals are not cyclic; sorting by angle is required by the
    // polygon kernel and preserves the support paired with each normal.
    let mut ix: Vec<usize> = (0..n).collect();
    ix.sort_by(|i, j| {
        let ai = normals[*i][1].atan2(normals[*i][0]);
        let aj = normals[*j][1].atan2(normals[*j][0]);
        ai.partial_cmp(&aj).unwrap()
    });
    let ns = ix.iter().map(|i| normals[*i]).collect();
    let hs = ix.iter().map(|i| heights[*i]).collect();
    Some(Factor {
        normals: ns,
        heights: hs,
    })
}

fn congruent(n: usize, phi: f64, rng: &mut ChaCha8Rng) -> (Factor, Factor) {
    let q = baseline(n, rng);
    let (pn, ph) = symplectic::geom::polygon::rotate_polygon_2d(&q.normals, &q.heights, phi);
    (
        q,
        Factor {
            normals: pn,
            heights: ph,
        },
    )
}

fn inscribed(n: usize, rng: &mut ChaCha8Rng) -> Option<Factor> {
    let angles = random_angles(n, rng, TAU);
    let mut gaps = Vec::with_capacity(n);
    for i in 0..n {
        let next = if i + 1 == n {
            angles[0] + TAU
        } else {
            angles[i + 1]
        };
        gaps.push(next - angles[i]);
    }
    if gaps.iter().any(|g| *g >= PI) {
        return None;
    }
    let normals: Vec<Vector2<f64>> = (0..n)
        .map(|i| {
            let mid = angles[i] + gaps[i] / 2.0;
            Vector2::new(mid.cos(), mid.sin())
        })
        .collect();
    let heights = gaps.iter().map(|g| (g / 2.0).cos()).collect();
    Some(Factor { normals, heights })
}

fn make_pair(
    law: &str,
    parameter: &str,
    k: usize,
    m: usize,
    rng: &mut ChaCha8Rng,
) -> Option<(Factor, Factor)> {
    if matches!(law, "factorial-q" | "factorial-p" | "factorial-both") {
        let mut q = baseline(k, rng);
        let mut p = baseline(m, rng);
        if matches!(law, "factorial-q" | "factorial-both") {
            q = equal_support(k, rng);
        }
        if matches!(law, "factorial-p" | "factorial-both") {
            p = equal_support(m, rng);
        }
        return Some((q, p));
    }
    let f = |n: usize, rng: &mut ChaCha8Rng| -> Option<Factor> {
        match law {
            "baseline" => Some(baseline(n, rng)),
            "equal-support" => Some(equal_support(n, rng)),
            "log-support" => log_support(n, parameter.parse().ok()?, rng),
            "dirichlet-gap" => dirichlet(n, parameter.parse().ok()?, rng),
            "jittered-regular" => Some(jittered_regular(n, parameter.parse().ok()?, rng)),
            "symmetric-strips" => strips(n, false, rng),
            "broken-antipodal" => strips(n, true, rng),
            "inscribed" => inscribed(n, rng),
            _ => None,
        }
    };
    if law == "congruent" {
        if k != m {
            return None;
        }
        let phi: f64 = parameter.parse().ok()?;
        let (q, p) = congruent(k, phi, rng);
        return Some((q, p));
    }
    let q = f(k, rng)?;
    let p = f(m, rng)?;
    Some((q, p))
}

fn evaluate_pair(
    q: Factor,
    p: Factor,
    args: &Args,
    law: &str,
    item: u8,
    parameter: &str,
    bucket: (usize, usize),
    seed: u64,
    attempt: usize,
) -> SmokeRow {
    let sample_id = format!(
        "altgen-v1/{law}/param={parameter}/seed={seed}/attempt={attempt}/{}x{}",
        bucket.0, bucket.1
    );
    let t0 = Instant::now();
    let q_area = polygon_area(&q.normals, &q.heights);
    let p_area = polygon_area(&p.normals, &p.heights);
    let generation_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let tv = Instant::now();
    let poly = SysLandscapePolytopeCache::from_lagrangian_product(
        &q.normals, &q.heights, &p.normals, &p.heights,
    );
    let validation_ms = tv.elapsed().as_secs_f64() * 1000.0;
    let bucket_name = format!("{}x{}", bucket.0, bucket.1);
    let Some(poly) = poly else {
        return SmokeRow {
            schema: "alternative-generator-smoke-row-v1",
            sample_id: sample_id.clone(),
            law: law.to_string(),
            wishlist_item: item,
            law_version: "wishlist-2026-07-14-v1",
            seed,
            attempt,
            attempts: attempt + 1,
            rejections: attempt,
            parameter: parameter.to_string(),
            pair_bucket: bucket_name,
            facet_count: bucket.0 + bucket.1,
            accepted: false,
            validation_status: "invalid".into(),
            rejection_reason: Some("exact product validation rejected geometry".into()),
            factor_q_area: q_area,
            factor_p_area: p_area,
            volume: None,
            capacity: None,
            sys: None,
            iterations: None,
            generation_ms,
            validation_ms,
            target_ms: 0.0,
        };
    };
    // Six-by-six and larger target searches can exceed the executor envelope;
    // retain geometry/validation evidence but classify those rows before
    // entering the backend.  Smaller rows use the existing product backend.
    if !args.target_backend || poly.facet_count() > 10 {
        return SmokeRow {
            schema: "alternative-generator-smoke-row-v1",
            sample_id: sample_id.clone(),
            law: law.to_string(),
            wishlist_item: item,
            law_version: "wishlist-2026-07-14-v1",
            seed,
            attempt,
            attempts: attempt + 1,
            rejections: attempt,
            parameter: parameter.to_string(),
            pair_bucket: bucket_name,
            facet_count: poly.facet_count(),
            accepted: true,
            validation_status: if args.target_backend {
                "runtime_cap"
            } else {
                "survived"
            }
            .into(),
            rejection_reason: Some(if args.target_backend {
                "target backend skipped above facet-count cap 10".into()
            } else {
                "target backend disabled for breadth-first generation-only smoke; measured product target path exceeded short cap".into()
            }),
            factor_q_area: q_area,
            factor_p_area: p_area,
            volume: Some(exact_volume_from_incidence_as_f64(
                &poly.vertices,
                &poly.vertex_facet_incidence,
            )),
            capacity: None,
            sys: None,
            iterations: None,
            generation_ms,
            validation_ms,
            target_ms: 0.0,
        };
    }
    let tt = Instant::now();
    let volume = exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
    let target = capacity_auto(
        &poly.dual_vertices_f64,
        &poly.dual_vertices,
        &poly.facet_intersection_is_nonempty,
        &poly.omega_signs,
    )
    .ok();
    let (capacity, sys, iterations) = target
        .as_ref()
        .map(|c| {
            (
                Some(c.min_action),
                compute_sys_from_capacity(&poly, c),
                Some(c.iterations),
            )
        })
        .unwrap_or((None, None, None));
    let target_ms = tt.elapsed().as_secs_f64() * 1000.0;
    let status = if target.is_some() {
        if generation_ms + validation_ms + target_ms > args.runtime_cap_ms {
            "runtime_cap"
        } else {
            "survived"
        }
    } else {
        "target_failed"
    };
    SmokeRow {
        schema: "alternative-generator-smoke-row-v1",
        sample_id,
        law: law.to_string(),
        wishlist_item: item,
        law_version: "wishlist-2026-07-14-v1",
        seed,
        attempt,
        attempts: attempt + 1,
        rejections: attempt,
        parameter: parameter.to_string(),
        pair_bucket: bucket_name,
        facet_count: poly.facet_count(),
        accepted: true,
        validation_status: status.into(),
        rejection_reason: None,
        factor_q_area: q_area,
        factor_p_area: p_area,
        volume: Some(volume),
        capacity,
        sys,
        iterations,
        generation_ms,
        validation_ms,
        target_ms,
    }
}

fn dispositions() -> Vec<Disposition> {
    vec![
        Disposition {
            wishlist_item: 1,
            law: "fresh baseline",
            disposition: "survived",
            evidence: "reuses random polygon kernel with explicit law/seed/attempt identity",
        },
        Disposition {
            wishlist_item: 2,
            law: "equal-support",
            disposition: "survived",
            evidence: "unit supports and area normalization are local",
        },
        Disposition {
            wishlist_item: 3,
            law: "log-support ladder",
            disposition: "survived",
            evidence: "bounded centered Gaussian support ladder",
        },
        Disposition {
            wishlist_item: 4,
            law: "smooth support field",
            disposition: "compile_or_api_block",
            evidence:
                "Fourier support convexity/active-facet conditioning needs a separate law owner",
        },
        Disposition {
            wishlist_item: 5,
            law: "shape-cell conditional",
            disposition: "backend_or_schema_expansion",
            evidence: "comparison design requires matched retained populations",
        },
        Disposition {
            wishlist_item: 6,
            law: "one-factor factorial",
            disposition: "survived",
            evidence: "three intervention arms share the product constructor",
        },
        Disposition {
            wishlist_item: 7,
            law: "Dirichlet angular gaps",
            disposition: "survived",
            evidence: "Gamma simplex draw with max-gap rejection",
        },
        Disposition {
            wishlist_item: 8,
            law: "jittered regular",
            disposition: "survived",
            evidence: "regular fan plus bounded order-preserving jitter",
        },
        Disposition {
            wishlist_item: 9,
            law: "symmetric strips",
            disposition: "survived",
            evidence: "paired antipodal lines, even side counts",
        },
        Disposition {
            wishlist_item: 10,
            law: "broken antipodal",
            disposition: "survived",
            evidence: "independent opposite supports on the same line law",
        },
        Disposition {
            wishlist_item: 11,
            law: "zonogon",
            disposition: "invalid_or_low_acceptance",
            evidence: "Minkowski-sum-to-H-representation conversion is outside this owner",
        },
        Disposition {
            wishlist_item: 12,
            law: "congruent factors",
            disposition: "survived",
            evidence: "same factor with explicit relative rotation",
        },
        Disposition {
            wishlist_item: 13,
            law: "shared latent",
            disposition: "compile_or_api_block",
            evidence: "coupling angular simplex and support laws is unresolved",
        },
        Disposition {
            wishlist_item: 14,
            law: "polar coupled",
            disposition: "compile_or_api_block",
            evidence: "canonical-center polarity would need a named exact center",
        },
        Disposition {
            wishlist_item: 15,
            law: "IID point hull",
            disposition: "invalid_or_low_acceptance",
            evidence: "hull-side-count conditioning not available in current narrow API",
        },
        Disposition {
            wishlist_item: 16,
            law: "inscribed polygon",
            disposition: "survived",
            evidence: "circle hull has direct edge-normal/support formula",
        },
        Disposition {
            wishlist_item: 17,
            law: "Poisson line cell",
            disposition: "runtime_cap",
            evidence: "faithful conditional stationary-line simulation exceeds tiny owner envelope",
        },
        Disposition {
            wishlist_item: 18,
            law: "SO(4)/U(2) orientation",
            disposition: "backend_or_schema_expansion",
            evidence: "mixed-coordinate target requires generic backend/cache identity",
        },
        Disposition {
            wishlist_item: 19,
            law: "quotient-transverse",
            disposition: "backend_or_schema_expansion",
            evidence: "actual Sp(4) orbit tangent projection is not a local helper",
        },
        Disposition {
            wishlist_item: 20,
            law: "generic centrally symmetric",
            disposition: "backend_or_schema_expansion",
            evidence: "generic exact capacity path is a separate owner",
        },
        Disposition {
            wishlist_item: 21,
            law: "SL(4) structured images",
            disposition: "backend_or_schema_expansion",
            evidence: "law on determinant-one matrices and generic reconstruction are separate",
        },
    ]
}

fn main() {
    let args = parse_args();
    create_dir_all(&args.out_dir).expect("create output directory");
    let rows_path = args.out_dir.join("smoke-rows.jsonl");
    let report_path = args.out_dir.join("batch-report.json");
    let mut rows_out = BufWriter::new(File::create(&rows_path).expect("create rows"));
    let mut rows_count = 0usize;
    let mut all_rows = Vec::new();
    let jobs: &[(&str, u8, &[&str])] = &[
        ("baseline", 1, &["0.2"]),
        ("equal-support", 2, &["area=1"]),
        ("log-support", 3, &["0.0", "0.1", "0.2"]),
        ("factorial-q", 6, &["q=tangential"]),
        ("factorial-p", 6, &["p=tangential"]),
        ("factorial-both", 6, &["q,p=tangential"]),
        ("dirichlet-gap", 7, &["0.5", "1.0", "2.0", "10.0"]),
        ("jittered-regular", 8, &["0.0", "0.1"]),
        ("symmetric-strips", 9, &["equal-width"]),
        ("broken-antipodal", 10, &["independent-supports"]),
        ("congruent", 12, &["0.0", "0.2617993877991494"]),
        ("inscribed", 16, &["circle-radius=1"]),
    ];
    for &(law, item, params) in jobs {
        for &parameter in params {
            for &bucket in PAIRS {
                if (law == "symmetric-strips" || law == "broken-antipodal")
                    && (bucket.0 % 2 != 0 || bucket.1 % 2 != 0)
                {
                    continue;
                }
                if law == "congruent" && bucket.0 != bucket.1 {
                    continue;
                }
                for row_index in 0..args.rows_per_law {
                    let mut accepted = None;
                    for attempt in 0..args.attempts {
                        let mut rng = ChaCha8Rng::from_seed(law_seed(
                            args.seed ^ row_index as u64,
                            law,
                            parameter,
                            attempt,
                        ));
                        let generated = make_pair(law, parameter, bucket.0, bucket.1, &mut rng)
                            .and_then(|(q, p)| Some((area_normalize(q)?, area_normalize(p)?)));
                        if let Some((q, p)) = generated {
                            let row = evaluate_pair(
                                q, p, &args, law, item, parameter, bucket, args.seed, attempt,
                            );
                            if row.validation_status != "invalid" {
                                accepted = Some(row);
                                break;
                            }
                        }
                    }
                    let row = accepted.unwrap_or_else(|| SmokeRow {
                        schema: "alternative-generator-smoke-row-v1",
                        sample_id: format!(
                            "altgen-v1/{law}/param={parameter}/seed={}/attempt={}/{}x{}",
                            args.seed, args.attempts, bucket.0, bucket.1
                        ),
                        law: law.to_string(),
                        wishlist_item: item,
                        law_version: "wishlist-2026-07-14-v1",
                        seed: args.seed,
                        attempt: args.attempts,
                        attempts: args.attempts,
                        rejections: args.attempts,
                        parameter: parameter.to_string(),
                        pair_bucket: format!("{}x{}", bucket.0, bucket.1),
                        facet_count: bucket.0 + bucket.1,
                        accepted: false,
                        validation_status: "invalid_or_low_acceptance".into(),
                        rejection_reason: Some(format!(
                            "no accepted geometry in {} bounded attempts",
                            args.attempts
                        )),
                        factor_q_area: None,
                        factor_p_area: None,
                        volume: None,
                        capacity: None,
                        sys: None,
                        iterations: None,
                        generation_ms: 0.0,
                        validation_ms: 0.0,
                        target_ms: 0.0,
                    });
                    serde_json::to_writer(&mut rows_out, &row).expect("write row");
                    rows_out.write_all(b"\n").expect("newline");
                    rows_count += 1;
                    all_rows.push(row);
                }
            }
        }
    }
    rows_out.flush().expect("flush rows");
    let mut status_counts = BTreeMap::new();
    let mut law_map: BTreeMap<String, LawSummary> = BTreeMap::new();
    for row in &all_rows {
        *status_counts
            .entry(row.validation_status.clone())
            .or_insert(0) += 1;
        let summary = law_map
            .entry(row.law.clone())
            .or_insert_with(|| LawSummary {
                law: row.law.clone(),
                rows: 0,
                accepted_rows: 0,
                survived_rows: 0,
                total_generation_ms: 0.0,
                total_validation_ms: 0.0,
                total_target_ms: 0.0,
                max_attempts_observed: 0,
            });
        summary.rows += 1;
        summary.accepted_rows += usize::from(row.accepted);
        summary.survived_rows += usize::from(row.validation_status == "survived");
        summary.total_generation_ms += row.generation_ms;
        summary.total_validation_ms += row.validation_ms;
        summary.total_target_ms += row.target_ms;
        summary.max_attempts_observed = summary.max_attempts_observed.max(row.attempts);
    }
    let source_revision = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".into());
    let report = Report { schema: "alternative-generator-smoke-report-v1", law_version: "wishlist-2026-07-14-v1", seed: args.seed, max_attempts_per_row: args.attempts, runtime_cap_ms: args.runtime_cap_ms, pairs: PAIRS.iter().map(|(k,m)| format!("{k}x{m}")).collect(), rows: rows_count, command: std::env::args().collect::<Vec<_>>().join(" "), source_revision, status_counts, per_law: law_map.into_values().collect(), dispositions: dispositions(), interpretation_boundary: "Tiny target-evaluated smoke is plumbing and feasibility evidence only; it does not establish population separation or a transfer conclusion." };
    serde_json::to_writer_pretty(File::create(&report_path).expect("create report"), &report)
        .expect("write report");
    println!(
        "wrote {} rows to {} and report to {}",
        rows_count,
        rows_path.display(),
        report_path.display()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_support_has_equal_heights_and_positive_area() {
        let mut rng = ChaCha8Rng::from_seed(law_seed(7, "equal-support", "area=1", 0));
        let f = area_normalize(equal_support(5, &mut rng)).unwrap();
        assert!(f.heights.iter().all(|h| (*h - f.heights[0]).abs() < 1e-12));
        assert!((polygon_area(&f.normals, &f.heights).unwrap() - 1.0).abs() < 1e-12);
    }
    #[test]
    fn dirichlet_and_inscribed_have_distinct_support_laws() {
        let mut a = ChaCha8Rng::from_seed(law_seed(8, "dirichlet-gap", "1.0", 0));
        let mut b = ChaCha8Rng::from_seed(law_seed(8, "inscribed", "circle-radius=1", 0));
        let da = area_normalize(dirichlet(5, 1.0, &mut a).unwrap()).unwrap();
        let ib = area_normalize(inscribed(5, &mut b).unwrap()).unwrap();
        assert!(da
            .heights
            .iter()
            .zip(ib.heights.iter())
            .any(|(x, y)| (x - y).abs() > 1e-4));
    }
    #[test]
    fn symmetric_strip_supports_are_antipodal_pairs() {
        let mut rng = ChaCha8Rng::from_seed(law_seed(9, "symmetric-strips", "equal-width", 0));
        let f = strips(6, false, &mut rng).unwrap();
        for i in 0..f.normals.len() {
            let has_opposite = f.normals.iter().any(|n| (n + f.normals[i]).norm() < 1e-12);
            assert!(has_opposite);
        }
    }
    #[test]
    fn congruent_rotation_preserves_factor_area() {
        let mut rng = ChaCha8Rng::from_seed(law_seed(10, "congruent", "0.2", 0));
        let (q, p) = congruent(5, 0.2, &mut rng);
        let aq = polygon_area(&q.normals, &q.heights).unwrap();
        let ap = polygon_area(&p.normals, &p.heights).unwrap();
        assert!((aq - ap).abs() < 1e-10);
    }
    #[test]
    fn inscribed_support_formula_is_positive() {
        let mut rng = ChaCha8Rng::from_seed(law_seed(11, "inscribed", "circle-radius=1", 0));
        let f = inscribed(6, &mut rng).unwrap();
        assert!(f.heights.iter().all(|h| *h > 0.0 && *h <= 1.0));
    }
}
