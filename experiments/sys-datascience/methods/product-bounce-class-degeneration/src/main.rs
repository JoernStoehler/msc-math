use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify_sigma_beta_action;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::kkt::rational_solver::solve_kkt_exact;

const RAW_SHA256: &str = "66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736";
const CLASS_SHA256: &str = "187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4";

#[derive(Clone, Deserialize)]
struct RawRow {
    name: String,
    k: usize,
    m: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    dual_vertices: Vec<[f64; 4]>,
    #[serde(rename = "volume")]
    _volume: f64,
    #[serde(rename = "capacity")]
    _capacity: f64,
    #[serde(rename = "sys")]
    _sys: f64,
}
#[derive(Clone, Deserialize)]
struct ClassMinimum {
    #[serde(rename = "action")]
    _action: f64,
    action_exact: String,
    #[serde(rename = "minimizer_count")]
    _minimizer_count: usize,
    minimizer_sigmas: Vec<Vec<usize>>,
}
#[derive(Clone, Deserialize)]
struct ClassRow {
    name: String,
    #[serde(rename = "k")]
    _k: usize,
    #[serde(rename = "m")]
    _m: usize,
    class_minima: BTreeMap<String, Option<ClassMinimum>>,
    normalized_three_minus_two_gap: Option<f64>,
}

#[derive(Serialize)]
struct Term {
    i: usize,
    j: usize,
    signed: String,
    abs: String,
    beta_i: String,
    beta_j: String,
    omega: String,
    normalized_abs: f64,
    normalized_pairing: f64,
}
#[derive(Serialize)]
struct Pair {
    name: String,
    pair_id: usize,
    a2_sigma: Vec<usize>,
    a3_sigma: Vec<usize>,
    support: Vec<usize>,
    exact_q2: String,
    exact_q3: String,
    exact_action2: String,
    exact_action3: String,
    signed_gap_q: String,
    normalized_gap: f64,
    flip_count: usize,
    gross_abs_q: String,
    cancellation_ratio: Option<f64>,
    min_abs_term_over_q3: Option<f64>,
    max_abs_term_over_q3: Option<f64>,
    beta_product: Option<String>,
    normalized_pairing_factor: Option<f64>,
    terms: Vec<Term>,
    recovery_valid: bool,
    max_violation: Option<f64>,
    closure_error: Option<f64>,
    action_error_rel: Option<f64>,
    alignment_convention: &'static str,
    a2_rotation_count: usize,
    a3_rotation_count: usize,
}
#[derive(Serialize)]
struct Row {
    name: String,
    k: usize,
    m: usize,
    gap_signed: f64,
    gap_abs: f64,
    exact_action2: Option<String>,
    exact_action3: Option<String>,
    a2_minimizer_count: usize,
    a3_minimizer_count: usize,
    a3_six_singleton_count: usize,
    a3_eight_facet_count: usize,
    a2_word_lengths: Vec<usize>,
    a3_word_lengths: Vec<usize>,
    a2_supports: Vec<Vec<usize>>,
    a3_supports: Vec<Vec<usize>>,
    max_support_intersection: usize,
    max_support_jaccard: f64,
    exact_support_equal: bool,
    same_support_pair_count: usize,
    different_support_pair_count: usize,
    primary_pair_ids: Vec<usize>,
    min_beta_product: Option<f64>,
    max_beta_product: Option<f64>,
    beta_product_range: Option<f64>,
    min_pairing_factor: Option<f64>,
    min_cancellation_ratio: Option<f64>,
    recovery_pass_count: usize,
    recovery_total_count: usize,
    eight_facet_gap_abs: Option<f64>,
    eight_facet_rank_kernel_dimensions: Vec<usize>,
}

struct Args {
    raw: PathBuf,
    class: PathBuf,
    out: PathBuf,
    limit: Option<usize>,
}
fn args() -> Args {
    let mut raw = None;
    let mut class = None;
    let mut out = None;
    let mut limit = None;
    let a: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < a.len() {
        match a[i].as_str() {
            "--raw" => {
                raw = Some(PathBuf::from(&a[i + 1]));
                i += 2
            }
            "--class" => {
                class = Some(PathBuf::from(&a[i + 1]));
                i += 2
            }
            "--out" => {
                out = Some(PathBuf::from(&a[i + 1]));
                i += 2
            }
            "--limit" => {
                limit = Some(a[i + 1].parse().unwrap());
                i += 2
            }
            "--help" => {
                println!("--raw FILE --class FILE --out DIR [--limit N]");
                std::process::exit(0)
            }
            x => panic!("unknown argument {x}"),
        }
    }
    Args {
        raw: raw.unwrap(),
        class: class.unwrap(),
        out: out.unwrap(),
        limit,
    }
}
fn sha(path: &Path) -> String {
    let bytes = std::fs::read(path).expect("read for hash");
    format!("{:x}", Sha256::digest(bytes))
}
fn rat(s: &str) -> BigRational {
    let (a, b) = s.split_once('/').expect("rational");
    BigRational::new(a.parse().unwrap(), b.parse().unwrap())
}
fn rats(v: Vec<[String; 4]>) -> Vec<[BigRational; 4]> {
    v.into_iter().map(|a| a.map(|x| rat(&x))).collect()
}
fn omega(a: &[BigRational; 4], b: &[BigRational; 4]) -> BigRational {
    &a[0] * &b[2] - &a[2] * &b[0] + &a[1] * &b[3] - &a[3] * &b[1]
}
fn q_of(dual: &[[BigRational; 4]], sigma: &[usize], beta: &[BigRational]) -> BigRational {
    let mut q = BigRational::zero();
    for i in 1..sigma.len() {
        for j in 0..i {
            q += &beta[i] * &beta[j] * omega(&dual[sigma[j]], &dual[sigma[i]]);
        }
    }
    q
}
fn kkt_kernel_dimension(dual: &[[BigRational; 4]], sigma: &[usize]) -> usize {
    let m = sigma.len();
    let n = m + 5;
    let mut a = vec![vec![BigRational::zero(); n]; n];
    for i in 0..m {
        for j in (i + 1)..m {
            let w = omega(&dual[sigma[i]], &dual[sigma[j]]);
            a[i][j] = w.clone();
            a[j][i] = w;
        }
        for d in 0..4 {
            let v = dual[sigma[i]][d].clone();
            a[i][m + d] = v.clone();
            a[m + d][i] = v;
        }
        a[i][m + 4] = BigRational::from_integer(1.into());
        a[m + 4][i] = BigRational::from_integer(1.into());
    }
    let mut rank = 0;
    for c in 0..n {
        let pivot = (rank..n).find(|&r| !a[r][c].is_zero());
        if let Some(p) = pivot {
            a.swap(rank, p);
            let v = a[rank][c].clone();
            for j in c..n {
                a[rank][j] = a[rank][j].clone() / v.clone();
            }
            let pivot_row = a[rank].clone();
            for r in 0..n {
                if r != rank && !a[r][c].is_zero() {
                    let f = a[r][c].clone();
                    for j in c..n {
                        a[r][j] -= f.clone() * pivot_row[j].clone();
                    }
                }
            }
            rank += 1;
        }
    }
    n - rank
}
fn f64r(x: &BigRational) -> f64 {
    x.to_f64().unwrap()
}
fn exact_action(q: &BigRational) -> BigRational {
    BigRational::new(1.into(), 2.into()) / q
}
fn support(s: &[usize]) -> Vec<usize> {
    let mut v = s.to_vec();
    v.sort_unstable();
    v
}
fn jaccard(a: &[usize], b: &[usize]) -> f64 {
    let x: HashSet<_> = a.iter().collect();
    let y: HashSet<_> = b.iter().collect();
    x.intersection(&y).count() as f64 / x.union(&y).count() as f64
}
fn rotations(s: &[usize]) -> Vec<Vec<usize>> {
    (0..s.len())
        .map(|i| s[i..].iter().chain(s[..i].iter()).copied().collect())
        .collect()
}
fn rotated_beta(original: &[usize], beta: &[BigRational], rotated: &[usize]) -> Vec<BigRational> {
    rotated
        .iter()
        .map(|facet| beta[original.iter().position(|x| x == facet).unwrap()].clone())
        .collect()
}
fn order_sign(s: &[usize], i: usize, j: usize) -> i8 {
    let pi = s.iter().position(|&x| x == i).unwrap();
    let pj = s.iter().position(|&x| x == j).unwrap();
    if pi < pj {
        1
    } else {
        -1
    }
}

struct Solved {
    sigma: Vec<usize>,
    beta: Vec<BigRational>,
    q: BigRational,
    action: BigRational,
}
fn solve(dual: &[[BigRational; 4]], sigma: &[usize]) -> Solved {
    let x = solve_kkt_exact(dual, sigma)
        .unwrap_or_else(|| panic!("exact solve failed for sigma {:?}", sigma));
    let q = x.q_exact;
    let action = exact_action(&q);
    Solved {
        sigma: sigma.to_vec(),
        beta: x.beta,
        q,
        action,
    }
}

fn align(
    dual: &[[BigRational; 4]],
    a2: &Solved,
    a3: &Solved,
    qset: &HashSet<usize>,
    _pset: &HashSet<usize>,
) -> (Vec<usize>, Vec<usize>, Vec<(usize, usize, BigRational)>) {
    let mut best: Option<(
        usize,
        BigRational,
        Vec<usize>,
        Vec<usize>,
        Vec<(usize, usize, BigRational)>,
    )> = None;
    for x in rotations(&a2.sigma) {
        for y in rotations(&a3.sigma) {
            let mut terms = Vec::new();
            let mut gross = BigRational::zero();
            for &i in &x {
                for &j in &x {
                    if i >= j || !(qset.contains(&i) ^ qset.contains(&j)) {
                        continue;
                    }
                    let d = order_sign(&x, i, j) - order_sign(&y, i, j);
                    if d != 0 {
                        let c = &a2.beta[a2.sigma.iter().position(|&z| z == i).unwrap()]
                            * &a2.beta[a2.sigma.iter().position(|&z| z == j).unwrap()]
                            * omega(&dual[i], &dual[j])
                            * BigRational::from_integer((d as i64).into());
                        gross += c.abs();
                        terms.push((i, j, c));
                    }
                }
            }
            let key = (
                terms.len(),
                gross.clone(),
                x.clone(),
                y.clone(),
                terms.clone(),
            );
            if best
                .as_ref()
                .map(|b| {
                    (b.0, b.1.clone(), b.2.clone(), b.3.clone())
                        > (key.0, key.1.clone(), key.2.clone(), key.3.clone())
                })
                .unwrap_or(true)
            {
                best = Some((key.0, key.1, key.2, key.3, key.4));
            }
        }
    }
    let (_, _, x, y, t) = best.unwrap();
    (x, y, t)
}

fn process(raw: RawRow, cr: ClassRow, pair_out: &mut Vec<Pair>, pair_seq: &mut usize) -> Row {
    let dual_exact = rats(raw.dual_vertices_rational.clone());
    let _poly = SysLandscapePolytopeCache::from_rational_parts(
        dual_exact.clone(),
        rats(raw.vertices_rational.clone()),
    )
    .unwrap();
    let duals: Vec<Vector4<f64>> = raw
        .dual_vertices
        .iter()
        .map(|a| Vector4::from_row_slice(a))
        .collect();
    let fc = classify_facets_from_dual_vertices(&duals).unwrap();
    let qset: HashSet<_> = fc.q_indices.iter().copied().collect();
    let pset: HashSet<_> = fc.p_indices.iter().copied().collect();
    let a2 = cr.class_minima.get("2").and_then(|x| x.as_ref());
    let a3 = cr.class_minima.get("3").and_then(|x| x.as_ref());
    let (a2, a3) = (a2, a3);
    let gap = cr.normalized_three_minus_two_gap.unwrap_or(f64::NAN);
    let mut row = Row {
        name: raw.name.clone(),
        k: raw.k,
        m: raw.m,
        gap_signed: gap,
        gap_abs: gap.abs(),
        exact_action2: a2.map(|x| x.action_exact.clone()),
        exact_action3: a3.map(|x| x.action_exact.clone()),
        a2_minimizer_count: a2.map_or(0, |x| x.minimizer_sigmas.len()),
        a3_minimizer_count: a3.map_or(0, |x| x.minimizer_sigmas.len()),
        a3_six_singleton_count: 0,
        a3_eight_facet_count: 0,
        a2_word_lengths: Vec::new(),
        a3_word_lengths: Vec::new(),
        a2_supports: Vec::new(),
        a3_supports: Vec::new(),
        max_support_intersection: 0,
        max_support_jaccard: 0.0,
        exact_support_equal: false,
        same_support_pair_count: 0,
        different_support_pair_count: 0,
        primary_pair_ids: Vec::new(),
        min_beta_product: None,
        max_beta_product: None,
        beta_product_range: None,
        min_pairing_factor: None,
        min_cancellation_ratio: None,
        recovery_pass_count: 0,
        recovery_total_count: 0,
        eight_facet_gap_abs: None,
        eight_facet_rank_kernel_dimensions: Vec::new(),
    };
    let a2s = a2.map(|x| x.minimizer_sigmas.clone()).unwrap_or_default();
    let a3s = a3.map(|x| x.minimizer_sigmas.clone()).unwrap_or_default();
    row.a2_supports = a2s.iter().map(|s| support(s)).collect();
    row.a3_supports = a3s.iter().map(|s| support(s)).collect();
    row.a2_word_lengths = a2s.iter().map(Vec::len).collect();
    row.a3_word_lengths = a3s.iter().map(Vec::len).collect();
    row.a3_six_singleton_count = a3s.iter().filter(|s| s.len() == 6).count();
    row.a3_eight_facet_count = a3s.iter().filter(|s| s.len() == 8).count();
    if row.a3_eight_facet_count > 0 {
        row.eight_facet_gap_abs = Some(row.gap_abs);
        row.eight_facet_rank_kernel_dimensions = a3s
            .iter()
            .filter(|s| s.len() == 8)
            .map(|s| kkt_kernel_dimension(&dual_exact, s))
            .collect();
    }
    for s2 in &a2s {
        for s3 in &a3s {
            let ss2 = support(s2);
            let ss3 = support(s3);
            let inter = ss2.iter().filter(|x| ss3.contains(x)).count();
            row.max_support_intersection = row.max_support_intersection.max(inter);
            row.max_support_jaccard = row.max_support_jaccard.max(jaccard(&ss2, &ss3));
            if ss2 != ss3 {
                row.different_support_pair_count += 1;
                continue;
            }
            row.exact_support_equal = true;
            if s2.len() != 6
                || s3.len() != 6
                || bounce_count_from_sigma_for_facets(&fc.q_indices, &fc.p_indices, s3) != Some(3)
            {
                continue;
            }
            row.same_support_pair_count += 1;
            let x = solve(&dual_exact, s2);
            let y = solve(&dual_exact, s3);
            if let (Some(c2), Some(c3)) = (a2, a3) {
                assert_eq!(
                    x.action,
                    rat(&c2.action_exact),
                    "A2 fixed-sigma action mismatch"
                );
                assert_eq!(
                    y.action,
                    rat(&c3.action_exact),
                    "A3 fixed-sigma action mismatch"
                );
            }
            let (ax, ay, terms) = align(&dual_exact, &x, &y, &qset, &pset);
            let bx = rotated_beta(&x.sigma, &x.beta, &ax);
            let by = rotated_beta(&y.sigma, &y.beta, &ay);
            for rotated in rotations(&x.sigma) {
                let beta = rotated_beta(&x.sigma, &x.beta, &rotated);
                assert_eq!(
                    q_of(&dual_exact, &rotated, &beta),
                    x.q,
                    "A2 cyclic Q invariance failed"
                );
            }
            for rotated in rotations(&y.sigma) {
                let beta = rotated_beta(&y.sigma, &y.beta, &rotated);
                assert_eq!(
                    q_of(&dual_exact, &rotated, &beta),
                    y.q,
                    "A3 cyclic Q invariance failed"
                );
            }
            assert_eq!(
                q_of(&dual_exact, &ax, &bx),
                x.q,
                "A2 within-word rotation beta/Q invariance failed"
            );
            assert_eq!(
                q_of(&dual_exact, &ay, &by),
                y.q,
                "A3 within-word rotation beta/Q invariance failed"
            );
            assert!(
                bx.iter().enumerate().all(|(i, beta)| beta
                    == &x.beta[x.sigma.iter().position(|facet| facet == &ax[i]).unwrap()]),
                "A2 within-word rotation beta mapping mismatch"
            );
            assert!(
                by.iter().enumerate().all(|(i, beta)| beta
                    == &y.beta[y.sigma.iter().position(|facet| facet == &ay[i]).unwrap()]),
                "A3 within-word rotation beta mapping mismatch"
            );
            let net = q_of(&dual_exact, &ax, &bx) - q_of(&dual_exact, &ay, &by);
            if net != &x.q - &y.q {
                panic!("swap identity failed {} net={} direct={} qx={} qy={} terms={:?} ax={:?} ay={:?}",raw.name,net,&x.q-&y.q,x.q,y.q,terms,ax,ay)
            }
            assert_eq!(
                &y.action / &x.action - BigRational::from_integer(1.into()),
                &x.q / &y.q - BigRational::from_integer(1.into()),
                "action-ratio identity failed"
            );
            let gross = terms
                .iter()
                .map(|(_, _, c)| c.abs())
                .fold(BigRational::zero(), |a, b| a + b);
            let norm = f64r(&net) / f64r(&y.q);
            let cancel = if gross.is_zero() {
                None
            } else {
                Some(f64r(&net.abs()) / f64r(&gross))
            };
            let mut beta_prod = None;
            let mut pairing = None;
            let mut minterm: Option<f64> = None;
            let mut maxterm: Option<f64> = None;
            let mut jt = Vec::new();
            for (i, j, c) in &terms {
                let bi = &x.beta[x.sigma.iter().position(|z| z == i).unwrap()];
                let bj = &x.beta[x.sigma.iter().position(|z| z == j).unwrap()];
                let w = omega(&dual_exact[*i], &dual_exact[*j]);
                let abs = c.abs();
                let nabs = f64r(&abs) / f64r(&y.q);
                minterm = Some(minterm.map_or(nabs, |x| x.min(nabs)));
                maxterm = Some(maxterm.map_or(nabs, |x| x.max(nabs)));
                if terms.len() == 1 {
                    beta_prod = Some(bi * bj);
                    pairing = Some(2.0 * f64r(&w.abs()) / f64r(&y.q));
                }
                jt.push(Term {
                    i: *i,
                    j: *j,
                    signed: c.to_string(),
                    abs: abs.to_string(),
                    beta_i: bi.to_string(),
                    beta_j: bj.to_string(),
                    omega: w.to_string(),
                    normalized_abs: nabs,
                    normalized_pairing: 2.0 * f64r(&w.abs()) / f64r(&y.q),
                });
            }
            let beta3: Vec<f64> = y.beta.iter().map(f64r).collect();
            let orbit =
                recover_and_verify_sigma_beta_action(&duals, &y.sigma, &beta3, f64r(&y.action));
            let (valid, mv, closure, ae) = match orbit {
                Some(orbit) => {
                    let action_error = (orbit.action - f64r(&y.action)).abs() / f64r(&y.action);
                    let valid = orbit.max_violation <= 1e-8
                        && orbit.closure_error <= 1e-8
                        && action_error <= 1e-8
                        && orbit.max_violation.is_finite()
                        && orbit.closure_error.is_finite()
                        && action_error.is_finite();
                    (
                        valid,
                        Some(orbit.max_violation),
                        Some(orbit.closure_error),
                        Some(action_error),
                    )
                }
                None => (false, None, None, None),
            };
            row.recovery_total_count += 1;
            if valid {
                row.recovery_pass_count += 1;
            }
            let bp = beta_prod.as_ref().map(f64r);
            if let Some(v) = bp {
                row.min_beta_product = Some(row.min_beta_product.map_or(v, |x| x.min(v)));
                row.max_beta_product = Some(row.max_beta_product.map_or(v, |x| x.max(v)));
            }
            let id = *pair_seq;
            *pair_seq += 1;
            row.primary_pair_ids.push(id);
            pair_out.push(Pair {
                name: raw.name.clone(),
                pair_id: id,
                a2_sigma: ax,
                a3_sigma: ay,
                support: ss2,
                exact_q2: x.q.to_string(),
                exact_q3: y.q.to_string(),
                exact_action2: x.action.to_string(),
                exact_action3: y.action.to_string(),
                signed_gap_q: net.to_string(),
                normalized_gap: norm,
                flip_count: terms.len(),
                gross_abs_q: gross.to_string(),
                cancellation_ratio: cancel,
                min_abs_term_over_q3: minterm,
                max_abs_term_over_q3: maxterm,
                beta_product: beta_prod.map(|v| v.to_string()),
                normalized_pairing_factor: pairing,
                terms: jt,
                recovery_valid: valid,
                max_violation: mv,
                closure_error: closure,
                action_error_rel: ae,
                alignment_convention: "symmetric_both_words",
                a2_rotation_count: x.sigma.len(),
                a3_rotation_count: y.sigma.len(),
            });
        }
    }
    row.beta_product_range = match (row.min_beta_product, row.max_beta_product) {
        (Some(a), Some(b)) => Some(b - a),
        _ => None,
    };
    row
}

fn main() {
    let a = args();
    // Exact bytes are advisory provenance, not a compatibility gate. The
    // schema, joins, and exact recomputations below remain blocking checks.
    for (label, actual, reviewed) in [
        ("raw", sha(&a.raw), RAW_SHA256),
        ("class", sha(&a.class), CLASS_SHA256),
    ] {
        if actual != reviewed {
            eprintln!(
                "warning: {label} input differs from the retained bytes; \
                 continuing with semantic checks. Reassess retained \
                 interpretations before treating this run as equivalent."
            );
        }
    }
    let raws: Vec<RawRow> = BufReader::new(File::open(&a.raw).unwrap())
        .lines()
        .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
        .collect();
    let cls: Vec<ClassRow> = BufReader::new(File::open(&a.class).unwrap())
        .lines()
        .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
        .collect();
    assert_eq!(raws.len(), 10240);
    assert_eq!(cls.len(), 10240);
    let cmap: HashMap<_, _> = cls.into_iter().map(|r| (r.name.clone(), r)).collect();
    assert_eq!(cmap.len(), raws.len());
    let selected: Vec<RawRow> = match a.limit {
        None => raws,
        Some(n) => {
            assert!(n <= raws.len());
            (0..n)
                .map(|i| raws[(i * raws.len() / n).min(raws.len() - 1)].clone())
                .collect()
        }
    };
    let mut pairs = Vec::new();
    let mut rows = Vec::new();
    let mut seq = 0;
    for raw in selected {
        let cr = cmap.get(&raw.name).unwrap();
        rows.push(process(raw, cr.clone(), &mut pairs, &mut seq));
    }
    std::fs::create_dir_all(&a.out).unwrap();
    let mut w = BufWriter::new(File::create(a.out.join("degeneration-pairs.jsonl")).unwrap());
    for x in &pairs {
        writeln!(w, "{}", serde_json::to_string(x).unwrap()).unwrap()
    }
    let mut w = BufWriter::new(File::create(a.out.join("degeneration-rows.jsonl")).unwrap());
    for x in &rows {
        writeln!(w, "{}", serde_json::to_string(x).unwrap()).unwrap()
    }
    eprintln!("wrote {} rows and {} pairs", rows.len(), pairs.len());
}
