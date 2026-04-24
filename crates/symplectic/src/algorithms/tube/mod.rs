//! Tube algorithm for EHZ capacity of symplectic polytopes.
//!
//! **Status: blocked, do not use.** The rotation-increment formula is
//! incorrect and the supporting math in `formal/library/algorithms.tex` contains
//! unverified `[TODO: JÖRN]` markers in the tube section
//! (`def:tube-data` through `alg:tube`, around lines 307, 375, 533, 597
//! of `formal/library/algorithms.tex`). `tube_capacity` is not re-exported from
//! `lib.rs`; callers should use `hk2017::ehz_capacity` instead. Tracked in
//! `tasks/numerics.md` under the tube benchmark/formula row and in
//! `tasks/writing.md` for thesis-facing tube TODOs.
//!
//! Implements Algorithm [alg:tube] from the thesis: an iterative search that
//! builds families of Reeb trajectories ("tubes") one facet at a time, pruning
//! branches that cannot contain a minimum-action orbit. This is an alternative
//! to the exhaustive HK2017 enumeration for polytopes whose 2-faces are all
//! symplectic (no Lagrangian 2-faces).
//!
//! The algorithm searches only for Type 1 orbits (breakpoints at 2-face
//! interiors). Type 2 orbits (breakpoints at 1-faces) are not enumerated.
//! By CH2021 Conj. 1.26, generic symplectic polytopes have no Type 2
//! minimum-action orbit, so this is correct for generic polytopes.
//!
//! # Complexity
//!
//! Polynomial to exponential depending on the tube structure and pruning
//! effectiveness. For well-pruned polytopes, significantly faster than HK2017.
//!
//! Mathematical correspondence: [alg:tube], [def:tube], [def:symplectic-polytope]

use crate::geom::polytope::Polytope4D;
use crate::geom::skeleton::Skeleton;
use crate::geom::symplectic_form::omega0;
use nalgebra::{Vector2, Vector4};

// ── Error types ──

/// Error type for the tube algorithm.
///
/// Returned when the input polytope violates the algorithm's preconditions.
#[derive(Debug, Clone)]
pub enum TubeError {
    /// The polytope has a Lagrangian 2-face (omega_0(n_i, n_j) = 0 for adjacent
    /// facets i, j that share a 2-face). The tube algorithm requires all 2-faces
    /// to be symplectic.
    ///
    /// [def:symplectic-polytope]
    HasLagrangian2Face {
        /// First facet index.
        facet_i: usize,
        /// Second facet index.
        facet_j: usize,
    },
    /// Fewer than 5 facets — degenerate for a 4D polytope.
    TooFewFacets(usize),
}

impl std::fmt::Display for TubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TubeError::HasLagrangian2Face { facet_i, facet_j } => write!(
                f,
                "Lagrangian 2-face at facets ({}, {}): omega_0(n_i, n_j) = 0",
                facet_i, facet_j
            ),
            TubeError::TooFewFacets(n) => {
                write!(f, "too few facets ({}, need >= 5)", n)
            }
        }
    }
}

impl std::error::Error for TubeError {}

// ── Result type ──

/// Result of the tube capacity computation.
///
/// Contains the capacity (minimum action over all closed orbits found) and
/// diagnostic information about the search.
///
/// Mathematical correspondence: [alg:tube]
#[derive(Clone, Debug)]
pub struct TubeResult {
    /// The EHZ capacity: minimum action over all closed orbits found.
    /// action = total elapsed time of the closed orbit.
    pub capacity: f64,
    /// Facet sequence of the minimum-action orbit.
    pub best_sequence: Vec<usize>,
    /// Fixed point (starting point) of the minimum-action orbit in R^4.
    #[allow(dead_code)]
    pub fixed_point: Vector4<f64>,
    /// Total number of tubes explored during the search.
    pub tubes_explored: u64,
    /// Total number of tubes pruned during the search.
    pub tubes_pruned: u64,
}

// ── Precomputed data ──

/// Precomputed directed 2-face graph and step maps for the tube algorithm.
///
/// Built once from the polytope and skeleton; reused across the entire DFS.
///
/// [alg:tube] precomputation step.
struct TubePrecomputation {
    /// Directed adjacency: `directed_edges[i]` = list of facet indices j such
    /// that F_i -> F_j is a directed 2-face (omega_0(a_i, a_j) > 0 and F_i, F_j
    /// share a 2-face).
    directed_edges: Vec<Vec<usize>>,
    /// 2-face vertex coordinates (in R^4) for the ridge between facets i and j.
    /// Indexed as `ridge_vertices[(i, j)]` for directed edges i -> j.
    /// Stored as Vec<Vector4<f64>> with vertices in convex polygon order.
    ridge_vertices: Vec<Vec<Vec<Vector4<f64>>>>,
    /// Full Reeb vector R_i = 2 J_0 a_i for each facet i.
    reeb_vectors: Vec<Vector4<f64>>,
    /// Dual vertices a_i.
    dual_vertices: Vec<Vector4<f64>>,
    /// Rotation increment Delta_rho_{j,l} for each directed edge j -> l.
    /// Indexed as rotation_increments[j][index_in_directed_edges[j]].
    rotation_increments: Vec<Vec<f64>>,
}

/// Data for a single tube T(sigma) during the DFS.
///
/// Represents the set of all pure Reeb trajectories sharing the facet
/// sequence sigma. The tube is parameterized by its starting point in the
/// Start polygon.
///
/// [def:tube-data]
#[derive(Clone, Debug)]
struct TubeData {
    /// Facet sequence sigma = (sigma(1), ..., sigma(k)).
    sequence: Vec<usize>,
    /// Start polygon: vertices in local 2D coordinates of F_{sigma(1)} ∩ F_{sigma(2)}.
    start_vertices_4d: Vec<Vector4<f64>>,
    /// End polygon: vertices in R^4 coordinates of F_{sigma(k-1)} ∩ F_{sigma(k)}.
    end_vertices_4d: Vec<Vector4<f64>>,
    /// Affine map phi: Start -> End, stored as (A, b) where phi(x) = A*x + b
    /// in R^4. Composition of step maps.
    phi_matrix: nalgebra::Matrix4<f64>,
    phi_offset: Vector4<f64>,
    /// Action function a: End -> R, affine. Stored as (gradient, constant)
    /// where a(y) = gradient . y + constant.
    action_gradient: Vector4<f64>,
    action_constant: f64,
    /// Accumulated rotation number.
    rotation: f64,
}

// ── Public API ──

/// Compute c_EHZ(K) for a symplectic polytope K using the tube algorithm.
///
/// The polytope must be "symplectic": every 2-face F_{ij} has
/// omega_0(n_i, n_j) != 0. Returns an error if a Lagrangian 2-face exists.
///
/// Returns `Ok(None)` if no closed orbit is found (should not happen for
/// valid symplectic polytopes, but guards against degenerate input).
///
/// Searches only for Type 1 orbits (breakpoints at 2-face interiors).
/// Correct for generic polytopes by CH2021 Conj. 1.26.
///
/// [alg:tube]
pub fn tube_capacity(polytope: &Polytope4D) -> Result<Option<TubeResult>, TubeError> {
    let f = polytope.facet_count();
    if f < 5 {
        return Err(TubeError::TooFewFacets(f));
    }

    let skeleton = Skeleton::compute(polytope);
    let precomp = precompute(polytope, &skeleton)?;

    // DFS search state.
    let mut best_action = f64::INFINITY;
    let mut best_sequence: Option<Vec<usize>> = None;
    let mut best_fixed_point = Vector4::zeros();
    let mut tubes_explored: u64 = 0;
    let mut tubes_pruned: u64 = 0;

    // Initialize: for each directed edge (i, j), create tube T(i, j).
    for i in 0..f {
        for (edge_idx, &j) in precomp.directed_edges[i].iter().enumerate() {
            let ridge_verts = &precomp.ridge_vertices[i][edge_idx];
            if ridge_verts.len() < 3 {
                continue;
            }

            let tube = TubeData {
                sequence: vec![i, j],
                start_vertices_4d: ridge_verts.clone(),
                end_vertices_4d: ridge_verts.clone(),
                phi_matrix: nalgebra::Matrix4::identity(),
                phi_offset: Vector4::zeros(),
                action_gradient: Vector4::zeros(),
                action_constant: 0.0,
                rotation: 0.0,
            };

            tubes_explored += 1;

            dfs_search(
                &precomp,
                &tube,
                &mut best_action,
                &mut best_sequence,
                &mut best_fixed_point,
                &mut tubes_explored,
                &mut tubes_pruned,
            );
        }
    }

    match best_sequence {
        Some(seq) => Ok(Some(TubeResult {
            capacity: best_action,
            best_sequence: seq,
            fixed_point: best_fixed_point,
            tubes_explored,
            tubes_pruned,
        })),
        None => Ok(None),
    }
}

/// Check if a polytope is symplectic (no Lagrangian 2-faces).
///
/// Returns `Ok(())` if the polytope is symplectic, or `Err` with the first
/// Lagrangian 2-face found.
///
/// [def:symplectic-polytope]
pub fn check_symplectic(polytope: &Polytope4D) -> Result<(), TubeError> {
    let skeleton = Skeleton::compute(polytope);
    for ridge in &skeleton.ridges {
        let [fi, fj] = ridge.facets;
        let ai = polytope.dual_vertices_f64()[fi];
        let aj = polytope.dual_vertices_f64()[fj];
        let omega = omega0(&ai, &aj);
        if omega.abs() < 1e-12 {
            return Err(TubeError::HasLagrangian2Face {
                facet_i: fi,
                facet_j: fj,
            });
        }
    }
    Ok(())
}

// ── Precomputation ──

/// Build the precomputed data structures for the tube algorithm.
///
/// [alg:tube] precomputation step: directed 2-face graph, step maps, rotation
/// increments.
fn precompute(polytope: &Polytope4D, skeleton: &Skeleton) -> Result<TubePrecomputation, TubeError> {
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();

    let reeb_vectors: Vec<Vector4<f64>> = duals
        .iter()
        .map(|a| crate::geom::reeb_trajectory::reeb_direction(a) * 2.0)
        .collect();

    // Build directed edge lists and ridge vertices from the skeleton.
    let mut directed_edges: Vec<Vec<usize>> = vec![Vec::new(); f];
    let mut ridge_vertices: Vec<Vec<Vec<Vector4<f64>>>> = vec![Vec::new(); f];
    let mut rotation_increments: Vec<Vec<f64>> = vec![Vec::new(); f];

    let all_vertices_f64 = polytope.vertices_f64();

    for ridge in &skeleton.ridges {
        let [fi, fj] = ridge.facets;
        let ai = &duals[fi];
        let aj = &duals[fj];
        let omega_val = omega0(ai, aj);

        if omega_val.abs() < 1e-12 {
            return Err(TubeError::HasLagrangian2Face {
                facet_i: fi,
                facet_j: fj,
            });
        }

        // Ridge vertices in R^4.
        let verts: Vec<Vector4<f64>> = ridge
            .vertices
            .iter()
            .map(|&vi| all_vertices_f64[vi])
            .collect();

        if omega_val > 0.0 {
            // Directed edge fi -> fj.
            directed_edges[fi].push(fj);
            ridge_vertices[fi].push(verts.clone());
            let delta_rho = compute_rotation_increment(ai, aj);
            rotation_increments[fi].push(delta_rho);
        }
        if omega_val < 0.0 {
            // Directed edge fj -> fi (omega_0(a_j, a_i) = -omega_val > 0).
            directed_edges[fj].push(fi);
            ridge_vertices[fj].push(verts);
            let delta_rho = compute_rotation_increment(aj, ai);
            rotation_increments[fj].push(delta_rho);
        }
    }

    Ok(TubePrecomputation {
        directed_edges,
        ridge_vertices,
        reeb_vectors,
        dual_vertices: duals.to_vec(),
        rotation_increments,
    })
}

/// Fallback rotation increment when Reeb vector norms are near-degenerate (<1e-15).
///
/// Used by `compute_rotation_increment`. Any value in (0, 1/2) is valid by CH2021
/// Cor. 2.22 for symplectic polytopes; 0.25 (midpoint) is a conservative choice
/// that does not bias the rotation bound toward either 0 or 1/2.
///
/// TODO [JÖRN]: The full compute_rotation_increment implementation does not match
/// [def:rotation-increment] — see inline comment. This fallback is correct, but
/// the non-fallback path also needs verification.
const ROTATION_INCREMENT_FALLBACK: f64 = 0.25;

/// Compute the rotation increment Delta_rho at a breakpoint j -> l.
///
/// Uses the transition matrix psi_{F_{jl}} from CH2021 Def. 2.20.
/// The rotation number of psi is in (0, 1/2) for directed 2-faces.
///
/// The transition matrix in the symplectic normal bundle is a 2x2 matrix
/// whose rotation angle gives the rotation increment.
///
/// [def:rotation-increment]
///
/// **Warning:** The current implementation approximates the CH2021 formula by
/// using the angle between Reeb vectors (not the exact transition matrix trace).
/// See TODO [JÖRN] below. The result is clamped to [0.01, 0.49] as a safety bound.
fn compute_rotation_increment(a_j: &Vector4<f64>, a_l: &Vector4<f64>) -> f64 {
    // Reeb vectors: R_i = 2 J_0 a_i.
    let r_j = Vector4::new(-a_j[2], -a_j[3], a_j[0], a_j[1]) * 2.0;
    let r_l = Vector4::new(-a_l[2], -a_l[3], a_l[0], a_l[1]) * 2.0;

    let r_j_norm = r_j.norm();
    let r_l_norm = r_l.norm();

    if r_j_norm < 1e-15 || r_l_norm < 1e-15 {
        return ROTATION_INCREMENT_FALLBACK;
    }
    // TODO [JÖRN]: The formula below (angle between Reeb vectors) is NOT the CH2021
    // transition matrix trace formula. [def:rotation-increment] in formal/library/algorithms.tex
    // defines Delta_rho via psi_{jl}, but the psi_{jl} computation was abandoned
    // (Sherman-Morrison singular). The angle heuristic below is a placeholder that
    // preserves the (0, 1/2) range but may give incorrect pruning bounds.
    // Write [lem:rotation-increment-approx] proving (or disproving) that this
    // angle is a valid bound on the CH2021 rotation number.

    let cos_angle = r_j.dot(&r_l) / (r_j_norm * r_l_norm);
    // The angle between Reeb vectors captures the geometric rotation.
    // Since omega_0(a_j, a_l) > 0 for directed edges, the transition
    // matrix is positive elliptic with rotation in (0, 1/2).
    let clamped = cos_angle.clamp(-1.0, 1.0);
    let theta = clamped.acos(); // theta in [0, pi]
    let rho = theta / (2.0 * std::f64::consts::PI);

    // Ensure the result is in the valid range (0, 1/2).
    // CH2021 Cor. 2.22 guarantees this for symplectic polytopes.
    rho.clamp(0.01, 0.49)
}

// ── DFS Search ──

/// Recursive DFS search over tube extensions.
///
/// At each node: prune, try closing, then extend to all valid successors.
///
/// [alg:tube] search step.
fn dfs_search(
    precomp: &TubePrecomputation,
    tube: &TubeData,
    best_action: &mut f64,
    best_sequence: &mut Option<Vec<usize>>,
    best_fixed_point: &mut Vector4<f64>,
    tubes_explored: &mut u64,
    tubes_pruned: &mut u64,
) {
    let k = tube.sequence.len();

    // ── Prune: empty tube ──
    // [lem:prune-empty]
    if tube.start_vertices_4d.is_empty() || tube.end_vertices_4d.is_empty() {
        *tubes_pruned += 1;
        return;
    }

    // ── Prune: action lower bound ──
    // [lem:prune-action]: if min_{End} a(y) > c*, no orbit in this tube can beat c*.
    if *best_action < f64::INFINITY {
        let min_action = min_action_on_polygon(
            &tube.end_vertices_4d,
            &tube.action_gradient,
            tube.action_constant,
        );
        if min_action > *best_action {
            *tubes_pruned += 1;
            return;
        }
    }

    // ── Prune: rotation upper bound ──
    // [lem:prune-rotation]: if rho >= 2, closed orbit would exceed the rotation bound.
    if tube.rotation >= 2.0 {
        *tubes_pruned += 1;
        return;
    }

    // ── Try closing ──
    // [def:tube-close]: append sigma(1) and sigma(2) to form a closed orbit.
    if k >= 3 {
        let sigma_1 = tube.sequence[0];
        let sigma_2 = tube.sequence[1];
        let sigma_k = tube.sequence[k - 1];

        // Check if closing edges exist: F_{sigma(k)} -> F_{sigma(1)} and
        // F_{sigma(1)} -> F_{sigma(2)}.
        let has_close_edge_1 = precomp.directed_edges[sigma_k].contains(&sigma_1);
        let has_close_edge_2 = precomp.directed_edges[sigma_1].contains(&sigma_2);

        if has_close_edge_1 && has_close_edge_2 {
            try_close_tube(precomp, tube, best_action, best_sequence, best_fixed_point);
        }
    }

    // ── Extend ──
    // [lem:prune-simple]: only extend to facets not already in the sequence.
    let sigma_k = tube.sequence[k - 1];
    let sigma_k_minus_1 = if k >= 2 { tube.sequence[k - 2] } else { return };

    for (edge_idx, &l) in precomp.directed_edges[sigma_k].iter().enumerate() {
        // Simple orbit pruning: no repeated facets.
        if tube.sequence.contains(&l) {
            continue;
        }

        if let Some(extended) = extend_tube(precomp, tube, l, sigma_k_minus_1, sigma_k, edge_idx) {
            *tubes_explored += 1;
            dfs_search(
                precomp,
                &extended,
                best_action,
                best_sequence,
                best_fixed_point,
                tubes_explored,
                tubes_pruned,
            );
        }
    }
}

/// Extend a tube by one facet: T(sigma(1),...,sigma(k)) -> T(sigma(1),...,sigma(k),l).
///
/// Computes the step map Phi_{sigma(k-1), sigma(k), l} and updates all tube data.
///
/// [def:tube-extension]
fn extend_tube(
    precomp: &TubePrecomputation,
    tube: &TubeData,
    l: usize,
    _sigma_k_minus_1: usize,
    sigma_k: usize,
    edge_idx: usize,
) -> Option<TubeData> {
    // Step map Phi_{sigma(k-1), sigma(k), l}: x -> x + t(x) * R_{sigma(k)}
    // where t(x) = (1 - <x, a_l>) / <R_{sigma(k)}, a_l>.
    let a_l = &precomp.dual_vertices[l];
    let r_k = &precomp.reeb_vectors[sigma_k];

    let denom = r_k.dot(a_l);
    if denom.abs() < 1e-12 {
        return None; // Reeb vector parallel to target facet — degenerate.
    }

    // Compute Phi(y) = y + ((h_l - y . n_l) / denom) * R_k for each End vertex.
    let new_end_raw: Vec<Vector4<f64>> = tube
        .end_vertices_4d
        .iter()
        .map(|y| {
            let t = (1.0 - y.dot(a_l)) / denom;
            y + r_k * t
        })
        .collect();

    // New End = Phi(End) ∩ (F_{sigma(k)} ∩ F_l).
    // The target 2-face is the ridge between sigma_k and l.
    let target_ridge_verts = &precomp.ridge_vertices[sigma_k][edge_idx];
    let new_end = clip_polygon_to_convex_hull(&new_end_raw, target_ridge_verts);

    if new_end.is_empty() {
        return None;
    }

    // Update phi' = Phi ∘ phi.
    // Phi(x) = x + ((h_l - x . n_l) / denom) * R_k
    //        = x + (1.0 / denom) * R_k - (1/denom) * (n_l^T x) * R_k
    //        = (I - (1/denom) * R_k * n_l^T) * x + (1.0 / denom) * R_k
    let step_matrix = nalgebra::Matrix4::identity() - (r_k * a_l.transpose()) / denom;
    let step_offset = r_k * (1.0 / denom);

    let new_phi_matrix = step_matrix * tube.phi_matrix;
    let new_phi_offset = step_matrix * tube.phi_offset + step_offset;

    // Update action: a'(y') = a(Phi^{-1}(y')) + Delta_a(Phi^{-1}(y')).
    // Delta_a_{k-1,k,l}(x) = t(x) = (h_l - x . n_l) / denom.
    // So Delta_a gradient (w.r.t. x) = -a_l / denom, constant = 1.0 / denom.
    //
    // a'(y') = a(Phi^{-1}(y')) + t(Phi^{-1}(y'))
    //        = (action_gradient . Phi^{-1}(y') + action_constant) + (h_l - Phi^{-1}(y') . n_l) / denom
    //
    // Since Phi is affine and invertible, and we're computing a' as a function of y' ∈ End',
    // we can express this more directly.
    //
    // Let Phi(x) = step_matrix * x + step_offset, so Phi^{-1}(y') = step_matrix^{-1} * (y' - step_offset).
    // But storing the inverse is expensive. Instead, express a' directly on the new End points.

    // For each vertex of new_end, compute the action value.
    // Then fit an affine function to get (gradient, constant).
    let action_values: Vec<f64> = new_end
        .iter()
        .map(|y_prime| {
            // Invert the step map: x = Phi^{-1}(y').
            // From y' = x + t(x) * R_k, we can recover x iteratively or use the formula.
            // Since t(x) = (h_l - x . n_l) / denom, and y' = x + t * R_k:
            //   x = y' - t * R_k
            //   t = (h_l - (y' - t * R_k) . n_l) / denom
            //   t = (h_l - y' . n_l + t * R_k . n_l) / denom
            //   t * denom = h_l - y' . n_l + t * denom
            //   Hmm, this is 0 = h_l - y' . n_l, which says y' . n_l = h_l.
            // This is correct: y' lies on H_l, so y' . n_l = h_l.
            // So we can recover t from x: t = (h_l - x . n_l) / denom.
            // And x = y' - t * R_k.
            //
            // But we need to find x from y'. Since y' is on H_l: y' . n_l = h_l.
            // From y' = x + t * R_k and t = (h_l - x . n_l) / denom:
            // We need to solve for x. Use the step matrix inverse:
            // y' = step_matrix * x + step_offset
            // x = step_matrix^{-1} * (y' - step_offset)
            //
            // For the action at y': old_action(x) + t(x).
            // old_action(x) = action_gradient . x + action_constant.
            // t(x) = (h_l - x . n_l) / denom.
            //
            // Combined: (action_gradient - a_l / denom) . x + (action_constant + 1.0 / denom).

            // We need x = Phi^{-1}(y'). Use direct formula:
            // step_matrix * x = y' - step_offset.
            // step_matrix = I - (R_k * n_l^T) / denom.
            // This matrix has a known inverse by Sherman-Morrison:
            // (I - u v^T)^{-1} = I + u v^T / (1 - v^T u)   when v^T u != 1.
            // v^T u = n_l^T R_k / denom = denom / denom = 1. So SM doesn't apply directly.
            //
            // Alternative: since y' lies on H_l (y' . n_l = h_l), we can solve directly.
            // x = y' - t * R_k where t satisfies y' = x + t * R_k.
            // From y' . n_l = h_l and x . n_l = h_l - t * denom (since x is on H_{sigma(k)}... wait, x is on End, not on H_l).
            //
            // Let's just compute action values directly for the vertices.
            // For each new_end vertex y', find the pre-image x in the old End.
            // x is the point in old End such that Phi(x) = y'.
            // Since Phi(x) = x + t(x) R_k, and t(x) = (h_l - x.n_l)/denom:

            // For a point y' in the new End that came from Phi(old_end_vertex):
            // We track which old vertex it came from. But after clipping, we may
            // have new vertices. Let's use the affine formula instead.

            // a'(y') in terms of y': since a'(y') = a(x) + t(x) where x = Phi^{-1}(y'),
            // and both a and t are affine in x, a' is affine in x and hence affine in y'
            // (since Phi is affine and invertible).
            //
            // For action_gradient_new . y' + action_constant_new, we can use any 5 non-degenerate
            // points... but actually it's simpler:
            //
            // a(x) + t(x) = (action_gradient + delta_a_gradient) . x + (action_constant + delta_a_constant)
            // where delta_a_gradient = -a_l / denom, delta_a_constant = 1.0 / denom.
            //
            // Let combined_gradient = action_gradient - a_l / denom
            // Let combined_constant = action_constant + 1.0 / denom
            //
            // Then a'(y') = combined_gradient . x + combined_constant
            //             = combined_gradient . (step_matrix^{-1} (y' - step_offset)) + combined_constant
            //
            // We need step_matrix^{-1}.
            // step_matrix = I - (R_k n_l^T)/denom.
            // For points y' on H_l (y'.n_l = h_l), the action on Phi^{-1} can be computed:
            //
            // Phi(x) = x + t(x) R_k, so x = y' - t(x) R_k.
            // n_l . x = n_l . y' - t(x) (n_l . R_k) = h_l - t(x) * denom.
            // Also t(x) = (h_l - n_l.x) / denom, so t(x) = t(x). Consistent but not helpful.
            //
            // From x = y' - t R_k:
            //   n_l . x = h_l - t * denom
            //   t = (h_l - n_l . x) / denom = (h_l - (h_l - t * denom)) / denom = t. (tautology)
            //
            // We need another equation. The old End point x lies on F_{sigma(k-1)} ∩ F_{sigma(k)}.
            // n_{sigma(k)} . x = h_{sigma(k)}.
            // From x = y' - t * R_k:
            //   n_{sigma(k)} . x = n_{sigma(k)} . y' - t * (n_{sigma(k)} . R_k)
            // The Reeb vector R_k = (2/h_k) J_0 n_k, so n_k . R_k = (2/h_k) n_k . (J_0 n_k) = (2/h_k) omega_0(n_k, n_k) = 0.
            // Therefore n_{sigma(k)} . x = n_{sigma(k)} . y'.
            // So x doesn't need t for this constraint.
            //
            // Actually, from R_k . n_k = 0 (Reeb tangent to its own facet):
            //   x = y' - t * R_k implies n_k . x = n_k . y'.
            //
            // We need t. Use the H_l constraint on x:
            //   n_l . x = n_l . y' - t * denom = h_l - t * denom.
            // But x is NOT necessarily on H_l. x is on End (which is on F_{sigma(k-1)} ∩ F_{sigma(k)}).
            // We know y' IS on H_l by construction (it's on the target ridge).
            // So n_l . y' = h_l (approximately).
            // And t = (h_l - n_l . x) / denom.
            //
            // Substitute x = y' - t R_k into t = (h_l - n_l . x) / denom:
            //   t = (h_l - n_l . (y' - t R_k)) / denom
            //   t = (h_l - n_l . y' + t * n_l . R_k) / denom
            //   t * denom = h_l - n_l . y' + t * denom
            //   0 = h_l - n_l . y'
            // So n_l . y' = h_l (which we already know). This means t is underdetermined
            // from just these two equations.
            //
            // The correct approach: store the combined action gradient+constant as a function of y'.
            // Use the relationship: a'(y') = a(x) + t(x) where Phi(x) = y'.
            // Since a and t are both affine in x, their sum is affine in x.
            // Since Phi is an affine bijection, x = Phi^{-1}(y') is affine in y'.
            // Hence a' is affine in y'.
            //
            // For the numerical computation, we can just evaluate at the vertices.
            // We track the old End vertices and their images through Phi.

            // PUNT: for simplicity, compute the action directly from scratch for each vertex.
            // This is O(k) per vertex but correct.
            compute_action_at_point(precomp, &tube.sequence, l, y_prime)
        })
        .collect();

    // Fit affine function: a(y) = g . y + c.
    // Use the first vertex and approximate gradient from finite differences.
    let (new_action_gradient, new_action_constant) = if new_end.len() >= 2 {
        fit_affine_function(&new_end, &action_values)
    } else {
        (
            Vector4::zeros(),
            action_values.first().copied().unwrap_or(0.0),
        )
    };

    // Update Start' = phi'^{-1}(End').
    // phi' maps Start -> new End. Start' is the preimage of new End under phi'.
    // For now, pull back by applying phi'^{-1} to new End vertices.
    // phi'(x) = new_phi_matrix * x + new_phi_offset.
    // x = new_phi_matrix^{-1} * (y' - new_phi_offset).
    let new_start = if let Some(phi_inv) = new_phi_matrix.try_inverse() {
        new_end
            .iter()
            .map(|y| phi_inv * (y - new_phi_offset))
            .collect()
    } else {
        // Degenerate: phi' is singular.
        return None;
    };

    // Rotation increment.
    let delta_rho = get_rotation_increment(precomp, sigma_k, l);
    let new_rotation = tube.rotation + delta_rho;

    let mut new_sequence = tube.sequence.clone();
    new_sequence.push(l);

    Some(TubeData {
        sequence: new_sequence,
        start_vertices_4d: new_start,
        end_vertices_4d: new_end,
        phi_matrix: new_phi_matrix,
        phi_offset: new_phi_offset,
        action_gradient: new_action_gradient,
        action_constant: new_action_constant,
        rotation: new_rotation,
    })
}

/// Try to close a tube and find fixed points.
///
/// Closing appends sigma(1) and sigma(2) to the sequence, producing a flow map
/// phi'' : Start -> F_{sigma(1)} ∩ F_{sigma(2)} (the same 2-face as Start).
/// Fixed points x with phi''(x) = x correspond to closed orbits.
///
/// [def:tube-close], [lem:fixed-point]
fn try_close_tube(
    precomp: &TubePrecomputation,
    tube: &TubeData,
    best_action: &mut f64,
    best_sequence: &mut Option<Vec<usize>>,
    best_fixed_point: &mut Vector4<f64>,
) {
    let k = tube.sequence.len();
    let sigma_1 = tube.sequence[0];
    let sigma_2 = tube.sequence[1];
    let sigma_k = tube.sequence[k - 1];
    let sigma_k_minus_1 = tube.sequence[k - 2];

    // First closing step: extend by sigma(1).
    // Triple: (sigma(k-1), sigma(k), sigma(1)).
    let edge_idx_1 = match precomp.directed_edges[sigma_k]
        .iter()
        .position(|&j| j == sigma_1)
    {
        Some(idx) => idx,
        None => return,
    };

    let extended_1 = match extend_tube(precomp, tube, sigma_1, sigma_k_minus_1, sigma_k, edge_idx_1)
    {
        Some(t) => t,
        None => return,
    };

    // Second closing step: extend by sigma(2).
    // Triple: (sigma(k), sigma(1), sigma(2)).
    let edge_idx_2 = match precomp.directed_edges[sigma_1]
        .iter()
        .position(|&j| j == sigma_2)
    {
        Some(idx) => idx,
        None => return,
    };

    let closed = match extend_tube(precomp, &extended_1, sigma_2, sigma_k, sigma_1, edge_idx_2) {
        Some(t) => t,
        None => return,
    };

    // Now phi'' maps Start'' ⊂ F_{sigma(1)} ∩ F_{sigma(2)} -> F_{sigma(1)} ∩ F_{sigma(2)}.
    // Find fixed points: phi''(x) = x, i.e., (phi_matrix - I) * x + phi_offset = 0.
    let a_matrix = closed.phi_matrix - nalgebra::Matrix4::identity();
    let rhs = -closed.phi_offset;

    // The fixed point equation is a 4x4 linear system. But the point must lie
    // in the 2D affine hull of the 2-face F_{sigma(1)} ∩ F_{sigma(2)}.
    // Solve in R^4 and check containment.
    if let Some(x) = solve_fixed_point(&a_matrix, &rhs, &tube.start_vertices_4d) {
        // Compute action at the fixed point.
        let action = closed.action_gradient.dot(&x) + closed.action_constant;

        if action > 0.0 && action < *best_action {
            *best_action = action;
            *best_sequence = Some(tube.sequence.clone());
            *best_fixed_point = x;
        }
    }
}

/// Solve the fixed-point equation (A - I)x = -b for x in the convex hull of polygon_verts.
///
/// Returns the fixed point if it exists and lies inside the polygon.
///
/// [lem:fixed-point]
fn solve_fixed_point(
    a_minus_i: &nalgebra::Matrix4<f64>,
    rhs: &Vector4<f64>,
    polygon_verts: &[Vector4<f64>],
) -> Option<Vector4<f64>> {
    if polygon_verts.len() < 3 {
        return None;
    }

    // The fixed point lies on the 2D affine hull of the polygon.
    // Set up a 2D parameterization: x = v0 + s * e1 + t * e2.
    let v0 = polygon_verts[0];
    let e1 = polygon_verts[1] - v0;
    let e2 = find_second_basis_vector(polygon_verts, &v0, &e1)?;

    // Substitute into (A-I)(v0 + s*e1 + t*e2) = rhs:
    //   (A-I)*v0 + s*(A-I)*e1 + t*(A-I)*e2 = rhs
    //   s*(A-I)*e1 + t*(A-I)*e2 = rhs - (A-I)*v0
    let b = rhs - a_minus_i * v0;
    let col1 = a_minus_i * e1;
    let col2 = a_minus_i * e2;

    // Solve the overdetermined 4x2 system [col1 | col2] [s; t] = b via least squares.
    let mut lhs = nalgebra::DMatrix::zeros(4, 2);
    for i in 0..4 {
        lhs[(i, 0)] = col1[i];
        lhs[(i, 1)] = col2[i];
    }
    let b_dyn = nalgebra::DVector::from_column_slice(&[b[0], b[1], b[2], b[3]]);

    let svd = lhs.svd(true, true);
    let st = svd.solve(&b_dyn, 1e-10).ok()?;

    let s = st[0];
    let t = st[1];

    let x = v0 + e1 * s + e2 * t;

    // Verify: residual should be small.
    let residual = (a_minus_i * x - rhs).norm();
    if residual > 1e-6 {
        return None;
    }

    // Check that x is inside the polygon (approximately).
    if point_in_polygon_4d(&x, polygon_verts) {
        Some(x)
    } else {
        None
    }
}

/// Find a second basis vector for the 2D affine hull, orthogonal to e1.
fn find_second_basis_vector(
    verts: &[Vector4<f64>],
    v0: &Vector4<f64>,
    e1: &Vector4<f64>,
) -> Option<Vector4<f64>> {
    let e1_norm = e1.norm();
    if e1_norm < 1e-12 {
        return None;
    }
    let e1_hat = e1 / e1_norm;

    for v in verts.iter().skip(2) {
        let d = v - v0;
        let proj = d - e1_hat * d.dot(&e1_hat);
        if proj.norm() > 1e-10 {
            return Some(proj.normalize() * e1_norm); // Scale similar to e1.
        }
    }
    None
}

/// Check if a point x (in R^4) lies inside the convex polygon defined by vertices.
///
/// Projects onto the 2D affine hull and checks containment.
fn point_in_polygon_4d(x: &Vector4<f64>, verts: &[Vector4<f64>]) -> bool {
    if verts.len() < 3 {
        return false;
    }

    let centroid: Vector4<f64> = verts.iter().sum::<Vector4<f64>>() / verts.len() as f64;

    // Build 2D basis.
    let d1_raw = verts[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < 1e-12 {
        return false;
    }
    let d1 = d1_raw / d1_norm;

    let d2 = match verts.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > 1e-10).then(|| proj.normalize())
    }) {
        Some(d) => d,
        None => return false,
    };

    // Project all vertices and x onto 2D.
    let project = |p: &Vector4<f64>| -> Vector2<f64> {
        let rel = p - centroid;
        Vector2::new(rel.dot(&d1), rel.dot(&d2))
    };

    let x_2d = project(x);
    let verts_2d: Vec<Vector2<f64>> = verts.iter().map(project).collect();

    // Winding number test.
    point_in_convex_polygon_2d(&x_2d, &verts_2d)
}

/// Check if a 2D point lies inside a convex polygon (vertices in convex order).
///
/// Uses the cross-product sign test: the point must be on the same side of
/// every edge.
fn point_in_convex_polygon_2d(p: &Vector2<f64>, verts: &[Vector2<f64>]) -> bool {
    let n = verts.len();
    if n < 3 {
        return false;
    }

    let mut sign = None;
    for i in 0..n {
        let a = &verts[i];
        let b = &verts[(i + 1) % n];
        let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
        if cross.abs() < 1e-10 {
            continue; // On the edge — considered inside.
        }
        let s = cross > 0.0;
        match sign {
            None => sign = Some(s),
            Some(prev) => {
                if prev != s {
                    return false;
                }
            }
        }
    }
    true
}

// ── Utility functions ──

/// Compute the minimum action value over the vertices of a polygon.
///
/// Since a is affine and the polygon is convex, the minimum over the polygon
/// is attained at a vertex.
fn min_action_on_polygon(verts: &[Vector4<f64>], gradient: &Vector4<f64>, constant: f64) -> f64 {
    verts
        .iter()
        .map(|v| gradient.dot(v) + constant)
        .fold(f64::INFINITY, f64::min)
}

/// Compute the action for a trajectory at a given endpoint by replaying the
/// step maps from the beginning.
///
/// This is the fallback method when the affine action formula is not easily
/// maintained through polygon clipping.
fn compute_action_at_point(
    precomp: &TubePrecomputation,
    sequence: &[usize],
    new_facet: usize,
    endpoint: &Vector4<f64>,
) -> f64 {
    // Walk backward through the sequence to find the start point,
    // accumulating action along the way.
    //
    // Actually, walk forward: start from a point on F_{sigma(1)} ∩ F_{sigma(2)}
    // and compute total time to reach the endpoint.
    //
    // Since the endpoint is on F_{sigma(k)} ∩ F_{new_facet}, we can invert
    // the step maps to find the start point, then compute total time forward.
    //
    // Simpler approach: compute time for each step from the known endpoint.
    // Work backward from endpoint through the step maps.

    let mut total_time = 0.0;
    let mut current_point = *endpoint;

    // The endpoint is the image after the last step to new_facet.
    // Invert the last step: triple (sigma(k-1), sigma(k), new_facet).
    // Phi(x) = x + t(x) * R_{sigma(k)}.
    // y = x + t * R_k, t = (h_l - x . n_l) / denom.
    // Since y . n_l = h_l (y is on F_l), t = (h_l - x . n_l) / denom.
    // x = y - t * R_k.
    // n_k . x = n_k . y (since n_k . R_k = 0).
    // x . n_l = y . n_l - t * (R_k . n_l) = h_l - t * denom.
    // So t = (h_l - x . n_l) / denom = (h_l - (h_l - t * denom)) / denom = t. (tautology)
    //
    // We need another approach. Use the formula:
    // From n_{sigma(k-1)} . x = h_{sigma(k-1)} (x is on F_{sigma(k-1)} ∩ F_{sigma(k)}):
    // n_{sigma(k-1)} . (y - t * R_k) = h_{sigma(k-1)}
    // t = (n_{sigma(k-1)} . y - h_{sigma(k-1)}) / (n_{sigma(k-1)} . R_k)

    // Invert step by step, accumulating action.
    // Walk backward: from endpoint through each extension step.
    let full_seq: Vec<usize> = sequence
        .iter()
        .copied()
        .chain(std::iter::once(new_facet))
        .collect();

    // For each step i from 2 to k (inclusive), the triple is (seq[i-2], seq[i-1], seq[i]).
    // The step map takes a point on F_{seq[i-2]} ∩ F_{seq[i-1]} to F_{seq[i-1]} ∩ F_{seq[i]}.
    // Walk backward from the last step.

    for step in (2..full_seq.len()).rev() {
        let sigma_prev = full_seq[step - 2];
        let sigma_curr = full_seq[step - 1];

        let r_curr = &precomp.reeb_vectors[sigma_curr];
        let a_prev = &precomp.dual_vertices[sigma_prev];

        // Invert: x = current_point - t * R_curr.
        // x lies on F_{sigma_prev} ∩ F_{sigma_curr}, so a_prev . x = 1.
        // a_prev . (current_point - t * R_curr) = 1.
        // t = (a_prev . current_point - 1) / (a_prev . R_curr).
        let denom = a_prev.dot(r_curr);
        if denom.abs() < 1e-15 {
            // Degenerate: just return the accumulated time.
            return total_time;
        }
        let t = (a_prev.dot(&current_point) - 1.0) / denom;
        total_time += t;
        current_point -= r_curr * t;
    }

    total_time
}

/// Clip a convex polygon (given as vertices in R^4) to the convex hull of
/// another polygon.
///
/// Both polygons lie in a common 2D affine subspace. Uses Sutherland-Hodgman
/// in the projected 2D coordinates.
fn clip_polygon_to_convex_hull(
    subject: &[Vector4<f64>],
    clip: &[Vector4<f64>],
) -> Vec<Vector4<f64>> {
    if subject.is_empty() || clip.len() < 3 {
        return Vec::new();
    }

    // Build a 2D basis from the clip polygon.
    let centroid: Vector4<f64> = clip.iter().sum::<Vector4<f64>>() / clip.len() as f64;
    let d1_raw = clip[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < 1e-12 {
        return Vec::new();
    }
    let d1 = d1_raw / d1_norm;

    let d2 = match clip.iter().skip(1).find_map(|v| {
        let rel = *v - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > 1e-10).then(|| proj.normalize())
    }) {
        Some(d) => d,
        None => return Vec::new(),
    };

    // Project to 2D.
    let project = |p: &Vector4<f64>| -> Vector2<f64> {
        let rel = p - centroid;
        Vector2::new(rel.dot(&d1), rel.dot(&d2))
    };
    let unproject = |p: &Vector2<f64>| -> Vector4<f64> { centroid + d1 * p.x + d2 * p.y };

    let subject_2d: Vec<Vector2<f64>> = subject.iter().map(&project).collect();
    let clip_2d: Vec<Vector2<f64>> = clip.iter().map(project).collect();

    // Sutherland-Hodgman clipping.
    let clipped_2d = sutherland_hodgman(&subject_2d, &clip_2d);

    clipped_2d.iter().map(unproject).collect()
}

/// Sutherland-Hodgman polygon clipping in 2D.
fn sutherland_hodgman(subject: &[Vector2<f64>], clip: &[Vector2<f64>]) -> Vec<Vector2<f64>> {
    let mut output = subject.to_vec();

    let n = clip.len();
    for i in 0..n {
        if output.is_empty() {
            return Vec::new();
        }

        let edge_start = clip[i];
        let edge_end = clip[(i + 1) % n];

        let input = output;
        output = Vec::new();

        let m = input.len();
        for j in 0..m {
            let current = input[j];
            let previous = input[(j + m - 1) % m];

            let curr_inside = is_inside_edge(&current, &edge_start, &edge_end);
            let prev_inside = is_inside_edge(&previous, &edge_start, &edge_end);

            if curr_inside {
                if !prev_inside {
                    if let Some(intersection) =
                        line_intersection_2d(&previous, &current, &edge_start, &edge_end)
                    {
                        output.push(intersection);
                    }
                }
                output.push(current);
            } else if prev_inside {
                if let Some(intersection) =
                    line_intersection_2d(&previous, &current, &edge_start, &edge_end)
                {
                    output.push(intersection);
                }
            }
        }
    }

    output
}

/// Check if point p is on the "inside" (left) side of the directed edge from a to b.
fn is_inside_edge(p: &Vector2<f64>, a: &Vector2<f64>, b: &Vector2<f64>) -> bool {
    let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    cross >= -1e-10 // Allow small tolerance for numerical stability.
}

/// Compute the intersection of two line segments (p1-p2 and p3-p4) in 2D.
fn line_intersection_2d(
    p1: &Vector2<f64>,
    p2: &Vector2<f64>,
    p3: &Vector2<f64>,
    p4: &Vector2<f64>,
) -> Option<Vector2<f64>> {
    let d1 = p2 - p1;
    let d2 = p4 - p3;
    let cross = d1.x * d2.y - d1.y * d2.x;

    if cross.abs() < 1e-15 {
        return None; // Parallel lines.
    }

    let d3 = p3 - p1;
    let t = (d3.x * d2.y - d3.y * d2.x) / cross;

    Some(p1 + d1 * t)
}

/// Fit an affine function f(x) = g . x + c to a set of (point, value) pairs.
///
/// Uses least-squares if more than 5 points.
fn fit_affine_function(points: &[Vector4<f64>], values: &[f64]) -> (Vector4<f64>, f64) {
    if points.is_empty() || values.is_empty() {
        return (Vector4::zeros(), 0.0);
    }

    if points.len() == 1 {
        return (Vector4::zeros(), values[0]);
    }

    // If we have 2+ points, use finite differences from the first point.
    let v0 = &points[0];
    let f0 = values[0];

    // Build an overdetermined system: g . (p_i - v0) = values[i] - f0.
    let n = points.len() - 1;
    let mut a_mat = nalgebra::DMatrix::zeros(n, 4);
    let mut b_vec = nalgebra::DVector::zeros(n);

    for i in 0..n {
        let dp = points[i + 1] - v0;
        for j in 0..4 {
            a_mat[(i, j)] = dp[j];
        }
        b_vec[i] = values[i + 1] - f0;
    }

    let svd = a_mat.svd(true, true);
    if let Ok(g) = svd.solve(&b_vec, 1e-10) {
        let gradient = Vector4::new(g[0], g[1], g[2], g[3]);
        let constant = f0 - gradient.dot(v0);
        (gradient, constant)
    } else {
        (Vector4::zeros(), f0)
    }
}

/// Look up the rotation increment for a directed edge j -> l.
fn get_rotation_increment(precomp: &TubePrecomputation, j: usize, l: usize) -> f64 {
    if let Some(idx) = precomp.directed_edges[j]
        .iter()
        .position(|&target| target == l)
    {
        precomp.rotation_increments[j][idx]
    } else {
        0.25 // Fallback — should not happen for valid directed edges.
    }
}

// Tests for tube algorithm: capacity computation on symplectic polytopes.
//
// Proposition: The tube algorithm correctly computes c_EHZ(K) for symplectic
// polytopes (those with no Lagrangian 2-faces), agreeing with the HK2017
// exhaustive algorithm.
// Reference: [alg:tube], [def:symplectic-polytope]
//
// Strategy: fixture-based comparison against known capacity values and
// cross-validation with hk2017::ehz_capacity. Uses `check_symplectic` to
// determine which known polytopes are eligible for the tube algorithm.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    /// Tolerance for capacity comparison with known values and cross-validation.
    const CAPACITY_TOL: f64 = 1e-4;

    // ── Symplecticity classification ──

    #[test]
    fn check_symplectic_classifies_all_known_polytopes() {
        // Document which known polytopes are symplectic. The tube algorithm
        // only applies to symplectic polytopes (no Lagrangian 2-faces).
        for kp in known_polytopes::all_known() {
            let result = check_symplectic(&kp.polytope);
            // Just verify no panics. The classification is polytope-dependent.
            eprintln!(
                "{}: {}",
                kp.name,
                if result.is_ok() {
                    "symplectic"
                } else {
                    "not symplectic"
                }
            );
        }
    }

    #[test]
    fn check_symplectic_reports_lagrangian_facets() {
        // For non-symplectic polytopes, the error should identify the offending facet pair.
        let kp = known_polytopes::simplex();
        if let Err(TubeError::HasLagrangian2Face { facet_i, facet_j }) =
            check_symplectic(&kp.polytope)
        {
            let f = kp.polytope.facet_count();
            assert!(facet_i < f, "facet_i {} out of range", facet_i);
            assert!(facet_j < f, "facet_j {} out of range", facet_j);
            assert_ne!(facet_i, facet_j, "should be distinct facets");
        }
        // If simplex is symplectic, this test just passes.
    }

    // ── Error conditions ──

    #[test]
    fn tube_error_on_non_symplectic_polytope() {
        // For any polytope that check_symplectic rejects, tube_capacity should
        // also return an error.
        for kp in known_polytopes::all_known() {
            if check_symplectic(&kp.polytope).is_err() {
                let result = tube_capacity(&kp.polytope);
                assert!(
                    result.is_err(),
                    "{}: tube_capacity should return error for non-symplectic polytope",
                    kp.name,
                );
                return; // One example suffices.
            }
        }
        // All known polytopes are symplectic — test is vacuously true.
    }

    // ── Capacity computation on symplectic polytopes ──

    #[test]
    fn tube_capacity_on_symplectic_polytopes() {
        // For every known polytope that passes check_symplectic, verify
        // tube_capacity returns a result (or at least doesn't error).
        for kp in known_polytopes::all_known() {
            if check_symplectic(&kp.polytope).is_err() {
                continue; // Skip non-symplectic.
            }

            let result = tube_capacity(&kp.polytope);
            assert!(
                result.is_ok(),
                "{}: tube_capacity should not error on symplectic polytope",
                kp.name,
            );

            if let Ok(Some(tr)) = result {
                // Capacity should be positive and finite.
                assert!(
                    tr.capacity > 0.0 && tr.capacity.is_finite(),
                    "{}: capacity should be positive and finite, got {}",
                    kp.name,
                    tr.capacity,
                );

                // Sequence should have at least 2 facets.
                assert!(
                    tr.best_sequence.len() >= 2,
                    "{}: orbit must visit at least 2 facets, got {:?}",
                    kp.name,
                    tr.best_sequence,
                );

                // All facet indices should be valid.
                let f = kp.polytope.facet_count();
                for &idx in &tr.best_sequence {
                    assert!(
                        idx < f,
                        "{}: facet index {} out of range (F={})",
                        kp.name,
                        idx,
                        f,
                    );
                }

                // Simple orbit: no repeated facets.
                let mut seen = std::collections::HashSet::new();
                for &idx in &tr.best_sequence {
                    assert!(
                        seen.insert(idx),
                        "{}: repeated facet {} in sequence {:?}",
                        kp.name,
                        idx,
                        tr.best_sequence,
                    );
                }

                // Should have explored at least one tube.
                assert!(
                    tr.tubes_explored > 0,
                    "{}: should explore at least one tube",
                    kp.name,
                );

                eprintln!(
                    "{}: capacity = {:.6} (known = {:.6}), explored {} tubes, pruned {}",
                    kp.name, tr.capacity, kp.capacity, tr.tubes_explored, tr.tubes_pruned
                );
            }
        }
    }

    #[test]
    fn tube_capacity_matches_known_values() {
        // For symplectic polytopes with known capacity values, verify agreement.
        for kp in known_polytopes::all_known() {
            if check_symplectic(&kp.polytope).is_err() {
                continue;
            }

            if let Ok(Some(tr)) = tube_capacity(&kp.polytope) {
                assert!(
                    (tr.capacity - kp.capacity).abs() < CAPACITY_TOL,
                    "{}: tube capacity {:.6} != known {:.6} (diff = {:.2e})",
                    kp.name,
                    tr.capacity,
                    kp.capacity,
                    (tr.capacity - kp.capacity).abs(),
                );
            }
        }
    }

    // ── Cross-validation with HK2017 ──

    #[test]
    #[ignore] // Some polytopes trigger large Q error bounds in hk2017 (see tasks/numerics.md)
    fn tube_agrees_with_hk2017_on_all_symplectic() {
        use crate::ehz_capacity;

        for kp in known_polytopes::all_known() {
            if check_symplectic(&kp.polytope).is_err() {
                continue;
            }

            let hk_result = ehz_capacity(&kp.polytope);
            let tube_result = tube_capacity(&kp.polytope);

            if let (Ok(hk), Ok(Some(tb))) = (hk_result, tube_result) {
                assert!(
                    (tb.capacity - hk.capacity()).abs() < CAPACITY_TOL,
                    "{}: tube {:.6} != hk2017 {:.6}",
                    kp.name,
                    tb.capacity,
                    hk.capacity(),
                );
            }
        }
    }

    // ── Diagnostic tests ──

    #[test]
    fn tube_capacity_returns_none_or_some_consistently() {
        // Run tube_capacity twice on the same polytope — should give same result.
        for kp in known_polytopes::all_known() {
            if check_symplectic(&kp.polytope).is_err() {
                continue;
            }

            let r1 = tube_capacity(&kp.polytope);
            let r2 = tube_capacity(&kp.polytope);

            match (&r1, &r2) {
                (Ok(Some(a)), Ok(Some(b))) => {
                    assert!(
                        (a.capacity - b.capacity).abs() < 1e-10,
                        "{}: inconsistent capacity: {:.6} vs {:.6}",
                        kp.name,
                        a.capacity,
                        b.capacity,
                    );
                }
                (Ok(None), Ok(None)) => {} // Both found nothing — consistent.
                _ => panic!(
                    "{}: inconsistent results: {:?} vs {:?}",
                    kp.name,
                    r1.as_ref().map(|r| r.as_ref().map(|t| t.capacity)),
                    r2.as_ref().map(|r| r.as_ref().map(|t| t.capacity)),
                ),
            }
        }
    }
}
