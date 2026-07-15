//! Target-free intervention ladder. Matrices act on primal `(q1,q2,p1,p2)`;
//! duals therefore use inverse transpose. Response fields are representations,
//! not a canonical quotient metric.
use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::{Matrix4, Vector2, Vector4};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};
use symplectic::{geom::polygon::random_polygon_2d, omega0};

const ARMS: &[&str] = &[
    "identity",
    "u2-haar",
    "so4-haar",
    "o4-det-minus-haar",
    "so4-align-0",
    "so4-align-pi-over-2",
    "so4-align-pi",
    "sp4-bounded-cartan",
    "sl4-bounded-weyl",
    "fixed-normal-type-cone",
];
const BUCKETS: &[(usize, usize)] = &[(3, 3)];

#[derive(Clone)]
struct Factor {
    n: Vec<Vector2<f64>>,
    h: Vec<f64>,
}
#[derive(Clone)]
struct Map {
    matrix: Option<Matrix4<f64>>,
    explicit_duals: Option<Vec<Vector4<f64>>>,
    params: BTreeMap<String, f64>,
    perturbations: Vec<[f64; 4]>,
    attempts: usize,
    rejections: usize,
}
#[derive(Serialize)]
struct Contract {
    symplectic_structure: bool,
    euclidean_inner_product: bool,
    volume: bool,
    linear_equivalence: bool,
    face_lattice: bool,
    source_incidence: bool,
}
#[derive(Serialize)]
struct Productness {
    coordinate_product: bool,
    lagrangian_product_guaranteed: bool,
    affine_product_equivalent: bool,
    combinatorial_product_preserved: bool,
}
#[derive(Serialize)]
struct Signature {
    raw_ordered_dual_coordinates: Vec<[f64; 4]>,
    dual_norms: Vec<f64>,
    euclidean_gram_upper: Vec<f64>,
    symplectic_gram_upper: Vec<f64>,
    labeled_vertex_facet_incidence: Vec<Vec<usize>>,
}
#[derive(Serialize)]
struct Row {
    schema: &'static str,
    id: String,
    arm: String,
    probability_law: &'static str,
    seed: u64,
    bucket: String,
    base_attempt: Option<usize>,
    intervention_attempts: usize,
    intervention_rejections: usize,
    resolved_parameters: BTreeMap<String, f64>,
    coordinate_order: &'static str,
    primal_action: &'static str,
    dual_action: &'static str,
    preservation_contract: Contract,
    productness: Productness,
    matrix_row_major: Option<[[f64; 4]; 4]>,
    perturbations: Vec<[f64; 4]>,
    condition_number: Option<f64>,
    determinant: Option<f64>,
    orthogonality_residual: Option<f64>,
    symplectic_residual: Option<f64>,
    exact_reconstruction_status: &'static str,
    source_incidence_preserved: Option<bool>,
    base_volume: Option<f64>,
    volume: Option<f64>,
    relative_volume_change: Option<f64>,
    base_signature: Option<Signature>,
    response_signature: Option<Signature>,
    failures: Vec<String>,
    generation_ms: f64,
    intervention_ms: f64,
    reconstruction_ms: f64,
}
#[derive(Serialize)]
struct Report {
    schema: &'static str,
    command: String,
    seed: u64,
    rows: usize,
    passed: usize,
    source_revision: String,
    arms: BTreeMap<String, &'static str>,
    interpretation_boundary: &'static str,
}

fn hash_seed(master: u64, label: &str, b: (usize, usize), row: usize) -> [u8; 32] {
    let mut x = Vec::new();
    x.extend_from_slice(&master.to_le_bytes());
    x.extend_from_slice(label.as_bytes());
    x.extend_from_slice(&(b.0 as u64).to_le_bytes());
    x.extend_from_slice(&(b.1 as u64).to_le_bytes());
    x.extend_from_slice(&(row as u64).to_le_bytes());
    *blake3::hash(&x).as_bytes()
}
fn active(f: &Factor) -> bool {
    for i in 0..f.n.len() {
        let j = (i + 1) % f.n.len();
        let a = f.n[i];
        let b = f.n[j];
        let d = a[0] * b[1] - a[1] * b[0];
        if d.abs() < 1e-12 {
            return false;
        };
        let x = (f.h[i] * b[1] - f.h[j] * a[1]) / d;
        let y = (a[0] * f.h[j] - b[0] * f.h[i]) / d;
        if f.n
            .iter()
            .zip(&f.h)
            .any(|(n, h)| n[0] * x + n[1] * y > *h + 1e-9)
        {
            return false;
        }
    }
    true
}
fn base(
    seed: [u8; 32],
    b: (usize, usize),
) -> (Option<SysLandscapePolytopeCache>, Option<usize>, f64) {
    let t = Instant::now();
    for a in 0..64 {
        let mut r = ChaCha8Rng::from_seed(hash_seed(
            u64::from_le_bytes(seed[..8].try_into().unwrap()),
            "base",
            b,
            a,
        ));
        let (qn, qh) = random_polygon_2d(b.0, 0.8, 1.2, &mut r);
        let (pn, ph) = random_polygon_2d(b.1, 0.8, 1.2, &mut r);
        let q = Factor { n: qn, h: qh };
        let p = Factor { n: pn, h: ph };
        if active(&q) && active(&p) {
            if let Some(x) =
                SysLandscapePolytopeCache::from_lagrangian_product(&q.n, &q.h, &p.n, &p.h)
            {
                return (Some(x), Some(a), t.elapsed().as_secs_f64() * 1e3);
            }
        }
    }
    (None, None, t.elapsed().as_secs_f64() * 1e3)
}
fn j() -> Matrix4<f64> {
    Matrix4::new(
        0., 0., -1., 0., 0., 0., 0., -1., 1., 0., 0., 0., 0., 1., 0., 0.,
    )
}
fn symp(m: &Matrix4<f64>) -> f64 {
    (m.transpose() * j() * m - j()).norm()
}
fn so4(seed: [u8; 32]) -> Matrix4<f64> {
    let mut r = ChaCha8Rng::from_seed(seed);
    let mut q = [[0.; 4]; 4];
    for c in 0..4 {
        let mut v: [f64; 4] = std::array::from_fn(|_| StandardNormal.sample(&mut r));
        for p in q.iter().take(c) {
            let dot: f64 = (0..4).map(|i| p[i] * v[i]).sum();
            for i in 0..4 {
                v[i] -= dot * p[i]
            }
        }
        let n: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        q[c] = v.map(|x| x / n)
    }
    let mut m = Matrix4::from_fn(|i, k| q[k][i]);
    if m.determinant() < 0. {
        for i in 0..4 {
            m[(i, 3)] *= -1.
        }
    }
    m
}

#[derive(Clone, Copy)]
struct C {
    re: f64,
    im: f64,
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
    fn div(self, x: f64) -> Self {
        Self {
            re: self.re / x,
            im: self.im / x,
        }
    }
}
fn u2(seed: [u8; 32]) -> Matrix4<f64> {
    let mut r = ChaCha8Rng::from_seed(seed);
    let z = |r: &mut ChaCha8Rng| C {
        re: StandardNormal.sample(r),
        im: StandardNormal.sample(r),
    };
    let a = [z(&mut r), z(&mut r)];
    let b = [z(&mut r), z(&mut r)];
    let norm = |v: [C; 2]| {
        (v[0].re * v[0].re + v[0].im * v[0].im + v[1].re * v[1].re + v[1].im * v[1].im).sqrt()
    };
    let q0 = a.map(|x| x / norm(a));
    let ip = C {
        re: q0[0].re * b[0].re + q0[0].im * b[0].im + q0[1].re * b[1].re + q0[1].im * b[1].im,
        im: q0[0].re * b[0].im - q0[0].im * b[0].re + q0[1].re * b[1].im - q0[1].im * b[1].re,
    };
    let w = [b[0] - q0[0] * ip, b[1] - q0[1] * ip];
    let q1 = w.map(|x| x / norm(w));
    let q = [[q0[0], q1[0]], [q0[1], q1[1]]];
    Matrix4::from_fn(|i, k| match (i < 2, k < 2) {
        (true, true) => q[i][k].re,
        (true, false) => -q[i][k - 2].im,
        (false, true) => q[i - 2][k].im,
        (false, false) => q[i - 2][k - 2].re,
    })
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
fn signature(p: &SysLandscapePolytopeCache) -> Signature {
    let a = &p.dual_vertices_f64;
    let mut e = Vec::new();
    let mut w = Vec::new();
    for i in 0..a.len() {
        for k in i..a.len() {
            e.push(a[i].dot(&a[k]));
            w.push(omega0(&a[i], &a[k]));
        }
    }
    Signature {
        raw_ordered_dual_coordinates: a.iter().map(|x| [x[0], x[1], x[2], x[3]]).collect(),
        dual_norms: a.iter().map(Vector4::norm).collect(),
        euclidean_gram_upper: e,
        symplectic_gram_upper: w,
        labeled_vertex_facet_incidence: incidence(p),
    }
}
fn contract(arm: &str) -> Contract {
    match arm {
        "identity" | "u2-haar" => Contract {
            symplectic_structure: true,
            euclidean_inner_product: true,
            volume: true,
            linear_equivalence: true,
            face_lattice: true,
            source_incidence: true,
        },
        "sp4-bounded-cartan" => Contract {
            symplectic_structure: true,
            euclidean_inner_product: false,
            volume: true,
            linear_equivalence: true,
            face_lattice: true,
            source_incidence: true,
        },
        "so4-haar"
        | "o4-det-minus-haar"
        | "so4-align-0"
        | "so4-align-pi-over-2"
        | "so4-align-pi" => Contract {
            symplectic_structure: false,
            euclidean_inner_product: true,
            volume: true,
            linear_equivalence: true,
            face_lattice: true,
            source_incidence: true,
        },
        "sl4-bounded-weyl" => Contract {
            symplectic_structure: false,
            euclidean_inner_product: false,
            volume: true,
            linear_equivalence: true,
            face_lattice: true,
            source_incidence: true,
        },
        _ => Contract {
            symplectic_structure: false,
            euclidean_inner_product: false,
            volume: false,
            linear_equivalence: false,
            face_lattice: true,
            source_incidence: true,
        },
    }
}
fn productness(arm: &str) -> Productness {
    match arm {
        "identity" | "fixed-normal-type-cone" => Productness {
            coordinate_product: true,
            lagrangian_product_guaranteed: true,
            affine_product_equivalent: true,
            combinatorial_product_preserved: true,
        },
        "u2-haar" | "sp4-bounded-cartan" | "so4-align-0" | "so4-align-pi" => Productness {
            coordinate_product: false,
            lagrangian_product_guaranteed: true,
            affine_product_equivalent: true,
            combinatorial_product_preserved: true,
        },
        _ => Productness {
            coordinate_product: false,
            lagrangian_product_guaranteed: false,
            affine_product_equivalent: true,
            combinatorial_product_preserved: true,
        },
    }
}

fn law(arm: &str) -> &'static str {
    match arm{"identity"=>"Dirac identity control.","u2-haar"=>"Haar U(2) by complex Gaussian QR in q,p block convention.","so4-haar"=>"Haar SO(4) by real Gaussian QR and determinant correction.","o4-det-minus-haar"=>"Haar det=-1 O(4) component: diag(-1,1,1,1) times Haar SO(4).",a if a.starts_with("so4-align")=>"R=U1 A_theta U2 with independent Haar U(2); A_theta rotates the q-plane. theta=pi gives anti-symplectic diag(-1,-1,1,1) in SO(4).", "sp4-bounded-cartan"=>"a,b iid Uniform[-log 2,log 2]; diag(e^a,e^b,e^-a,e^-b) U, U Haar U(2). Explicit bounded Cartan law, not Haar Sp(4).","sl4-bounded-weyl"=>"Draw x1,x2,x3 iid Uniform[-log2,log2], set x4=-sum xi, reject if |x4|>log2, and sort the four values descending before L diag(e^xi) R for Haar SO(4) L,R. This is an explicit coordinate-dependent pushforward law in one bounded Weyl chamber, not Haar SL(4).",_=>"Fixed normal rays; support multipliers 1+epsilon z_i with |z_i|<=1. epsilon is one quarter of measured minimum inactive incidence-slack fraction, halved until exact labeled incidence survives. Not quotient-transverse."}
}
fn map(
    arm: &str,
    master: u64,
    b: (usize, usize),
    row: usize,
    base: &SysLandscapePolytopeCache,
) -> Option<Map> {
    let seed = hash_seed(master, arm, b, row);
    let mut r = ChaCha8Rng::from_seed(seed);
    let l = 2f64.ln();
    let linear = |m| Map {
        matrix: Some(m),
        explicit_duals: None,
        params: BTreeMap::new(),
        perturbations: Vec::new(),
        attempts: 1,
        rejections: 0,
    };
    match arm {
        "identity" => Some(linear(Matrix4::identity())),
        "u2-haar" => Some(linear(u2(seed))),
        "so4-haar" => Some(linear(so4(seed))),
        "o4-det-minus-haar" => {
            let mut m = so4(seed);
            for k in 0..4 {
                m[(0, k)] *= -1.;
            }
            Some(linear(m))
        }
        a if a.starts_with("so4-align") => {
            let theta = if a.ends_with("0") {
                0.
            } else if a.ends_with("pi-over-2") {
                std::f64::consts::FRAC_PI_2
            } else {
                std::f64::consts::PI
            };
            let (c, s) = (theta.cos(), theta.sin());
            let a = Matrix4::new(c, -s, 0., 0., s, c, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.);
            let mut x = linear(
                u2(hash_seed(master, "align-left", b, row))
                    * a
                    * u2(hash_seed(master, "align-right", b, row)),
            );
            x.params.insert("theta".into(), theta);
            Some(x)
        }
        "sp4-bounded-cartan" => {
            let a = r.gen_range(-l..=l);
            let c = r.gen_range(-l..=l);
            let mut x = linear(
                Matrix4::from_diagonal(&Vector4::new(a.exp(), c.exp(), (-a).exp(), (-c).exp()))
                    * u2(hash_seed(master, "sp-u", b, row)),
            );
            x.params.insert("log_s1".into(), a);
            x.params.insert("log_s2".into(), c);
            Some(x)
        }
        "sl4-bounded-weyl" => {
            for q in 0..64 {
                let x1 = r.gen_range(-l..=l);
                let x2 = r.gen_range(-l..=l);
                let x3 = r.gen_range(-l..=l);
                let x4 = -x1 - x2 - x3;
                if x4.abs() > l {
                    continue;
                }
                let mut exponents = [x1, x2, x3, x4];
                exponents.sort_by(|a, b| b.partial_cmp(a).expect("finite exponents"));
                let mut x = linear(
                    so4(hash_seed(master, "sl-left", b, row + q))
                        * Matrix4::from_diagonal(&Vector4::new(
                            exponents[0].exp(),
                            exponents[1].exp(),
                            exponents[2].exp(),
                            exponents[3].exp(),
                        ))
                        * so4(hash_seed(master, "sl-right", b, row + q)),
                );
                for (i, v) in exponents.iter().enumerate() {
                    x.params.insert(format!("log_s{}", i + 1), *v);
                }
                x.attempts = q + 1;
                x.rejections = q;
                return Some(x);
            }
            None
        }
        _ => {
            let slack = base
                .vertices_f64
                .iter()
                .enumerate()
                .flat_map(|(i, v)| {
                    base.dual_vertices_f64
                        .iter()
                        .enumerate()
                        .filter_map(move |(k, a)| {
                            (!base.vertex_facet_incidence[(i, k)]).then(|| {
                                let n = a.norm();
                                ((n.recip()) - (a / n).dot(v)) * n
                            })
                        })
                })
                .fold(f64::INFINITY, f64::min);
            if !slack.is_finite() || slack <= 0. {
                return None;
            }
            let z: Vec<f64> = base
                .dual_vertices_f64
                .iter()
                .map(|_| r.gen_range(-1.0..=1.0))
                .collect();
            for k in 0..8 {
                let e = 0.25 * slack * 2f64.powi(-(k as i32));
                let ds: Vec<_> = base
                    .dual_vertices_f64
                    .iter()
                    .zip(&z)
                    .map(|(a, z)| a / (1.0 + e * z))
                    .collect();
                if let Some(p) = SysLandscapePolytopeCache::from_f64_dual_vertices(ds) {
                    if incidence(&p) == incidence(base) {
                        let mut params = BTreeMap::new();
                        params.insert("minimum_inactive_slack_fraction".into(), slack);
                        params.insert("epsilon".into(), e);
                        return Some(Map {
                            matrix: None,
                            explicit_duals: Some(p.dual_vertices_f64),
                            params,
                            perturbations: z.iter().map(|z| [*z, 0., 0., 0.]).collect(),
                            attempts: k + 1,
                            rejections: k,
                        });
                    }
                }
            }
            None
        }
    }
}
fn duals(base: &SysLandscapePolytopeCache, m: &Map) -> Option<Vec<Vector4<f64>>> {
    if let Some(x) = &m.explicit_duals {
        Some(x.clone())
    } else {
        let inv = m.matrix?.try_inverse()?.transpose();
        Some(base.dual_vertices_f64.iter().map(|a| inv * a).collect())
    }
}
fn matrix_rows(m: &Matrix4<f64>) -> [[f64; 4]; 4] {
    std::array::from_fn(|i| std::array::from_fn(|k| m[(i, k)]))
}
fn cond(m: &Matrix4<f64>) -> f64 {
    let s = m.svd(false, false).singular_values;
    let lo = s.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    s.iter().fold(0.0_f64, |a, &b| a.max(b)) / lo
}

fn evaluate(
    arm: &str,
    master: u64,
    b: (usize, usize),
    row: usize,
    base: Option<&SysLandscapePolytopeCache>,
    attempt: Option<usize>,
    gen: f64,
) -> Row {
    let id = format!(
        "orbit-zoo-v2/seed={master}/bucket={}x{}/row={row}/arm={arm}",
        b.0, b.1
    );
    let c = contract(arm);
    let pn = productness(arm);
    let Some(base) = base else {
        return Row {
            schema: "orbit-zoo-row-v2",
            id,
            arm: arm.into(),
            probability_law: law(arm),
            seed: master,
            bucket: format!("{}x{}", b.0, b.1),
            base_attempt: None,
            intervention_attempts: 0,
            intervention_rejections: 0,
            resolved_parameters: BTreeMap::new(),
            coordinate_order: "q1,q2,p1,p2",
            primal_action: "not reached",
            dual_action: "not reached",
            preservation_contract: c,
            productness: pn,
            matrix_row_major: None,
            perturbations: Vec::new(),
            condition_number: None,
            determinant: None,
            orthogonality_residual: None,
            symplectic_residual: None,
            exact_reconstruction_status: "base_rejected",
            source_incidence_preserved: None,
            base_volume: None,
            volume: None,
            relative_volume_change: None,
            base_signature: None,
            response_signature: None,
            failures: vec!["base_generation_exhausted".into()],
            generation_ms: gen,
            intervention_ms: 0.,
            reconstruction_ms: 0.,
        };
    };
    let ti = Instant::now();
    let Some(m) = map(arm, master, b, row, base) else {
        return failed(
            id,
            arm,
            master,
            b,
            attempt,
            gen,
            c,
            pn,
            base,
            "map_or_support_law_exhausted",
            ti.elapsed().as_secs_f64() * 1e3,
        );
    };
    let inter = ti.elapsed().as_secs_f64() * 1e3;
    let Some(ds) = duals(base, &m) else {
        return failed(
            id,
            arm,
            master,
            b,
            attempt,
            gen,
            c,
            pn,
            base,
            "inverse_transpose_failed",
            inter,
        );
    };
    let tr = Instant::now();
    let p = SysLandscapePolytopeCache::from_f64_dual_vertices(ds);
    let rec = tr.elapsed().as_secs_f64() * 1e3;
    let Some(p) = p else {
        return failed(
            id,
            arm,
            master,
            b,
            attempt,
            gen,
            c,
            pn,
            base,
            "exact_reconstruction_rejected",
            inter,
        );
    };
    let bv = exact_volume_from_incidence_as_f64(&base.vertices, &base.vertex_facet_incidence);
    let v = exact_volume_from_incidence_as_f64(&p.vertices, &p.vertex_facet_incidence);
    let rel = (v - bv) / bv;
    let inc = incidence(&p) == incidence(base);
    let mut fails = Vec::new();
    if c.source_incidence && !inc {
        fails.push("source_incidence_changed".into())
    }
    if c.volume && rel.abs() > 1e-9 {
        fails.push("volume_changed".into())
    }
    let (matrix, det, orth, sy, cn) = if let Some(x) = &m.matrix {
        let o = (x.transpose() * x - Matrix4::identity()).norm();
        let s = symp(x);
        if c.euclidean_inner_product && o > 1e-10 {
            fails.push("euclidean_contract_failed".into())
        }
        if c.symplectic_structure && s > 1e-10 {
            fails.push("symplectic_contract_failed".into())
        }
        (
            Some(matrix_rows(x)),
            Some(x.determinant()),
            Some(o),
            Some(s),
            Some(cond(x)),
        )
    } else {
        (None, None, None, None, None)
    };
    Row {
        schema: "orbit-zoo-row-v2",
        id,
        arm: arm.into(),
        probability_law: law(arm),
        seed: master,
        bucket: format!("{}x{}", b.0, b.1),
        base_attempt: attempt,
        intervention_attempts: m.attempts,
        intervention_rejections: m.rejections,
        resolved_parameters: m.params,
        coordinate_order: "q1,q2,p1,p2",
        primal_action: if matrix.is_some() {
            "linear map"
        } else {
            "fixed-normal support perturbation"
        },
        dual_action: if matrix.is_some() {
            "inverse transpose"
        } else {
            "rescaled dual normal rays"
        },
        preservation_contract: c,
        productness: pn,
        matrix_row_major: matrix,
        perturbations: m.perturbations,
        condition_number: cn,
        determinant: det,
        orthogonality_residual: orth,
        symplectic_residual: sy,
        exact_reconstruction_status: "reconstructed",
        source_incidence_preserved: Some(inc),
        base_volume: Some(bv),
        volume: Some(v),
        relative_volume_change: Some(rel),
        base_signature: Some(signature(base)),
        response_signature: Some(signature(&p)),
        failures: fails,
        generation_ms: gen,
        intervention_ms: inter,
        reconstruction_ms: rec,
    }
}
fn failed(
    id: String,
    arm: &str,
    seed: u64,
    b: (usize, usize),
    attempt: Option<usize>,
    gen: f64,
    c: Contract,
    pn: Productness,
    base: &SysLandscapePolytopeCache,
    why: &str,
    inter: f64,
) -> Row {
    Row {
        schema: "orbit-zoo-row-v2",
        id,
        arm: arm.into(),
        probability_law: law(arm),
        seed,
        bucket: format!("{}x{}", b.0, b.1),
        base_attempt: attempt,
        intervention_attempts: 0,
        intervention_rejections: 0,
        resolved_parameters: BTreeMap::new(),
        coordinate_order: "q1,q2,p1,p2",
        primal_action: "failed",
        dual_action: "failed",
        preservation_contract: c,
        productness: pn,
        matrix_row_major: None,
        perturbations: Vec::new(),
        condition_number: None,
        determinant: None,
        orthogonality_residual: None,
        symplectic_residual: None,
        exact_reconstruction_status: "rejected",
        source_incidence_preserved: None,
        base_volume: None,
        volume: None,
        relative_volume_change: None,
        base_signature: Some(signature(base)),
        response_signature: None,
        failures: vec![why.into()],
        generation_ms: gen,
        intervention_ms: inter,
        reconstruction_ms: 0.,
    }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out=a.windows(2).find(|x|x[0]=="--out-dir").map(|x|PathBuf::from(&x[1])).unwrap_or_else(||PathBuf::from("experiments/sys-datascience/methods/generator-orbit-perturbation-zoo/artifacts/smoke"));
    let seed = a
        .windows(2)
        .find(|x| x[0] == "--seed")
        .and_then(|x| x[1].parse().ok())
        .unwrap_or(20260715u64);
    create_dir_all(&out).unwrap();
    let mut w = BufWriter::new(File::create(out.join("rows.jsonl")).unwrap());
    let mut rows = Vec::new();
    for &b in BUCKETS {
        let (p, n, t) = base(hash_seed(seed, "base", b, 0), b);
        for arm in ARMS {
            let x = evaluate(arm, seed, b, 0, p.as_ref(), n, t);
            serde_json::to_writer(&mut w, &x).unwrap();
            writeln!(&mut w).unwrap();
            rows.push(x)
        }
    }
    w.flush().unwrap();
    let mut arms = BTreeMap::new();
    for arm in ARMS {
        arms.insert((*arm).into(), law(arm));
    }
    let rev = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .map(|x| String::from_utf8_lossy(&x.stdout).trim().into())
        .unwrap_or_else(|| "unknown".into());
    let report=Report{schema:"orbit-zoo-report-v2",command:a.join(" "),seed,rows:rows.len(),passed:rows.iter().filter(|x|x.failures.is_empty()).count(),source_revision:rev,arms,interpretation_boundary:"Target-free geometry/reconstruction smoke only: it neither evaluates sys nor establishes a canonical metric, quotient-natural law, invariance of sys, or population effect."};
    serde_json::to_writer_pretty(File::create(out.join("report.json")).unwrap(), &report).unwrap();
    if rows.iter().any(|x| !x.failures.is_empty()) {
        std::process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_maps_are_symplectic() {
        assert!(symp(&u2(hash_seed(1, "u", (3, 3), 0))) < 1e-10);
        let a: f64 = 0.3;
        let d = Matrix4::from_diagonal(&Vector4::new(a.exp(), a.exp(), (-a).exp(), (-a).exp()));
        assert!(symp(&d) < 1e-10)
    }
    #[test]
    fn anti_endpoint_is_so4() {
        let m = Matrix4::new(
            -1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        );
        assert!((m.determinant() - 1.0_f64).abs() < 1e-12);
        assert!(symp(&m) > 1.)
    }
}
