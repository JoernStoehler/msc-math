//! Exact, target-free smoke for fixed-normal boundary fractions and productness.
//! Coordinates are `(q1,q2,p1,p2)` and inequalities are `n_i^T x <= h_i`.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    process::Command,
};

type Q = BigRational;
type VecQ = Vec<Q>;
type MatQ = Vec<VecQ>;

const COORDINATE_ORDER: &str = "q1,q2,p1,p2";
const LOCAL_SYSTEM: &str = "intended-simple-vertex inactive-slack local sufficient system";
const PRODUCER_PATH: &str =
    "experiments/sys-datascience/methods/generator-type-cone-productness/main.rs";

fn qi(x: i64) -> Q {
    Q::from_integer(BigInt::from(x))
}
fn qr(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}
fn qs(x: &Q) -> String {
    if x.denom() == &BigInt::one() {
        x.numer().to_string()
    } else {
        format!("{}/{}", x.numer(), x.denom())
    }
}
fn qvec(xs: &[Q]) -> Vec<String> {
    xs.iter().map(qs).collect()
}
fn qmat(xs: &[VecQ]) -> Vec<Vec<String>> {
    xs.iter().map(|x| qvec(x)).collect()
}
fn dot(a: &[Q], b: &[Q]) -> Q {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn identity(n: usize) -> MatQ {
    (0..n)
        .map(|i| (0..n).map(|j| if i == j { qi(1) } else { qi(0) }).collect())
        .collect()
}
fn transpose(a: &[VecQ]) -> MatQ {
    (0..a[0].len())
        .map(|j| (0..a.len()).map(|i| a[i][j].clone()).collect())
        .collect()
}
fn mat_vec(a: &[VecQ], x: &[Q]) -> VecQ {
    a.iter().map(|row| dot(row, x)).collect()
}
fn mat_mul(a: &[VecQ], b: &[VecQ]) -> MatQ {
    let bt = transpose(b);
    a.iter()
        .map(|row| bt.iter().map(|col| dot(row, col)).collect())
        .collect()
}

fn solve(a: &[VecQ], b: &[Q]) -> Option<VecQ> {
    let n = a.len();
    if n == 0 || a.iter().any(|row| row.len() != n) || b.len() != n {
        return None;
    }
    let mut aug: MatQ = a
        .iter()
        .zip(b)
        .map(|(row, rhs)| {
            let mut out = row.clone();
            out.push(rhs.clone());
            out
        })
        .collect();
    for col in 0..n {
        let pivot = (col..n).find(|&row| !aug[row][col].is_zero())?;
        aug.swap(col, pivot);
        let scale = aug[col][col].clone();
        for j in col..=n {
            aug[col][j] /= scale.clone();
        }
        for row in 0..n {
            if row == col || aug[row][col].is_zero() {
                continue;
            }
            let factor = aug[row][col].clone();
            for j in col..=n {
                let subtract = factor.clone() * aug[col][j].clone();
                aug[row][j] -= subtract;
            }
        }
    }
    Some(aug.into_iter().map(|row| row[n].clone()).collect())
}

fn inverse(a: &[VecQ]) -> Option<MatQ> {
    let n = a.len();
    let mut columns = Vec::new();
    for j in 0..n {
        let rhs: VecQ = (0..n).map(|i| if i == j { qi(1) } else { qi(0) }).collect();
        columns.push(solve(a, &rhs)?);
    }
    Some(transpose(&columns))
}

fn determinant(a: &[VecQ]) -> Q {
    let n = a.len();
    let mut m = a.to_vec();
    let mut det = qi(1);
    for col in 0..n {
        let Some(pivot) = (col..n).find(|&row| !m[row][col].is_zero()) else {
            return qi(0);
        };
        if pivot != col {
            m.swap(pivot, col);
            det = -det;
        }
        let p = m[col][col].clone();
        det *= p.clone();
        for row in (col + 1)..n {
            if m[row][col].is_zero() {
                continue;
            }
            let factor = m[row][col].clone() / p.clone();
            for j in col..n {
                let subtract = factor.clone() * m[col][j].clone();
                m[row][j] -= subtract;
            }
        }
    }
    det
}

#[derive(Clone)]
struct Fixture {
    name: &'static str,
    k: usize,
    m: usize,
    normals: MatQ,
    supports: VecQ,
    intended_incidence: Vec<Vec<usize>>,
}

fn polygon(kind: usize) -> (Vec<[i64; 2]>, Vec<i64>) {
    match kind {
        3 => (vec![[1, 0], [0, 1], [-1, -1]], vec![1; 3]),
        4 => (vec![[1, 0], [0, 1], [-1, 0], [0, -1]], vec![1; 4]),
        6 => (
            vec![[1, 0], [1, 1], [0, 1], [-1, 0], [-1, -1], [0, -1]],
            vec![1; 6],
        ),
        _ => unreachable!(),
    }
}

fn fixture(name: &'static str, k: usize, m: usize) -> Fixture {
    let (qn, qh) = polygon(k);
    let (pn, ph) = polygon(m);
    let mut normals = Vec::new();
    let mut supports = Vec::new();
    for (n, h) in qn.into_iter().zip(qh) {
        normals.push(vec![qi(n[0]), qi(n[1]), qi(0), qi(0)]);
        supports.push(qi(h));
    }
    for (n, h) in pn.into_iter().zip(ph) {
        normals.push(vec![qi(0), qi(0), qi(n[0]), qi(n[1])]);
        supports.push(qi(h));
    }
    let mut intended_incidence = Vec::new();
    for i in 0..k {
        for j in 0..m {
            let mut active = vec![i, (i + 1) % k, k + j, k + (j + 1) % m];
            active.sort_unstable();
            intended_incidence.push(active);
        }
    }
    intended_incidence.sort();
    Fixture {
        name,
        k,
        m,
        normals,
        supports,
        intended_incidence,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SlackIndex {
    vertex_facets: Vec<usize>,
    nonincident_facet: usize,
}

fn vertex_for(normals: &[VecQ], h: &[Q], incidence: &[usize]) -> Option<VecQ> {
    let a: MatQ = incidence.iter().map(|&i| normals[i].clone()).collect();
    let rhs: VecQ = incidence.iter().map(|&i| h[i].clone()).collect();
    solve(&a, &rhs)
}

fn inactive_slacks(
    normals: &[VecQ],
    h: &[Q],
    intended: &[Vec<usize>],
) -> Option<BTreeMap<SlackIndex, Q>> {
    let mut out = BTreeMap::new();
    for incidence in intended {
        let x = vertex_for(normals, h, incidence)?;
        for j in 0..normals.len() {
            if !incidence.contains(&j) {
                out.insert(
                    SlackIndex {
                        vertex_facets: incidence.clone(),
                        nonincident_facet: j,
                    },
                    h[j].clone() - dot(&normals[j], &x),
                );
            }
        }
    }
    Some(out)
}

fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn rec(n: usize, k: usize, start: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if prefix.len() == k {
            out.push(prefix.clone());
            return;
        }
        for x in start..=n - (k - prefix.len()) {
            prefix.push(x);
            rec(n, k, x + 1, prefix, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    rec(n, k, 0, &mut Vec::new(), &mut out);
    out
}

fn point_key(x: &[Q]) -> String {
    x.iter().map(qs).collect::<Vec<_>>().join(",")
}

fn reconstruct_incidence(normals: &[VecQ], h: &[Q]) -> Vec<Vec<usize>> {
    let mut vertices: BTreeMap<String, (VecQ, BTreeSet<usize>)> = BTreeMap::new();
    for subset in combinations(normals.len(), 4) {
        let Some(x) = vertex_for(normals, h, &subset) else {
            continue;
        };
        if normals.iter().zip(h).any(|(n, hi)| dot(n, &x) > *hi) {
            continue;
        }
        let active: BTreeSet<usize> = normals
            .iter()
            .zip(h)
            .enumerate()
            .filter(|(_, (n, hi))| dot(n, &x) == **hi)
            .map(|(i, _)| i)
            .collect();
        vertices.entry(point_key(&x)).or_insert((x, active));
    }
    let mut incidences: Vec<Vec<usize>> = vertices
        .into_values()
        .map(|(_, active)| active.into_iter().collect())
        .collect();
    incidences.sort();
    incidences
}

fn add_scaled(h: &[Q], v: &[Q], t: &Q) -> VecQ {
    h.iter().zip(v).map(|(x, y)| x + y * t).collect()
}

fn seeded_direction(f: &Fixture, seed: u64) -> (VecQ, usize) {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(f.name.as_bytes());
    let mut rng = ChaCha8Rng::from_seed(*blake3::hash(&material).as_bytes());
    let base = inactive_slacks(&f.normals, &f.supports, &f.intended_incidence).unwrap();
    for attempt in 0..128 {
        let v: VecQ = (0..f.normals.len())
            .map(|_| qi(rng.gen_range(-7_i64..=7_i64)))
            .collect();
        if v.iter().all(Zero::is_zero) {
            continue;
        }
        let directional = inactive_slacks(&f.normals, &v, &f.intended_incidence).unwrap();
        if base.keys().any(|key| directional[key].is_negative()) {
            return (v, attempt);
        }
    }
    panic!("seeded bounded direction search exhausted")
}

#[derive(Clone)]
struct BoundaryData {
    direction: VecQ,
    attempt: usize,
    time: Q,
    witnesses: Vec<(SlackIndex, Q, Q)>,
    translation_ok: bool,
    scale_slacks_ok: bool,
    fixed_direction_scale_ok: bool,
    coscale_ok: bool,
    boundary_nonnegative: bool,
    boundary_witness_zero: bool,
}

fn boundary_data(f: &Fixture, seed: u64) -> BoundaryData {
    let base = inactive_slacks(&f.normals, &f.supports, &f.intended_incidence).unwrap();
    assert!(base.values().all(|x| x.is_positive()));
    let (direction, attempt) = seeded_direction(f, seed);
    let derivative = inactive_slacks(&f.normals, &direction, &f.intended_incidence).unwrap();
    let time = base
        .iter()
        .filter(|(key, _)| derivative[*key].is_negative())
        .map(|(key, slack)| slack / -derivative[key].clone())
        .min()
        .unwrap();
    let witnesses: Vec<_> = base
        .iter()
        .filter(|(key, slack)| (*slack).clone() + derivative[*key].clone() * time.clone() == qi(0))
        .map(|(key, slack)| (key.clone(), slack.clone(), derivative[key].clone()))
        .collect();
    let t = vec![qr(2, 3), qr(-1, 2), qr(3, 5), qr(-2, 7)];
    let translated_h: VecQ = f
        .supports
        .iter()
        .zip(&f.normals)
        .map(|(h, n)| h + dot(n, &t))
        .collect();
    let translated = inactive_slacks(&f.normals, &translated_h, &f.intended_incidence).unwrap();
    let translation_ok = translated == base;
    let lambda = qr(3, 2);
    let scaled_h: VecQ = f.supports.iter().map(|x| x * lambda.clone()).collect();
    let scaled = inactive_slacks(&f.normals, &scaled_h, &f.intended_incidence).unwrap();
    let scale_slacks_ok = base
        .iter()
        .all(|(key, x)| scaled[key] == x * lambda.clone());
    let scaled_time = scaled
        .iter()
        .filter(|(key, _)| derivative[*key].is_negative())
        .map(|(key, slack)| slack / -derivative[key].clone())
        .min()
        .unwrap();
    let fixed_direction_scale_ok = scaled_time == time.clone() * lambda.clone();
    let scaled_v: VecQ = direction.iter().map(|x| x * lambda.clone()).collect();
    let scaled_derivative = inactive_slacks(&f.normals, &scaled_v, &f.intended_incidence).unwrap();
    let coscaled_time = scaled
        .iter()
        .filter(|(key, _)| scaled_derivative[*key].is_negative())
        .map(|(key, slack)| slack / -scaled_derivative[key].clone())
        .min()
        .unwrap();
    let coscale_ok = coscaled_time == time;
    let at_boundary = add_scaled(&f.supports, &direction, &time);
    let boundary_slacks = inactive_slacks(&f.normals, &at_boundary, &f.intended_incidence).unwrap();
    let boundary_nonnegative = boundary_slacks.values().all(|x| !x.is_negative());
    let boundary_witness_zero = witnesses
        .iter()
        .all(|(key, _, _)| boundary_slacks[key].is_zero());
    BoundaryData {
        direction,
        attempt,
        time,
        witnesses,
        translation_ok,
        scale_slacks_ok,
        fixed_direction_scale_ok,
        coscale_ok,
        boundary_nonnegative,
        boundary_witness_zero,
    }
}

#[derive(Serialize)]
struct WitnessRow {
    vertex_facets: Vec<usize>,
    nonincident_facet: usize,
    base_slack: String,
    directional_slack: String,
}

#[derive(Serialize)]
struct BoundaryRow {
    schema: &'static str,
    id: String,
    terminal_status: &'static str,
    fixture: &'static str,
    factor_sides: [usize; 2],
    seed: u64,
    direction_attempt: usize,
    fraction: String,
    coordinate_order: &'static str,
    slack_system: &'static str,
    normals: Vec<Vec<String>>,
    base_supports: Vec<String>,
    direction: Vec<String>,
    first_positive_boundary_time: String,
    sample_time: String,
    sampled_supports: Vec<String>,
    minimum_inactive_slack: String,
    fixed_normals_verified: bool,
    inactive_slacks_strictly_positive: bool,
    facet_labeled_incidence_multiset_unchanged: bool,
    exact_reconstruction_status: &'static str,
    translation_invariance_verified: bool,
    positive_scale_slacks_verified: bool,
    boundary_time_scales_with_support_at_fixed_direction: bool,
    boundary_time_invariant_when_support_and_direction_coscale: bool,
    predicted_boundary_witnesses: Vec<WitnessRow>,
    boundary_all_local_slacks_nonnegative: bool,
    predicted_witnesses_zero_at_boundary: bool,
}

fn boundary_rows(fixtures: &[Fixture], seed: u64) -> Vec<BoundaryRow> {
    let fractions = [qr(1, 10), qr(1, 2), qr(9, 10)];
    let mut rows = Vec::new();
    for f in fixtures {
        let data = boundary_data(f, seed);
        for fraction in &fractions {
            let sample_time = data.time.clone() * fraction;
            let sampled_supports = add_scaled(&f.supports, &data.direction, &sample_time);
            let slacks =
                inactive_slacks(&f.normals, &sampled_supports, &f.intended_incidence).unwrap();
            let min_slack = slacks.values().min().unwrap().clone();
            let reconstructed = reconstruct_incidence(&f.normals, &sampled_supports);
            let incidence_ok = reconstructed == f.intended_incidence;
            let pass = min_slack.is_positive()
                && incidence_ok
                && data.translation_ok
                && data.scale_slacks_ok
                && data.fixed_direction_scale_ok
                && data.coscale_ok
                && data.boundary_nonnegative
                && data.boundary_witness_zero;
            rows.push(BoundaryRow {
                schema: "generator-type-cone-boundary-fraction-row-v1",
                id: format!(
                    "type-cone-boundary-v1/{}/seed={seed}/fraction={}",
                    f.name,
                    qs(fraction)
                ),
                terminal_status: if pass { "passed" } else { "failed" },
                fixture: f.name,
                factor_sides: [f.k, f.m],
                seed,
                direction_attempt: data.attempt,
                fraction: qs(fraction),
                coordinate_order: COORDINATE_ORDER,
                slack_system: LOCAL_SYSTEM,
                normals: qmat(&f.normals),
                base_supports: qvec(&f.supports),
                direction: qvec(&data.direction),
                first_positive_boundary_time: qs(&data.time),
                sample_time: qs(&sample_time),
                sampled_supports: qvec(&sampled_supports),
                minimum_inactive_slack: qs(&min_slack),
                fixed_normals_verified: true,
                inactive_slacks_strictly_positive: min_slack.is_positive(),
                facet_labeled_incidence_multiset_unchanged: incidence_ok,
                exact_reconstruction_status: if incidence_ok {
                    "passed-full-exact-vertex-enumeration"
                } else {
                    "failed"
                },
                translation_invariance_verified: data.translation_ok,
                positive_scale_slacks_verified: data.scale_slacks_ok,
                boundary_time_scales_with_support_at_fixed_direction: data.fixed_direction_scale_ok,
                boundary_time_invariant_when_support_and_direction_coscale: data.coscale_ok,
                predicted_boundary_witnesses: data
                    .witnesses
                    .iter()
                    .map(|(key, slack, derivative)| WitnessRow {
                        vertex_facets: key.vertex_facets.clone(),
                        nonincident_facet: key.nonincident_facet,
                        base_slack: qs(slack),
                        directional_slack: qs(derivative),
                    })
                    .collect(),
                boundary_all_local_slacks_nonnegative: data.boundary_nonnegative,
                predicted_witnesses_zero_at_boundary: data.boundary_witness_zero,
            });
        }
    }
    rows
}

fn transform_fixture(f: &Fixture, a: &[VecQ], supports: Option<VecQ>) -> Fixture {
    let inverse_transpose = transpose(&inverse(a).expect("invertible transform"));
    let normals = f
        .normals
        .iter()
        .map(|n| mat_vec(&inverse_transpose, n))
        .collect();
    Fixture {
        name: f.name,
        k: f.k,
        m: f.m,
        normals,
        supports: supports.unwrap_or_else(|| f.supports.clone()),
        intended_incidence: f.intended_incidence.clone(),
    }
}

fn coordinate_product_recognized(normals: &[VecQ], k: usize) -> bool {
    normals.iter().enumerate().all(|(i, n)| {
        if i < k {
            n[2].is_zero() && n[3].is_zero()
        } else {
            n[0].is_zero() && n[1].is_zero()
        }
    })
}

fn omega(u: &[Q], v: &[Q]) -> Q {
    &u[0] * &v[2] + &u[1] * &v[3] - &u[2] * &v[0] - &u[3] * &v[1]
}

fn symplectic_form() -> MatQ {
    vec![
        vec![qi(0), qi(0), qi(1), qi(0)],
        vec![qi(0), qi(0), qi(0), qi(1)],
        vec![qi(-1), qi(0), qi(0), qi(0)],
        vec![qi(0), qi(-1), qi(0), qi(0)],
    ]
}

fn residual_max(a: &[VecQ], b: &[VecQ]) -> Q {
    a.iter()
        .zip(b)
        .flat_map(|(x, y)| x.iter().zip(y).map(|(u, v)| (u - v).abs()))
        .max()
        .unwrap_or_else(|| qi(0))
}

fn givens(i: usize, j: usize, c: Q, s: Q) -> MatQ {
    let mut out = identity(4);
    out[i][i] = c.clone();
    out[j][j] = c;
    out[i][j] = -s.clone();
    out[j][i] = s;
    out
}

fn columns(a: &[VecQ], pair: [usize; 2]) -> [VecQ; 2] {
    [
        a.iter().map(|row| row[pair[0]].clone()).collect(),
        a.iter().map(|row| row[pair[1]].clone()).collect(),
    ]
}

#[derive(Serialize)]
struct PlaneWitness {
    factor_q_basis_columns: Vec<Vec<String>>,
    factor_p_basis_columns: Vec<Vec<String>>,
    complementary_four_volume_determinant: String,
    q_plane_kahler_residual: String,
    p_plane_kahler_residual: String,
    both_planes_lagrangian: bool,
    verification: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct ProductnessFlags {
    coordinate_product: bool,
    lagrangian_product: bool,
    affine_product: bool,
    combinatorial_product: bool,
}

#[derive(Serialize)]
struct PreservationRow {
    schema: &'static str,
    id: String,
    terminal_status: &'static str,
    seed: u64,
    coordinate_order: &'static str,
    transform_row_major: Vec<Vec<String>>,
    transform_determinant: String,
    exact_orthogonality_residual_max: String,
    exact_symplectic_residual_max: String,
    transform_is_orthogonal: bool,
    transform_is_symplectic: bool,
    expected_transform_is_orthogonal: bool,
    expected_transform_is_symplectic: bool,
    expected_productness: ProductnessFlags,
    transformed_facet_normals: Vec<Vec<String>>,
    transformed_facet_supports: Vec<String>,
    productness: ProductnessFlags,
    coordinate_product_recognition: &'static str,
    intrinsic_affine_product_witness: &'static str,
    combinatorial_evidence: &'static str,
    exact_facet_labeled_incidence_multiset_preserved: bool,
    exact_reconstruction_simple: bool,
    factor_planes: PlaneWitness,
    classifier_failure_interpretation: &'static str,
}

fn preservation_row(
    id: &'static str,
    seed: u64,
    base: &Fixture,
    transform: MatQ,
    supports: Option<VecQ>,
    expected_productness: ProductnessFlags,
    expected_orthogonal: bool,
    expected_symplectic: bool,
) -> PreservationRow {
    let transformed = transform_fixture(base, &transform, supports);
    let coordinate = coordinate_product_recognized(&transformed.normals, base.k);
    let reconstructed = reconstruct_incidence(&transformed.normals, &transformed.supports);
    let incidence_ok = reconstructed == base.intended_incidence;
    let simple = reconstructed.iter().all(|x| x.len() == 4);
    let q_basis = columns(&transform, [0, 1]);
    let p_basis = columns(&transform, [2, 3]);
    let q_residual = omega(&q_basis[0], &q_basis[1]).abs();
    let p_residual = omega(&p_basis[0], &p_basis[1]).abs();
    let lagrangian = q_residual.is_zero() && p_residual.is_zero();
    let det = determinant(&transform);
    let orthogonality_residual =
        residual_max(&mat_mul(&transpose(&transform), &transform), &identity(4));
    let j = symplectic_form();
    let symplectic_residual = residual_max(
        &mat_mul(&mat_mul(&transpose(&transform), &j), &transform),
        &j,
    );
    let productness = ProductnessFlags {
        coordinate_product: coordinate,
        lagrangian_product: lagrangian,
        affine_product: true,
        combinatorial_product: incidence_ok && simple,
    };
    let is_orthogonal = orthogonality_residual.is_zero();
    let is_symplectic = symplectic_residual.is_zero();
    let pass = incidence_ok
        && simple
        && det == qi(1)
        && productness == expected_productness
        && is_orthogonal == expected_orthogonal
        && is_symplectic == expected_symplectic;
    PreservationRow {
        schema: "generator-productness-preservation-row-v1",
        id: format!("generator-productness-v1/{id}/seed={seed}"),
        terminal_status: if pass { "passed" } else { "failed" },
        seed,
        coordinate_order: COORDINATE_ORDER,
        transform_row_major: qmat(&transform),
        transform_determinant: qs(&det),
        exact_orthogonality_residual_max: qs(&orthogonality_residual),
        exact_symplectic_residual_max: qs(&symplectic_residual),
        transform_is_orthogonal: is_orthogonal,
        transform_is_symplectic: is_symplectic,
        expected_transform_is_orthogonal: expected_orthogonal,
        expected_transform_is_symplectic: expected_symplectic,
        expected_productness,
        transformed_facet_normals: qmat(&transformed.normals),
        transformed_facet_supports: qvec(&transformed.supports),
        productness,
        coordinate_product_recognition: if coordinate {
            "recognized exactly from the labeled q-only and p-only normal blocks"
        } else {
            "not recognized in the ambient coordinate q/p split"
        },
        intrinsic_affine_product_witness: "the recorded invertible linear transform maps the exact source product and its two factor planes to this realization",
        combinatorial_evidence: "full exact vertex enumeration preserves the labeled source incidence; simplicity (four active facets per vertex) is checked explicitly; no graph-factorization theorem is invoked",
        exact_facet_labeled_incidence_multiset_preserved: incidence_ok,
        exact_reconstruction_simple: simple,
        factor_planes: PlaneWitness {
            factor_q_basis_columns: qmat(&q_basis),
            factor_p_basis_columns: qmat(&p_basis),
            complementary_four_volume_determinant: qs(&det.abs()),
            q_plane_kahler_residual: qs(&q_residual),
            p_plane_kahler_residual: qs(&p_residual),
            both_planes_lagrangian: lagrangian,
            verification: "exact construction witness; Kähler residual is the absolute restriction coefficient |omega(u,v)| in the recorded factor basis (zero iff that two-plane is Lagrangian)",
        },
        classifier_failure_interpretation: if coordinate {
            "coordinate recognition succeeds; this is stronger than the separately recorded intrinsic witnesses"
        } else {
            "coordinate recognition failure is not loss of affine or combinatorial productness"
        },
    }
}

fn preservation_rows(base: &Fixture, seed: u64) -> Vec<PreservationRow> {
    let identity_transform = identity(4);
    let u2 = vec![
        vec![qr(3, 5), qi(0), qr(-4, 5), qi(0)],
        vec![qi(0), qr(3, 5), qi(0), qr(-4, 5)],
        vec![qr(4, 5), qi(0), qr(3, 5), qi(0)],
        vec![qi(0), qr(4, 5), qi(0), qr(3, 5)],
    ];
    // A dense exact SO(4) control: a fixed product of rational Givens
    // rotations across q/q, q/p, and p/p coordinate pairs. It is deliberately
    // not a Haar draw; the packet needs a discriminating group witness, not a
    // probability law.
    let so4 = [
        (0, 1, qr(3, 5), qr(4, 5)),
        (1, 2, qr(5, 13), qr(12, 13)),
        (2, 3, qr(8, 17), qr(15, 17)),
        (0, 3, qr(7, 25), qr(24, 25)),
    ]
    .into_iter()
    .fold(identity(4), |a, (i, j, c, s)| {
        mat_mul(&givens(i, j, c, s), &a)
    });
    let sl4 = vec![
        vec![qi(1), qi(0), qi(1), qi(1)],
        vec![qi(0), qi(1), qi(0), qi(0)],
        vec![qi(0), qi(1), qi(1), qi(0)],
        vec![qi(0), qi(0), qi(0), qi(1)],
    ];
    let data = boundary_data(base, seed);
    let perturbed = add_scaled(&base.supports, &data.direction, &(data.time * qr(1, 2)));
    vec![
        preservation_row(
            "exact-coordinate-product",
            seed,
            base,
            identity_transform.clone(),
            None,
            ProductnessFlags {
                coordinate_product: true,
                lagrangian_product: true,
                affine_product: true,
                combinatorial_product: true,
            },
            true,
            true,
        ),
        preservation_row(
            "exact-u2-image",
            seed,
            base,
            u2,
            None,
            ProductnessFlags {
                coordinate_product: false,
                lagrangian_product: true,
                affine_product: true,
                combinatorial_product: true,
            },
            true,
            true,
        ),
        preservation_row(
            "exact-generic-so4-image",
            seed,
            base,
            so4,
            None,
            ProductnessFlags {
                coordinate_product: false,
                lagrangian_product: false,
                affine_product: true,
                combinatorial_product: true,
            },
            true,
            false,
        ),
        preservation_row(
            "exact-sl4-image",
            seed,
            base,
            sl4,
            None,
            ProductnessFlags {
                coordinate_product: false,
                lagrangian_product: false,
                affine_product: true,
                combinatorial_product: true,
            },
            false,
            false,
        ),
        preservation_row(
            "same-incidence-fixed-normal-half-boundary-perturbation",
            seed,
            base,
            identity_transform,
            Some(perturbed),
            ProductnessFlags {
                coordinate_product: true,
                lagrangian_product: true,
                affine_product: true,
                combinatorial_product: true,
            },
            true,
            true,
        ),
    ]
}

#[derive(Serialize)]
struct Deferral {
    item: &'static str,
    status: &'static str,
    reason: &'static str,
}

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    terminal_status: &'static str,
    command: String,
    seed: u64,
    boundary_rows: usize,
    boundary_passed: usize,
    preservation_rows: usize,
    preservation_passed: usize,
    source_revision: String,
    source_repository_tree: String,
    source_tracked_clean: bool,
    producer_source_sha256: String,
    cargo_lock_sha256: String,
    boundary_artifact_sha256: String,
    preservation_artifact_sha256: String,
    build_source_closure: &'static str,
    independence_unit: &'static str,
    local_slack_system_boundary: &'static str,
    allowed_claims: Vec<&'static str>,
    prohibited_claims: Vec<&'static str>,
    deferrals: Vec<Deferral>,
}

fn required_command_output(args: &[&str]) -> Result<String, String> {
    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .map_err(|error| format!("failed to execute {}: {error}", args.join(" ")))?;
    if !output.status.success() {
        return Err(format!(
            "command failed ({}): {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|x| x.trim().to_owned())
        .map_err(|error| format!("non-UTF-8 output from {}: {error}", args.join(" ")))
}

fn sha256(path: &str) -> Result<String, String> {
    let digest = required_command_output(&["sha256sum", path])?
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("sha256sum returned no digest for {path}"))?
        .to_owned();
    if digest.len() != 64 || !digest.bytes().all(|x| x.is_ascii_hexdigit()) {
        return Err(format!("invalid SHA-256 digest for {path}: {digest}"));
    }
    Ok(digest)
}

fn sha256_path(path: &std::path::Path) -> Result<String, String> {
    sha256(
        path.to_str()
            .ok_or_else(|| format!("non-UTF-8 artifact path: {}", path.display()))?,
    )
}

fn source_provenance() -> Result<(String, String, bool, String, String), String> {
    let status =
        required_command_output(&["git", "status", "--porcelain", "--untracked-files=no"])?;
    let revision = required_command_output(&["git", "rev-parse", "HEAD"])?;
    let tree = required_command_output(&["git", "rev-parse", "HEAD^{tree}"])?;
    for (role, object) in [("revision", &revision), ("repository tree", &tree)] {
        if object.len() != 40 || !object.bytes().all(|x| x.is_ascii_hexdigit()) {
            return Err(format!("invalid git {role} identity: {object}"));
        }
    }
    Ok((
        revision,
        tree,
        status.is_empty(),
        sha256(PRODUCER_PATH)?,
        sha256("Cargo.lock")?,
    ))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Args {
    out_dir: PathBuf,
    seed: u64,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut out = Args {
        out_dir: PathBuf::from(
            "experiments/sys-datascience/methods/generator-type-cone-productness/artifacts",
        ),
        seed: 20260715,
    };
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--out-dir" => {
                out.out_dir = PathBuf::from(argv.get(i + 1).ok_or("--out-dir requires DIR")?);
                i += 2;
            }
            "--seed" => {
                out.seed = argv
                    .get(i + 1)
                    .ok_or("--seed requires U64")?
                    .parse()
                    .map_err(|_| "--seed requires U64")?;
                i += 2;
            }
            "--help" | "-h" => return Err("usage: --out-dir DIR --seed U64".to_owned()),
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }
    Ok(out)
}

fn run(args: &Args, command: String) -> Result<Report, String> {
    let fixtures = [
        fixture("exact-3x3-product", 3, 3),
        fixture("exact-4x6-product", 4, 6),
    ];
    let boundary = boundary_rows(&fixtures, args.seed);
    let preservation = preservation_rows(&fixtures[0], args.seed);
    let boundary_passed = boundary
        .iter()
        .filter(|x| x.terminal_status == "passed")
        .count();
    let preservation_passed = preservation
        .iter()
        .filter(|x| x.terminal_status == "passed")
        .count();
    if boundary_passed != boundary.len() || preservation_passed != preservation.len() {
        return Err("one or more fail-closed rows did not pass".to_owned());
    }
    let (revision, tree, clean, source_hash, lock_hash) = source_provenance()?;
    if !clean {
        return Err(
            "tracked repository state is dirty; refusing to write retained artifacts".to_owned(),
        );
    }
    create_dir_all(&args.out_dir).map_err(|x| x.to_string())?;
    let boundary_path = args.out_dir.join("boundary-fractions.jsonl");
    let preservation_path = args.out_dir.join("preservation-matrix.json");
    let report_path = args.out_dir.join("report.json");
    let boundary_temp = args.out_dir.join(".boundary-fractions.jsonl.tmp");
    let preservation_temp = args.out_dir.join(".preservation-matrix.json.tmp");
    let report_temp = args.out_dir.join(".report.json.tmp");
    let mut writer = BufWriter::new(File::create(&boundary_temp).map_err(|x| x.to_string())?);
    for row in &boundary {
        serde_json::to_writer(&mut writer, row).map_err(|x| x.to_string())?;
        writeln!(&mut writer).map_err(|x| x.to_string())?;
    }
    writer.flush().map_err(|x| x.to_string())?;
    serde_json::to_writer_pretty(
        File::create(&preservation_temp).map_err(|x| x.to_string())?,
        &preservation,
    )
    .map_err(|x| x.to_string())?;
    let boundary_hash = sha256_path(&boundary_temp)?;
    let preservation_hash = sha256_path(&preservation_temp)?;
    let report = Report {
        schema: "generator-type-cone-productness-report-v1",
        terminal_status: "passed",
        command,
        seed: args.seed,
        boundary_rows: boundary.len(),
        boundary_passed,
        preservation_rows: preservation.len(),
        preservation_passed,
        source_revision: revision,
        source_repository_tree: tree,
        source_tracked_clean: clean,
        producer_source_sha256: source_hash,
        cargo_lock_sha256: lock_hash,
        boundary_artifact_sha256: boundary_hash,
        preservation_artifact_sha256: preservation_hash,
        build_source_closure: "The full repository revision/tree plus repo-wide tracked-clean predicate bind tracked transitive inputs; file hashes are convenient local checks, not the closure definition.",
        independence_unit: "This deterministic smoke has two constructed boundary fixtures and five constructed preservation controls. Rows sharing a fixture/direction are paired diagnostic views, not independent samples.",
        local_slack_system_boundary: "Only inactive slacks of the declared intended simple vertices are used for first-boundary timing. Full exact vertex enumeration independently checks each pre-boundary sample, but this packet does not claim a complete global type-cone chamber characterization.",
        allowed_claims: vec![
            "the exact implementation realizes paired 0.1, 0.5, and 0.9 fractions before the first local inactive-slack boundary for the two named fixtures",
            "the named translation, positive-scale, exact reconstruction, labeled-incidence, and boundary-witness controls pass for this deterministic smoke",
            "the explicit image constructions preserve affine/combinatorial productness even where ambient coordinate-product recognition fails",
        ],
        prohibited_claims: vec![
            "complete characterization of a type-cone chamber",
            "population frequency, generator-law ranking, target transfer, mechanism, or intrinsic dimension",
            "loss of productness from failure of the ambient coordinate-product classifier",
            "any sys or capacity conclusion",
        ],
        deferrals: vec![
            Deferral {
                item: "projective/Hilbert-type slack-ratio distance",
                status: "deferred",
                reason: "not needed for the boundary-fraction decision; no domain/finite-value contract was introduced",
            },
            Deferral {
                item: "graph Cartesian factorization and affine-factor recovery from an unknown realization",
                status: "deferred",
                reason: "the exact construction witnesses and full incidence controls answer this smoke without expanding into a separate recovery/classification packet",
            },
        ],
    };
    serde_json::to_writer_pretty(
        File::create(&report_temp).map_err(|x| x.to_string())?,
        &report,
    )
    .map_err(|x| x.to_string())?;
    // Publish validated payloads first and the completion report last. If a
    // rename fails, an old report either still binds the old payload hashes or
    // fails hash validation against a mixed set; no new run is complete until
    // the new report is present.
    std::fs::rename(&boundary_temp, &boundary_path).map_err(|x| x.to_string())?;
    std::fs::rename(&preservation_temp, &preservation_path).map_err(|x| x.to_string())?;
    std::fs::rename(&report_temp, &report_path).map_err(|x| x.to_string())?;
    Ok(report)
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let args = parse_args(&argv).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2)
    });
    let reproduction_command = format!(
        "cargo run -p exp-sys-landscape --bin sys-datascience-generator-type-cone-productness -- --out-dir {} --seed {}",
        args.out_dir.display(),
        args.seed
    );
    match run(&args, reproduction_command) {
        Ok(report) => println!(
            "passed {} boundary rows and {} preservation rows",
            report.boundary_passed, report.preservation_passed
        ),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_fixtures_reconstruct_with_declared_incidence() {
        for f in [fixture("3x3", 3, 3), fixture("4x6", 4, 6)] {
            assert_eq!(
                reconstruct_incidence(&f.normals, &f.supports),
                f.intended_incidence
            );
            assert!(
                inactive_slacks(&f.normals, &f.supports, &f.intended_incidence)
                    .unwrap()
                    .values()
                    .all(|x| x.is_positive())
            );
        }
    }

    #[test]
    fn boundary_fraction_rows_pass_exact_controls() {
        let rows = boundary_rows(
            &[
                fixture("exact-3x3-product", 3, 3),
                fixture("exact-4x6-product", 4, 6),
            ],
            20260715,
        );
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().all(|x| x.terminal_status == "passed"));
    }

    #[test]
    fn productness_hierarchy_separates_coordinate_from_intrinsic() {
        let rows = preservation_rows(&fixture("exact-3x3-product", 3, 3), 20260715);
        assert_eq!(rows.len(), 5);
        assert!(rows.iter().all(|x| x.terminal_status == "passed"));
        assert!(rows[0].productness.coordinate_product);
        assert!(rows[0].transform_is_orthogonal);
        assert!(rows[0].transform_is_symplectic);
        assert!(rows[1].productness.lagrangian_product);
        assert!(!rows[1].productness.coordinate_product);
        assert!(rows[1].transform_is_orthogonal);
        assert!(rows[1].transform_is_symplectic);
        assert!(!rows[2].productness.lagrangian_product);
        assert!(rows[2].productness.affine_product);
        assert!(rows[2].transform_is_orthogonal);
        assert!(!rows[2].transform_is_symplectic);
        assert!(rows[3].productness.combinatorial_product);
        assert!(!rows[3].productness.coordinate_product);
        assert_eq!(rows[3].transform_determinant, "1");
        assert!(!rows[3].transform_is_orthogonal);
        assert!(rows[4].productness.coordinate_product);
    }

    #[test]
    fn exact_linear_algebra_witnesses_are_consistent() {
        let a = vec![
            vec![qr(3, 5), qi(0), qr(-4, 5), qi(0)],
            vec![qi(0), qr(3, 5), qi(0), qr(-4, 5)],
            vec![qr(4, 5), qi(0), qr(3, 5), qi(0)],
            vec![qi(0), qr(4, 5), qi(0), qr(3, 5)],
        ];
        assert_eq!(determinant(&a), qi(1));
        assert_eq!(mat_mul(&a, &inverse(&a).unwrap()), identity(4));
    }

    #[test]
    fn cli_rejects_unknown_arguments() {
        assert!(parse_args(&["bin".to_owned(), "--wat".to_owned()]).is_err());
    }
}
