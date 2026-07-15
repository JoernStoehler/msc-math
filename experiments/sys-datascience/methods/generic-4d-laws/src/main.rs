//! Target-free breadth-first comparison of mathematically distinct 4D laws.
//!
//! This binary deliberately stops at exact Euclidean reconstruction.  It does
//! not import a capacity evaluator, `sys`, a target table, or the ridge-tail
//! selector.  Every generated row records the proposal law, reconstruction
//! disposition, combinatorial fingerprint, and inexpensive symplectic
//! diagnostics.

use euclidean_polytopes::{
    all_points_are_extreme_exact, edges_from_vertex_facet_incidence,
    facet_vertices_from_vertex_facet_incidence, origin_in_interior_of_conv_exact,
    polar_vertices_exact_rational, two_faces_from_vertex_facet_incidence,
    volume_from_incidence_exact,
};
use nalgebra::{DMatrix, Vector4};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use serde::Serialize;
use std::f64::consts::PI;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

const SEEDS: [u64; 3] = [11, 29, 47];
const BALL_VOLUME_4D: f64 = PI * PI / 2.0;
const EPS: f64 = 1e-10;

#[derive(Debug, Serialize, serde::Deserialize)]
struct Row {
    row_id: String,
    seed: u64,
    arm: String,
    parameter: String,
    status: String,
    disposition: String,
    proposal_count: usize,
    retained_vertex_count: usize,
    facet_count: usize,
    f_vector: Option<[usize; 4]>,
    volume: Option<f64>,
    volume_normalized_to_unit_ball: Option<f64>,
    central_symmetry_witness: Option<bool>,
    product_cross_block_fraction: Option<f64>,
    omega_abs_mean: Option<f64>,
    affine_diagonal: Option<[f64; 4]>,
    conditioning_min_abs_det: Option<f64>,
    conditioning_singular_tuple_count: usize,
    exact_candidate_4sets: usize,
    exact_feasibility_checks_upper_bound: usize,
    source: String,
}

#[derive(Debug, Serialize)]
struct Manifest {
    packet: &'static str,
    question: &'static str,
    seeds: Vec<u64>,
    grid: Vec<&'static str>,
    source_commit: String,
    target_fields: Vec<&'static str>,
    variable_facet_distance: &'static str,
    generated_rows: usize,
    accepted_rows: usize,
    abandoned_arms: Vec<String>,
}

struct Body {
    primal: Vec<Vector4<BigRational>>,
    dual: Vec<Vector4<BigRational>>,
    incidence: DMatrix<bool>,
}

fn rat(x: f64) -> Result<BigRational, String> {
    if !x.is_finite() {
        return Err(format!("non-finite coordinate {x}"));
    }
    if x == 0.0 {
        Ok(BigRational::zero())
    } else {
        // The generator's f64 coordinates are rounded to a fixed decimal
        // lattice before exact validation.  This keeps the exact replay
        // tractable while making the represented rational polytope explicit.
        const SCALE: f64 = 1_000_000.0;
        let numerator = (x * SCALE).round();
        if !numerator.is_finite() {
            return Err(format!("cannot rationalize {x}"));
        }
        Ok(BigRational::new(
            BigInt::from(numerator as i64),
            BigInt::from(SCALE as i64),
        ))
    }
}

fn rationalize(points: &[Vector4<f64>]) -> Result<Vec<Vector4<BigRational>>, String> {
    points
        .iter()
        .map(|p| Ok(Vector4::new(rat(p[0])?, rat(p[1])?, rat(p[2])?, rat(p[3])?)))
        .collect()
}

fn f64_points(points: &[Vector4<BigRational>]) -> Vec<Vector4<f64>> {
    points
        .iter()
        .map(|p| {
            Vector4::new(
                p[0].to_f64().unwrap(),
                p[1].to_f64().unwrap(),
                p[2].to_f64().unwrap(),
                p[3].to_f64().unwrap(),
            )
        })
        .collect()
}

fn transpose_incidence(input: &DMatrix<bool>, active: &[usize]) -> DMatrix<bool> {
    DMatrix::from_fn(active.len(), input.nrows(), |row, col| {
        input[(col, active[row])]
    })
}

fn reconstruct_h(dual_f64: &[Vector4<f64>]) -> Result<Body, String> {
    if dual_f64.len() < 5 {
        return Err("H-representation has fewer than five facets".into());
    }
    let dual = rationalize(dual_f64)?;
    if !origin_in_interior_of_conv_exact(&dual) {
        return Err("unbounded or lower-dimensional H-law".into());
    }
    let polar = polar_vertices_exact_rational(&dual);
    if polar.vertices.is_empty() {
        return Err("exact polar returned no vertices".into());
    }
    if (0..dual.len())
        .any(|f| !(0..polar.vertices.len()).any(|v| polar.vertex_facet_incidence[(v, f)]))
    {
        return Err("redundant or inactive H facet".into());
    }
    Ok(Body {
        primal: polar.vertices,
        dual,
        incidence: polar.vertex_facet_incidence,
    })
}

fn reconstruct_v(primal_f64: &[Vector4<f64>]) -> Result<Body, String> {
    if primal_f64.len() < 5 {
        return Err("V-representation has fewer than five proposals".into());
    }
    let input = rationalize(primal_f64)?;
    if !origin_in_interior_of_conv_exact(&input) {
        return Err("origin is not in the interior; hull is unbounded as a polar".into());
    }
    let polar = polar_vertices_exact_rational(&input);
    let active: Vec<usize> = (0..input.len())
        .filter(|&i| (0..polar.vertices.len()).any(|v| polar.vertex_facet_incidence[(v, i)]))
        .collect();
    if active.len() < 5 {
        return Err(format!("only {} active V-points", active.len()));
    }
    let primal = active.iter().map(|&i| input[i].clone()).collect::<Vec<_>>();
    let incidence = transpose_incidence(&polar.vertex_facet_incidence, &active);
    if !all_points_are_extreme_exact(&primal) {
        return Err("active V-points failed exact extremality check".into());
    }
    Ok(Body {
        primal,
        dual: polar.vertices,
        incidence,
    })
}

fn unit_sphere(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    loop {
        let mut x: Vector4<f64> = Vector4::new(
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
            rng.sample::<f64, _>(StandardNormal),
        );
        let norm = x.norm();
        if norm.is_finite() && norm > 1e-12 {
            x /= norm;
            return x;
        }
    }
}

fn det4(rows: [Vector4<f64>; 4]) -> f64 {
    nalgebra::Matrix4::from_rows(&[
        rows[0].transpose(),
        rows[1].transpose(),
        rows[2].transpose(),
        rows[3].transpose(),
    ])
    .determinant()
}

fn determinant_conditioning(points: &[Vector4<f64>]) -> (f64, usize) {
    if points.len() < 4 {
        return (0.0, 0);
    }
    let mut best = f64::INFINITY;
    let mut singular = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            for k in (j + 1)..points.len() {
                for l in (k + 1)..points.len() {
                    let determinant = det4([points[i], points[j], points[k], points[l]]).abs();
                    if determinant <= EPS {
                        singular += 1;
                    } else {
                        best = best.min(determinant);
                    }
                }
            }
        }
    }
    (if best.is_finite() { best } else { 0.0 }, singular)
}

fn central_symmetric(points: &[Vector4<f64>]) -> bool {
    points.iter().all(|p| {
        points
            .iter()
            .any(|q| (p + q).norm() <= EPS * (1.0 + p.norm().max(q.norm())))
    })
}

fn body_row(
    seed: u64,
    arm: &str,
    parameter: &str,
    proposal_count: usize,
    result: Result<Body, String>,
    affine_diagonal: Option<[f64; 4]>,
) -> Row {
    let (status, disposition, body) = match result {
        Ok(body) => (
            "accepted".to_string(),
            "exactly_reconstructed".to_string(),
            Some(body),
        ),
        Err(error) => ("rejected".to_string(), error, None),
    };
    let candidate_4sets = (0..4).fold(1usize, |acc, k| {
        if k == 0 {
            acc
        } else {
            acc.saturating_mul(proposal_count.saturating_sub(k - 1)) / k
        }
    });
    let checks = candidate_4sets.saturating_mul(proposal_count);
    let (
        retained_vertex_count,
        facet_count,
        f_vector,
        volume,
        volume_norm,
        central,
        product,
        omega,
        conditioning,
        singular_tuples,
    ) = if let Some(body) = body {
        let primal_f64 = f64_points(&body.primal);
        let dual_f64 = f64_points(&body.dual);
        let edges = edges_from_vertex_facet_incidence(&body.incidence).len();
        let two_faces = two_faces_from_vertex_facet_incidence(&body.incidence).len();
        let facet_vertices = facet_vertices_from_vertex_facet_incidence(&body.incidence);
        let volume = volume_from_incidence_exact(&body.primal, &body.incidence)
            .to_f64()
            .unwrap_or(f64::NAN);
        let cross = dual_f64
            .iter()
            .filter(|a| a.fixed_rows::<2>(0).norm() > EPS && a.fixed_rows::<2>(2).norm() > EPS)
            .count() as f64
            / dual_f64.len().max(1) as f64;
        let omega_sum = dual_f64
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                dual_f64
                    .iter()
                    .skip(i + 1)
                    .map(move |b| symplectic::omega0(a, b).abs())
            })
            .sum::<f64>();
        let pairs = facet_count_pairs(dual_f64.len());
        let (conditioning, singular_tuples) = determinant_conditioning(&dual_f64);
        (
            body.primal.len(),
            body.dual.len(),
            Some([body.primal.len(), edges, two_faces, facet_vertices.len()]),
            Some(volume),
            Some(volume / BALL_VOLUME_4D),
            Some(central_symmetric(&primal_f64) && central_symmetric(&dual_f64)),
            Some(cross),
            Some(omega_sum / pairs.max(1) as f64),
            Some(conditioning),
            singular_tuples,
        )
    } else {
        (0, 0, None, None, None, None, None, None, None, 0)
    };
    Row {
        row_id: format!("{arm}:{seed}:{parameter}"),
        seed,
        arm: arm.to_string(),
        parameter: parameter.to_string(),
        status,
        disposition,
        proposal_count,
        retained_vertex_count,
        facet_count,
        f_vector,
        volume,
        volume_normalized_to_unit_ball: volume_norm,
        central_symmetry_witness: central,
        product_cross_block_fraction: product,
        omega_abs_mean: omega,
        affine_diagonal,
        conditioning_min_abs_det: conditioning,
        conditioning_singular_tuple_count: singular_tuples,
        exact_candidate_4sets: candidate_4sets,
        exact_feasibility_checks_upper_bound: checks,
        source: "generic-4d-laws/src/main.rs".to_string(),
    }
}

fn facet_count_pairs(n: usize) -> usize {
    n.saturating_mul(n.saturating_sub(1)) / 2
}

fn h_law(seed: u64, r: usize, sigma: f64) -> Result<Body, String> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0x484c4159 ^ (r as u64) << 16);
    let mut dual = Vec::with_capacity(2 * r);
    for _ in 0..r {
        let u = unit_sphere(&mut rng);
        let z: f64 = rng.sample(StandardNormal);
        let width = (sigma * z).exp();
        dual.push(u * (2.0 / width));
        dual.push(-u * (2.0 / width));
    }
    reconstruct_h(&dual)
}

fn v_law(seed: u64, n: usize, ball: bool) -> Result<Body, String> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0x564c4159 ^ (n as u64) << 16 ^ ball as u64);
    let points = (0..n)
        .map(|_| {
            let u = unit_sphere(&mut rng);
            if ball {
                u * rng.gen::<f64>().powf(0.25)
            } else {
                u
            }
        })
        .flat_map(|p| [p, -p])
        .collect::<Vec<_>>();
    reconstruct_v(&points)
}

fn zonotope(seed: u64, m: usize, sigma: f64) -> Result<Body, String> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0x5a4f4e4f ^ (m as u64) << 16);
    let generators = (0..m)
        .map(|_| {
            let z: f64 = rng.sample(StandardNormal);
            unit_sphere(&mut rng) * (sigma * z).exp()
        })
        .collect::<Vec<_>>();
    let proposal_count = 1usize << m;
    let points = (0..proposal_count)
        .map(|mask| {
            generators
                .iter()
                .enumerate()
                .fold(Vector4::zeros(), |acc, (j, g)| {
                    acc + *g * if mask & (1 << j) == 0 { -1.0 } else { 1.0 }
                })
        })
        .collect::<Vec<_>>();
    reconstruct_v(&points)
}

fn iid_hull(seed: u64, n: usize) -> Result<Body, String> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0x49494448 ^ (n as u64) << 16);
    for _attempt in 0..8 {
        let points = (0..n)
            .map(|_| unit_sphere(&mut rng) * rng.gen::<f64>().powf(0.25))
            .collect::<Vec<_>>();
        if let Ok(body) = reconstruct_v(&points) {
            return Ok(body);
        }
    }
    Err("eight non-symmetric 4-ball proposals failed origin/full-dimensional gate".into())
}

fn affine_control(seed: u64, name: &str) -> Result<(Body, [f64; 4]), String> {
    let base = match name {
        "simplex" => symplectic::known_polytopes::simplex(),
        "cube" => symplectic::known_polytopes::hypercube(),
        "crosspolytope" => symplectic::known_polytopes::crosspolytope(),
        _ => return Err(format!("unknown control {name}")),
    };
    let mut rng = ChaCha8Rng::seed_from_u64(seed ^ 0x41464649);
    let z = [
        rng.sample::<f64, _>(StandardNormal),
        rng.sample::<f64, _>(StandardNormal),
        rng.sample::<f64, _>(StandardNormal),
        rng.sample::<f64, _>(StandardNormal),
    ];
    let mean = z.iter().sum::<f64>() / 4.0;
    let d = [
        (0.35 * (z[0] - mean)).exp(),
        (0.35 * (z[1] - mean)).exp(),
        (0.35 * (z[2] - mean)).exp(),
        (0.35 * (z[3] - mean)).exp(),
    ];
    let points = base
        .vertices_f64
        .iter()
        .map(|p| Vector4::new(d[0] * p[0], d[1] * p[1], d[2] * p[2], d[3] * p[3]))
        .collect::<Vec<_>>();
    Ok((reconstruct_v(&points)?, d))
}

fn source_commit() -> String {
    std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn write_jsonl(path: &Path, rows: &[Row]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let file = File::create(path).map_err(|e| e.to_string())?;
    let mut out = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut out, row).map_err(|e| e.to_string())?;
        writeln!(out).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn run(out_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let wall_start = Instant::now();
    let mut rows = Vec::new();
    let mut abandoned = Vec::new();

    // Breadth-first order and predeclared grid.  Every arm uses all three seeds.
    for &seed in &SEEDS {
        for &(r, sigma) in &[(4usize, 0.0), (6, 0.15)] {
            rows.push(body_row(
                seed,
                "central_h",
                &format!("r={r},sigma={sigma}"),
                2 * r,
                h_law(seed, r, sigma),
                None,
            ));
        }
        for &(n, ball) in &[(5usize, false), (7, true)] {
            rows.push(body_row(
                seed,
                "central_v",
                &format!(
                    "n={n},proposal=±n,source={}",
                    if ball { "ball" } else { "sphere" }
                ),
                2 * n,
                v_law(seed, n, ball),
                None,
            ));
        }
        for &(m, sigma) in &[(4usize, 0.0)] {
            rows.push(body_row(
                seed,
                "zonotope",
                &format!("m={m},lognormal_sigma={sigma}"),
                1usize << m,
                zonotope(seed, m, sigma),
                None,
            ));
        }
        for &n in &[12usize] {
            rows.push(body_row(
                seed,
                "iid_ball_hull",
                &format!("proposal_n={n}"),
                n,
                iid_hull(seed, n),
                None,
            ));
        }
        for name in ["simplex", "cube", "crosspolytope"] {
            let result = affine_control(seed, name);
            let (body, diagonal) = match result {
                Ok((body, d)) => (Ok(body), Some(d)),
                Err(error) => (Err(error), None),
            };
            let proposals = match name {
                "simplex" => 5,
                "cube" => 16,
                "crosspolytope" => 8,
                _ => 0,
            };
            rows.push(body_row(
                seed,
                "affine_control",
                &format!("base={name},law=diag_log_normal_det1"),
                proposals,
                body,
                diagonal,
            ));
        }
    }

    let accepted = rows.iter().filter(|row| row.status == "accepted").count();
    for arm in [
        "central_h",
        "central_v",
        "zonotope",
        "iid_ball_hull",
        "affine_control",
    ] {
        if !rows
            .iter()
            .any(|row| row.arm == arm && row.status == "accepted")
        {
            abandoned.push(arm.to_string());
        }
    }
    write_jsonl(&out_dir.join("rows.jsonl"), &rows)?;
    let manifest = Manifest {
        packet: "generic-4d-laws",
        question: "Can cheap non-product and structured 4D laws broaden combinatorial/geometric coverage beyond IID halfspaces and planar products?",
        seeds: SEEDS.to_vec(),
        grid: vec!["central_h r={4,6}, sigma={0,0.15}", "central_v n={5,7}, sphere/ball", "zonotope m=4, equal lengths (m=5 modest-lognormal abandoned: exact smoke excessive)", "iid_ball_hull proposal_n=12 (proposal_n=20 abandoned: exact smoke excessive)", "affine_control base={simplex,cube,crosspolytope}, diagonal lognormal det=1"],
        source_commit: source_commit(),
        target_fields: vec!["capacity", "sys", "target", "selection_rank"],
        variable_facet_distance: "omitted: no copy-local variable-facet distance contract was cheap enough for this breadth smoke",
        generated_rows: rows.len(),
        accepted_rows: accepted,
        abandoned_arms: abandoned,
    };
    let manifest_path = out_dir.join("manifest.json");
    let manifest_file = File::create(&manifest_path).map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(manifest_file, &manifest).map_err(|e| e.to_string())?;
    let summary = serde_json::json!({
        "generated_rows": rows.len(),
        "accepted_rows": accepted,
        "rejected_rows": rows.len() - accepted,
        "arms": rows.iter().fold(serde_json::Map::new(), |mut map, row| {
            let entry = map.entry(row.arm.clone()).or_insert_with(|| serde_json::json!({"accepted": 0, "rejected": 0}));
            let key = if row.status == "accepted" { "accepted" } else { "rejected" };
            entry[key] = serde_json::json!(entry[key].as_u64().unwrap_or(0) + 1);
            map
        }),
        "wall_seconds_non_deterministic": wall_start.elapsed().as_secs_f64(),
        "note": "Wall time is reported for smoke-cost disposition only; rows.jsonl and manifest parameters are deterministic.",
    });
    let summary_file = File::create(out_dir.join("summary.json")).map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(summary_file, &summary).map_err(|e| e.to_string())?;
    Ok(())
}

fn validate(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path.join("rows.jsonl")).map_err(|e| e.to_string())?;
    let rows: Vec<Row> = text
        .lines()
        .map(|line| serde_json::from_str(line).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    if rows.len() != SEEDS.len() * (2 + 2 + 1 + 1 + 3) {
        return Err(format!("expected 27 rows, found {}", rows.len()));
    }
    if rows.iter().any(|r| r.target_fields_present()) {
        return Err("target field leaked into row".into());
    }
    for row in &rows {
        if row.status == "accepted" {
            let f = row
                .f_vector
                .ok_or_else(|| format!("accepted row {} lacks f-vector", row.row_id))?;
            if f.iter().any(|&x| x == 0) || row.volume.unwrap_or(0.0) <= 0.0 {
                return Err(format!("invalid accepted geometry {}", row.row_id));
            }
            if row.facet_count == 0 || row.retained_vertex_count == 0 {
                return Err(format!("empty accepted geometry {}", row.row_id));
            }
        }
    }
    Ok(())
}

impl Row {
    fn target_fields_present(&self) -> bool {
        let encoded = serde_json::to_string(self).unwrap_or_default();
        ["capacity", "sys", "target", "selection_rank"]
            .iter()
            .any(|field| encoded.contains(&format!("\"{field}\"")))
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "smoke".to_string());
    let default_out =
        PathBuf::from("experiments/sys-datascience/methods/generic-4d-laws/artifacts/smoke");
    let path = args.next().map(PathBuf::from).unwrap_or(default_out);
    let result = match command.as_str() {
        "smoke" => run(&path),
        "validate" => validate(&path),
        _ => Err(format!(
            "usage: generic-4d-laws [smoke [OUT_DIR] | validate OUT_DIR]"
        )),
    };
    if let Err(error) = result {
        eprintln!("generic-4d-laws: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f_vector(body: &Body) -> [usize; 4] {
        [
            body.primal.len(),
            edges_from_vertex_facet_incidence(&body.incidence).len(),
            two_faces_from_vertex_facet_incidence(&body.incidence).len(),
            body.incidence.ncols(),
        ]
    }

    #[test]
    fn h_and_v_fixtures_keep_their_distinct_semantics() {
        let h = h_law(11, 4, 0.0).expect("H fixture");
        let v = v_law(11, 5, false).expect("V fixture");
        assert_eq!(h.dual.len(), 8, "H law retains the requested 2r facets");
        assert!(
            v.dual.len() > 10,
            "V law records its actual hull facet count"
        );
        assert!(central_symmetric(&f64_points(&h.dual)));
        assert!(central_symmetric(&f64_points(&v.primal)));
    }

    #[test]
    fn zonotope_is_a_centrally_symmetric_minkowski_hull() {
        let z = zonotope(11, 4, 0.0).expect("zonotope fixture");
        assert!(central_symmetric(&f64_points(&z.primal)));
        assert!(
            z.primal.len() <= 16,
            "retained vertices cannot exceed 2^m proposals"
        );
        assert_eq!(z.dual.len(), z.incidence.ncols());
    }

    #[test]
    fn affine_controls_preserve_the_base_incidence_fingerprint() {
        for name in ["simplex", "cube", "crosspolytope"] {
            let base = match name {
                "simplex" => symplectic::known_polytopes::simplex(),
                "cube" => symplectic::known_polytopes::hypercube(),
                _ => symplectic::known_polytopes::crosspolytope(),
            };
            let base_body = reconstruct_v(&base.vertices_f64).expect("base reconstruction");
            let (control, _) = affine_control(11, name).expect("affine control");
            assert_eq!(
                f_vector(&control),
                f_vector(&base_body),
                "{name} incidence changed"
            );
        }
    }
}
