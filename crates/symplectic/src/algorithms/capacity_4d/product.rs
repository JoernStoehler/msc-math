//! Production KKT-free capacity route for four-dimensional Lagrangian products.
//!
//! Mathematical contract: `formal/product-qp-six-facet-reduction.tex`,
//! especially `thm:product-qp-six-facet-maximizer` and
//! `cor:product-qp-closure-vertex-enumeration`.
//!
//! This module computes the scalar capacity and sparse maximizing witnesses.
//! It does not classify every maximizing or near-maximizing HK branch.

use crate::geom::rational_arithmetic::f64_to_rational;
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::BTreeMap;
use std::hint::black_box;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ProductClosureError {
    NonFiniteFacet { facet: usize },
    NotStructuralProduct { facet: usize },
    ZeroFacet { facet: usize },
    TooFewFacets { factor: &'static str, count: usize },
    NoClosureVertex { factor: &'static str },
    NoPositiveCandidate,
    InternalSupportMismatch,
}

#[derive(Clone, Debug, Default)]
struct ProductClosureStats {
    q_closure_vertices: usize,
    p_closure_vertices: usize,
    support_pairs_tested: usize,
    support_triples_tested: usize,
    interval_certified_vertices: usize,
    interval_certified_rejections: usize,
    support_exact_fallbacks: usize,
    support_fallback_vertices: usize,
    support_fallback_rejections: usize,
    cyclic_orders_evaluated: usize,
    exact_winner_contenders: usize,
    gradual_underflow_available: bool,
    full_exact_fallback: bool,
    closure_enumeration_ms: f64,
    objective_enumeration_ms: f64,
    exact_winner_resolution_ms: f64,
    total_ms: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ProductClosureWinner {
    pub(super) sigma: Vec<usize>,
    pub(super) beta_exact: Vec<BigRational>,
}

#[derive(Clone, Debug)]
pub(super) struct ProductClosureCapacityReport {
    pub(super) capacity_exact: BigRational,
    pub(super) winners: Vec<ProductClosureWinner>,
    stats: ProductClosureStats,
}

#[derive(Clone, Debug)]
struct ExactClosureVertex {
    facets: Vec<usize>,
    weights: Vec<BigRational>,
}

#[derive(Clone, Debug)]
struct FloatClosureVertex {
    facets: Vec<usize>,
    weight_intervals: Vec<Interval>,
}

#[derive(Clone, Debug)]
struct FloatCandidate {
    sigma: Vec<usize>,
    q_support: Vec<usize>,
    p_support: Vec<usize>,
    q_interval: Interval,
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn point(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            lo: next_down(self.lo + rhs.lo),
            hi: next_up(self.hi + rhs.hi),
        }
    }

    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }

    fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        Self {
            lo: next_down(products.iter().copied().fold(f64::INFINITY, f64::min)),
            hi: next_up(products.iter().copied().fold(f64::NEG_INFINITY, f64::max)),
        }
    }

    fn div(self, rhs: Self) -> Option<Self> {
        if rhs.lo <= 0.0 && rhs.hi >= 0.0 {
            return None;
        }
        let reciprocal = Self {
            lo: next_down(1.0 / rhs.hi),
            hi: next_up(1.0 / rhs.lo),
        };
        Some(self.mul(reciprocal))
    }

    fn contains_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }

    fn is_finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite() && self.lo <= self.hi
    }
}

#[derive(Clone, Debug)]
enum IntervalSupportDecision {
    Vertex(FloatClosureVertex),
    Reject,
    Indeterminate,
}

/// Certified hybrid route on an already validated structural product.
///
/// The caller must establish that the supplied finite binary64 dual vertices
/// define two full-dimensional bounded factor polygons with the origin in
/// their interiors. This function validates only finite structural q/p
/// splitting and the existence of factor closure vertices. It returns the
/// exact capacity of the binary64 input and sparse exact maximizing witnesses.
pub(super) fn solve_product_closure_capacity_hybrid(
    dual_vertices: &[Vector4<f64>],
) -> Result<ProductClosureCapacityReport, ProductClosureError> {
    let total_started = Instant::now();
    if !gradual_underflow_available() {
        let mut exact = solve_product_closure_capacity_exact_binary64(dual_vertices)?;
        exact.stats.gradual_underflow_available = false;
        exact.stats.full_exact_fallback = true;
        return Ok(exact);
    }
    let (q_indices, p_indices) = split_product_indices_f64(dual_vertices)?;
    let exact_vertices = exact_binary64_vertices(dual_vertices);
    let mut stats = ProductClosureStats {
        gradual_underflow_available: true,
        ..ProductClosureStats::default()
    };

    let closure_started = Instant::now();
    let q_vertices = enumerate_float_closure_vertices(
        dual_vertices,
        &exact_vertices,
        &q_indices,
        true,
        &mut stats,
    );
    let p_vertices = enumerate_float_closure_vertices(
        dual_vertices,
        &exact_vertices,
        &p_indices,
        false,
        &mut stats,
    );
    stats.closure_enumeration_ms = elapsed_ms(closure_started);
    stats.q_closure_vertices = q_vertices.len();
    stats.p_closure_vertices = p_vertices.len();
    if q_vertices.is_empty() {
        return Err(ProductClosureError::NoClosureVertex { factor: "q" });
    }
    if p_vertices.is_empty() {
        return Err(ProductClosureError::NoClosureVertex { factor: "p" });
    }

    let objective_started = Instant::now();
    let candidates = enumerate_float_candidates(dual_vertices, &q_vertices, &p_vertices);
    stats.objective_enumeration_ms = elapsed_ms(objective_started);
    stats.cyclic_orders_evaluated = candidates.len();
    if candidates.is_empty() {
        return Err(ProductClosureError::NoPositiveCandidate);
    }
    if candidates
        .iter()
        .any(|candidate| !candidate.q_interval.is_finite())
    {
        let mut exact = solve_product_closure_capacity_exact_binary64(dual_vertices)?;
        exact.stats.gradual_underflow_available = true;
        exact.stats.full_exact_fallback = true;
        return Ok(exact);
    }

    let max_lower = candidates
        .iter()
        .map(|candidate| candidate.q_interval.lo)
        .fold(f64::NEG_INFINITY, f64::max);
    let contenders = candidates
        .iter()
        .filter(|candidate| candidate.q_interval.hi >= max_lower)
        .collect::<Vec<_>>();
    stats.exact_winner_contenders = contenders.len();

    let exact_started = Instant::now();
    let q_exact_cache = q_vertices
        .iter()
        .map(|vertex| {
            exact_closure_vertex_for_support(&exact_vertices, &vertex.facets, true)
                .map(|exact| (vertex.facets.clone(), exact))
                .ok_or(ProductClosureError::InternalSupportMismatch)
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let p_exact_cache = p_vertices
        .iter()
        .map(|vertex| {
            exact_closure_vertex_for_support(&exact_vertices, &vertex.facets, false)
                .map(|exact| (vertex.facets.clone(), exact))
                .ok_or(ProductClosureError::InternalSupportMismatch)
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;
    let mut resolved = Vec::with_capacity(contenders.len());
    for candidate in contenders {
        let q_vertex = q_exact_cache
            .get(&candidate.q_support)
            .cloned()
            .ok_or(ProductClosureError::InternalSupportMismatch)?;
        let p_vertex = p_exact_cache
            .get(&candidate.p_support)
            .cloned()
            .ok_or(ProductClosureError::InternalSupportMismatch)?;
        let q_exact = exact_q_for_word(&exact_vertices, &candidate.sigma, &q_vertex, &p_vertex);
        resolved.push((candidate.sigma.clone(), q_vertex, p_vertex, q_exact));
    }
    stats.exact_winner_resolution_ms = elapsed_ms(exact_started);

    let report = report_from_resolved(resolved, stats, total_started)?;
    Ok(report)
}

/// Complete exact closure-vertex reference on an already validated product.
///
/// This has the same input contract and scalar/sparse-witness output scope as
/// [`solve_product_closure_capacity_hybrid`], but evaluates every candidate
/// over the exact rational values represented by the binary64 inputs.
pub fn solve_product_closure_capacity_exact_binary64(
    dual_vertices: &[Vector4<f64>],
) -> Result<ProductClosureCapacityReport, ProductClosureError> {
    let total_started = Instant::now();
    split_product_indices_f64(dual_vertices)?;
    let exact_vertices = exact_binary64_vertices(dual_vertices);
    let (q_indices, p_indices) = split_product_indices_exact(&exact_vertices)?;
    let mut stats = ProductClosureStats::default();

    let closure_started = Instant::now();
    let q_vertices =
        enumerate_exact_closure_vertices(&exact_vertices, &q_indices, true, &mut stats);
    let p_vertices =
        enumerate_exact_closure_vertices(&exact_vertices, &p_indices, false, &mut stats);
    stats.closure_enumeration_ms = elapsed_ms(closure_started);
    stats.q_closure_vertices = q_vertices.len();
    stats.p_closure_vertices = p_vertices.len();
    if q_vertices.is_empty() {
        return Err(ProductClosureError::NoClosureVertex { factor: "q" });
    }
    if p_vertices.is_empty() {
        return Err(ProductClosureError::NoClosureVertex { factor: "p" });
    }

    let objective_started = Instant::now();
    let mut resolved = Vec::new();
    for q_vertex in &q_vertices {
        for p_vertex in &p_vertices {
            let mut labels = q_vertex.facets.clone();
            labels.extend_from_slice(&p_vertex.facets);
            for_each_cyclic_order(&labels, |sigma| {
                let q_exact = exact_q_for_word(&exact_vertices, sigma, q_vertex, p_vertex);
                resolved.push((sigma.to_vec(), q_vertex.clone(), p_vertex.clone(), q_exact));
            });
        }
    }
    stats.objective_enumeration_ms = elapsed_ms(objective_started);
    stats.cyclic_orders_evaluated = resolved.len();
    stats.exact_winner_contenders = resolved.len();
    report_from_resolved(resolved, stats, total_started)
}

fn report_from_resolved(
    resolved: Vec<(
        Vec<usize>,
        ExactClosureVertex,
        ExactClosureVertex,
        BigRational,
    )>,
    mut stats: ProductClosureStats,
    total_started: Instant,
) -> Result<ProductClosureCapacityReport, ProductClosureError> {
    let q_max_exact = resolved
        .iter()
        .map(|(_, _, _, q)| q)
        .filter(|q| q.is_positive())
        .max()
        .cloned()
        .ok_or(ProductClosureError::NoPositiveCandidate)?;
    let capacity_exact = BigRational::one() / (q_max_exact.clone() + q_max_exact.clone());

    let mut winners = Vec::new();
    for (sigma, q_vertex, p_vertex, q) in resolved {
        if q != q_max_exact {
            continue;
        }
        let beta_exact = beta_for_sigma(&sigma, &q_vertex, &p_vertex);
        winners.push(ProductClosureWinner { sigma, beta_exact });
    }
    winners.sort_by(|left, right| left.sigma.cmp(&right.sigma));
    winners.dedup_by(|left, right| left.sigma == right.sigma);
    stats.total_ms = elapsed_ms(total_started);

    Ok(ProductClosureCapacityReport {
        capacity_exact,
        winners,
        stats,
    })
}

fn enumerate_exact_closure_vertices(
    exact_vertices: &[[BigRational; 4]],
    indices: &[usize],
    q_factor: bool,
    stats: &mut ProductClosureStats,
) -> Vec<ExactClosureVertex> {
    let mut vertices = Vec::new();
    for_each_combination(indices, 2, |support| {
        stats.support_pairs_tested += 1;
        if let Some(vertex) = exact_closure_vertex_for_support(exact_vertices, support, q_factor) {
            vertices.push(vertex);
        }
    });
    for_each_combination(indices, 3, |support| {
        stats.support_triples_tested += 1;
        if let Some(vertex) = exact_closure_vertex_for_support(exact_vertices, support, q_factor) {
            vertices.push(vertex);
        }
    });
    vertices.sort_by(|left, right| left.facets.cmp(&right.facets));
    vertices
}

fn enumerate_float_closure_vertices(
    dual_vertices: &[Vector4<f64>],
    exact_vertices: &[[BigRational; 4]],
    indices: &[usize],
    q_factor: bool,
    stats: &mut ProductClosureStats,
) -> Vec<FloatClosureVertex> {
    let mut vertices = Vec::new();
    for_each_combination(indices, 2, |support| {
        stats.support_pairs_tested += 1;
        resolve_float_support(
            dual_vertices,
            exact_vertices,
            support,
            q_factor,
            stats,
            &mut vertices,
        );
    });
    for_each_combination(indices, 3, |support| {
        stats.support_triples_tested += 1;
        resolve_float_support(
            dual_vertices,
            exact_vertices,
            support,
            q_factor,
            stats,
            &mut vertices,
        );
    });
    vertices.sort_by(|left, right| left.facets.cmp(&right.facets));
    vertices
}

fn resolve_float_support(
    dual_vertices: &[Vector4<f64>],
    exact_vertices: &[[BigRational; 4]],
    support: &[usize],
    q_factor: bool,
    stats: &mut ProductClosureStats,
    vertices: &mut Vec<FloatClosureVertex>,
) {
    match interval_closure_vertex_for_support(dual_vertices, support, q_factor) {
        IntervalSupportDecision::Vertex(vertex) => {
            stats.interval_certified_vertices += 1;
            vertices.push(vertex);
        }
        IntervalSupportDecision::Reject => {
            stats.interval_certified_rejections += 1;
        }
        IntervalSupportDecision::Indeterminate => {
            stats.support_exact_fallbacks += 1;
            match exact_closure_vertex_for_support(exact_vertices, support, q_factor) {
                Some(vertex) => {
                    stats.support_fallback_vertices += 1;
                    vertices.push(float_vertex_from_exact(&vertex));
                }
                None => {
                    stats.support_fallback_rejections += 1;
                }
            }
        }
    }
}

fn exact_closure_vertex_for_support(
    exact_vertices: &[[BigRational; 4]],
    support: &[usize],
    q_factor: bool,
) -> Option<ExactClosureVertex> {
    let points = support
        .iter()
        .map(|&facet| exact_factor_point(&exact_vertices[facet], q_factor))
        .collect::<Vec<_>>();
    let weights = match support.len() {
        2 => exact_pair_weights(&points[0], &points[1])?,
        3 => exact_triple_weights(&points[0], &points[1], &points[2])?,
        _ => return None,
    };
    weights
        .iter()
        .all(|weight| weight.is_positive())
        .then(|| ExactClosureVertex {
            facets: support.to_vec(),
            weights,
        })
}

fn exact_pair_weights(
    left: &[BigRational; 2],
    right: &[BigRational; 2],
) -> Option<Vec<BigRational>> {
    if cross_exact(left, right) != BigRational::zero() {
        return None;
    }
    let difference = [
        left[0].clone() - right[0].clone(),
        left[1].clone() - right[1].clone(),
    ];
    let coordinate = if !difference[0].is_zero() {
        0
    } else if !difference[1].is_zero() {
        1
    } else {
        return None;
    };
    let alpha = -right[coordinate].clone() / difference[coordinate].clone();
    let other = BigRational::one() - alpha.clone();
    let closure_x = alpha.clone() * left[0].clone() + other.clone() * right[0].clone();
    let closure_y = alpha.clone() * left[1].clone() + other.clone() * right[1].clone();
    (closure_x.is_zero() && closure_y.is_zero()).then_some(vec![alpha, other])
}

fn exact_triple_weights(
    a: &[BigRational; 2],
    b: &[BigRational; 2],
    c: &[BigRational; 2],
) -> Option<Vec<BigRational>> {
    let numerators = [cross_exact(b, c), cross_exact(c, a), cross_exact(a, b)];
    let denominator = numerators[0].clone() + numerators[1].clone() + numerators[2].clone();
    if denominator.is_zero() {
        return None;
    }
    Some(
        numerators
            .into_iter()
            .map(|numerator| numerator / denominator.clone())
            .collect(),
    )
}

fn interval_closure_vertex_for_support(
    dual_vertices: &[Vector4<f64>],
    support: &[usize],
    q_factor: bool,
) -> IntervalSupportDecision {
    let points = support
        .iter()
        .map(|&facet| float_factor_point(&dual_vertices[facet], q_factor))
        .collect::<Vec<_>>();
    match support.len() {
        2 => {
            let determinant = cross_interval(points[0], points[1]);
            if !determinant.contains_zero() {
                IntervalSupportDecision::Reject
            } else {
                IntervalSupportDecision::Indeterminate
            }
        }
        3 => {
            let numerators = [
                cross_interval(points[1], points[2]),
                cross_interval(points[2], points[0]),
                cross_interval(points[0], points[1]),
            ];
            let denominator = numerators[0].add(numerators[1]).add(numerators[2]);
            if !denominator.is_finite()
                || numerators.iter().any(|numerator| !numerator.is_finite())
                || denominator.contains_zero()
            {
                return IntervalSupportDecision::Indeterminate;
            }
            let Some(weight_intervals) = numerators
                .iter()
                .map(|numerator| numerator.div(denominator))
                .collect::<Option<Vec<_>>>()
            else {
                return IntervalSupportDecision::Indeterminate;
            };
            if weight_intervals.iter().any(|weight| !weight.is_finite()) {
                return IntervalSupportDecision::Indeterminate;
            }
            if weight_intervals.iter().all(|weight| weight.lo > 0.0) {
                IntervalSupportDecision::Vertex(FloatClosureVertex {
                    facets: support.to_vec(),
                    weight_intervals,
                })
            } else if weight_intervals.iter().any(|weight| weight.hi <= 0.0) {
                IntervalSupportDecision::Reject
            } else {
                IntervalSupportDecision::Indeterminate
            }
        }
        _ => IntervalSupportDecision::Reject,
    }
}

fn enumerate_float_candidates(
    dual_vertices: &[Vector4<f64>],
    q_vertices: &[FloatClosureVertex],
    p_vertices: &[FloatClosureVertex],
) -> Vec<FloatCandidate> {
    let mut candidates = Vec::new();
    for q_vertex in q_vertices {
        for p_vertex in p_vertices {
            let mut labels = q_vertex.facets.clone();
            labels.extend_from_slice(&p_vertex.facets);
            for_each_cyclic_order(&labels, |sigma| {
                let q_interval =
                    float_q_interval_for_word(dual_vertices, sigma, q_vertex, p_vertex);
                candidates.push(FloatCandidate {
                    sigma: sigma.to_vec(),
                    q_support: q_vertex.facets.clone(),
                    p_support: p_vertex.facets.clone(),
                    q_interval,
                });
            });
        }
    }
    candidates
}

fn exact_q_for_word(
    exact_vertices: &[[BigRational; 4]],
    sigma: &[usize],
    q_vertex: &ExactClosureVertex,
    p_vertex: &ExactClosureVertex,
) -> BigRational {
    let beta = beta_map_exact(q_vertex, p_vertex);
    let mut q = BigRational::zero();
    for earlier in 0..sigma.len() {
        for later in earlier + 1..sigma.len() {
            let left = sigma[earlier];
            let right = sigma[later];
            q += beta[&left].clone()
                * beta[&right].clone()
                * omega_exact(&exact_vertices[left], &exact_vertices[right]);
        }
    }
    q
}

fn float_q_interval_for_word(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
    q_vertex: &FloatClosureVertex,
    p_vertex: &FloatClosureVertex,
) -> Interval {
    let beta = beta_map_float(q_vertex, p_vertex);
    let mut q_interval = Interval::point(0.0);
    for earlier in 0..sigma.len() {
        for later in earlier + 1..sigma.len() {
            let left = sigma[earlier];
            let right = sigma[later];
            q_interval = q_interval.add(
                beta[&left]
                    .mul(beta[&right])
                    .mul(omega_interval(&dual_vertices[left], &dual_vertices[right])),
            );
        }
    }
    q_interval
}

fn beta_map_exact(
    q_vertex: &ExactClosureVertex,
    p_vertex: &ExactClosureVertex,
) -> BTreeMap<usize, BigRational> {
    q_vertex
        .facets
        .iter()
        .zip(&q_vertex.weights)
        .chain(p_vertex.facets.iter().zip(&p_vertex.weights))
        .map(|(&facet, weight)| (facet, weight.clone() / BigRational::from_integer(2.into())))
        .collect()
}

fn beta_map_float(
    q_vertex: &FloatClosureVertex,
    p_vertex: &FloatClosureVertex,
) -> BTreeMap<usize, Interval> {
    q_vertex
        .facets
        .iter()
        .zip(&q_vertex.weight_intervals)
        .chain(p_vertex.facets.iter().zip(&p_vertex.weight_intervals))
        .map(|(&facet, &interval)| (facet, interval.mul(Interval::point(0.5))))
        .collect()
}

fn beta_for_sigma(
    sigma: &[usize],
    q_vertex: &ExactClosureVertex,
    p_vertex: &ExactClosureVertex,
) -> Vec<BigRational> {
    let beta = beta_map_exact(q_vertex, p_vertex);
    sigma.iter().map(|facet| beta[facet].clone()).collect()
}

fn float_vertex_from_exact(vertex: &ExactClosureVertex) -> FloatClosureVertex {
    FloatClosureVertex {
        facets: vertex.facets.clone(),
        weight_intervals: vertex.weights.iter().map(rational_interval).collect(),
    }
}

fn rational_interval(value: &BigRational) -> Interval {
    let point = rational_to_f64(value);
    let point_rational = f64_to_rational(point);
    match point_rational.cmp(value) {
        std::cmp::Ordering::Less => Interval {
            lo: point,
            hi: next_up(point),
        },
        std::cmp::Ordering::Equal => Interval::point(point),
        std::cmp::Ordering::Greater => Interval {
            lo: next_down(point),
            hi: point,
        },
    }
}

fn split_product_indices_f64(
    dual_vertices: &[Vector4<f64>],
) -> Result<(Vec<usize>, Vec<usize>), ProductClosureError> {
    let mut q = Vec::new();
    let mut p = Vec::new();
    for (facet, vertex) in dual_vertices.iter().enumerate() {
        if vertex.iter().any(|coordinate| !coordinate.is_finite()) {
            return Err(ProductClosureError::NonFiniteFacet { facet });
        }
        let q_zero = vertex[0] == 0.0 && vertex[1] == 0.0;
        let p_zero = vertex[2] == 0.0 && vertex[3] == 0.0;
        match (q_zero, p_zero) {
            (false, true) => q.push(facet),
            (true, false) => p.push(facet),
            (true, true) => return Err(ProductClosureError::ZeroFacet { facet }),
            (false, false) => {
                return Err(ProductClosureError::NotStructuralProduct { facet });
            }
        }
    }
    validate_factor_counts(q, p)
}

fn split_product_indices_exact(
    dual_vertices: &[[BigRational; 4]],
) -> Result<(Vec<usize>, Vec<usize>), ProductClosureError> {
    let mut q = Vec::new();
    let mut p = Vec::new();
    for (facet, vertex) in dual_vertices.iter().enumerate() {
        let q_zero = vertex[0].is_zero() && vertex[1].is_zero();
        let p_zero = vertex[2].is_zero() && vertex[3].is_zero();
        match (q_zero, p_zero) {
            (false, true) => q.push(facet),
            (true, false) => p.push(facet),
            (true, true) => return Err(ProductClosureError::ZeroFacet { facet }),
            (false, false) => {
                return Err(ProductClosureError::NotStructuralProduct { facet });
            }
        }
    }
    validate_factor_counts(q, p)
}

fn validate_factor_counts(
    q: Vec<usize>,
    p: Vec<usize>,
) -> Result<(Vec<usize>, Vec<usize>), ProductClosureError> {
    if q.len() < 3 {
        return Err(ProductClosureError::TooFewFacets {
            factor: "q",
            count: q.len(),
        });
    }
    if p.len() < 3 {
        return Err(ProductClosureError::TooFewFacets {
            factor: "p",
            count: p.len(),
        });
    }
    Ok((q, p))
}

fn exact_binary64_vertices(dual_vertices: &[Vector4<f64>]) -> Vec<[BigRational; 4]> {
    dual_vertices
        .iter()
        .map(|vertex| {
            [
                f64_to_rational(vertex[0]),
                f64_to_rational(vertex[1]),
                f64_to_rational(vertex[2]),
                f64_to_rational(vertex[3]),
            ]
        })
        .collect()
}

fn exact_factor_point(vertex: &[BigRational; 4], q_factor: bool) -> [BigRational; 2] {
    if q_factor {
        [vertex[0].clone(), vertex[1].clone()]
    } else {
        [vertex[2].clone(), vertex[3].clone()]
    }
}

fn float_factor_point(vertex: &Vector4<f64>, q_factor: bool) -> [Interval; 2] {
    if q_factor {
        [Interval::point(vertex[0]), Interval::point(vertex[1])]
    } else {
        [Interval::point(vertex[2]), Interval::point(vertex[3])]
    }
}

fn cross_exact(left: &[BigRational; 2], right: &[BigRational; 2]) -> BigRational {
    left[0].clone() * right[1].clone() - left[1].clone() * right[0].clone()
}

fn cross_interval(left: [Interval; 2], right: [Interval; 2]) -> Interval {
    left[0].mul(right[1]).sub(left[1].mul(right[0]))
}

fn omega_exact(left: &[BigRational; 4], right: &[BigRational; 4]) -> BigRational {
    left[0].clone() * right[2].clone() - left[2].clone() * right[0].clone()
        + left[1].clone() * right[3].clone()
        - left[3].clone() * right[1].clone()
}

fn omega_interval(left: &Vector4<f64>, right: &Vector4<f64>) -> Interval {
    Interval::point(left[0])
        .mul(Interval::point(right[2]))
        .sub(Interval::point(left[2]).mul(Interval::point(right[0])))
        .add(Interval::point(left[1]).mul(Interval::point(right[3])))
        .sub(Interval::point(left[3]).mul(Interval::point(right[1])))
}

fn for_each_combination(values: &[usize], size: usize, mut visit: impl FnMut(&[usize])) {
    fn recurse(
        values: &[usize],
        size: usize,
        start: usize,
        selected: &mut Vec<usize>,
        visit: &mut impl FnMut(&[usize]),
    ) {
        if selected.len() == size {
            visit(selected);
            return;
        }
        let remaining = size - selected.len();
        if values.len().saturating_sub(start) < remaining {
            return;
        }
        for index in start..=values.len() - remaining {
            selected.push(values[index]);
            recurse(values, size, index + 1, selected, visit);
            selected.pop();
        }
    }
    let mut selected = Vec::with_capacity(size);
    recurse(values, size, 0, &mut selected, &mut visit);
}

fn for_each_cyclic_order(labels: &[usize], mut visit: impl FnMut(&[usize])) {
    let Some((&first, rest)) = labels.split_first() else {
        return;
    };
    let mut rest = rest.to_vec();
    let mut word = Vec::with_capacity(labels.len());
    permute(&mut rest, 0, &mut |permutation| {
        word.clear();
        word.push(first);
        word.extend_from_slice(permutation);
        visit(&word);
    });
}

fn permute(values: &mut [usize], start: usize, visit: &mut impl FnMut(&[usize])) {
    if start == values.len() {
        visit(values);
        return;
    }
    for index in start..values.len() {
        values.swap(start, index);
        permute(values, start + 1, visit);
        values.swap(start, index);
    }
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or(f64::NAN)
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

/// Runtime check for both flush-to-zero outputs and denormals-are-zero inputs.
///
/// Outward one-ulp interval widening is sound only when the floating-point
/// environment preserves subnormals. Opaque operands prevent release builds
/// from constant-folding this environment check.
#[inline(never)]
fn gradual_underflow_available() -> bool {
    let minimum_normal = black_box(f64::MIN_POSITIVE);
    let half = black_box(0.5_f64);
    let expected_half_normal = f64::from_bits(1_u64 << 51);
    let half_normal = black_box(minimum_normal * half);

    let minimum_subnormal = black_box(f64::from_bits(1));
    let one = black_box(1.0_f64);
    let preserved_subnormal = black_box(minimum_subnormal * one);

    half_normal == expected_half_normal && preserved_subnormal == f64::from_bits(1)
}

fn next_up(value: f64) -> f64 {
    if value.is_nan() || value == f64::INFINITY {
        return value;
    }
    if value == -0.0 {
        return f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value >= 0.0 { bits + 1 } else { bits - 1 })
}

fn next_down(value: f64) -> f64 {
    if value.is_nan() || value == f64::NEG_INFINITY {
        return value;
    }
    if value == 0.0 {
        return -f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits - 1 } else { bits + 1 })
}
