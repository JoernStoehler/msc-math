//! Controlled target-free SO(4) alignment ladder in `(q1,q2,p1,p2)` order.
use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::{Matrix4, Vector2};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::{
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command,
};
use symplectic::{
    geom::polygon::{polygon_area, random_polygon_2d},
    omega0,
};

const SEED: u64 = 20_260_714;
const ATTEMPTS: usize = 128;
const BUCKETS: &[(usize, usize)] = &[(3, 3), (4, 4), (4, 6), (6, 6)];
const THETAS: &[(&str, f64)] = &[
    ("0", 0.0),
    ("pi_over_4", std::f64::consts::FRAC_PI_4),
    ("pi_over_2", std::f64::consts::FRAC_PI_2),
    ("3pi_over_4", 3.0 * std::f64::consts::FRAC_PI_4),
    ("pi", std::f64::consts::PI),
];
const MATRIX_TOL: f64 = 1e-10;
const VOLUME_TOL: f64 = 1e-9;
const ORIENTATION_SOURCE_REVISION: &str = "8174467dbd171281eb5746480b06629aa41ebfa7";
const ORIENTATION_ROWS_LFS_OID: &str =
    "sha256:b5ded0a5e83d41f35ca035660d222326a161ce5001fd18c12f74f0ed9f3bc367";
const SOURCE_BASE_CONTRACT: &str = "bit-for-bit copied generator-orientation-smoke v1 base law (including area-normalization operation order and geometry-ID byte layout); identity is established only by the analyzer's recorded comparison against the pinned orientation LFS rows";
const REPRODUCTION_COMMAND: &str = "cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-alignment-ladder -- --out-dir experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel";

#[derive(Clone)]
struct Factor {
    normals: Vec<Vector2<f64>>,
    heights: Vec<f64>,
}
#[derive(Clone, Copy, Default)]
struct C {
    re: f64,
    im: f64,
}
impl C {
    fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
    fn norm2(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}
impl std::ops::Add for C {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self {
            re: self.re + b.re,
            im: self.im + b.im,
        }
    }
}
impl std::ops::Sub for C {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self {
            re: self.re - b.re,
            im: self.im - b.im,
        }
    }
}
impl std::ops::Mul for C {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self {
            re: self.re * b.re - self.im * b.im,
            im: self.re * b.im + self.im * b.re,
        }
    }
}
impl std::ops::Div<f64> for C {
    type Output = Self;
    fn div(self, b: f64) -> Self {
        Self {
            re: self.re / b,
            im: self.im / b,
        }
    }
}

#[derive(Serialize)]
struct Signature {
    raw_ordered_dual_coordinates: Vec<[f64; 4]>,
    euclidean_gram_upper: Vec<f64>,
    symplectic_gram_upper: Vec<f64>,
    omega_signs: Vec<i8>,
}
#[derive(Serialize)]
struct Row {
    schema: &'static str,
    id: String,
    base_id: String,
    source_base_contract: &'static str,
    bucket: String,
    row_index: usize,
    seed: u64,
    base_seed: u64,
    accepted_base_attempt: Option<usize>,
    theta_label: &'static str,
    theta_radians: f64,
    kahler_departure_sin_sq_half_theta: f64,
    left_u2_seed: u64,
    right_u2_seed: u64,
    coordinate_order: &'static str,
    a_theta_convention: &'static str,
    primal_action: &'static str,
    dual_action: &'static str,
    matrix_row_major: Option<[[f64; 4]; 4]>,
    determinant: Option<f64>,
    orthogonality_residual: Option<f64>,
    symplectic_residual: Option<f64>,
    anti_symplectic_residual: Option<f64>,
    condition_number: Option<f64>,
    exact_reconstruction_status: &'static str,
    source_incidence_preserved: Option<bool>,
    exact_base_volume: Option<f64>,
    exact_response_volume: Option<f64>,
    relative_volume_change: Option<f64>,
    base_geometry_id: Option<String>,
    base_signature: Option<Signature>,
    response_signature: Option<Signature>,
    euclidean_gram_max_abs_change: Option<f64>,
    symplectic_gram_l2_change: Option<f64>,
    symplectic_gram_max_abs_change: Option<f64>,
    failures: Vec<String>,
}
#[derive(Serialize)]
struct Report {
    schema: &'static str,
    command: String,
    source_revision: String,
    source_repository_tree: String,
    source_dirty: bool,
    producer_source_sha256: String,
    analyzer_source_sha256: String,
    cargo_lock_sha256: String,
    source_base_contract: &'static str,
    orientation_source_revision: &'static str,
    orientation_rows_lfs_oid: &'static str,
    orientation_geometry_id_comparison_status: &'static str,
    expected_bases: usize,
    expected_rows: usize,
    observed_rows: usize,
    passed_rows: usize,
    all_requested_rows_passed: bool,
    formula_controls_passed: bool,
    coordinate_order: &'static str,
    a_theta_convention: &'static str,
    kahler_departure_coordinate: &'static str,
    proof_review_cruxes: Vec<&'static str>,
    interpretation_boundary: &'static str,
}

fn derive_seed(
    master: u64,
    label: &str,
    b: (usize, usize),
    row: usize,
    attempt: usize,
) -> [u8; 32] {
    let mut x = Vec::new();
    x.extend_from_slice(&master.to_le_bytes());
    x.extend_from_slice(label.as_bytes());
    x.push(0);
    x.extend_from_slice(&(b.0 as u64).to_le_bytes());
    x.extend_from_slice(&(b.1 as u64).to_le_bytes());
    x.extend_from_slice(&(row as u64).to_le_bytes());
    x.extend_from_slice(&(attempt as u64).to_le_bytes());
    *blake3::hash(&x).as_bytes()
}
fn seed_u64(x: [u8; 32]) -> u64 {
    u64::from_le_bytes(x[..8].try_into().unwrap())
}
fn all_active(f: &Factor) -> bool {
    (0..f.normals.len()).all(|i| {
        let j = (i + 1) % f.normals.len();
        let a = f.normals[i];
        let b = f.normals[j];
        let d = a[0] * b[1] - a[1] * b[0];
        if d.abs() < 1e-12 {
            return false;
        };
        let x = (f.heights[i] * b[1] - f.heights[j] * a[1]) / d;
        let y = (a[0] * f.heights[j] - b[0] * f.heights[i]) / d;
        !f.normals
            .iter()
            .zip(&f.heights)
            .any(|(n, h)| n[0] * x + n[1] * y > *h + 1e-9)
    })
}
fn factor(sides: usize, seed: [u8; 32]) -> Option<Factor> {
    let mut r = ChaCha8Rng::from_seed(seed);
    let (n, mut h) = random_polygon_2d(sides, 0.8, 1.2, &mut r);
    let f = Factor {
        normals: n,
        heights: h.clone(),
    };
    if !all_active(&f) {
        return None;
    };
    let a = polygon_area(&f.normals, &h)?;
    if !a.is_finite() || a <= 0. {
        return None;
    };
    // Match generator-orientation-smoke's floating-point operation order so
    // binary-rational reconstruction can reproduce its base geometry IDs.
    let scale = a.sqrt().recip();
    h.iter_mut().for_each(|height| *height *= scale);
    let f = Factor {
        normals: f.normals,
        heights: h,
    };
    (polygon_area(&f.normals, &f.heights)?.is_finite()
        && (polygon_area(&f.normals, &f.heights)? - 1.).abs() <= 1e-10)
        .then_some(f)
}
fn base(b: (usize, usize), row: usize) -> (Option<SysLandscapePolytopeCache>, Option<usize>, u64) {
    let base_seed = seed_u64(derive_seed(SEED, "base", b, row, 0));
    for a in 0..ATTEMPTS {
        let q = factor(b.0, derive_seed(SEED, "base-q", b, row, a));
        let p = factor(b.1, derive_seed(SEED, "base-p", b, row, a));
        if let Some((q, p)) = q.zip(p) {
            if let Some(x) = SysLandscapePolytopeCache::from_lagrangian_product(
                &q.normals, &q.heights, &p.normals, &p.heights,
            ) {
                return (Some(x), Some(a), base_seed);
            }
        }
    }
    (None, None, base_seed)
}
fn inner(a: &[C; 2], b: &[C; 2]) -> C {
    a.iter()
        .zip(b)
        .fold(C::default(), |s, (&x, &y)| s + x.conj() * y)
}
fn norm(v: [C; 2]) -> Option<[C; 2]> {
    let n = v.iter().map(|x| x.norm2()).sum::<f64>().sqrt();
    (n > 1e-14 && n.is_finite()).then(|| v.map(|x| x / n))
}
fn u2(seed: [u8; 32]) -> Option<Matrix4<f64>> {
    let mut r = ChaCha8Rng::from_seed(seed);
    let z = |r: &mut ChaCha8Rng| C {
        re: StandardNormal.sample(r),
        im: StandardNormal.sample(r),
    };
    let a = [z(&mut r), z(&mut r)];
    let b = [z(&mut r), z(&mut r)];
    let q0 = norm(a)?;
    let q1 = norm([b[0] - q0[0] * inner(&q0, &b), b[1] - q0[1] * inner(&q0, &b)])?;
    let q = [[q0[0], q1[0]], [q0[1], q1[1]]];
    Some(Matrix4::from_fn(|i, k| match (i < 2, k < 2) {
        (true, true) => q[i][k].re,
        (true, false) => -q[i][k - 2].im,
        (false, true) => q[i - 2][k].im,
        (false, false) => q[i - 2][k - 2].re,
    }))
}
fn j() -> Matrix4<f64> {
    Matrix4::new(
        0., 0., -1., 0., 0., 0., 0., -1., 1., 0., 0., 0., 0., 1., 0., 0.,
    )
}
fn a_theta(theta: f64) -> Matrix4<f64> {
    let (c, s) = (theta.cos(), theta.sin());
    Matrix4::new(c, -s, 0., 0., s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.)
}
fn orth(m: &Matrix4<f64>) -> f64 {
    (m.transpose() * m - Matrix4::identity()).norm()
}
fn symp(m: &Matrix4<f64>) -> f64 {
    (m.transpose() * j() * m - j()).norm()
}
fn anti(m: &Matrix4<f64>) -> f64 {
    (m.transpose() * j() * m + j()).norm()
}
fn rows(m: &Matrix4<f64>) -> [[f64; 4]; 4] {
    std::array::from_fn(|i| std::array::from_fn(|k| m[(i, k)]))
}
fn cond(m: &Matrix4<f64>) -> f64 {
    let s = m.svd(false, false).singular_values;
    let lo = s.iter().fold(f64::INFINITY, |a, &x| a.min(x));
    s.iter().fold(0.0_f64, |a, &x| a.max(x)) / lo
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
fn geometry_id(p: &SysLandscapePolytopeCache) -> String {
    let mut h = blake3::Hasher::new();
    for v in &p.dual_vertices {
        for x in v {
            h.update(x.numer().to_string().as_bytes());
            h.update(b"/");
            h.update(x.denom().to_string().as_bytes());
            h.update(b";");
        }
        h.update(b"|");
    }
    h.finalize().to_hex().to_string()
}
fn signature(p: &SysLandscapePolytopeCache) -> Signature {
    let mut e = Vec::new();
    let mut w = Vec::new();
    for i in 0..p.dual_vertices_f64.len() {
        for k in i..p.dual_vertices_f64.len() {
            e.push(p.dual_vertices_f64[i].dot(&p.dual_vertices_f64[k]));
            w.push(omega0(&p.dual_vertices_f64[i], &p.dual_vertices_f64[k]));
        }
    }
    Signature {
        raw_ordered_dual_coordinates: p
            .dual_vertices_f64
            .iter()
            .map(|x| [x[0], x[1], x[2], x[3]])
            .collect(),
        euclidean_gram_upper: e,
        symplectic_gram_upper: w,
        omega_signs: (0..p.omega_signs.nrows())
            .flat_map(|i| (0..p.omega_signs.ncols()).map(move |k| p.omega_signs[(i, k)]))
            .collect(),
    }
}
fn delta(a: &[f64], b: &[f64]) -> (f64, f64) {
    let mut mx: f64 = 0.;
    let mut sq = 0.;
    for (x, y) in a.iter().zip(b) {
        let d = x - y;
        mx = mx.max(d.abs());
        sq += d * d
    }
    (mx, sq.sqrt())
}
fn empty(
    b: (usize, usize),
    row: usize,
    label: &'static str,
    theta: f64,
    base_seed: u64,
    attempt: Option<usize>,
) -> Row {
    let name = format!("{}x{}", b.0, b.1);
    let id = format!("alignment-ladder-v1/seed={SEED}/bucket={name}/row={row}/theta={label}");
    Row {
        schema: "alignment-ladder-row-v1",
        id,
        base_id: format!("generator-orientation-v1/base/seed={SEED}/bucket={name}/row={row}"),
        source_base_contract: SOURCE_BASE_CONTRACT,
        bucket: name,
        row_index: row,
        seed: SEED,
        base_seed,
        accepted_base_attempt: attempt,
        theta_label: label,
        theta_radians: theta,
        kahler_departure_sin_sq_half_theta: (theta / 2.).sin().powi(2),
        left_u2_seed: seed_u64(derive_seed(SEED, "alignment-left-u2", b, row, 0)),
        right_u2_seed: seed_u64(derive_seed(SEED, "alignment-right-u2", b, row, 0)),
        coordinate_order: "q1,q2,p1,p2",
        a_theta_convention: "diag(Q(theta),I_2), Q rotates (q1,q2) positively",
        primal_action: "not reached",
        dual_action: "not reached",
        matrix_row_major: None,
        determinant: None,
        orthogonality_residual: None,
        symplectic_residual: None,
        anti_symplectic_residual: None,
        condition_number: None,
        exact_reconstruction_status: "base_rejected",
        source_incidence_preserved: None,
        exact_base_volume: None,
        exact_response_volume: None,
        relative_volume_change: None,
        base_geometry_id: None,
        base_signature: None,
        response_signature: None,
        euclidean_gram_max_abs_change: None,
        symplectic_gram_l2_change: None,
        symplectic_gram_max_abs_change: None,
        failures: vec!["base_generation_exhausted".into()],
    }
}
fn failed_after_base(
    b: (usize, usize),
    row: usize,
    label: &'static str,
    theta: f64,
    base: &SysLandscapePolytopeCache,
    attempt: usize,
    base_seed: u64,
    status: &'static str,
    failure: &'static str,
) -> Row {
    let mut failed = empty(b, row, label, theta, base_seed, Some(attempt));
    failed.exact_reconstruction_status = status;
    failed.base_geometry_id = Some(geometry_id(base));
    failed.base_signature = Some(signature(base));
    failed.primal_action = "map generation/reconstruction failed";
    failed.dual_action = "not reached";
    failed.failures = vec![failure.into()];
    failed
}
fn evaluate(
    b: (usize, usize),
    row: usize,
    label: &'static str,
    theta: f64,
    base: &SysLandscapePolytopeCache,
    attempt: usize,
    base_seed: u64,
) -> Row {
    let name = format!("{}x{}", b.0, b.1);
    let id = format!("alignment-ladder-v1/seed={SEED}/bucket={name}/row={row}/theta={label}");
    let ls = derive_seed(SEED, "alignment-left-u2", b, row, 0);
    let rs = derive_seed(SEED, "alignment-right-u2", b, row, 0);
    let Some(left) = u2(ls) else {
        return failed_after_base(
            b,
            row,
            label,
            theta,
            base,
            attempt,
            base_seed,
            "map_rejected",
            "left_u2_generation_rejected",
        );
    };
    let Some(right) = u2(rs) else {
        return failed_after_base(
            b,
            row,
            label,
            theta,
            base,
            attempt,
            base_seed,
            "map_rejected",
            "right_u2_generation_rejected",
        );
    };
    let m = left * a_theta(theta) * right;
    let Some(inv) = m.try_inverse() else {
        return failed_after_base(
            b,
            row,
            label,
            theta,
            base,
            attempt,
            base_seed,
            "inverse_transpose_rejected",
            "inverse_transpose_failed",
        );
    };
    let Some(p) = SysLandscapePolytopeCache::from_f64_dual_vertices(
        base.dual_vertices_f64
            .iter()
            .map(|x| inv.transpose() * x)
            .collect(),
    ) else {
        return failed_after_base(
            b,
            row,
            label,
            theta,
            base,
            attempt,
            base_seed,
            "exact_reconstruction_rejected",
            "exact_reconstruction_rejected",
        );
    };
    let bv = exact_volume_from_incidence_as_f64(&base.vertices, &base.vertex_facet_incidence);
    let v = exact_volume_from_incidence_as_f64(&p.vertices, &p.vertex_facet_incidence);
    let rel = (v - bv) / bv;
    let bs = signature(base);
    let ps = signature(&p);
    let (euclid, _) = delta(&bs.euclidean_gram_upper, &ps.euclidean_gram_upper);
    let (symmax, syml2) = delta(&bs.symplectic_gram_upper, &ps.symplectic_gram_upper);
    let mut failures = Vec::new();
    let o = orth(&m);
    let d = m.determinant();
    let s = symp(&m);
    let a = anti(&m);
    let inc = incidence(&p) == incidence(base);
    if o > MATRIX_TOL {
        failures.push("orthogonality_contract_failed".into())
    }
    if (d - 1.).abs() > MATRIX_TOL {
        failures.push("determinant_contract_failed".into())
    }
    if !inc {
        failures.push("source_incidence_changed".into())
    }
    if rel.abs() > VOLUME_TOL {
        failures.push("volume_changed".into())
    }
    if euclid > MATRIX_TOL {
        failures.push("euclidean_control_changed".into())
    }
    Row {
        schema: "alignment-ladder-row-v1",
        id,
        base_id: format!(
            "generator-orientation-v1/base/seed={SEED}/bucket={name}/row={row}/attempt={attempt}"
        ),
        source_base_contract: SOURCE_BASE_CONTRACT,
        bucket: name,
        row_index: row,
        seed: SEED,
        base_seed,
        accepted_base_attempt: Some(attempt),
        theta_label: label,
        theta_radians: theta,
        kahler_departure_sin_sq_half_theta: (theta / 2.).sin().powi(2),
        left_u2_seed: seed_u64(ls),
        right_u2_seed: seed_u64(rs),
        coordinate_order: "q1,q2,p1,p2",
        a_theta_convention: "diag(Q(theta),I_2), Q rotates (q1,q2) positively",
        primal_action: "R_theta = U1 A_theta U2",
        dual_action: "inverse_transpose",
        matrix_row_major: Some(rows(&m)),
        determinant: Some(d),
        orthogonality_residual: Some(o),
        symplectic_residual: Some(s),
        anti_symplectic_residual: Some(a),
        condition_number: Some(cond(&m)),
        exact_reconstruction_status: "reconstructed",
        source_incidence_preserved: Some(inc),
        exact_base_volume: Some(bv),
        exact_response_volume: Some(v),
        relative_volume_change: Some(rel),
        base_geometry_id: Some(geometry_id(base)),
        base_signature: Some(bs),
        response_signature: Some(ps),
        euclidean_gram_max_abs_change: Some(euclid),
        symplectic_gram_l2_change: Some(syml2),
        symplectic_gram_max_abs_change: Some(symmax),
        failures,
    }
}
fn formula_controls() -> bool {
    THETAS.iter().all(|(_, t)| {
        let m = a_theta(*t);
        orth(&m) < 1e-12 && (m.determinant() - 1.).abs() < 1e-12
    }) && symp(&a_theta(0.)) < 1e-12
        && anti(&a_theta(std::f64::consts::PI)) < 1e-12
        && symp(&a_theta(std::f64::consts::PI)) > 1.
        && symp(&a_theta(std::f64::consts::FRAC_PI_2)) > 0.1
        && anti(&a_theta(std::f64::consts::FRAC_PI_2)) > 0.1
        && (symp(&a_theta(std::f64::consts::FRAC_PI_4))
            - symp(&a_theta(-std::f64::consts::FRAC_PI_4)))
        .abs()
            < 1e-12
}
fn git(args: &[&str]) -> String {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|x| x.status.success())
        .map(|x| String::from_utf8_lossy(&x.stdout).trim().into())
        .unwrap_or_else(|| "unknown".into())
}
fn sha(p: &str) -> String {
    Command::new("sha256sum")
        .arg(p)
        .output()
        .ok()
        .filter(|x| x.status.success())
        .and_then(|x| String::from_utf8(x.stdout).ok())
        .and_then(|x| x.split_whitespace().next().map(str::to_owned))
        .unwrap_or_else(|| "unavailable".into())
}
fn main() {
    let mut out = PathBuf::from(
        "experiments/sys-datascience/methods/generator-alignment-ladder/artifacts/panel",
    );
    let a: Vec<String> = std::env::args().collect();
    if a.len() == 3 && a[1] == "--out-dir" {
        out = PathBuf::from(&a[2])
    } else if a.len() != 1 {
        eprintln!("usage: --out-dir DIR");
        std::process::exit(2)
    }
    let revision = git(&["rev-parse", "HEAD"]);
    let tree = git(&["rev-parse", "HEAD^{tree}"]);
    let dirty = !git(&["status", "--porcelain", "--untracked-files=no"]).is_empty();
    create_dir_all(&out).unwrap();
    let mut w = BufWriter::new(File::create(out.join("rows.jsonl")).unwrap());
    let mut all = Vec::new();
    for &b in BUCKETS {
        for row in 0..2 {
            let (x, attempt, seed) = base(b, row);
            for &(label, theta) in THETAS {
                let r = match (&x, attempt) {
                    (Some(x), Some(a)) => evaluate(b, row, label, theta, x, a, seed),
                    _ => empty(b, row, label, theta, seed, attempt),
                };
                serde_json::to_writer(&mut w, &r).unwrap();
                writeln!(w).unwrap();
                all.push(r)
            }
        }
    }
    w.flush().unwrap();
    let pass = formula_controls()
        && all.len() == 40
        && all
            .iter()
            .all(|r| r.exact_reconstruction_status == "reconstructed" && r.failures.is_empty());
    let report=Report{schema:"alignment-ladder-report-v1",command:REPRODUCTION_COMMAND.into(),source_revision:revision,source_repository_tree:tree,source_dirty:dirty,producer_source_sha256:sha("experiments/sys-datascience/methods/generator-alignment-ladder/main.rs"),analyzer_source_sha256:sha("experiments/sys-datascience/methods/generator-alignment-ladder/analyze.py"),cargo_lock_sha256:sha("Cargo.lock"),source_base_contract:SOURCE_BASE_CONTRACT,orientation_source_revision:ORIENTATION_SOURCE_REVISION,orientation_rows_lfs_oid:ORIENTATION_ROWS_LFS_OID,orientation_geometry_id_comparison_status:"pending_external_analyzer_comparison",expected_bases:8,expected_rows:40,observed_rows:all.len(),passed_rows:all.iter().filter(|r|r.failures.is_empty()).count(),all_requested_rows_passed:pass,formula_controls_passed:formula_controls(),coordinate_order:"q1,q2,p1,p2",a_theta_convention:"A_theta=diag(Q(theta),I_2), Q(theta) rotates q1,q2; A_pi=diag(-1,-1,1,1)",kahler_departure_coordinate:"sin^2(theta/2)",proof_review_cruxes:vec!["Do not promote this representative family to an exhaustion or unique parameterization of U(2)\\SO(4)/U(2) without proof review.","Do not assume anti-symplectic capacity invariance; review the capacity definition and proof separately before later target interpretation."],interpretation_boundary:"Target-free finite-panel geometry only: no sys, capacity, target-derived field, capacity dose claim, population claim, or quotient-natural-law claim."};
    serde_json::to_writer_pretty(File::create(out.join("report.json")).unwrap(), &report).unwrap();
    if !pass {
        std::process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formula_contracts_all_five_angles_and_reverse_control() {
        assert!(formula_controls());
        for (_, t) in THETAS {
            let m = a_theta(*t);
            assert!(orth(&m) < 1e-12);
            assert!((m.determinant() - 1.).abs() < 1e-12)
        }
        assert!(anti(&a_theta(std::f64::consts::PI)) < 1e-12);
        assert!(
            (a_theta(std::f64::consts::PI)
                - Matrix4::new(-1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.))
            .norm()
                < 1e-12
        )
    }
    #[test]
    fn u2_is_symplectic_and_same_seeds_are_theta_independent() {
        let s = derive_seed(SEED, "alignment-left-u2", (4, 6), 1, 0);
        let u = u2(s).unwrap();
        assert!(orth(&u) < 1e-12);
        assert!(symp(&u) < 1e-12);
        assert_eq!(
            seed_u64(s),
            seed_u64(derive_seed(SEED, "alignment-left-u2", (4, 6), 1, 0))
        );
    }
    #[test]
    fn regenerated_source_base_is_available() {
        assert!(base((3, 3), 0).0.is_some());
    }
    #[test]
    fn retained_contract_excludes_timing_and_uses_repo_relative_command() {
        let (base, attempt, base_seed) = base((3, 3), 0);
        let row = evaluate(
            (3, 3),
            0,
            "0",
            0.0,
            base.as_ref().expect("fixture base"),
            attempt.expect("fixture attempt"),
            base_seed,
        );
        let object = serde_json::to_value(row).expect("serializable row");
        assert!(object.get("generation_ms").is_none());
        assert!(object.get("reconstruction_ms").is_none());
        assert!(!REPRODUCTION_COMMAND.starts_with('/'));
        assert!(!REPRODUCTION_COMMAND.contains("/workspaces/"));
        assert!(REPRODUCTION_COMMAND.starts_with("cargo run -p "));
    }
}
