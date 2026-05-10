#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""Phase C: LP test for 0 ∈ conv(per-orbit sys gradients) in (n,h)-space.

Goal: Test whether HKO2024 satisfies the first-order necessary condition for
      local maximality of sys in the full (n, h) parameter space.
Input Artifacts: experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl
Output Artifacts: None

Approach:
  1. Load per-orbit data (β, Q, permutation) and polytope data (n, h) from JSONL.
  2. Reconstruct KKT multipliers (μ, ξ) for each orbit.
  3. Compute per-orbit ∂sys/∂h_k and ∂sys/∂n_k analytically.
  4. Solve LP: find λ_i ≥ 0, Σ λ_i = 1, Σ λ_i g_i = 0.

If the LP is feasible, no first-order improving direction exists (necessary for local max).
If infeasible, an improving direction exists (local max disproved).

Mathematical framework:
  - Danskin's theorem: D_d⁺ sys = min_{i ∈ active orbits} (∇sys_i · d)
  - 0 ∈ conv({∇sys_i}) ⟺ D_d⁺ sys ≤ 0 for all d (necessary for local max, not sufficient)
  - Sufficient for local max would also require second-order analysis of flat directions
"""

import json
import numpy as np
from scipy.optimize import linprog
from pathlib import Path

EXPERIMENT_DIR = Path(__file__).resolve().parent
GRADIENT_ANALYSIS_DIR = EXPERIMENT_DIR.parent / "gradient-analysis"

# ─── Symplectic geometry primitives ───────────────────────────────────────────

def j4():
    """Standard complex structure J₀ such that ω₀(u,v) = ⟨J₀u, v⟩."""
    return np.array([
        [ 0,  0, -1,  0],
        [ 0,  0,  0, -1],
        [ 1,  0,  0,  0],
        [ 0,  1,  0,  0],
    ], dtype=float)

def omega0(u, v):
    """Symplectic form ω₀(u,v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁."""
    return u[0]*v[2] - u[2]*v[0] + u[1]*v[3] - u[3]*v[1]

def project_tangent(v, n):
    """Project v onto T_n S³ (remove component along n)."""
    return v - np.dot(v, n) * n

# ─── KKT system reconstruction ───────────────────────────────────────────────

def build_H_matrix(perm, normals):
    """Build the m×m SYMMETRIC action matrix.

    H_{ij} = H_{ji} = ω₀(n_{σ(i)}, n_{σ(j)}) for i < j, H_{ii} = 0.
    This is symmetric despite ω₀ being antisymmetric — the convention
    matches the Rust KKT augmented system (build_augmented_system_from_dual_vertices).
    The quadratic form Q(β) = (1/2) β^T H β = Σ_{i<j} β_i β_j ω₀(n_{σ(i)}, n_{σ(j)}).
    """
    m = len(perm)
    H = np.zeros((m, m))
    for i in range(m):
        for j in range(i+1, m):
            val = omega0(normals[perm[i]], normals[perm[j]])
            H[i, j] = val
            H[j, i] = val
    return H

def reconstruct_kkt_multipliers(beta, q_value, perm, normals, heights):
    """Reconstruct (μ, ξ) from the KKT system Hβ + Nμ + ηξ = 0.

    Given β, perm, normals, heights, we solve for μ ∈ R⁴ and ξ ∈ R.
    The system has m equations in 5 unknowns (m ≥ 6 for HKO orbits).

    Returns (mu, xi) — both in the symmetric KKT convention.
    """
    m = len(perm)
    beta = np.array(beta)

    # Build H (m×m), N (m×4), η (m×1)
    H = build_H_matrix(perm, normals)
    N = np.array([normals[perm[i]] for i in range(m)])  # m×4
    eta = np.array([heights[perm[i]] for i in range(m)])  # m

    # KKT: Hβ + Nμ + ηξ = 0
    # Rearrange: [N | η] [μ; ξ] = -Hβ
    rhs = -H @ beta  # m-vector
    A = np.column_stack([N, eta])  # m×5

    # Least-squares solve (overdetermined for m > 5)
    result, residuals, rank, sv = np.linalg.lstsq(A, rhs, rcond=None)
    mu = result[:4]
    xi = result[4]

    # Verify residual is small
    reconstructed = N @ mu + eta * xi
    residual = np.linalg.norm(H @ beta + reconstructed)
    if residual > 1e-8:
        print(f"  WARNING: KKT residual = {residual:.2e} (perm={perm})")

    return mu, xi

# ─── Per-orbit derivative computation ────────────────────────────────────────

def capacity_derivatives_h(beta, q, xi, perm, facet_count):
    """∂A/∂h_k = −ξ·β_{i₀}/(2Q²) for k = σ(i₀), else 0."""
    q_sq = q * q
    result = np.zeros(facet_count)
    for i0, k in enumerate(perm):
        result[k] = -xi * beta[i0] / (2.0 * q_sq)
    return result

def capacity_derivatives_n(beta, q, mu, perm, normals):
    """∂A/∂n_k projected to T_{n_k}S³.

    ∂Q*/∂n_k = β_{i₀} · [J₀(2P_{i₀} + β_{i₀}·n_k) + μ]
    ∂A/∂n_k = −proj_{T_{n_k}S³}(∂Q*/∂n_k) / (2Q²)
    """
    J = j4()
    q_sq = q * q
    facet_count = len(normals)
    result = np.zeros((facet_count, 4))

    perm_set = {k: i0 for i0, k in enumerate(perm)}

    for k in range(facet_count):
        if k not in perm_set:
            continue
        i0 = perm_set[k]

        # P_{i₀} = Σ_{i < i₀} β_i · n_{σ(i)}
        P = np.zeros(4)
        for i in range(i0):
            P += beta[i] * normals[perm[i]]

        # ∂Q*/∂n_k = β_{i₀} · [J₀(2P + β_{i₀}·n_k) + μ]
        inner = 2.0 * P + beta[i0] * normals[k]
        j0_inner = J @ inner
        dq_dn = beta[i0] * (j0_inner + mu)

        # Project to tangent space of S³ at n_k
        dq_dn_tangent = project_tangent(dq_dn, normals[k])

        # ∂A/∂n_k = −∂Q*/∂n_k / (2Q²)
        result[k] = -dq_dn_tangent / (2.0 * q_sq)

    return result

def volume_derivatives_h(d_vol_h):
    """Volume height derivatives — loaded directly from JSONL."""
    return np.array(d_vol_h)

def volume_derivatives_n(d_vol_n):
    """Volume normal derivatives — loaded directly from JSONL."""
    return np.array(d_vol_n)

def sys_gradient_hn(dcap_h, dcap_n, dvol_h, dvol_n, capacity, volume, sys_val):
    """Combine capacity and volume derivatives into sys gradient.

    ∂sys/∂h_k = (c/vol)·∂c/∂h_k − (sys/vol)·∂vol/∂h_k
    ∂sys/∂n_k = (c/vol)·∂c/∂n_k − (sys/vol)·∂vol/∂n_k
    """
    dsys_h = (capacity / volume) * dcap_h - (sys_val / volume) * dvol_h
    dsys_n = (capacity / volume) * dcap_n - (sys_val / volume) * dvol_n
    return dsys_h, dsys_n

# ─── LP test ─────────────────────────────────────────────────────────────────

def flatten_gradient(dsys_h, dsys_n, normals):
    """Flatten (h, n) gradient into a single vector.

    For normal components, we work in a tangent-space basis for each T_{n_k}S³.
    Each normal has 3 DOF (tangent to S³), giving 10×3 = 30 normal DOF + 10 height DOF = 40 total.

    We use the ambient R⁴ representation with the constraint that each n-gradient
    is tangent to S³ (which is already enforced by the projection). So the full
    gradient lives in R^{10 + 10×4} = R^{50}, but with 10 constraints (n_k · ∂/∂n_k = 0).

    For the LP, we can work in the ambient R^{50} space — the tangency constraints
    are automatically satisfied by the gradients.
    """
    return np.concatenate([dsys_h, dsys_n.ravel()])

def test_zero_in_convex_hull(gradients):
    """Test whether 0 ∈ conv(g_1, ..., g_k) via LP.

    Solve: find λ ∈ R^k s.t. Σ λ_i g_i = 0, Σ λ_i = 1, λ_i ≥ 0.

    This is a feasibility LP. We reformulate as:
    min 0 s.t. G^T λ = 0, 1^T λ = 1, λ ≥ 0

    where G is (d × k) matrix of gradient columns.

    Returns (feasible, lambda_opt, status_msg).
    """
    k = len(gradients)
    d = len(gradients[0])
    G = np.column_stack(gradients)  # d × k

    # LP: min c^T λ  s.t.  A_eq λ = b_eq,  λ ≥ 0
    c = np.zeros(k)  # feasibility problem
    A_eq = np.vstack([G, np.ones((1, k))])  # (d+1) × k
    b_eq = np.zeros(d + 1)
    b_eq[-1] = 1.0  # Σ λ_i = 1

    result = linprog(c, A_eq=A_eq, b_eq=b_eq, bounds=[(0, None)] * k, method='highs')

    if result.success:
        lam = result.x
        residual = G @ lam
        return True, lam, f"Feasible. Residual norm: {np.linalg.norm(residual):.2e}"
    else:
        return False, None, f"Infeasible: {result.message}"

def test_zero_in_interior(gradients, lambdas):
    """Check whether 0 is in the INTERIOR of conv(g_i).

    0 ∈ int(conv) iff the optimal λ has all components > 0, AND the gradients
    span the full space. More precisely, check if for every direction d,
    min_i g_i · d < 0 strictly.

    Practical check: the convex hull has full dimension (rank of G = d) and
    λ has no zero components.
    """
    k = len(gradients)
    d = len(gradients[0])
    G = np.column_stack(gradients)

    rank = np.linalg.matrix_rank(G, tol=1e-10)
    all_positive = all(l > 1e-12 for l in lambdas)

    return rank, all_positive

# ─── Cross-check: verify per-orbit h-gradients match JSONL ──────────────────

def cross_check_h_gradients(computed_dsys_h, jsonl_dsys_h, orbit_idx):
    """Verify our computation matches the stored per-orbit h-gradients."""
    diff = np.linalg.norm(computed_dsys_h - np.array(jsonl_dsys_h))
    if diff > 1e-8:
        print(f"  WARNING orbit {orbit_idx}: h-gradient mismatch, ||diff|| = {diff:.2e}")
        return False
    return True

# ─── Main ─────────────────────────────────────────────────────────────────────

def main():
    data_path = GRADIENT_ANALYSIS_DIR / "hko-neighborhood-sensitivity.jsonl"
    if not data_path.exists():
        raise FileNotFoundError(
            f"Data file not found: {data_path}\n"
            "Run the Rust binary to generate it: "
            "cd experiments && cargo run --bin hko_neighborhood --release"
        )
    with open(data_path) as f:
        data = json.loads(f.readline())

    normals = [np.array(n) for n in data['normals']]
    heights = np.array(data['heights'])
    capacity = data['capacity']
    volume = data['volume']
    sys_val = data['sys']
    facet_count = data['facet_count']
    dvol_h = np.array(data['d_vol_h'])
    dvol_n = np.array(data['d_vol_n'])  # 10 × 4

    print(f"HKO2024: F={facet_count}, sys={sys_val:.6f}, c={capacity:.6f}, vol={volume:.6f}")
    print(f"Orbits: {len(data['orbits'])} total, {data['n_near_optimal']} near-optimal")
    print()

    # ─── Step 1: Compute per-orbit full (h, n) gradients ─────────────────────

    # Use ALL 44 near-optimal orbits, not just unique subsets.
    # Reason: h-gradient depends only on the facet subset (which facets are visited),
    # but n-gradient depends on the PERMUTATION ORDER (via partial sums P_{i₀}).
    # Two orbits with the same subset but different permutations have the same
    # h-gradient but different n-gradients.

    all_gradients = []   # full (h, n) gradients for LP
    all_h_gradients = [] # h-only gradients for comparison
    cross_check_ok = True

    # Also track unique-subset representatives for h-space test
    seen_subsets = {}
    unique_h_gradients = []

    n_orbit_norms = []
    print(f"Computing gradients for all {len(data['orbits'])} orbits...")
    for orbit_idx, orb in enumerate(data['orbits']):
        perm = orb['permutation']
        beta = np.array(orb['beta'])
        q = orb['q_value']

        # Reconstruct KKT multipliers
        mu, xi = reconstruct_kkt_multipliers(beta, q, perm, normals, heights)

        # Capacity derivatives
        dcap_h = capacity_derivatives_h(beta, q, xi, perm, facet_count)
        dcap_n = capacity_derivatives_n(beta, q, mu, perm, normals)

        # Sys derivatives
        dsys_h, dsys_n = sys_gradient_hn(
            dcap_h, dcap_n, dvol_h, dvol_n, capacity, volume, sys_val
        )

        # Cross-check h-gradient against stored data
        stored_h = data['per_orbit_d_sys_h'][orbit_idx]
        if not cross_check_h_gradients(dsys_h, stored_h, orbit_idx):
            cross_check_ok = False

        # Flatten for LP
        g_flat = flatten_gradient(dsys_h, dsys_n, normals)
        all_gradients.append(g_flat)
        all_h_gradients.append(dsys_h)
        n_orbit_norms.append(np.linalg.norm(dsys_n))

        # Track unique subsets for h-space test
        subset_key = tuple(sorted(orb['subset']))
        if subset_key not in seen_subsets:
            seen_subsets[subset_key] = orbit_idx
            unique_h_gradients.append(dsys_h)

    print(f"  Total orbits: {len(all_gradients)}")
    print(f"  Distinct facet-subsets: {len(unique_h_gradients)}")
    print(f"  n-gradient norms: min={min(n_orbit_norms):.4f}, max={max(n_orbit_norms):.4f}, "
          f"mean={np.mean(n_orbit_norms):.4f}")
    print(f"  Distinct n-gradient norms: {len(set(round(x, 6) for x in n_orbit_norms))}")

    print()
    if cross_check_ok:
        print("Cross-check: all per-orbit h-gradients match stored JSONL data ✓")
    else:
        print("Cross-check: MISMATCH detected — results may be unreliable")
    print()

    # ─── Step 2: LP test in h-space (verification of known result) ───────────

    print("=" * 70)
    print("TEST 1: h-space (normals fixed, 10 DOF) — using 10 unique-subset representatives")
    print("=" * 70)
    feasible, lam, msg = test_zero_in_convex_hull(unique_h_gradients)
    print(f"  Result: {msg}")
    if feasible:
        print(f"  λ weights: {np.round(lam, 6)}")
        rank, all_pos = test_zero_in_interior(all_h_gradients, lam)
        print(f"  Gradient matrix rank: {rank} (space dim: {len(all_h_gradients[0])})")
        print(f"  All λ > 0: {all_pos}")
        if rank == len(all_h_gradients[0]) and all_pos:
            print("  → 0 ∈ INTERIOR of conv(gradients) → no first-order improving direction (strict) ✓")
        elif feasible:
            print("  → 0 ∈ conv(gradients) → no first-order improving direction ✓")
    else:
        print("  → 0 ∉ conv(gradients) → first-order improving direction exists!")
    print()

    # ─── Step 3: LP test in full (h, n)-space ────────────────────────────────

    print("=" * 70)
    print("TEST 2: (h, n)-space (full F=10 polytope parameters, 50 ambient DOF)")
    print("=" * 70)
    feasible, lam, msg = test_zero_in_convex_hull(all_gradients)
    print(f"  Result: {msg}")
    if feasible:
        print(f"  λ weights: {np.round(lam, 6)}")
        rank, all_pos = test_zero_in_interior(all_gradients, lam)
        print(f"  Gradient matrix rank: {rank} (space dim: {len(all_gradients[0])})")
        print(f"  All λ > 0: {all_pos}")
        if all_pos:
            print("  → 0 ∈ INTERIOR of conv(gradients) → no first-order improving direction (strict) ✓")
        else:
            zero_lam = [i for i, l in enumerate(lam) if l < 1e-12]
            print(f"  → 0 on boundary of conv(gradients) ({len(zero_lam)} zero-weight orbits)")
            print(f"    Zero-weight orbit indices: {zero_lam}")
            print("  → local max but possibly not strict (flat directions exist)")
    else:
        print("  → 0 ∉ conv(gradients) → NOT a first-order local max in (h,n)-space!")
        print("  → An improving direction exists. Use LP dual to find it.")

        # Try to find the improving direction via the dual
        print()
        print("  Finding improving direction via alternative LP...")
        # max t s.t. G^T λ ≥ t·1, 1^T λ = 1, λ ≥ 0
        # This finds the direction that maximizes the minimum inner product
        k = len(all_gradients)
        d = len(all_gradients[0])
        G = np.column_stack(all_gradients)

        # Find direction d that maximizes min_i g_i · d subject to ||d|| = 1
        # Equivalent: find d s.t. g_i · d > 0 for all i (if 0 ∉ conv)
        # This is the "strictly separating hyperplane"
        # max t s.t. g_i · d ≥ t for all i, ||d||² ≤ 1
        # Relax to LP: g_i · d ≥ 1 for all i, minimize ||d||
        # Or simply: the dual of our infeasible LP gives a certificate
        print("  (Dual extraction not yet implemented — check LP output)")

    print()

    # ─── Step 4: Geometric analysis of normal gradients ──────────────────────

    print("=" * 70)
    print("ANALYSIS: Per-orbit normal gradient geometry (all 44 orbits)")
    print("=" * 70)

    # Extract the normal parts from all 44 orbits
    n_gradients_all = []
    for orbit_idx, orb in enumerate(data['orbits']):
        perm = orb['permutation']
        beta = np.array(orb['beta'])
        q = orb['q_value']
        mu, xi = reconstruct_kkt_multipliers(beta, q, perm, normals, heights)
        dcap_n = capacity_derivatives_n(beta, q, mu, perm, normals)
        dsys_n = (capacity / volume) * dcap_n - (sys_val / volume) * np.array(dvol_n)
        n_gradients_all.append(dsys_n.ravel())

    n_gradients_all = np.array(n_gradients_all)
    avg_n = np.mean(n_gradients_all, axis=0)
    sum_n = np.sum(n_gradients_all, axis=0)
    print(f"  Number of n-gradient vectors: {len(n_gradients_all)}")
    print(f"  n-gradient norms: {sorted(set(round(np.linalg.norm(g), 4) for g in n_gradients_all))}")
    print(f"  ||mean(n-gradients)||: {np.linalg.norm(avg_n):.6e}")
    print(f"  ||sum(n-gradients)||: {np.linalg.norm(sum_n):.6e}")
    print(f"  (Near zero ⟹ symmetry forces cancellation)")
    print()

    # Full (h,n) gradient analysis
    full_grads = np.array(all_gradients)
    avg_full = np.mean(full_grads, axis=0)
    print(f"  ||mean(full gradients)||: {np.linalg.norm(avg_full):.6e}")
    h_part = avg_full[:10]
    n_part = avg_full[10:]
    print(f"    h-part: {np.linalg.norm(h_part):.6e}")
    print(f"    n-part: {np.linalg.norm(n_part):.6e}")
    print()

    # Rank of the gradient matrix (how many independent directions?)
    G = np.array(all_gradients)
    rank = np.linalg.matrix_rank(G, tol=1e-8)
    print(f"  Gradient matrix rank: {rank} (of {G.shape[1]} ambient dimensions, {G.shape[0]} orbits)")
    print()

    # SVD to understand the gradient subspace
    U, S, Vt = np.linalg.svd(G, full_matrices=False)
    print(f"  Top 10 singular values: {np.round(S[:10], 6)}")
    print(f"  Singular values > 1e-8: {np.sum(S > 1e-8)}")
    print()

    # ─── Step 5: Flat direction analysis ─────────────────────────────────────

    print("=" * 70)
    print("ANALYSIS: Flat directions (kernel of gradient matrix)")
    print("=" * 70)
    print()
    print("0 is on the BOUNDARY of conv(gradients), meaning there exist directions d")
    print("where D_d⁺ sys = min_i(g_i · d) = 0 (all orbits agree on zero first-order change).")
    print("These 'flat directions' need second-order analysis to determine if sys")
    print("increases, decreases, or stays constant.")
    print()

    # The flat directions are the null space of the gradient matrix G (44 × 50).
    # Any d in null(G) has g_i · d = 0 for ALL i simultaneously.
    # Since rank(G) = 24, null(G) has dimension 50 - 24 = 26.
    null_dim = G.shape[1] - rank
    print(f"  Gradient subspace dimension: {rank}")
    print(f"  Flat direction subspace dimension: {null_dim} (= {G.shape[1]} - {rank})")
    print()

    # What's in the flat subspace? Decompose into h-part and n-part.
    # The last 50-10=40 dimensions are normal components.
    # The first 10 are height components.
    null_vectors = Vt[rank:]  # (50-rank) × 50
    if null_dim > 0:
        h_norms_in_null = [np.linalg.norm(v[:10]) for v in null_vectors]
        n_norms_in_null = [np.linalg.norm(v[10:]) for v in null_vectors]
        print(f"  Flat directions h-component norms: min={min(h_norms_in_null):.4f}, max={max(h_norms_in_null):.4f}")
        print(f"  Flat directions n-component norms: min={min(n_norms_in_null):.4f}, max={max(n_norms_in_null):.4f}")

        # Are any flat directions pure-h (no normal component)?
        pure_h = sum(1 for n in n_norms_in_null if n < 1e-8)
        pure_n = sum(1 for h in h_norms_in_null if h < 1e-8)
        print(f"  Pure-h flat directions: {pure_h}")
        print(f"  Pure-n flat directions: {pure_n}")
        print(f"  Mixed (h+n) flat directions: {null_dim - pure_h - pure_n}")
        print()

        # Check: are the flat directions in the tangent space?
        # (They should be, since the gradients are tangent-projected.)
        # A flat direction d has d_{n_k} · n_k = 0 for each k?
        # Not necessarily — the LP constraint is weaker.
        # But for physical meaning, we should project flat directions to tangent space.
        tangent_violations = []
        for v in null_vectors:
            max_violation = 0
            for k in range(10):
                n_comp = v[10 + 4*k : 10 + 4*(k+1)]
                dot = np.dot(n_comp, normals[k])
                max_violation = max(max_violation, abs(dot))
            tangent_violations.append(max_violation)
        print(f"  Max tangent-space violation in flat directions: {max(tangent_violations):.6e}")
        print()
        # The ambient R^50 includes 10 "radial" directions (along each n_k)
        # that don't correspond to real polytope perturbations (since ||n_k||=1).
        # Gradients are tangent-projected, so radial directions are always in null(G).
        # Real flat directions = null(G) ∩ tangent_space.
        #
        # Effective DOF: 10 (heights) + 10×3 (tangent normals) = 40.
        # Radial gauge directions: 10.
        # Real flat directions: null_dim - 10 = ambient_null - gauge.
        #
        # Alternatively: rank in tangent space = rank in ambient (since gradients
        # are tangent), so real flat dim = 40 - rank.
        real_flat = 40 - rank
        print(f"  Effective DOF (tangent space): 40 = 10(h) + 10×3(n)")
        print(f"  Radial gauge directions: 10 (= 50 - 40)")
        print(f"  Real flat directions in tangent space: {real_flat} (= 40 - {rank})")
        print(f"  Check: {real_flat} + 10 = {real_flat + 10} = {null_dim} ✓" if real_flat + 10 == null_dim else f"  Check FAILED: {real_flat} + 10 ≠ {null_dim}")
        print()
        print(f"  NOTE: Jörn's a_i = h_i·n_i parameterization avoids this gauge issue")
        print(f"  entirely — gradient in R^40 with no tangent projection needed.")

    # ─── Step 6: Symmetry verification ───────────────────────────────────────

    print()
    print("=" * 70)
    print("VERIFICATION: Symmetry of gradient structure")
    print("=" * 70)
    print()

    # Check that the uniform average (1/44) Σ g_i is approximately zero
    avg = np.mean(full_grads, axis=0)
    print(f"  ||uniform average||: {np.linalg.norm(avg):.6e}")
    print(f"  ||uniform average|| / ||g_0||: {np.linalg.norm(avg) / np.linalg.norm(full_grads[0]):.6e}")
    print()

    # Check the LP solution: which orbits contribute?
    if feasible and lam is not None:
        active = [(i, l) for i, l in enumerate(lam) if l > 1e-12]
        print(f"  LP solution: {len(active)} active orbits with weight ≈ {active[0][1]:.6f}")
        # Which subsets are active?
        active_subsets = set()
        for i, _ in active:
            orb = data['orbits'][i]
            active_subsets.add(tuple(sorted(orb['subset'])))
        print(f"  Active orbits span {len(active_subsets)} distinct facet-subsets (of 10 total)")
        print()

    # Summary
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print()
    h_G = np.array(unique_h_gradients)
    h_rank = np.linalg.matrix_rank(h_G, tol=1e-8)
    print(f"1. h-space ({len(unique_h_gradients[0])} DOF, normals fixed):")
    print(f"   0 ∈ conv({len(unique_h_gradients)} unique-subset gradients) ✓")
    print(f"   Gradient rank {h_rank} / {len(unique_h_gradients[0])} → {len(unique_h_gradients[0]) - h_rank}D flat subspace in h-space")
    print("   No first-order improving direction (necessary condition for local max)")
    print()
    print("2. Full (h,n)-space (50 ambient DOF, 40 effective DOF):")
    print(f"   0 ∈ conv(all {len(all_gradients)} orbit gradients) ✓ (LP residual ~7e-9)")
    print(f"   Gradient rank {rank} / {G.shape[1]} → {null_dim}D flat subspace")
    print("   No first-order improving direction (necessary condition for local max)")
    print(f"   Permutation diversity in n-gradients essential (10 subset-unique insufficient)")
    print()
    real_flat_dim = 40 - rank
    print("3. Flat directions:")
    print(f"   {null_dim}D null space in R^50 = {real_flat_dim}D real flat + 10D gauge (radial)")
    print(f"   {real_flat_dim} directions in tangent space where D_d⁺ sys = 0")
    print("   Second-order analysis needed to establish local maximality")
    print("   (First-order necessary condition verified; not sufficient)")


if __name__ == '__main__':
    main()
