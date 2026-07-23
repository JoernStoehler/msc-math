use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, Matrix4, Vector4};
use std::time::Instant;
use symplectic::omega0;

const EPS_DET: f64 = 1e-12;
const EPS_INEQUALITY: f64 = 1e-9;
const EPS_VERTEX_DUPLICATE: f64 = 1e-8;
const EPS_OMEGA: f64 = 1e-12;
const EPS_SINGULAR_RESIDUAL: f64 = 1e-8;
const MAX_BOUNDED_VERTEX_COORD: f64 = 1e3;
const EPS_LP_MARGIN: f64 = 1e-10;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum F64Predicate {
    True,
    False,
    Indeterminate,
}

#[derive(Clone, Debug)]
pub(crate) struct F64Combinatorics {
    pub(crate) facet_statuses: Vec<F64Predicate>,
    pub(crate) facet_intersections: DMatrix<F64Predicate>,
    pub(crate) omega_signs: DMatrix<i8>,
    pub(crate) vertex_count: usize,
    pub(crate) facets_with_definite_vertex_count: usize,
    pub(crate) facets_with_possible_vertex_count: usize,
    pub(crate) vertex_indeterminate_count: usize,
    pub(crate) near_singular_vertex_count: usize,
    pub(crate) bounded_near_singular_vertex_count: usize,
    pub(crate) ambiguous_vertex_incidence_count: usize,
    pub(crate) facet_intersection_true_count: usize,
    pub(crate) facet_intersection_false_count: usize,
    pub(crate) facet_intersection_indeterminate_count: usize,
    pub(crate) omega_indeterminate_count: usize,
    pub(crate) minimum_primal_vertex_norm_inf: Option<f64>,
    pub(crate) maximum_primal_vertex_norm_inf: Option<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct F64CombinatoricsTiming {
    pub input_check_ms: f64,
    pub vertex_scan_ms: f64,
    pub facet_coverage_ms: f64,
    pub facet_intersections_ms: f64,
    pub omega_signs_ms: f64,
    pub count_facet_intersections_ms: f64,
    pub lp_facet_statuses_ms: f64,
    pub lp_facet_intersections_ms: f64,
    pub lp_omega_recompute_ms: f64,
    pub lp_count_facet_intersections_ms: f64,
}

#[derive(Clone, Debug)]
struct VertexWitness {
    point: Vector4<f64>,
    definite_incident: Vec<usize>,
    possible_incident: Vec<usize>,
}

#[derive(Clone, Debug)]
pub(crate) struct F64VertexWitness {
    pub(crate) point: Vector4<f64>,
    pub(crate) definite_incident: Vec<usize>,
    pub(crate) possible_incident: Vec<usize>,
}

#[derive(Clone, Debug)]
pub(crate) struct F64VertexScanReport {
    pub(crate) vertices: Vec<F64VertexWitness>,
    pub(crate) near_singular_vertex_count: usize,
    pub(crate) bounded_near_singular_vertex_count: usize,
    pub(crate) ambiguous_vertex_incidence_count: usize,
}

impl F64VertexScanReport {
    pub(crate) fn has_indeterminate_geometry(&self) -> bool {
        self.bounded_near_singular_vertex_count > 0 || self.ambiguous_vertex_incidence_count > 0
    }
}

pub(crate) fn f64_combinatorics(dual_vertices: &[Vector4<f64>]) -> Result<F64Combinatorics, ()> {
    f64_combinatorics_profiled(dual_vertices).map(|(combinatorics, _)| combinatorics)
}

pub(crate) fn f64_vertex_scan_report(
    dual_vertices: &[Vector4<f64>],
) -> Result<F64VertexScanReport, ()> {
    if dual_vertices.len() < 5
        || dual_vertices
            .iter()
            .any(|v| !v.iter().all(|entry| entry.is_finite()))
    {
        return Err(());
    }
    let scan = enumerate_vertex_witnesses(dual_vertices);
    Ok(F64VertexScanReport {
        vertices: scan
            .vertices
            .into_iter()
            .map(|witness| F64VertexWitness {
                point: witness.point,
                definite_incident: witness.definite_incident,
                possible_incident: witness.possible_incident,
            })
            .collect(),
        near_singular_vertex_count: scan.near_singular_vertex_count,
        bounded_near_singular_vertex_count: scan.bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count: scan.ambiguous_vertex_incidence_count,
    })
}

pub(crate) fn f64_combinatorics_profiled(
    dual_vertices: &[Vector4<f64>],
) -> Result<(F64Combinatorics, F64CombinatoricsTiming), ()> {
    let mut timing = F64CombinatoricsTiming::default();
    let started = Instant::now();
    if dual_vertices.len() < 5
        || dual_vertices
            .iter()
            .any(|v| !v.iter().all(|entry| entry.is_finite()))
    {
        return Err(());
    }
    timing.input_check_ms = started.elapsed().as_secs_f64() * 1000.0;

    let started = Instant::now();
    let vertex_scan = enumerate_vertex_witnesses(dual_vertices);
    timing.vertex_scan_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let facet_coverage = facet_coverage(dual_vertices.len(), &vertex_scan);
    timing.facet_coverage_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let facet_intersections =
        facet_intersections_from_vertex_scan(dual_vertices.len(), &vertex_scan);
    timing.facet_intersections_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let (omega_signs, omega_indeterminate_count) =
        omega_signs_f64(dual_vertices, &facet_intersections);
    timing.omega_signs_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let counts = count_facet_intersections(&facet_intersections);
    timing.count_facet_intersections_ms = started.elapsed().as_secs_f64() * 1000.0;

    let minimum_primal_vertex_norm_inf = vertex_scan
        .vertices
        .iter()
        .map(|vertex| norm_inf(&vertex.point))
        .min_by(f64::total_cmp);
    let maximum_primal_vertex_norm_inf = vertex_scan
        .vertices
        .iter()
        .map(|vertex| norm_inf(&vertex.point))
        .max_by(f64::total_cmp);

    Ok((
        F64Combinatorics {
            facet_intersections,
            facet_statuses: facet_coverage.statuses,
            omega_signs,
            vertex_count: vertex_scan.vertices.len(),
            facets_with_definite_vertex_count: facet_coverage.definite_count,
            facets_with_possible_vertex_count: facet_coverage.possible_count,
            vertex_indeterminate_count: vertex_scan.indeterminate_count(),
            near_singular_vertex_count: vertex_scan.near_singular_vertex_count,
            bounded_near_singular_vertex_count: vertex_scan.bounded_near_singular_vertex_count,
            ambiguous_vertex_incidence_count: vertex_scan.ambiguous_vertex_incidence_count,
            facet_intersection_true_count: counts.true_count,
            facet_intersection_false_count: counts.false_count,
            facet_intersection_indeterminate_count: counts.indeterminate_count,
            omega_indeterminate_count,
            minimum_primal_vertex_norm_inf,
            maximum_primal_vertex_norm_inf,
        },
        timing,
    ))
}

fn norm_inf(vector: &Vector4<f64>) -> f64 {
    vector.iter().copied().map(f64::abs).fold(0.0, f64::max)
}

pub(crate) fn f64_combinatorics_with_lp_transitions(
    dual_vertices: &[Vector4<f64>],
) -> Result<F64Combinatorics, ()> {
    f64_combinatorics_with_lp_transitions_profiled(dual_vertices)
        .map(|(combinatorics, _)| combinatorics)
}

pub(crate) fn f64_combinatorics_with_lp_transitions_profiled(
    dual_vertices: &[Vector4<f64>],
) -> Result<(F64Combinatorics, F64CombinatoricsTiming), ()> {
    let (mut combinatorics, mut timing) = f64_combinatorics_profiled(dual_vertices)?;
    let started = Instant::now();
    let facet_statuses = lp_facet_statuses(dual_vertices);
    timing.lp_facet_statuses_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let facet_intersections = lp_facet_intersections(dual_vertices);
    timing.lp_facet_intersections_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let counts = count_facet_intersections(&facet_intersections);
    timing.lp_count_facet_intersections_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let (omega_signs, omega_indeterminate_count) =
        omega_signs_f64(dual_vertices, &facet_intersections);
    timing.lp_omega_recompute_ms = started.elapsed().as_secs_f64() * 1000.0;

    combinatorics.facets_with_definite_vertex_count = facet_statuses
        .iter()
        .filter(|status| **status == F64Predicate::True)
        .count();
    combinatorics.facets_with_possible_vertex_count = facet_statuses
        .iter()
        .filter(|status| **status != F64Predicate::False)
        .count();
    combinatorics.facet_statuses = facet_statuses;
    combinatorics.facet_intersections = facet_intersections;
    combinatorics.facet_intersection_true_count = counts.true_count;
    combinatorics.facet_intersection_false_count = counts.false_count;
    combinatorics.facet_intersection_indeterminate_count = counts.indeterminate_count;
    combinatorics.omega_signs = omega_signs;
    combinatorics.omega_indeterminate_count = omega_indeterminate_count;
    Ok((combinatorics, timing))
}

fn lp_facet_statuses(dual_vertices: &[Vector4<f64>]) -> Vec<F64Predicate> {
    (0..dual_vertices.len())
        .map(|facet| lp_facet_exists(dual_vertices, facet))
        .collect()
}

fn lp_facet_intersections(dual_vertices: &[Vector4<f64>]) -> DMatrix<F64Predicate> {
    let facet_count = dual_vertices.len();
    let mut result = DMatrix::from_element(facet_count, facet_count, F64Predicate::False);
    for i in 0..facet_count {
        result[(i, i)] = F64Predicate::True;
        for j in i + 1..facet_count {
            let status = lp_facet_pair_intersects(dual_vertices, i, j);
            result[(i, j)] = status;
            result[(j, i)] = status;
        }
    }
    result
}

fn lp_facet_exists(dual_vertices: &[Vector4<f64>], facet: usize) -> F64Predicate {
    let mut vars = variables!();
    let x = [
        vars.add(variable()),
        vars.add(variable()),
        vars.add(variable()),
        vars.add(variable()),
    ];
    let tau = vars.add(variable());
    let mut model = vars.maximise(tau).using(default_solver);

    model = model.with(constraint!(dot_expr(dual_vertices[facet], &x) == 1.0));
    for (idx, normal) in dual_vertices.iter().enumerate() {
        if idx != facet {
            model = model.with(constraint!(dot_expr(*normal, &x) <= 1.0 - tau));
        }
    }

    lp_margin_status(model.solve().ok().map(|solution| solution.value(tau)))
}

fn lp_facet_pair_intersects(
    dual_vertices: &[Vector4<f64>],
    left: usize,
    right: usize,
) -> F64Predicate {
    let mut vars = variables!();
    let x = [
        vars.add(variable()),
        vars.add(variable()),
        vars.add(variable()),
        vars.add(variable()),
    ];
    let tau = vars.add(variable());
    let mut model = vars.maximise(tau).using(default_solver);

    model = model.with(constraint!(dot_expr(dual_vertices[left], &x) == 1.0));
    model = model.with(constraint!(dot_expr(dual_vertices[right], &x) == 1.0));
    for (idx, normal) in dual_vertices.iter().enumerate() {
        if idx != left && idx != right {
            model = model.with(constraint!(dot_expr(*normal, &x) <= 1.0 - tau));
        }
    }

    match model.solve().ok().map(|solution| solution.value(tau)) {
        Some(tau) if tau > EPS_LP_MARGIN => F64Predicate::True,
        Some(tau) if tau < -EPS_LP_MARGIN => F64Predicate::False,
        Some(_) => F64Predicate::Indeterminate,
        None => F64Predicate::Indeterminate,
    }
}

fn lp_margin_status(value: Option<f64>) -> F64Predicate {
    match value {
        Some(tau) if tau > EPS_LP_MARGIN => F64Predicate::True,
        Some(tau) if tau < -EPS_LP_MARGIN => F64Predicate::False,
        Some(_) => F64Predicate::Indeterminate,
        None => F64Predicate::Indeterminate,
    }
}

fn dot_expr(normal: Vector4<f64>, x: &[good_lp::Variable; 4]) -> Expression {
    normal[0] * x[0] + normal[1] * x[1] + normal[2] * x[2] + normal[3] * x[3]
}

struct FacetCoverage {
    statuses: Vec<F64Predicate>,
    definite_count: usize,
    possible_count: usize,
}

fn facet_coverage(facet_count: usize, vertex_scan: &VertexScan) -> FacetCoverage {
    let mut definite = vec![false; facet_count];
    let mut possible = vec![false; facet_count];
    for witness in &vertex_scan.vertices {
        for &facet in &witness.definite_incident {
            definite[facet] = true;
            possible[facet] = true;
        }
        for &facet in &witness.possible_incident {
            possible[facet] = true;
        }
    }
    for facets in &vertex_scan.near_singular_facets {
        for &facet in facets {
            possible[facet] = true;
        }
    }
    let statuses = definite
        .iter()
        .zip(&possible)
        .map(|(definite, possible)| {
            if *definite {
                F64Predicate::True
            } else if *possible {
                F64Predicate::Indeterminate
            } else {
                F64Predicate::False
            }
        })
        .collect::<Vec<_>>();
    FacetCoverage {
        definite_count: statuses
            .iter()
            .filter(|status| **status == F64Predicate::True)
            .count(),
        possible_count: statuses
            .iter()
            .filter(|status| **status != F64Predicate::False)
            .count(),
        statuses,
    }
}

struct VertexScan {
    vertices: Vec<VertexWitness>,
    near_singular_facets: Vec<[usize; 4]>,
    near_singular_vertex_count: usize,
    bounded_near_singular_vertex_count: usize,
    ambiguous_vertex_incidence_count: usize,
}

impl VertexScan {
    fn indeterminate_count(&self) -> usize {
        self.near_singular_vertex_count + self.ambiguous_vertex_incidence_count
    }
}

fn enumerate_vertex_witnesses(dual_vertices: &[Vector4<f64>]) -> VertexScan {
    let mut vertices = Vec::new();
    let mut near_singular_facets = Vec::new();
    let mut near_singular_vertex_count = 0usize;
    let mut bounded_near_singular_vertex_count = 0usize;
    let mut ambiguous_vertex_incidence_count = 0usize;

    for i in 0..dual_vertices.len() {
        for j in i + 1..dual_vertices.len() {
            for k in j + 1..dual_vertices.len() {
                for l in k + 1..dual_vertices.len() {
                    let facets = [i, j, k, l];
                    let Some(vertex) = intersection_vertex(dual_vertices, facets) else {
                        near_singular_vertex_count += 1;
                        if is_bounded_near_singular_candidate(dual_vertices, facets) {
                            bounded_near_singular_vertex_count += 1;
                            near_singular_facets.push(facets);
                        }
                        continue;
                    };
                    match classify_vertex_witness(dual_vertices, &vertex, facets) {
                        VertexPredicate::Rejected => {}
                        VertexPredicate::Accepted(witness) => {
                            if witness.definite_incident.len() != witness.possible_incident.len() {
                                ambiguous_vertex_incidence_count += 1;
                            }
                            merge_vertex_witness(&mut vertices, witness);
                        }
                    }
                }
            }
        }
    }

    VertexScan {
        vertices,
        near_singular_facets,
        near_singular_vertex_count,
        bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count,
    }
}

enum VertexPredicate {
    Accepted(VertexWitness),
    Rejected,
}

fn classify_vertex_witness(
    dual_vertices: &[Vector4<f64>],
    vertex: &Vector4<f64>,
    defining_facets: [usize; 4],
) -> VertexPredicate {
    let mut possible_incident = defining_facets.to_vec();
    for (facet, normal) in dual_vertices.iter().enumerate() {
        let gap = normal.dot(vertex) - 1.0;
        if gap > EPS_INEQUALITY {
            return VertexPredicate::Rejected;
        }
        if !defining_facets.contains(&facet) && gap.abs() <= EPS_INEQUALITY {
            possible_incident.push(facet);
        }
    }
    possible_incident.sort_unstable();
    possible_incident.dedup();

    VertexPredicate::Accepted(VertexWitness {
        point: *vertex,
        definite_incident: defining_facets.to_vec(),
        possible_incident,
    })
}

fn merge_vertex_witness(vertices: &mut Vec<VertexWitness>, witness: VertexWitness) {
    if let Some(known) = vertices
        .iter_mut()
        .find(|known| (known.point - witness.point).norm() <= EPS_VERTEX_DUPLICATE)
    {
        merge_indices(&mut known.definite_incident, &witness.definite_incident);
        merge_indices(&mut known.possible_incident, &witness.possible_incident);
    } else {
        vertices.push(witness);
    }
}

fn merge_indices(target: &mut Vec<usize>, source: &[usize]) {
    target.extend(source.iter().copied());
    target.sort_unstable();
    target.dedup();
}

fn facet_intersections_from_vertex_scan(
    facet_count: usize,
    vertex_scan: &VertexScan,
) -> DMatrix<F64Predicate> {
    let mut result = DMatrix::from_element(facet_count, facet_count, F64Predicate::False);

    for facets in &vertex_scan.near_singular_facets {
        for &i in facets {
            for &j in facets {
                result[(i, j)] = F64Predicate::Indeterminate;
            }
        }
    }

    for witness in &vertex_scan.vertices {
        for &i in &witness.possible_incident {
            for &j in &witness.possible_incident {
                if result[(i, j)] != F64Predicate::True {
                    result[(i, j)] = F64Predicate::Indeterminate;
                }
            }
        }
        for &i in &witness.definite_incident {
            for &j in &witness.definite_incident {
                result[(i, j)] = F64Predicate::True;
            }
        }
    }

    result
}

struct FacetIntersectionCounts {
    true_count: usize,
    false_count: usize,
    indeterminate_count: usize,
}

fn count_facet_intersections(
    facet_intersections: &DMatrix<F64Predicate>,
) -> FacetIntersectionCounts {
    let mut true_count = 0usize;
    let mut false_count = 0usize;
    let mut indeterminate_count = 0usize;
    for predicate in facet_intersections.iter() {
        match predicate {
            F64Predicate::True => true_count += 1,
            F64Predicate::False => false_count += 1,
            F64Predicate::Indeterminate => indeterminate_count += 1,
        }
    }
    FacetIntersectionCounts {
        true_count,
        false_count,
        indeterminate_count,
    }
}

fn omega_signs_f64(
    dual_vertices: &[Vector4<f64>],
    facet_intersections: &DMatrix<F64Predicate>,
) -> (DMatrix<i8>, usize) {
    let mut omega_signs = DMatrix::from_element(dual_vertices.len(), dual_vertices.len(), 0i8);
    let mut omega_indeterminate_count = 0usize;
    for i in 0..dual_vertices.len() {
        for j in 0..dual_vertices.len() {
            let value = omega0(&dual_vertices[i], &dual_vertices[j]);
            if value.abs() <= EPS_OMEGA {
                if i != j && facet_intersections[(i, j)] != F64Predicate::False {
                    omega_indeterminate_count += 1;
                }
                omega_signs[(i, j)] = 0;
            } else {
                omega_signs[(i, j)] = if value > 0.0 { 1 } else { -1 };
            }
        }
    }
    (omega_signs, omega_indeterminate_count)
}

fn intersection_vertex(dual_vertices: &[Vector4<f64>], facets: [usize; 4]) -> Option<Vector4<f64>> {
    let matrix = facet_matrix(dual_vertices, facets);
    if matrix.determinant().abs() <= EPS_DET {
        return None;
    }
    matrix.lu().solve(&Vector4::repeat(1.0))
}

fn is_bounded_near_singular_candidate(dual_vertices: &[Vector4<f64>], facets: [usize; 4]) -> bool {
    let matrix = facet_matrix(dual_vertices, facets);
    let Ok(solution) = matrix.svd(true, true).solve(&Vector4::repeat(1.0), EPS_DET) else {
        return false;
    };
    let residual = (matrix * solution - Vector4::repeat(1.0)).norm();
    residual <= EPS_SINGULAR_RESIDUAL
        && solution
            .iter()
            .all(|coordinate| coordinate.abs() <= MAX_BOUNDED_VERTEX_COORD)
        && dual_vertices
            .iter()
            .all(|normal| normal.dot(&solution) <= 1.0 + EPS_INEQUALITY)
}

fn facet_matrix(dual_vertices: &[Vector4<f64>], facets: [usize; 4]) -> Matrix4<f64> {
    let rows = facets.map(|idx| dual_vertices[idx]);
    Matrix4::new(
        rows[0][0], rows[0][1], rows[0][2], rows[0][3], rows[1][0], rows[1][1], rows[1][2],
        rows[1][3], rows[2][0], rows[2][1], rows[2][2], rows[2][3], rows[3][0], rows[3][1],
        rows[3][2], rows[3][3],
    )
}
