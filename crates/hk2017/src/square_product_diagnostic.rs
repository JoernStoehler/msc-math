/// Diagnostic test for square-based Lagrangian products.
///
/// Compares ehz_capacity (unpruned), ehz_capacity_pruned, and billiard_capacity
/// on (4,4), (4,5), (4,6), and (3,4) products at various angles.
///
/// Prints raw data for investigation — does NOT assert correctness.
use crate::{ehz_capacity, ehz_capacity_pruned, solve_kkt, build_adjacency_matrix};
use billiard::billiard_capacity;
use geom::lagrangian_product::lagrangian_product;
use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

/// Run all three algorithms on a Lagrangian product and print results.
fn diagnose(n1: usize, n2: usize, angle_deg: f64) {
    let theta = angle_deg.to_radians();
    let (qn, qh) = regular_polygon_2d(n1, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(n2, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);

    let polytope = lagrangian_product(&qn, &qh, &pn, &ph)
        .expect("product construction failed");

    let f = polytope.facet_count();
    let verts = polytope.vertices().len();

    // Adjacency
    let adj = build_adjacency_matrix(&polytope);
    let adj_count = (0..f)
        .flat_map(|i| (i + 1..f).map(move |j| (i, j)))
        .filter(|&(i, j)| adj[i][j])
        .count();

    // Unpruned
    let res_unpruned = ehz_capacity(&polytope);
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
    let res_pruned = ehz_capacity_pruned(&polytope);
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
    let normals = polytope.normals();
    let heights = polytope.heights();
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

        let normals = polytope.normals();
        let heights = polytope.heights();

        match solve_kkt(normals, heights, orbit_perm) {
            Some((beta, q_val)) => {
                let action = 0.5 / q_val;
                let all_pos = beta.iter().all(|&b| b > 1e-12);
                eprintln!(
                    "  θ={angle_deg:>8.4}°: action={action:.10} Q={q_val:.10} β_pos={all_pos} β={:.6?}",
                    beta
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
    let normals = polytope.normals();
    let heights = polytope.heights();
    let f = polytope.facet_count();

    eprintln!("  Facet normals for (3,4) at θ=0°:");
    for i in 0..f {
        let n = &normals[i];
        eprintln!("    facet {i}: n=({:.4},{:.4},{:.4},{:.4}) h={:.4}",
            n[0], n[1], n[2], n[3], heights[i]);
    }

    // Try every cyclic permutation of every 4-facet and 5-facet subset
    use crate::permutations::for_each_cyclic_permutation;
    for m in 4..=5 {
        eprintln!("  Testing all {m}-facet orbits:");
        let mut found_any = false;
        for subset in crate::combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some((beta, q_val)) = solve_kkt(normals, heights, perm) {
                    let action = 0.5 / q_val;
                    let all_pos = beta.iter().all(|&b| b > 1e-12);
                    if all_pos && q_val > 1e-15 {
                        found_any = true;
                        eprintln!("    VALID perm={:?} action={:.6} β={:.6?}", perm, action, beta);
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
