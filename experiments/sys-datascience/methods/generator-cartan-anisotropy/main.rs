//! Target-free matched Cartan anisotropy intervention.
//!
//! The producer consumes the reviewed identity rows of the orientation panel.
//! It never regenerates a base and never calls a capacity backend.  For each
//! retained base and each `t in {1,5/4,3/2,2}`, it applies the paired positive
//! diagonal maps
//!
//!   S_t = diag(t,t^-1,t^-1,t),    N_t = diag(t,t^-1,t,t^-1).
//!
//! The two arms have the same Euclidean singular values and determinant, while
//! their canonical symplectic pair weights are respectively `(1,1)` and
//! `(t^2,t^-2)`.  All arithmetic describing the intervention is retained as
//! exact rational strings; reconstruction is performed through the exact polar
//! path, with f64 used only for diagnostic summaries.

use euclidean_polytopes::volume_from_incidence_exact;
use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::{Matrix4, Vector4};
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    fs::{create_dir_all, read, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

const SOURCE_SCHEMA: &str = "generator-orientation-smoke-row-v2";
const ROW_SCHEMA: &str = "generator-cartan-anisotropy-row-v1";
const REPORT_SCHEMA: &str = "generator-cartan-anisotropy-report-v1";
const COORDINATE_ORDER: &str = "q1,q2,p1,p2";
const BUCKETS: &[(usize, usize)] = &[(3, 3), (4, 4), (4, 6), (6, 6)];
const LEVELS: &[Level] = &[
    Level {
        name: "1",
        num: 1,
        den: 1,
    },
    Level {
        name: "5/4",
        num: 5,
        den: 4,
    },
    Level {
        name: "3/2",
        num: 3,
        den: 2,
    },
    Level {
        name: "2",
        num: 2,
        den: 1,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Level {
    name: &'static str,
    num: i64,
    den: i64,
}

#[derive(Clone, Debug)]
struct Args {
    out_dir: PathBuf,
    source: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct SourceRow {
    schema: String,
    map_variant: String,
    base_id: String,
    base_geometry_id: Option<String>,
    transformed_geometry_id: Option<String>,
    bucket: String,
    q_sides: usize,
    p_sides: usize,
    base_accepted: bool,
    transformed_dual_vertices_f64: Vec<[f64; 4]>,
    transformed_dual_vertices_rational: Vec<[String; 4]>,
    labeled_incidence_signature: Vec<Vec<usize>>,
}

#[derive(Clone, Serialize)]
struct Signature {
    dual_norms: Vec<f64>,
    euclidean_gram_upper: Vec<f64>,
    symplectic_gram_upper: Vec<f64>,
    symplectic_gram_upper_exact: Vec<String>,
    omega_sign_signature: Vec<i8>,
}

#[derive(Clone, Serialize)]
struct ExactFeatures {
    facet_count: usize,
    vertex_count: usize,
    volume: f64,
    dual_norm_l1: f64,
    symplectic_gram_l1: f64,
    symplectic_gram_max_abs: f64,
}

#[derive(Clone, Serialize)]
struct EuclideanChecks {
    determinant_one: bool,
    singular_spectrum_control: bool,
    squared_singular_values_exact: Vec<String>,
    singular_values: Vec<f64>,
    euclidean_control_error: f64,
    volume_relative_error: f64,
    exact_volume_matches_base: bool,
}

#[derive(Clone, Serialize)]
struct Row {
    schema: &'static str,
    row_id: String,
    pairing_id: String,
    base_id: String,
    base_geometry_id: String,
    transformed_geometry_id: String,
    bucket: String,
    map_family: &'static str,
    t: String,
    t_num: i64,
    t_den: i64,
    coordinate_order: &'static str,
    primal_action: &'static str,
    dual_action: &'static str,
    matrix_row_major: [[f64; 4]; 4],
    matrix_exact_row_major: [[String; 4]; 4],
    determinant: f64,
    determinant_exact: String,
    singular_values: Vec<f64>,
    squared_singular_values_exact: Vec<String>,
    symplectic_residual_f64: f64,
    symplectic_residual_exact: String,
    canonical_pair_weights_f64: [f64; 2],
    canonical_pair_weights_exact: [String; 2],
    exact_reconstruction_status: &'static str,
    reconstruction_passed: bool,
    incidence_matches_base: bool,
    volume_matches_base: bool,
    euclidean_checks: EuclideanChecks,
    base_signature: Signature,
    response_signature: Signature,
    base_exact_features: ExactFeatures,
    response_exact_features: ExactFeatures,
    transformed_dual_vertices_exact: Vec<[String; 4]>,
    exact_matrix_action_matches: bool,
    failures: Vec<String>,
}

#[derive(Serialize)]
struct PairedRow {
    base_id: String,
    base_geometry_id: String,
    bucket: String,
    t: String,
    s_row_id: String,
    n_row_id: String,
    singular_spectrum_max_abs_delta: f64,
    singular_spectrum_exact_equal: bool,
    symplectic_residual_s: f64,
    symplectic_residual_n: f64,
    symplectic_feature_l1_delta: f64,
    response_difference_l1: f64,
    negative_control_equal_at_t1: bool,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    command: String,
    source_path: String,
    source_input_sha256: String,
    source_revision: String,
    source_repository_tree: String,
    source_dirty_tracked: bool,
    producer_source_sha256: String,
    cargo_lock_sha256: String,
    coordinate_order: &'static str,
    requested_buckets: Vec<String>,
    requested_base_count: usize,
    requested_rows: usize,
    observed_rows: usize,
    passed_rows: usize,
    failure_rows: usize,
    pair_count: usize,
    failures: Vec<String>,
    output_rows_sha256: String,
    output_rows_count: usize,
    output_paired_sha256: String,
    output_paired_count: usize,
    output_paired_tsv_sha256: String,
    regeneration_note: &'static str,
    diagonal_quotient_control: DiagonalQuotientControl,
    exactness_boundary: &'static str,
    interpretation_boundary: &'static str,
}

#[derive(Serialize)]
struct DiagonalQuotientControl {
    input_d_exact: [String; 4],
    input_positive: bool,
    input_determinant_exact: String,
    input_determinant_one: bool,
    t_squared_exact: String,
    symplectic_factor_exact: [[String; 4]; 4],
    symplectic_factor_residual_exact: String,
    symplectic_factor_is_exact: bool,
    nonidentity_factor: bool,
    reconstructed_d_exact: [[String; 4]; 4],
    reconstruction_passed: bool,
    scope: &'static str,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut out_dir = None;
    let mut source = PathBuf::from(
        "experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl",
    );
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--out-dir" => {
                i += 1;
                out_dir = Some(PathBuf::from(argv.get(i).ok_or("--out-dir needs a path")?));
            }
            "--source" => {
                i += 1;
                source = PathBuf::from(argv.get(i).ok_or("--source needs a path")?);
            }
            "--help" => return Err("usage: --out-dir PATH [--source PATH]".into()),
            x => return Err(format!("unknown argument {x}")),
        }
        i += 1;
    }
    Ok(Args {
        out_dir: out_dir.ok_or("--out-dir is required")?,
        source,
    })
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

fn repo_identity() -> (String, String, bool) {
    let revision = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_owned();
    let tree = Command::new("git")
        .args(["rev-parse", "HEAD^{tree}"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_owned();
    let dirty = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(true);
    (revision, tree, dirty)
}

fn rat(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}
fn rat_string(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}
fn f64_string(x: &BigRational) -> String {
    x.to_f64().unwrap_or(f64::NAN).to_string()
}

fn matrix_for(level: Level, family: &str) -> ([[BigRational; 4]; 4], Matrix4<f64>) {
    let t = rat(level.num, level.den);
    let ti = rat(level.den, level.num);
    let d = if family == "S" {
        [t.clone(), ti.clone(), ti, t]
    } else {
        [t.clone(), ti.clone(), t, ti]
    };
    let exact = std::array::from_fn(|i| {
        std::array::from_fn(|j| if i == j { d[i].clone() } else { rat(0, 1) })
    });
    let float = Matrix4::from_diagonal(&Vector4::new(
        d[0].to_f64().unwrap(),
        d[1].to_f64().unwrap(),
        d[2].to_f64().unwrap(),
        d[3].to_f64().unwrap(),
    ));
    (exact, float)
}

fn parse_exact_duals(row: &SourceRow) -> Result<Vec<[BigRational; 4]>, String> {
    row.transformed_dual_vertices_rational
        .iter()
        .map(|a| {
            a.iter()
                .map(|x| BigRational::from_str(x).map_err(|e| format!("invalid rational {x}: {e}")))
                .collect::<Result<Vec<_>, _>>()
                .and_then(|v| {
                    v.try_into()
                        .map_err(|_| "rational dual has wrong dimension".to_owned())
                })
        })
        .collect()
}

fn apply_exact_dual(
    duals: &[[BigRational; 4]],
    matrix: &[[BigRational; 4]; 4],
) -> Vec<[BigRational; 4]> {
    duals
        .iter()
        .map(|a| std::array::from_fn(|i| a[i].clone() / matrix[i][i].clone()))
        .collect()
}

fn exact_matrix_determinant(m: &[[BigRational; 4]; 4]) -> BigRational {
    (0..4)
        .map(|i| m[i][i].clone())
        .fold(rat(1, 1), |x, y| x * y)
}

fn exact_volume(p: &SysLandscapePolytopeCache) -> BigRational {
    let vertices = p
        .vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect::<Vec<_>>();
    volume_from_incidence_exact(&vertices, &p.vertex_facet_incidence)
}

fn j_exact() -> [[BigRational; 4]; 4] {
    std::array::from_fn(|i| {
        std::array::from_fn(|k| match (i, k) {
            (0, 2) | (1, 3) => rat(-1, 1),
            (2, 0) | (3, 1) => rat(1, 1),
            _ => rat(0, 1),
        })
    })
}

fn exact_symplectic_residual(m: &[[BigRational; 4]; 4]) -> String {
    let j = j_exact();
    let mut max = rat(0, 1);
    for i in 0..4 {
        for k in 0..4 {
            let mut x = rat(0, 1);
            for a in 0..4 {
                for b in 0..4 {
                    x += m[a][i].clone() * j[a][b].clone() * m[b][k].clone();
                }
            }
            x -= j[i][k].clone();
            if x.clone().abs() > max {
                max = x.abs();
            }
        }
    }
    rat_string(&max)
}

fn symplectic_residual(m: &Matrix4<f64>) -> f64 {
    let j = Matrix4::new(
        0., 0., -1., 0., 0., 0., 0., -1., 1., 0., 0., 0., 0., 1., 0., 0.,
    );
    (m.transpose() * j * m - j).norm()
}

fn incidence(p: &SysLandscapePolytopeCache) -> Vec<Vec<usize>> {
    let mut x: Vec<Vec<usize>> = (0..p.vertex_facet_incidence.nrows())
        .map(|i| {
            (0..p.vertex_facet_incidence.ncols())
                .filter(|&k| p.vertex_facet_incidence[(i, k)])
                .collect()
        })
        .collect();
    x.sort();
    x
}

fn omega(a: &Vector4<f64>, b: &Vector4<f64>) -> f64 {
    a[2] * b[0] + a[3] * b[1] - a[0] * b[2] - a[1] * b[3]
}

fn omega_exact(a: &[BigRational; 4], b: &[BigRational; 4]) -> BigRational {
    a[2].clone() * b[0].clone() + a[3].clone() * b[1].clone()
        - a[0].clone() * b[2].clone()
        - a[1].clone() * b[3].clone()
}

fn signature(p: &SysLandscapePolytopeCache) -> Signature {
    let a = &p.dual_vertices_f64;
    let ae = &p.dual_vertices;
    let mut e = Vec::new();
    let mut w = Vec::new();
    let mut we = Vec::new();
    let mut signs = Vec::new();
    for i in 0..a.len() {
        for k in i..a.len() {
            e.push(a[i].dot(&a[k]));
            let x = omega(&a[i], &a[k]);
            w.push(x);
            let xe = omega_exact(&ae[i], &ae[k]);
            we.push(rat_string(&xe));
            signs.push((x > 1e-12) as i8 - (x < -1e-12) as i8);
        }
    }
    Signature {
        dual_norms: a.iter().map(Vector4::norm).collect(),
        euclidean_gram_upper: e,
        symplectic_gram_upper: w,
        symplectic_gram_upper_exact: we,
        omega_sign_signature: signs,
    }
}

fn features(p: &SysLandscapePolytopeCache) -> ExactFeatures {
    let sig = signature(p);
    ExactFeatures {
        facet_count: p.dual_vertices.len(),
        vertex_count: p.vertices.len(),
        volume: exact_volume_from_incidence_as_f64(&p.vertices, &p.vertex_facet_incidence),
        dual_norm_l1: sig.dual_norms.iter().sum(),
        symplectic_gram_l1: sig.symplectic_gram_upper.iter().map(|x| x.abs()).sum(),
        symplectic_gram_max_abs: sig
            .symplectic_gram_upper
            .iter()
            .map(|x| x.abs())
            .fold(0.0, f64::max),
    }
}

fn exact_matrix_strings(m: &[[BigRational; 4]; 4]) -> [[String; 4]; 4] {
    std::array::from_fn(|i| std::array::from_fn(|j| rat_string(&m[i][j])))
}
fn spectrum_from_matrix(m: &[[BigRational; 4]; 4]) -> (Vec<String>, Vec<f64>) {
    let mut exact: Vec<BigRational> = (0..4).map(|i| m[i][i].clone() * m[i][i].clone()).collect();
    exact.sort();
    let strings = exact.iter().map(rat_string).collect();
    let floats = exact.iter().map(|x| x.to_f64().unwrap().sqrt()).collect();
    (strings, floats)
}

fn quotient_control() -> DiagonalQuotientControl {
    // Noncanonical positive rational determinant-one diagonal.  In general
    // t^2=d1*d3 and d2*d4=t^-2; here t=2 and A is nonidentity.
    let d = [rat(1, 1), rat(1, 1), rat(4, 1), rat(1, 4)];
    let t = rat(2, 1);
    let a = [
        d[0].clone() / t.clone(),
        d[1].clone() * t.clone(),
        d[2].clone() / t.clone(),
        d[3].clone() * t.clone(),
    ];
    let factor = std::array::from_fn(|i| {
        std::array::from_fn(|j| if i == j { a[i].clone() } else { rat(0, 1) })
    });
    let n = [
        t.clone(),
        rat(1, 1) / t.clone(),
        t.clone(),
        rat(1, 1) / t.clone(),
    ];
    let rec = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            if i == j {
                factor[i][i].clone() * n[i].clone()
            } else {
                rat(0, 1)
            }
        })
    });
    let input_d_exact = std::array::from_fn(|i| rat_string(&d[i]));
    let determinant = d.iter().cloned().fold(rat(1, 1), |x, y| x * y);
    let factor_residual = exact_symplectic_residual(&factor);
    let reconstruction_passed = rec.iter().enumerate().all(|(i, row)| {
        row[i] == d[i]
            && row
                .iter()
                .enumerate()
                .all(|(j, x)| j == i || x == &rat(0, 1))
    });
    DiagonalQuotientControl {
        input_d_exact,
        input_positive: d.iter().all(|x| x > &rat(0, 1)),
        input_determinant_exact: rat_string(&determinant),
        input_determinant_one: determinant == rat(1, 1),
        t_squared_exact: rat_string(&(t.clone() * t)),
        symplectic_factor_exact: exact_matrix_strings(&factor),
        symplectic_factor_residual_exact: factor_residual.clone(),
        symplectic_factor_is_exact: factor_residual == "0/1",
        nonidentity_factor: factor != std::array::from_fn(|i| {
            std::array::from_fn(|j| if i == j { rat(1, 1) } else { rat(0, 1) })
        }),
        reconstructed_d_exact: exact_matrix_strings(&rec),
        reconstruction_passed,
        scope: "positive diagonal determinant-one Cartan quotient only; not a classification of Sp(4)\\SL(4)\\Sp(4)",
    }
}

fn geometry_id(p: &SysLandscapePolytopeCache) -> String {
    let mut payload = String::new();
    for a in &p.dual_vertices {
        for x in a {
            payload.push_str(&rat_string(x));
            payload.push(';');
        }
    }
    payload.push_str(&format!("|{:?}", incidence(p)));
    format!("cartan-geometry-{}", sha256_hex(payload.as_bytes()))
}

fn load_bases(path: &Path) -> Result<(Vec<SourceRow>, String), String> {
    let bytes = read(path).map_err(|e| format!("source read failed: {e}"))?;
    let digest = sha256_hex(&bytes);
    let text = std::str::from_utf8(&bytes).map_err(|_| "source is not UTF-8".to_string())?;
    let mut bases = Vec::new();
    let mut seen = BTreeSet::new();
    for (line_no, line) in text.lines().enumerate() {
        let row: SourceRow =
            serde_json::from_str(line).map_err(|e| format!("source line {}: {e}", line_no + 1))?;
        if row.schema != SOURCE_SCHEMA {
            continue;
        }
        if row.map_variant == "identity" && row.base_accepted {
            if !seen.insert(row.base_id.clone()) {
                return Err(format!("duplicate base identity {}", row.base_id));
            }
            bases.push(row);
        }
    }
    bases.sort_by(|a, b| a.base_id.cmp(&b.base_id));
    let expected: BTreeSet<String> = BUCKETS.iter().map(|(q, p)| format!("{q}x{p}")).collect();
    let mut counts = BTreeMap::<String, usize>::new();
    for b in &bases {
        *counts.entry(b.bucket.clone()).or_default() += 1;
    }
    if bases.len() != 8
        || counts.values().any(|&x| x != 2)
        || counts.keys().any(|k| !expected.contains(k))
    {
        return Err(format!(
            "requested retained panel incomplete: counts={counts:?}"
        ));
    }
    for b in &bases {
        if b.base_geometry_id.is_none()
            || b.transformed_geometry_id.is_none()
            || b.transformed_dual_vertices_f64.is_empty()
            || b.transformed_dual_vertices_rational.len() != b.transformed_dual_vertices_f64.len()
            || b.labeled_incidence_signature.is_empty()
        {
            return Err(format!("base {} lacks exact identity payload", b.base_id));
        }
    }
    Ok((bases, digest))
}

fn row_for(
    base: &SourceRow,
    level: Level,
    family: &str,
    base_cache: &SysLandscapePolytopeCache,
    base_duals_exact: &[[BigRational; 4]],
    base_sig: &Signature,
    base_feat: &ExactFeatures,
) -> Row {
    let (exact_m, m) = matrix_for(level, family);
    let transformed_duals_exact = apply_exact_dual(base_duals_exact, &exact_m);
    let transformed_dual_vertices_exact = transformed_duals_exact
        .iter()
        .map(|a| std::array::from_fn(|i| rat_string(&a[i])))
        .collect::<Vec<_>>();
    let mut failures = Vec::new();
    let reconstructed =
        SysLandscapePolytopeCache::from_rational_dual_vertices(transformed_duals_exact.clone());
    let (
        inc_match,
        vol_match,
        exact_volume_match,
        exact_action_match,
        response_sig,
        response_feat,
        geom_id,
        vol_err,
    ) = if let Some(p) = reconstructed {
        let inc = incidence(&p) == incidence(base_cache)
            && incidence(&p) == base.labeled_incidence_signature;
        let v = exact_volume_from_incidence_as_f64(&p.vertices, &p.vertex_facet_incidence);
        let bv = base_feat.volume;
        let err = (v - bv) / bv;
        let volume_exact_match = exact_volume(&p) == exact_volume(base_cache);
        let action_exact_match = p.dual_vertices == transformed_duals_exact;
        (
            inc,
            err.abs() <= 1e-10,
            volume_exact_match,
            action_exact_match,
            signature(&p),
            features(&p),
            geometry_id(&p),
            err,
        )
    } else {
        failures.push("exact_reconstruction_rejected".into());
        (
            false,
            false,
            false,
            false,
            base_sig.clone(),
            base_feat.clone(),
            "unreconstructed".into(),
            f64::NAN,
        )
    };
    if !inc_match {
        failures.push("incidence_changed_or_identity_mismatch".into());
    }
    if !vol_match {
        failures.push("volume_changed".into());
    }
    if !exact_volume_match {
        failures.push("exact_volume_changed".into());
    }
    if !exact_action_match {
        failures.push("exact_matrix_action_mismatch".into());
    }
    if family == "S"
        && response_sig.symplectic_gram_upper_exact != base_sig.symplectic_gram_upper_exact
    {
        failures.push("symplectic_signature_changed_under_exact_S".into());
    }
    let exact_res = exact_symplectic_residual(&exact_m);
    let residual_f64 = symplectic_residual(&m);
    let (squares_exact, singular_values) = spectrum_from_matrix(&exact_m);
    let pair = [
        exact_m[0][0].clone() * exact_m[2][2].clone(),
        exact_m[1][1].clone() * exact_m[3][3].clone(),
    ];
    let row_id = format!(
        "cartan-anisotropy-v1/base={}/map={family}/t={}",
        base.base_id, level.name
    );
    let pairing_id = format!(
        "cartan-anisotropy-v1/base={}/t={}",
        base.base_id, level.name
    );
    let determinant_exact = exact_matrix_determinant(&exact_m);
    let det = determinant_exact.to_f64().unwrap();
    let det_ok = determinant_exact == rat(1, 1);
    if !det_ok {
        failures.push("determinant_not_one".into());
    }
    let mut matrix = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            matrix[i][j] = m[(i, j)];
        }
    }
    let matrix_exact = exact_matrix_strings(&exact_m);
    let transformed_geometry_id = if level.num == level.den {
        base.base_geometry_id.clone().unwrap()
    } else {
        geom_id
    };
    Row {
        schema: ROW_SCHEMA,
        row_id,
        pairing_id,
        base_id: base.base_id.clone(),
        base_geometry_id: base.base_geometry_id.clone().unwrap(),
        transformed_geometry_id,
        bucket: base.bucket.clone(),
        map_family: if family == "S" {
            "symplectic-control"
        } else {
            "non-symplectic-anisotropy"
        },
        t: level.name.into(),
        t_num: level.num,
        t_den: level.den,
        coordinate_order: COORDINATE_ORDER,
        primal_action: "left multiplication by positive diagonal matrix",
        dual_action: "inverse transpose on dual normals",
        matrix_row_major: matrix,
        matrix_exact_row_major: matrix_exact,
        determinant: det,
        determinant_exact: rat_string(&determinant_exact),
        singular_values: singular_values.clone(),
        squared_singular_values_exact: squares_exact.clone(),
        symplectic_residual_f64: residual_f64,
        symplectic_residual_exact: exact_res,
        canonical_pair_weights_f64: [pair[0].to_f64().unwrap(), pair[1].to_f64().unwrap()],
        canonical_pair_weights_exact: [rat_string(&pair[0]), rat_string(&pair[1])],
        exact_reconstruction_status: if failures
            .iter()
            .any(|x| x == "exact_reconstruction_rejected")
        {
            "rejected"
        } else {
            "reconstructed"
        },
        reconstruction_passed: failures
            .iter()
            .all(|x| x != "exact_reconstruction_rejected"),
        incidence_matches_base: inc_match,
        volume_matches_base: vol_match,
        euclidean_checks: EuclideanChecks {
            determinant_one: det_ok,
            singular_spectrum_control: true,
            squared_singular_values_exact: squares_exact.clone(),
            singular_values: singular_values.clone(),
            euclidean_control_error: 0.0,
            volume_relative_error: vol_err,
            exact_volume_matches_base: exact_volume_match,
        },
        base_signature: base_sig.clone(),
        response_signature: response_sig,
        base_exact_features: base_feat.clone(),
        response_exact_features: response_feat,
        transformed_dual_vertices_exact,
        exact_matrix_action_matches: exact_action_match,
        failures,
    }
}

fn l1_diff(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
}

fn max_abs_diff(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0, f64::max)
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let args = match parse_args(&argv) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    if let Err(e) = run(&args, &argv) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: &Args, _argv: &[String]) -> Result<(), String> {
    create_dir_all(&args.out_dir).map_err(|e| e.to_string())?;
    let (bases, source_hash) = load_bases(&args.source)?;
    let mut rows = Vec::new();
    let mut pairs = Vec::new();
    for base in &bases {
        let base_duals_exact = parse_exact_duals(base)?;
        let cache =
            SysLandscapePolytopeCache::from_rational_dual_vertices(base_duals_exact.clone())
                .ok_or_else(|| format!("base {} exact reconstruction failed", base.base_id))?;
        if incidence(&cache) != base.labeled_incidence_signature {
            return Err(format!("base {} incidence identity mismatch", base.base_id));
        }
        let sig = signature(&cache);
        let feat = features(&cache);
        for level in LEVELS {
            let s = row_for(base, *level, "S", &cache, &base_duals_exact, &sig, &feat);
            let n = row_for(base, *level, "N", &cache, &base_duals_exact, &sig, &feat);
            let p = PairedRow {
                base_id: base.base_id.clone(),
                base_geometry_id: base.base_geometry_id.clone().unwrap(),
                bucket: base.bucket.clone(),
                t: level.name.into(),
                s_row_id: s.row_id.clone(),
                n_row_id: n.row_id.clone(),
                singular_spectrum_max_abs_delta: max_abs_diff(
                    &s.singular_values,
                    &n.singular_values,
                ),
                singular_spectrum_exact_equal: s.squared_singular_values_exact
                    == n.squared_singular_values_exact,
                symplectic_residual_s: s.symplectic_residual_f64,
                symplectic_residual_n: n.symplectic_residual_f64,
                symplectic_feature_l1_delta: (s.response_exact_features.symplectic_gram_l1
                    - n.response_exact_features.symplectic_gram_l1)
                    .abs(),
                response_difference_l1: l1_diff(
                    &s.response_signature.symplectic_gram_upper,
                    &n.response_signature.symplectic_gram_upper,
                ),
                negative_control_equal_at_t1: level.num == level.den
                    && s.matrix_exact_row_major == n.matrix_exact_row_major,
            };
            if !p.singular_spectrum_exact_equal {
                return Err(format!(
                    "matched singular spectrum failed for {} {}",
                    base.base_id, level.name
                ));
            }
            pairs.push(p);
            rows.push(s);
            rows.push(n);
        }
    }
    let rows_path = args.out_dir.join("rows.jsonl");
    let mut w = BufWriter::new(File::create(&rows_path).map_err(|e| e.to_string())?);
    for r in &rows {
        serde_json::to_writer(&mut w, r).map_err(|e| e.to_string())?;
        writeln!(w).map_err(|e| e.to_string())?;
    }
    w.flush().map_err(|e| e.to_string())?;
    let paired_path = args.out_dir.join("paired.jsonl");
    let mut pw = BufWriter::new(File::create(&paired_path).map_err(|e| e.to_string())?);
    for p in &pairs {
        serde_json::to_writer(&mut pw, p).map_err(|e| e.to_string())?;
        writeln!(pw).map_err(|e| e.to_string())?;
    }
    pw.flush().map_err(|e| e.to_string())?;
    let paired_tsv_path = args.out_dir.join("paired.tsv");
    let mut tw = BufWriter::new(File::create(&paired_tsv_path).map_err(|e| e.to_string())?);
    writeln!(tw, "base_id\tbase_geometry_id\tbucket\tt\ts_row_id\tn_row_id\tsingular_spectrum_max_abs_delta\tsingular_spectrum_exact_equal\tsymplectic_residual_s\tsymplectic_residual_n\tsymplectic_feature_l1_delta\tresponse_difference_l1\tnegative_control_equal_at_t1").map_err(|e| e.to_string())?;
    for p in &pairs {
        writeln!(
            tw,
            "{}\t{}\t{}\t{}\t{}\t{}\t{:.17e}\t{}\t{:.17e}\t{:.17e}\t{:.17e}\t{:.17e}\t{}",
            p.base_id,
            p.base_geometry_id,
            p.bucket,
            p.t,
            p.s_row_id,
            p.n_row_id,
            p.singular_spectrum_max_abs_delta,
            p.singular_spectrum_exact_equal,
            p.symplectic_residual_s,
            p.symplectic_residual_n,
            p.symplectic_feature_l1_delta,
            p.response_difference_l1,
            p.negative_control_equal_at_t1
        )
        .map_err(|e| e.to_string())?;
    }
    tw.flush().map_err(|e| e.to_string())?;
    let (rev, tree, dirty) = repo_identity();
    let producer_path =
        Path::new("experiments/sys-datascience/methods/generator-cartan-anisotropy/main.rs");
    let lock_path = Path::new("Cargo.lock");
    let stable_command = "cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-cartan-anisotropy -- --out-dir experiments/sys-datascience/methods/generator-cartan-anisotropy/artifacts/panel-2-per-bucket".to_owned();
    let report = Report { schema: REPORT_SCHEMA, command: stable_command, source_path: args.source.display().to_string(), source_input_sha256: source_hash, source_revision: rev, source_repository_tree: tree, source_dirty_tracked: dirty, producer_source_sha256: sha256_hex(&read(producer_path).map_err(|e|e.to_string())?), cargo_lock_sha256: sha256_hex(&read(lock_path).map_err(|e|e.to_string())?), coordinate_order: COORDINATE_ORDER, requested_buckets: BUCKETS.iter().map(|(q,p)|format!("{q}x{p}")).collect(), requested_base_count: bases.len(), requested_rows: bases.len()*LEVELS.len()*2, observed_rows: rows.len(), passed_rows: rows.iter().filter(|r| r.failures.is_empty()).count(), failure_rows: rows.iter().filter(|r| !r.failures.is_empty()).count(), pair_count: pairs.len(), failures: rows.iter().flat_map(|r|r.failures.clone()).collect(), output_rows_sha256: sha256_hex(&read(&rows_path).map_err(|e|e.to_string())?), output_rows_count: rows.len(), output_paired_sha256: sha256_hex(&read(&paired_path).map_err(|e|e.to_string())?), output_paired_count: pairs.len(), output_paired_tsv_sha256: sha256_hex(&read(&paired_tsv_path).map_err(|e|e.to_string())?), regeneration_note: "Rows were regenerated after cold review found that the prior packet applied inverse diagonals in f64 and rationalized afterward. The retained rows now apply exact rational M^{-T} to the retained rational source payload and reconstruct through the exact polar API; changed hashes are intentional.", diagonal_quotient_control: quotient_control(), exactness_boundary: "Intervention matrices and Cartan pair weights are exact rationals. Retained source rational duals are transformed by exact rational inverse-transpose matrices and reconstructed through the exact polar API; incidence, volume, and symplectic signatures are exact checks. Floating singular values and residuals are diagnostic views.", interpretation_boundary: "Target-free paired geometry only. This packet does not evaluate sys or capacity, estimate population effects, claim an intrinsic Sp(4)\\SL(4)\\Sp(4) distance, classify the full double coset, or treat the symplectic arm as new coverage for orbit-invariant consumers." };
    serde_json::to_writer_pretty(
        File::create(args.out_dir.join("report.json")).map_err(|e| e.to_string())?,
        &report,
    )
    .map_err(|e| e.to_string())?;
    if rows.len() != report.requested_rows || report.failure_rows != 0 {
        return Err(
            "requested Cartan rows were lost or failed; retained report is non-interpretable"
                .into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_ladder_is_exact_and_bounded() {
        assert_eq!(
            LEVELS.iter().map(|x| x.name).collect::<Vec<_>>(),
            vec!["1", "5/4", "3/2", "2"]
        );
        assert!(LEVELS.iter().all(|x| x.num > 0 && x.den > 0));
    }
    #[test]
    fn symplectic_and_non_symplectic_formulas() {
        for l in LEVELS {
            let (s, sm) = matrix_for(*l, "S");
            let (n, nm) = matrix_for(*l, "N");
            assert_eq!(exact_symplectic_residual(&s), "0/1");
            assert!(symplectic_residual(&sm) < 1e-12);
            let expected = if l.num == l.den {
                "0/1".to_owned()
            } else {
                rat_string(&(rat(l.num * l.num, l.den * l.den) - rat(1, 1)).abs())
            };
            assert_eq!(exact_symplectic_residual(&n), expected);
            assert!(nm.determinant() - 1.0 < 1e-12);
        }
    }
    #[test]
    fn paired_singular_spectra_match() {
        for l in LEVELS {
            let (s, _) = matrix_for(*l, "S");
            let (n, _) = matrix_for(*l, "N");
            let a: Vec<_> = (0..4).map(|i| s[i][i].clone() * s[i][i].clone()).collect();
            let b: Vec<_> = (0..4).map(|i| n[i][i].clone() * n[i][i].clone()).collect();
            let mut aa = a;
            let mut bb = b;
            aa.sort();
            bb.sort();
            assert_eq!(aa, bb);
        }
    }
    #[test]
    fn t1_is_negative_control() {
        let (s, _) = matrix_for(LEVELS[0], "S");
        let (n, _) = matrix_for(LEVELS[0], "N");
        assert_eq!(s, n);
    }
    #[test]
    fn diagonal_quotient_control_is_noncanonical_and_exact() {
        let q = quotient_control();
        assert!(q.input_positive);
        assert_eq!(q.input_determinant_exact, "1/1");
        assert!(q.input_determinant_one);
        assert_eq!(q.t_squared_exact, "4/1");
        assert_eq!(q.symplectic_factor_residual_exact, "0/1");
        assert!(q.symplectic_factor_is_exact);
        assert!(q.nonidentity_factor);
        assert!(q.reconstruction_passed);
    }
    #[test]
    fn f64_inverse_path_is_not_exact_at_nonbinary_levels() {
        let x = rat(1, 5);
        for level in &LEVELS[1..3] {
            let exact = x.clone() / rat(level.num, level.den);
            let rounded = BigRational::from_float(
                x.to_f64().unwrap() / (level.num as f64 / level.den as f64),
            )
            .unwrap();
            assert_ne!(
                exact, rounded,
                "old f64 path unexpectedly exact at {}",
                level.name
            );
        }
    }
    #[test]
    fn exact_s_preserves_exact_symplectic_signatures() {
        let base = vec![
            [rat(1, 2), rat(2, 1), rat(0, 1), rat(0, 1)],
            [rat(0, 1), rat(0, 1), rat(3, 1), rat(5, 2)],
        ];
        for level in LEVELS {
            let (matrix, _) = matrix_for(*level, "S");
            let transformed = apply_exact_dual(&base, &matrix);
            assert_eq!(
                omega_exact(&base[0], &base[1]),
                omega_exact(&transformed[0], &transformed[1])
            );
        }
    }
    #[test]
    fn cli_fails_closed_on_unknown() {
        assert!(parse_args(&["x".into(), "--unknown".into()]).is_err());
        assert!(parse_args(&["x".into(), "--out-dir".into()]).is_err());
    }
}
