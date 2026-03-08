/// Diagnostic test for square-based Lagrangian products.
///
/// Compares ehz_capacity_unpruned (unpruned), ehz_capacity, and billiard_capacity
/// on (4,4), (4,5), (4,6), and (3,4) products at various angles.
///
/// Prints raw data for investigation — does NOT assert correctness.
use super::{ehz_capacity_unpruned, ehz_capacity, build_adjacency_matrix};
use crate::kkt::solve_kkt;
use crate::algorithms::billiard::billiard_capacity;
use crate::geom::lagrangian_product::lagrangian_product;
use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

/// Run all three algorithms on a Lagrangian product and print results.
fn diagnose(n1: usize, n2: usize, angle_deg: f64) {
    let theta = angle_deg.to_radians();
    let (qn, qh) = regular_polygon_2d(n1, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);

    let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
        .expect("product construction failed");

    let f = polytope.facet_count();
    let verts = polytope.vertices_f64().len();

    // Adjacency
    let adj = build_adjacency_matrix(&polytope);
    let adj_count = (0..f)
        .flat_map(|i| (i + 1..f).map(move |j| (i, j)))
        .filter(|&(i, j)| adj[i][j])
        .count();

    // Unpruned
    let res_unpruned = ehz_capacity_unpruned(&polytope);
    let (cap_u, perm_u, beta_u, iter_u) = match &res_unpruned {
        Some(r) => (
            r.capacity,
            format!("{:?}", r.best_permutation),
            format!("{:.6?}", r.best_beta),
            r.iterations,
        ),
        None => (f64::NAN, "None".into(), "None".into(), 0),
    };

    // Pruned
    let res_pruned = ehz_capacity(&polytope);
    let (cap_p, perm_p, beta_p, iter_p) = match &res_pruned {
        Some(r) => (
            r.capacity,
            format!("{:?}", r.best_permutation),
            format!("{:.6?}", r.best_beta),
            r.iterations,
        ),
        None => (f64::NAN, "None".into(), "None".into(), 0),
    };

    // Billiard
    let res_bil = billiard_capacity(&polytope);
    let (cap_b, perm_b, beta_b, bounces, iter_b) = match &res_bil {
        Ok(Some(r)) => (
            r.capacity,
            format!("{:?}", r.best_permutation),
            format!("{:.6?}", r.best_beta),
            r.bounce_count,
            r.iterations,
        ),
        Ok(None) => (f64::NAN, "None".into(), "None".into(), 0, 0),
        Err(e) => (f64::NAN, format!("Err: {e}"), "".into(), 0, 0),
    };

    // Agreement checks
    let u_p_agree = if cap_u.is_nan() || cap_p.is_nan() {
        "N/A"
    } else if (cap_u - cap_p).abs() / cap_u.max(1e-15) < 1e-8 {
        "YES"
    } else {
        "NO"
    };

    let u_b_agree = if cap_u.is_nan() || cap_b.is_nan() {
        "N/A"
    } else if (cap_u - cap_b).abs() / cap_u.max(1e-15) < 1e-8 {
        "YES"
    } else {
        "NO"
    };

    eprintln!("=== ({},{}) θ={:.4}° ===", n1, n2, angle_deg);
    eprintln!("  F={f}, V={verts}, adj_pairs={adj_count}");
    eprintln!("  cap_unpruned = {cap_u:.10}  (iters={iter_u})");
    eprintln!("  cap_pruned   = {cap_p:.10}  (iters={iter_p})");
    eprintln!("  cap_billiard = {cap_b:.10}  (k={bounces}, iters={iter_b})");
    eprintln!("  unpruned==pruned: {u_p_agree}");
    eprintln!("  unpruned==billiard: {u_b_agree}");
    eprintln!("  perm_unpruned: {perm_u}");
    eprintln!("  perm_pruned:   {perm_p}");
    eprintln!("  perm_billiard: {perm_b}");
    eprintln!("  beta_unpruned: {beta_u}");
    eprintln!("  beta_pruned:   {beta_p}");
    eprintln!("  beta_billiard: {beta_b}");
    eprintln!();
}

/// Print the full adjacency matrix for a product.
fn print_adjacency(n1: usize, n2: usize, angle_deg: f64) {
    let theta = angle_deg.to_radians();
    let (qn, qh) = regular_polygon_2d(n1, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);

    let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
        .expect("product construction failed");

    let f = polytope.facet_count();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let adj = build_adjacency_matrix(&polytope);

    eprintln!("=== Adjacency ({},{}) θ={:.4}° ===", n1, n2, angle_deg);
    for i in 0..f {
        let n = &normals[i];
        eprintln!(
            "  facet {i}: n=({:.6},{:.6},{:.6},{:.6}) h={:.6}",
            n[0], n[1], n[2], n[3], heights[i]
        );
    }
    eprintln!("  Adjacent pairs:");
    #[allow(clippy::needless_range_loop)]
    for i in 0..f {
        for j in (i + 1)..f {
            if adj[i][j] {
                eprintln!("    {i}-{j}");
            }
        }
    }
    eprintln!();
}

/// Manually inject the optimal orbit from θ=0° into the KKT solver at θ=ε.
fn manual_orbit_injection(n1: usize, n2: usize, orbit_perm: &[usize], test_angles: &[f64]) {
    eprintln!("=== Manual orbit injection ({},{}) ===", n1, n2);
    eprintln!("  Orbit permutation: {:?}", orbit_perm);

    for &angle_deg in test_angles {
        let theta = angle_deg.to_radians();
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);

        let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("product construction failed");

        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();

        match solve_kkt(normals, heights, orbit_perm) {
            Some(result) => {
                let action = 0.5 / result.q_corrected;
                let all_pos = result.beta.iter().all(|&b| b > 1e-12);
                eprintln!(
                    "  θ={angle_deg:>8.4}°: action={action:.10} Q={:.10} β_pos={all_pos} β={:.6?}",
                    result.q_corrected, result.beta
                );
            }
            None => {
                eprintln!("  θ={angle_deg:>8.4}°: solve_kkt returned None (singular or bad residual)");
            }
        }
    }
    eprintln!();
}

#[test]
#[ignore] // ~2min debug. Diagnostic: prints HK/billiard comparison, no assertions.
fn square_product_diagnostic() {
    eprintln!();
    eprintln!("======================================");
    eprintln!("  SQUARE PRODUCT DIAGNOSTIC");
    eprintln!("======================================");
    eprintln!();

    // --- (3,4) control ---
    eprintln!("--- (3,4) CONTROL (known cap=1.5) ---");
    for &a in &[0.0, 15.0, 30.0] {
        diagnose(3, 4, a);
    }

    // --- (4,4) near θ=0° ---
    eprintln!("--- (4,4) NEAR θ=0° ---");
    for &a in &[0.0, 0.001, 0.005, 0.01, 0.05, 0.1, 0.125, 0.25, 0.5, 1.0] {
        diagnose(4, 4, a);
    }

    // --- (4,4) mid-range ---
    eprintln!("--- (4,4) MID-RANGE ---");
    for &a in &[5.0, 10.0, 20.0, 30.0] {
        diagnose(4, 4, a);
    }

    // --- (4,4) near θ=45° ---
    eprintln!("--- (4,4) NEAR θ=45° ---");
    for &a in &[40.0, 42.0, 43.0, 43.125, 44.0, 44.5, 44.875, 44.99, 45.0] {
        diagnose(4, 4, a);
    }

    // --- (4,5) ---
    eprintln!("--- (4,5) ---");
    for &a in &[0.0, 1.0, 4.5, 9.0] {
        diagnose(4, 5, a);
    }

    // --- Adjacency comparison ---
    eprintln!("--- ADJACENCY MATRICES ---");
    print_adjacency(4, 4, 0.0);
    print_adjacency(4, 4, 0.125);
    print_adjacency(4, 4, 45.0);

    // --- Manual orbit injection ---
    // Inject the optimal 4-facet orbit from θ=0° ([1,7,3,5]) into perturbed polytopes
    eprintln!("--- MANUAL ORBIT INJECTION (4,4) ---");
    eprintln!("Injecting orbit [1,7,3,5] from θ=0° (cap=2.0)");
    manual_orbit_injection(
        4,
        4,
        &[1, 7, 3, 5],
        &[0.0, 0.001, 0.005, 0.01, 0.05, 0.1, 0.125, 0.5, 1.0, 5.0, 10.0, 20.0, 30.0, 43.0, 45.0],
    );

    // Also inject the 8-facet orbit that HK2017 finds at θ>0°
    eprintln!("--- MANUAL ORBIT INJECTION (4,4) 8-FACET ---");
    eprintln!("Injecting orbit [0,6,3,5,2,4,1,7] from θ>0° (cap≈4.0)");
    manual_orbit_injection(
        4,
        4,
        &[0, 6, 3, 5, 2, 4, 1, 7],
        &[0.0, 0.001, 0.01, 0.1, 1.0, 5.0, 10.0, 20.0, 30.0, 43.0, 45.0],
    );

    // Also try the 4-facet orbit [0,6,2,4] that HK2017 finds near θ=43°
    eprintln!("--- MANUAL ORBIT INJECTION (4,4) 4-FACET [0,6,2,4] ---");
    eprintln!("Injecting orbit [0,6,2,4] from θ=43° (cap≈2.73)");
    manual_orbit_injection(
        4,
        4,
        &[0, 6, 2, 4],
        &[0.0, 0.001, 0.01, 0.1, 1.0, 5.0, 10.0, 20.0, 30.0, 43.0, 45.0],
    );

    eprintln!("======================================");
    eprintln!("  DIAGNOSTIC COMPLETE");
    eprintln!("======================================");
}

#[test]
#[ignore] // ~30s debug. Diagnostic: prints broken case comparison, no assertions.
fn minimal_broken_cases() {
    eprintln!();
    eprintln!("======================================");
    eprintln!("  MINIMAL BROKEN CASES");
    eprintln!("======================================");
    eprintln!();

    // (3,3) at θ=0° — smallest possible product (6 facets)
    eprintln!("--- (3,3) at θ=0° (6 facets) ---");
    diagnose(3, 3, 0.0);
    diagnose(3, 3, 1.0);
    diagnose(3, 3, 10.0);
    diagnose(3, 3, 30.0);

    // (3,4) at θ=0° — known cap=1.5, returns None (7 facets)
    eprintln!("--- (3,4) at θ=0° (7 facets, known cap=1.5) ---");
    diagnose(3, 4, 0.0);

    // Manual orbit injection for (3,4) at θ=0°: what orbits does solve_kkt accept?
    eprintln!("--- (3,4) θ=0°: try ALL 4-facet and 5-facet perms ---");
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let f = polytope.facet_count();

    eprintln!("  Facet normals for (3,4) at θ=0°:");
    for i in 0..f {
        let n = &normals[i];
        eprintln!("    facet {i}: n=({:.4},{:.4},{:.4},{:.4}) h={:.4}",
            n[0], n[1], n[2], n[3], heights[i]);
    }

    // Try every cyclic permutation of every 4-facet and 5-facet subset
    use super::permutations::for_each_cyclic_permutation;
    for m in 4..=5 {
        eprintln!("  Testing all {m}-facet orbits:");
        let mut found_any = false;
        for subset in super::combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(result) = solve_kkt(normals, heights, perm) {
                    let action = 0.5 / result.q_corrected;
                    let all_pos = result.beta.iter().all(|&b| b > 1e-12);
                    if all_pos && result.q_corrected > 1e-15 {
                        found_any = true;
                        eprintln!("    VALID perm={:?} action={:.6} β={:.6?}", perm, action, result.beta);
                    }
                }
            });
        }
        if !found_any {
            eprintln!("    NO valid {m}-facet orbit found!");
        }
    }

    eprintln!();
}

#[test]
#[ignore] // ~2min debug. Diagnostic: prints post-fix comparison, no assertions.
fn post_fix_remaining() {
    eprintln!();
    eprintln!("======================================");
    eprintln!("  POST-FIX REMAINING CASES");
    eprintln!("======================================");

    // (4,4) at exactly θ=45°
    eprintln!("--- (4,4) at θ=45° ---");
    diagnose(4, 4, 45.0);

    // (4,5) — check if values changed
    eprintln!("--- (4,5) post-fix ---");
    for &a in &[0.0, 1.0, 4.5, 9.0] {
        diagnose(4, 5, a);
    }

    // Manual orbit injection: orbit [1,7,3,5] post-fix
    eprintln!("--- MANUAL ORBIT [1,7,3,5] post-fix ---");
    manual_orbit_injection(
        4, 4,
        &[1, 7, 3, 5],
        &[0.0, 0.001, 0.01, 1.0, 5.0, 45.0],
    );

    // Manual orbit injection: 8-facet orbit post-fix
    eprintln!("--- MANUAL ORBIT [0,6,3,5,2,4,1,7] post-fix ---");
    manual_orbit_injection(
        4, 4,
        &[0, 6, 3, 5, 2, 4, 1, 7],
        &[0.0, 0.001, 1.0, 5.0, 45.0],
    );

    eprintln!("======================================");
    eprintln!("  REMAINING CASES COMPLETE");
    eprintln!("======================================");
}

/// Test whether Q(β) is cyclically invariant on a simple non-degenerate case.
#[test]
#[ignore] // ~23s debug. Diagnostic: prints cyclic invariance data, no assertions.
fn cyclic_invariance_check() {
    eprintln!();
    eprintln!("======================================");
    eprintln!("  CYCLIC INVARIANCE CHECK");
    eprintln!("======================================");

    // (3,3) at θ=10° — known to work perfectly, no degeneracy
    let theta = 10.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let result = ehz_capacity_unpruned(&polytope).unwrap();
    let perm = &result.best_permutation;
    let m = perm.len();

    eprintln!("(3,3) θ=10°: best perm={:?} cap={:.10}", perm, result.capacity);
    eprintln!("All rotations:");
    for rot in 0..m {
        let rotated: Vec<usize> = (0..m).map(|i| perm[(i + rot) % m]).collect();
        match solve_kkt(normals, heights, &rotated) {
            Some(result) => {
                let action = 0.5 / result.q_corrected;
                eprintln!("  rot={}: {:?} → action={:.10} Q={:.10} β={:.6?}", rot, rotated, action, result.q_corrected, result.beta);
            }
            None => {
                eprintln!("  rot={}: {:?} → None", rot, rotated);
            }
        }
    }
    eprintln!();

    // (4,4) at θ=20° — non-degenerate, likely non-uniform β
    let theta = 20.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let result = ehz_capacity_unpruned(&polytope).unwrap();
    let perm = &result.best_permutation;
    let m = perm.len();
    eprintln!("(4,4) θ=20°: best perm={:?} cap={:.10}", perm, result.capacity);
    eprintln!("  β={:.6?}", result.best_beta);
    eprintln!("All rotations:");
    for rot in 0..m {
        let rotated: Vec<usize> = (0..m).map(|i| perm[(i + rot) % m]).collect();
        match solve_kkt(normals, heights, &rotated) {
            Some(result) => {
                let action = 0.5 / result.q_corrected;
                eprintln!("  rot={}: {:?} → action={:.10} Q={:.10} β={:.6?}", rot, rotated, action, result.q_corrected, result.beta);
            }
            None => {
                eprintln!("  rot={}: {:?} → None", rot, rotated);
            }
        }
    }
    eprintln!();

    // Also test (3,4) at θ=15° — non-degenerate
    let theta = 15.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let result = ehz_capacity_unpruned(&polytope).unwrap();
    let perm = &result.best_permutation;
    let m = perm.len();

    eprintln!("(3,4) θ=15°: best perm={:?} cap={:.10}", perm, result.capacity);
    eprintln!("All rotations:");
    for rot in 0..m {
        let rotated: Vec<usize> = (0..m).map(|i| perm[(i + rot) % m]).collect();
        match solve_kkt(normals, heights, &rotated) {
            Some(result) => {
                let action = 0.5 / result.q_corrected;
                eprintln!("  rot={}: {:?} → action={:.10} Q={:.10} β={:.6?}", rot, rotated, action, result.q_corrected, result.beta);
            }
            None => {
                eprintln!("  rot={}: {:?} → None", rot, rotated);
            }
        }
    }
    eprintln!();
}

/// Debug SVD internals for a specific rank-deficient permutation.
/// Verifies whether Q is truly constant along the null space.
#[test]
#[ignore] // <1s debug. Diagnostic: prints SVD internals, no assertions.
fn svd_null_space_debug() {
    use nalgebra::{DMatrix, DVector};

    let theta = 43.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    // Billiard's perm that gives the lowest action
    let perm = &[1usize, 0, 6, 3, 2, 4];
    let m = perm.len();
    let size = m + 5;

    eprintln!();
    eprintln!("=== SVD NULL SPACE DEBUG ===");
    eprintln!("Perm: {:?}", perm);

    // Build KKT matrix
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);
    for i in 0..m {
        for j in (i + 1)..m {
            let val = crate::geom::symplectic::omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = -n;
            kkt[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = -h;
        kkt[(m + 4, i)] = h;
    }
    rhs[m + 4] = 1.0;

    // SVD
    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);

    eprintln!("Singular values:");
    for (i, s) in sv.iter().enumerate() {
        eprintln!("  sv[{i}] = {s:.6e}");
    }

    let rank_tol = max_sv * 1e-10;
    let rank = sv.iter().filter(|&&s| s > rank_tol).count();
    eprintln!("Rank: {rank}/{size} (tol={rank_tol:.2e})");

    // Particular solution
    let x0 = svd.solve(&rhs, 1e-10).unwrap();
    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    eprintln!("SVD particular β₀: {:.8?}", beta0);
    let q0 = crate::kkt::q_from_beta(normals, perm, &beta0);
    eprintln!("Q(β₀) = {:.10}", q0);

    // Null space direction
    let v_t = svd.v_t.as_ref().unwrap();
    let null_vec: Vec<f64> = (0..m).map(|j| v_t[(rank, j)]).collect();
    eprintln!("Null space direction (β part): {:.8?}", null_vec);

    // Verify Q is constant along null space
    eprintln!("Q along null space:");
    for &alpha in &[-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0] {
        let beta: Vec<f64> = (0..m).map(|j| beta0[j] + alpha * null_vec[j]).collect();
        let q = crate::kkt::q_from_beta(normals, perm, &beta);
        let all_pos = beta.iter().all(|&b| b > 1e-12);
        eprintln!("  α={alpha:+.1}: Q={q:.10} β_pos={all_pos} β={:.6?}", beta);
    }

    // Now do the same for rot=1
    let perm_rot = &[0usize, 6, 3, 2, 4, 1];
    eprintln!();
    eprintln!("Rotated perm: {:?}", perm_rot);

    let mut kkt2 = DMatrix::zeros(size, size);
    for i in 0..m {
        for j in (i + 1)..m {
            let val = crate::geom::symplectic::omega0(&normals[perm_rot[i]], &normals[perm_rot[j]]);
            kkt2[(i, j)] = val;
            kkt2[(j, i)] = val;
        }
    }
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm_rot[i]][d];
            kkt2[(i, m + d)] = -n;
            kkt2[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm_rot[i]];
        kkt2[(i, m + 4)] = -h;
        kkt2[(m + 4, i)] = h;
    }

    let svd2 = kkt2.clone().svd(true, true);
    let sv2 = &svd2.singular_values;
    eprintln!("Rotated singular values:");
    for (i, s) in sv2.iter().enumerate() {
        eprintln!("  sv[{i}] = {s:.6e}");
    }

    let x0_rot = svd2.solve(&rhs, 1e-10).unwrap();
    let beta0_rot: Vec<f64> = (0..m).map(|i| x0_rot[i]).collect();
    eprintln!("Rotated SVD particular β₀: {:.8?}", beta0_rot);
    let q0_rot = crate::kkt::q_from_beta(normals, perm_rot, &beta0_rot);
    eprintln!("Q(rotated β₀) = {:.10}", q0_rot);

    // Expected: β₀_rot should be cyclic shift of β₀
    let beta0_expected: Vec<f64> = (0..m).map(|i| beta0[(i + 1) % m]).collect();
    eprintln!("Expected (shifted) β₀: {:.8?}", beta0_expected);

    let v_t2 = svd2.v_t.as_ref().unwrap();
    let null_vec2: Vec<f64> = (0..m).map(|j| v_t2[(rank, j)]).collect();
    eprintln!("Rotated null space direction: {:.8?}", null_vec2);
    let null_expected: Vec<f64> = (0..m).map(|i| null_vec[(i + 1) % m]).collect();
    eprintln!("Expected (shifted) null dir: {:.8?}", null_expected);

    // Now call solve_kkt directly and compare
    eprintln!();
    eprintln!("Direct solve_kkt call on perm {:?}:", perm);
    match solve_kkt(normals, heights, perm) {
        Some(result) => {
            let action = 0.5 / result.q_corrected;
            eprintln!("  action={:.10} Q={:.10} β={:.8?}", action, result.q_corrected, result.beta);
            // Check: does this β satisfy the constraints?
            let n_constraint: Vec<f64> = (0..4).map(|d| {
                (0..m).map(|i| result.beta[i] * normals[perm[i]][d]).sum::<f64>()
            }).collect();
            let eta_constraint: f64 = (0..m).map(|i| result.beta[i] * heights[perm[i]]).sum();
            eprintln!("  N^T β = {:?}", n_constraint.iter().map(|x| format!("{:.2e}", x)).collect::<Vec<_>>());
            eprintln!("  η^T β = {:.10}", eta_constraint);
        }
        None => eprintln!("  returned None!"),
    }

    // Also check with near-zero singular value sensitivity
    eprintln!();
    eprintln!("sv[9] = {:.6e} — treating as zero with rank_tol={:.6e}", sv[9], rank_tol);
    eprintln!("If we also zero sv[9] (rank=9), what null space do we get?");
    let rank9 = 9;
    let null_vecs_9: Vec<Vec<f64>> = (rank9..size)
        .map(|i| (0..m).map(|j| v_t[(i, j)]).collect())
        .collect();
    for (i, nv) in null_vecs_9.iter().enumerate() {
        eprintln!("  null_vec[{i}]: {:.8?}", nv);
    }
    // Check if beta0 from rank-9 pseudoinverse has better values
    // Recompute with sv[9] zeroed
    let u = svd.u.as_ref().unwrap();
    let mut x0_rank9 = DVector::zeros(size);
    for i in 0..rank9 {
        let s = sv[i];
        let ui: DVector<f64> = u.column(i).into();
        let vi: DVector<f64> = DVector::from_iterator(size, (0..size).map(|j| v_t[(i, j)]));
        let coeff = ui.dot(&rhs) / s;
        x0_rank9 += coeff * vi;
    }
    let beta0_r9: Vec<f64> = (0..m).map(|i| x0_rank9[i]).collect();
    eprintln!("  β₀ (rank-9 pseudoinverse): {:.8?}", beta0_r9);
    let q_r9 = crate::kkt::q_from_beta(normals, perm, &beta0_r9);
    eprintln!("  Q(β₀ rank-9) = {:.10}", q_r9);
    eprintln!();
}

/// Focused test: compare HK2017 vs billiard at disagreement angles.
/// If billiard finds lower capacity than exhaustive HK2017, that's a bug.
#[test]
#[ignore] // ~105s debug. Diagnostic: prints disagreement analysis, no assertions.
fn disagreement_angles() {
    eprintln!();
    eprintln!("======================================");
    eprintln!("  DISAGREEMENT ANGLE INVESTIGATION");
    eprintln!("======================================");

    // Angles where post-fix report showed HK != billiard
    for &angle in &[40.0_f64, 42.0, 43.0, 44.5] {
        let theta = angle.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let hk = ehz_capacity_unpruned(&polytope);
        let hk_pruned = ehz_capacity(&polytope);
        let bil = billiard_capacity(&polytope);

        let cap_hk = hk.as_ref().map(|r| r.capacity).unwrap_or(f64::NAN);
        let cap_hkp = hk_pruned.as_ref().map(|r| r.capacity).unwrap_or(f64::NAN);
        let cap_bil = bil.as_ref().ok().and_then(|r| r.as_ref().map(|r| r.capacity)).unwrap_or(f64::NAN);

        eprintln!("=== θ={:.1}° ===", angle);
        eprintln!("  HK2017 unpruned: cap={:.10}", cap_hk);
        eprintln!("  HK2017 pruned:   cap={:.10}", cap_hkp);
        eprintln!("  Billiard:        cap={:.10}", cap_bil);

        if let Some(ref r) = hk {
            eprintln!("  HK perm:  {:?}", r.best_permutation);
            eprintln!("  HK beta:  {:.6?}", r.best_beta);
            eprintln!("  HK iters: {}", r.iterations);
        }
        if let Some(ref r) = hk_pruned {
            eprintln!("  HKp perm: {:?}", r.best_permutation);
            eprintln!("  HKp beta: {:.6?}", r.best_beta);
        }
        if let Ok(Some(ref r)) = bil {
            eprintln!("  Bil perm: {:?}", r.best_permutation);
            eprintln!("  Bil beta: {:.6?}", r.best_beta);
            eprintln!("  Bil k:    {}", r.bounce_count);
            eprintln!("  Bil iters:{}", r.iterations);

            // Cross-check: inject billiard's orbit into HK2017's solve_kkt
            let normals = polytope.normals_f64();
            let heights = polytope.heights_f64();
            eprintln!("  Cross-check: HK2017 solve_kkt on billiard's perm {:?}:", r.best_permutation);
            match solve_kkt(normals, heights, &r.best_permutation) {
                Some(result) => {
                    let action = 0.5 / result.q_corrected;
                    eprintln!("    action={:.10} beta={:.6?}", action, result.beta);
                }
                None => eprintln!("    solve_kkt returned None!"),
            }
        }

        // Always show all rotations of HK's best permutation too
        if let Some(ref r) = hk {
            let perm = &r.best_permutation;
            let normals = polytope.normals_f64();
            let heights = polytope.heights_f64();
            let m = perm.len();
            eprintln!("  HK perm rotations:");
            for rot in 0..m {
                let rotated: Vec<usize> = (0..m).map(|i| perm[(i + rot) % m]).collect();
                // Also compute KKT matrix rank for this rotation
                let size = m + 5;
                let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
                for i in 0..m {
                    for j in (i + 1)..m {
                        let val = crate::geom::symplectic::omega0(&normals[rotated[i]], &normals[rotated[j]]);
                        kkt_mat[(i, j)] = val;
                        kkt_mat[(j, i)] = val;
                    }
                }
                for i in 0..m {
                    for d in 0..4 {
                        let n = normals[rotated[i]][d];
                        kkt_mat[(i, m + d)] = -n;
                        kkt_mat[(m + d, i)] = n;
                    }
                }
                for i in 0..m {
                    let h = heights[rotated[i]];
                    kkt_mat[(i, m + 4)] = -h;
                    kkt_mat[(m + 4, i)] = h;
                }
                let svd = kkt_mat.svd(false, false);
                let sv = &svd.singular_values;
                let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);
                let rank = sv.iter().filter(|&&s| s > max_sv * 1e-10).count();
                let min_sv = sv.iter().cloned().fold(f64::INFINITY, f64::min);

                match solve_kkt(normals, heights, &rotated) {
                    Some(result) => {
                        let action = 0.5 / result.q_corrected;
                        eprintln!("    rot={}: {:?} → action={:.10} Q={:.10} rank={}/{} min_sv={:.2e} β={:.6?}",
                            rot, rotated, action, result.q_corrected, rank, size, min_sv, result.beta);
                    }
                    None => {
                        eprintln!("    rot={}: {:?} → None rank={}/{} min_sv={:.2e}",
                            rot, rotated, rank, size, min_sv);
                    }
                }
            }
        }

        // Check: billiard < HK is impossible
        if cap_bil < cap_hk - 1e-8 {
            eprintln!("  *** BUG: billiard ({:.10}) < HK ({:.10}) ***", cap_bil, cap_hk);

            // Test all rotations of billiard's permutation through HK2017's solve_kkt
            if let Ok(Some(ref r)) = bil {
                let perm = &r.best_permutation;
                let normals = polytope.normals_f64();
                let heights = polytope.heights_f64();
                let m = perm.len();
                eprintln!("  All rotations of billiard's perm:");
                for rot in 0..m {
                    let rotated: Vec<usize> = (0..m).map(|i| perm[(i + rot) % m]).collect();
                    match solve_kkt(normals, heights, &rotated) {
                        Some(result) => {
                            let action = 0.5 / result.q_corrected;
                            let min_beta = result.beta.iter().cloned().fold(f64::INFINITY, f64::min);
                            eprintln!("    rot={}: {:?} → action={:.10} min_β={:.2e}",
                                rot, rotated, action, min_beta);
                        }
                        None => {
                            eprintln!("    rot={}: {:?} → None", rot, rotated);
                        }
                    }
                }
            }
        }
        eprintln!();
    }
}

/// Trace solve_kkt step-by-step to find where the contradiction lies.
/// The SVD debug shows β₀ = [10.48, -9.77, ...] with null space β = [0,...],
/// which should make solve_kkt return None. But it returns Some.
#[test]
#[ignore] // <1s debug. Diagnostic: traces solve_kkt code path, no assertions.
fn solve_kkt_trace() {
    use nalgebra::{DMatrix, DVector};

    let theta = 43.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    let perm: &[usize] = &[1, 0, 6, 3, 2, 4];
    let m = perm.len();
    let size = m + 5; // 11

    eprintln!();
    eprintln!("=== SOLVE_KKT TRACE for perm {:?} ===", perm);

    // 1. Build KKT matrix (exact replica of solve_kkt)
    let mut kkt = DMatrix::zeros(size, size);
    let mut rhs = DVector::zeros(size);

    for i in 0..m {
        for j in (i + 1)..m {
            let val = crate::geom::symplectic::omega0(&normals[perm[i]], &normals[perm[j]]);
            kkt[(i, j)] = val;
            kkt[(j, i)] = val;
        }
    }
    for i in 0..m {
        for d in 0..4 {
            let n = normals[perm[i]][d];
            kkt[(i, m + d)] = -n;
            kkt[(m + d, i)] = n;
        }
    }
    for i in 0..m {
        let h = heights[perm[i]];
        kkt[(i, m + 4)] = -h;
        kkt[(m + 4, i)] = h;
    }
    rhs[m + 4] = 1.0;

    // 2. SVD
    let svd = kkt.clone().svd(true, true);
    let sv = &svd.singular_values;
    let max_sv = sv.iter().cloned().fold(0.0f64, f64::max);

    eprintln!("Step 2: SVD computed. max_sv={:.6e}", max_sv);
    eprintln!("  max_sv < EPS_EIGEN_FLOOR(1e-12)? {}", max_sv < 1e-12);

    let rank_tol = max_sv * 1e-10;
    let rank = sv.iter().filter(|&&s| s > rank_tol).count();
    eprintln!("Step 3: rank_tol={:.6e}, rank={}/{}", rank_tol, rank, size);

    // 3. Solve
    let x0 = svd.solve(&rhs, 1e-10).unwrap();
    let residual = (&kkt * &x0 - &rhs).norm();
    eprintln!("Step 4: svd.solve residual = {:.6e} (threshold: 1e-6)", residual);
    eprintln!("  residual > EPS_KKT_RESIDUAL? {}", residual > 1e-6);

    let beta0: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    eprintln!("Step 5: beta0 = {:.8?}", beta0);
    eprintln!("  all beta0 > EPS_BETA_POSITIVE? {}", beta0.iter().all(|&b| b > 1e-12));

    // 4. Full rank check
    eprintln!("Step 6: rank ({}) == size ({})? {}", rank, size, rank == size);

    // 5. Null space
    let v_t = svd.v_t.as_ref().unwrap();
    let null_beta: Vec<Vec<f64>> = (rank..size)
        .map(|i| (0..m).map(|j| v_t[(i, j)]).collect())
        .collect();
    eprintln!("Step 7: null space dimension = {}", null_beta.len());
    for (i, nv) in null_beta.iter().enumerate() {
        eprintln!("  null_beta[{}] = {:.10?}", i, nv);
        eprintln!("  max |null_beta[{}]| = {:.2e}", i, nv.iter().map(|x| x.abs()).fold(0.0f64, f64::max));
    }

    // 6. Simulate find_positive_beta_1d
    if null_beta.len() == 1 {
        let v = &null_beta[0];
        eprintln!("Step 8: find_positive_beta_1d trace:");
        let mut lo = f64::NEG_INFINITY;
        let mut hi = f64::INFINITY;
        for j in 0..m {
            if v[j].abs() < 1e-15 {
                eprintln!("  j={}: v[j]={:.2e} < 1e-15, beta0[j]={:.2e}, ok={}",
                    j, v[j], beta0[j], beta0[j] > 1e-12);
                if beta0[j] <= 1e-12 {
                    eprintln!("  → WOULD RETURN NONE at j={}", j);
                }
            } else {
                let bound = -beta0[j] / v[j];
                if v[j] > 0.0 { lo = lo.max(bound); }
                else { hi = hi.min(bound); }
                eprintln!("  j={}: v[j]={:.6e}, bound={:.6e}, lo={:.6e}, hi={:.6e}",
                    j, v[j], bound, lo, hi);
            }
        }
        eprintln!("  Final: lo={:.6e}, hi={:.6e}", lo, hi);
    }

    // 7. What does solve_kkt ACTUALLY return?
    eprintln!();
    eprintln!("Step 9: Actual solve_kkt result:");
    match solve_kkt(normals, heights, perm) {
        Some(result) => {
            let action = 0.5 / result.q_corrected;
            eprintln!("  RETURNED Some: action={:.10}, Q={:.10}", action, result.q_corrected);
            eprintln!("  beta={:.8?}", result.beta);
            // Verify: does this beta come from the SVD path?
            let q_check = crate::kkt::q_from_beta(normals, perm, &result.beta);
            eprintln!("  Q(beta) = {:.10} (matches? {})", q_check, (result.q_corrected - q_check).abs() < 1e-12);
            // Is this beta equal to beta0?
            let diff: f64 = result.beta.iter().zip(beta0.iter()).map(|(a, b)| (a - b).abs()).sum();
            eprintln!("  ||beta - beta0|| = {:.6e}", diff);
        }
        None => {
            eprintln!("  RETURNED None");
        }
    }

    // 8. Check nalgebra SVD solve semantics
    // Maybe svd.solve interprets eps differently?
    // Let's manually compute the pseudoinverse solution
    let u = svd.u.as_ref().unwrap();
    eprintln!();
    eprintln!("Step 10: Manual pseudoinverse check");
    let mut x_manual = DVector::zeros(size);
    for i in 0..size {
        let s = sv[i];
        if s > rank_tol {
            let ui: DVector<f64> = u.column(i).into();
            let vi: DVector<f64> = DVector::from_iterator(size, (0..size).map(|j| v_t[(i, j)]));
            let coeff = ui.dot(&rhs) / s;
            x_manual += coeff * &vi;
            if i >= 8 {
                eprintln!("  i={}: s={:.6e}, coeff={:.6e}, |vi_beta|_max={:.6e}",
                    i, s, coeff, (0..m).map(|j| v_t[(i,j)].abs()).fold(0.0f64, f64::max));
            }
        }
    }
    let beta_manual: Vec<f64> = (0..m).map(|i| x_manual[i]).collect();
    eprintln!("  Manual pseudoinverse beta: {:.8?}", beta_manual);

    // Compare manual vs svd.solve
    let diff_solve: f64 = (0..size).map(|i| (x0[i] - x_manual[i]).abs()).sum();
    eprintln!("  ||x0 - x_manual|| = {:.6e}", diff_solve);

    eprintln!();
}
