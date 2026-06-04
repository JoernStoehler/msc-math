#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scipy"]
# ///

"""
Goal: Analyze second-order behavior along flat directions at HKO2024.
      Part A: Phase C LP replacement — verify 0 ∈ conv(gradients) in a_i space.
      Part B: Curvature analysis — fit second-order behavior, produce figures.
Input Artifacts: second-order-base.jsonl, second-order-curves.jsonl
Output Artifacts: second_order_curves.png, second_order_curvatures.png,
        second_order_curvatures.tex, second_order_random_hist.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import numpy as np
from scipy.optimize import linprog

def experiments_dir() -> Path:
    for parent in Path(__file__).resolve().parents:
        if (parent / "figure_config.py").exists():
            return parent
    raise RuntimeError("could not find experiments/figure_config.py")


sys.path.insert(0, str(experiments_dir()))
from figure_config import setup, FIGSIZE_SINGLE, MARKER_SIZE, LINE_WIDTH

import matplotlib.pyplot as plt

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent

# ─── Load data ───────────────────────────────────────────────────────────────

def load_base():
    path = EXPERIMENT_DIR / "second-order-base.jsonl"
    with open(path) as f:
        return json.loads(f.readline())

def load_curves():
    path = EXPERIMENT_DIR / "second-order-curves.jsonl"
    rows = []
    with open(path) as f:
        for line in f:
            rows.append(json.loads(line))
    return rows

# ─── Part A: Phase C LP in a_i space ────────────────────────────────────────

def phase_c_lp(base):
    """Test 0 ∈ conv(per-orbit sys gradients) in R^40 via LP."""
    grad_matrix = base["gradient_matrix"]  # list of lists of [f64; 4]
    n_orbits = len(grad_matrix)

    # Flatten each orbit's gradient to R^40
    gradients = []
    for orbit_grad in grad_matrix:
        flat = []
        for facet_grad in orbit_grad:
            flat.extend(facet_grad)
        gradients.append(flat)
    gradients = np.array(gradients)  # (n_orbits, 40)
    d = gradients.shape[1]

    print("=" * 70)
    print("PHASE C LP: 0 ∈ conv(per-orbit ∇sys) in a_i space (R^40)")
    print("=" * 70)
    print(f"  Orbits: {n_orbits}, Dimension: {d}")

    # LP: find λ ≥ 0, Σ λ_i = 1, Σ λ_i g_i = 0
    G = gradients.T  # (40, n_orbits)
    c = np.zeros(n_orbits)
    A_eq = np.vstack([G, np.ones((1, n_orbits))])
    b_eq = np.zeros(d + 1)
    b_eq[-1] = 1.0

    result = linprog(c, A_eq=A_eq, b_eq=b_eq, bounds=[(0, None)] * n_orbits, method='highs')

    if result.success:
        lam = result.x
        residual = G @ lam
        res_norm = np.linalg.norm(residual)
        n_active = np.sum(lam > 1e-12)
        print(f"  Result: FEASIBLE (residual norm = {res_norm:.2e})")
        print(f"  Active orbits (λ > 1e-12): {n_active}")
        all_positive = all(l > 1e-12 for l in lam)
        if all_positive:
            print(f"  0 ∈ INTERIOR of conv(gradients) → strict first-order local max")
        else:
            print(f"  0 on BOUNDARY of conv(gradients) → flat directions exist")

            # Verify rank condition for lem:cone-equals-kernel: rank(G_{A+}) = rank(G)
            # Use relative threshold (1e-8 × σ_max) matching the Rust SVD convention,
            # not an absolute threshold. σ[25] ≈ 1.6e-8 is numerical noise (1.7e-9 relative).
            active_mask = lam > 1e-12
            G_active = gradients[active_mask]
            sv_all = np.linalg.svd(gradients, compute_uv=False)
            rel_tol = 1e-8 * sv_all[0]
            rank_active = int(np.sum(np.linalg.svd(G_active, compute_uv=False) > rel_tol))
            rank_all = int(np.sum(sv_all > rel_tol))
            print(f"\n  Rank condition (lem:cone-equals-kernel):")
            print(f"    rank(G_all {n_orbits} orbits) = {rank_all}")
            print(f"    rank(G_active {int(active_mask.sum())} orbits) = {rank_active}")
            if rank_active == rank_all:
                print(f"    rank(G_active) = rank(G_all) → C = ker(G): flat directions form a subspace")
            else:
                print(f"    WARNING: rank(G_active) < rank(G_all) → C ⊋ ker(G)")
    else:
        print(f"  Result: INFEASIBLE — {result.message}")
        print(f"  0 ∉ conv(gradients) → improving direction exists!")

    # SVD cross-check
    rank = base["rank"]
    n_flat = base["n_flat_directions"]
    print(f"\n  SVD rank: {rank} / {d}")
    print(f"  Flat directions: {n_flat}")
    print(f"  Singular values (top 5): {[f'{s:.4e}' for s in base['singular_values'][:5]]}")
    sv = base['singular_values']
    if rank < len(sv):
        print(f"  Singular values near boundary: σ[{rank-1}]={sv[rank-1]:.4e}, σ[{rank}]={sv[rank]:.4e}")
    print()
    return result.success

# ─── Part B: Curvature analysis ─────────────────────────────────────────────

def analyze_curvatures(base, curves):
    """Compute symmetric curvature ratios and produce figures."""
    sys_base = base["sys_base"]
    n_flat = base["n_flat_directions"]

    # Group by direction
    by_dir = defaultdict(list)
    for c in curves:
        by_dir[c["direction_index"]].append(c)

    print("=" * 70)
    print("CURVATURE ANALYSIS: Second-order behavior along flat directions")
    print("=" * 70)
    print(f"  sys(HKO2024) = {sys_base:.10f}")
    print(f"  Flat directions: {n_flat}")
    print()

    # For each direction, compute curvature ratio r(ε) = (sys(+ε)+sys(-ε)-2*sys(0))/ε²
    dir_curvatures = {}
    dir_curvature_arrays = {}

    for d_idx in sorted(by_dir.keys()):
        rows = by_dir[d_idx]
        # Build eps → sys map
        eps_sys = {}
        for r in rows:
            eps_sys[r["epsilon"]] = r["sys"]

        # Compute curvature ratios for each |ε| that has both +ε and -ε
        eps_abs_vals = sorted(set(abs(r["epsilon"]) for r in rows))
        curv_eps = []
        curv_vals = []
        for ea in eps_abs_vals:
            if ea in eps_sys and -ea in eps_sys:
                r = (eps_sys[ea] + eps_sys[-ea] - 2 * sys_base) / (ea * ea)
                curv_eps.append(ea)
                curv_vals.append(r)

        dir_curvature_arrays[d_idx] = (np.array(curv_eps), np.array(curv_vals))

        # Robust curvature: median over fine+medium range (ε ≤ 5e-3)
        fine_medium = [(e, v) for e, v in zip(curv_eps, curv_vals) if e <= 5e-3]
        if fine_medium:
            median_curv = np.median([v for _, v in fine_medium])
        else:
            median_curv = np.nan
        dir_curvatures[d_idx] = median_curv

    # Print summary
    print(f"  {'Dir':>4}  {'Curvature':>12}  {'Sign':>5}  {'Consistent':>11}")
    print(f"  {'---':>4}  {'--------':>12}  {'----':>5}  {'----------':>11}")
    all_negative = True
    for d_idx in sorted(dir_curvatures.keys()):
        curv = dir_curvatures[d_idx]
        sign = "−" if curv < 0 else "+" if curv > 0 else "0"
        if curv >= 0:
            all_negative = False
        # Check consistency: is curvature ratio approximately constant across ε?
        eps_arr, vals_arr = dir_curvature_arrays[d_idx]
        fine_mask = eps_arr <= 5e-3
        if fine_mask.sum() > 1:
            cv = np.std(vals_arr[fine_mask]) / abs(np.mean(vals_arr[fine_mask])) if abs(np.mean(vals_arr[fine_mask])) > 1e-15 else float('inf')
            consistent = "yes" if cv < 0.5 else f"CV={cv:.2f}"
        else:
            consistent = "n/a"
        print(f"  {d_idx:4d}  {curv:+12.4e}  {sign:>5}  {consistent:>11}")

    print()
    if all_negative:
        print("  *** ALL curvatures negative → numerical evidence supports local maximality ***")
    else:
        print("  WARNING: Some curvatures are non-negative!")
    print()

    return dir_curvatures, dir_curvature_arrays

# ─── Figures ─────────────────────────────────────────────────────────────────

def plot_curves(base, curves, dir_curvatures):
    """Per-direction sys(ε) - sys(0) curves overlaid."""
    sys_base = base["sys_base"]
    by_dir = defaultdict(list)
    for c in curves:
        by_dir[c["direction_index"]].append(c)

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    cmap = plt.cm.viridis
    n_dirs = len(by_dir)
    colors = [cmap(i / max(n_dirs - 1, 1)) for i in range(n_dirs)]

    for d_idx in sorted(by_dir.keys()):
        rows = sorted(by_dir[d_idx], key=lambda r: r["epsilon"])
        eps = [r["epsilon"] for r in rows]
        delta = [r["sys"] - sys_base for r in rows]
        ax.plot(eps, delta, 'o-', color=colors[d_idx], markersize=MARKER_SIZE, linewidth=LINE_WIDTH,
                label=f"$d_{{{d_idx}}}$" if d_idx % 3 == 0 else None)

    ax.axhline(0, color='k', linewidth=0.5, linestyle='--')
    ax.axvline(0, color='k', linewidth=0.5, linestyle='--')
    ax.set_xlabel(r"$\varepsilon$")
    ax.set_ylabel(r"$\mathrm{sys}(K + \varepsilon d) - \mathrm{sys}(K)$")
    ax.legend(loc="lower center", ncol=3)

    fig.savefig(EXPERIMENT_DIR / "second_order_curves.png")
    plt.close(fig)
    print(f"  Wrote second_order_curves.png")

def plot_curvatures(dir_curvatures):
    """Bar chart of curvatures across directions."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    indices = sorted(dir_curvatures.keys())
    curvs = [dir_curvatures[i] for i in indices]
    colors = ['#2ca02c' if c < 0 else '#d62728' if c > 0 else '#ff7f0e' for c in curvs]

    ax.bar(indices, curvs, color=colors, edgecolor='k', linewidth=0.5)
    ax.axhline(0, color='k', linewidth=0.5)
    ax.set_xlabel("Flat direction index")
    ax.set_ylabel(r"Second-order curvature $r(\varepsilon)$")
    ax.set_xticks(indices)

    fig.savefig(EXPERIMENT_DIR / "second_order_curvatures.png")
    plt.close(fig)
    print(f"  Wrote second_order_curvatures.png")

def write_curvature_table(dir_curvatures, dir_curvature_arrays):
    """Write LaTeX table of curvature results."""
    path = EXPERIMENT_DIR / "second_order_curvatures.tex"
    with open(path, "w") as f:
        f.write("\\begin{tabular}{rrl}\n")
        f.write("\\toprule\n")
        f.write("Direction & Curvature & Sign \\\\\n")
        f.write("\\midrule\n")
        for d_idx in sorted(dir_curvatures.keys()):
            curv = dir_curvatures[d_idx]
            sign = "$-$" if curv < 0 else "$+$" if curv > 0 else "$0$"
            f.write(f"{d_idx} & ${curv:+.4e}$ & {sign} \\\\\n")
        f.write("\\bottomrule\n")
        f.write("\\end{tabular}\n")
    print(f"  Wrote second_order_curvatures.tex")

# ─── Part C: Random direction check + symmetry decomposition ────────────────

def load_random():
    path = EXPERIMENT_DIR / "second-order-random.jsonl"
    if not path.exists():
        return None
    rows = []
    with open(path) as f:
        for line in f:
            rows.append(json.loads(line))
    return rows

def analyze_random(random_rows):
    """Analyze curvatures of random directions in the flat subspace."""
    print("=" * 70)
    print("NEGATIVE DEFINITENESS: Random directions in flat subspace")
    print("=" * 70)

    curvatures = [r["curvature"] for r in random_rows]
    n_total = len(curvatures)
    n_neg = sum(1 for c in curvatures if c < -1e-6)
    n_pos = sum(1 for c in curvatures if c > 1e-6)
    n_amb = n_total - n_neg - n_pos
    worst = max(curvatures)

    print(f"  Sampled {n_total} random unit directions in 15D flat subspace")
    print(f"  Negative: {n_neg}, Ambiguous: {n_amb}, Positive: {n_pos}")
    print(f"  Worst (most positive) curvature: {worst:+.4e}")
    print(f"  Mean curvature: {np.mean(curvatures):+.4e}")
    print(f"  Std curvature: {np.std(curvatures):.4e}")
    print()

    if n_pos == 0:
        print("  → No positive curvature among random samples.")
        print("    Combined with 15 basis directions: strong evidence for negative definiteness.")
    else:
        print(f"  → WARNING: {n_pos} directions with positive curvature!")
    print()

    return curvatures

def symmetry_decomposition(base):
    """Decompose the flat subspace under the HKO2024 symplectic symmetry group C₅ × Z₂."""
    print("=" * 70)
    print("SYMMETRY: Decomposition of flat subspace under G_symp = C₅ × Z₂")
    print("=" * 70)

    duals = np.array(base["dual_vertices"])  # (10, 4)
    flat_dirs = np.array([np.array(d).flatten() for d in base["flat_directions"]])  # (15, 40)
    n_flat = flat_dirs.shape[0]

    # Build group generators in R^40
    theta = 2 * np.pi / 5
    co, si = np.cos(theta), np.sin(theta)
    R72 = np.array([[co, -si], [si, co]])
    Delta72 = np.block([[R72, np.zeros((2,2))], [np.zeros((2,2)), R72]])
    Phi = np.array([[0,0,0,-1],[0,0,1,0],[0,1,0,0],[-1,0,0,0]], dtype=float)

    perm_delta = [1,2,3,4,0,6,7,8,9,5]
    perm_phi = [5,6,7,8,9,0,1,2,3,4]

    def build_rep40(M4, perm):
        inv_perm = [0]*10
        for i, p in enumerate(perm): inv_perm[p] = i
        rep = np.zeros((40, 40))
        for k in range(10):
            rep[4*k:4*k+4, 4*inv_perm[k]:4*inv_perm[k]+4] = M4
        return rep

    rep_delta = build_rep40(Delta72, perm_delta)
    rep_phi = build_rep40(Phi, perm_phi)

    # Restrict to flat subspace
    M_d = flat_dirs @ rep_delta @ flat_dirs.T
    M_p = flat_dirs @ rep_phi @ flat_dirs.T

    # Verify group structure
    print(f"  G_symp = ⟨Δ₇₂°, φ⟩, order 10")
    print(f"  Abelian (C₅ × Z₂): {np.allclose(M_p @ M_d @ M_p, M_d)}")

    # Eigenvalues of Δ₇₂°
    eigvals_d = np.linalg.eigvals(M_d)
    from collections import Counter
    angle_counts = Counter()
    for ev in eigvals_d:
        angle = round(np.degrees(np.angle(ev)))
        angle_counts[angle] += 1

    print(f"\n  Δ₇₂° eigenvalue spectrum on flat subspace:")
    for angle in sorted(angle_counts.keys()):
        print(f"    e^{{i·{angle}°}}: multiplicity {angle_counts[angle]}")

    # Eigenvalues of φ
    eigvals_p = np.linalg.eigvals(M_p)
    n_phi_plus = sum(1 for ev in eigvals_p if abs(ev - 1) < 1e-6)
    n_phi_minus = sum(1 for ev in eigvals_p if abs(ev + 1) < 1e-6)
    print(f"\n  φ eigenvalue spectrum: +1 × {n_phi_plus}, -1 × {n_phi_minus}")

    # Joint fixed subspace (Δ=1 AND φ=+1)
    M_dm1 = M_d - np.eye(n_flat)
    M_pm1 = M_p - np.eye(n_flat)
    combined = np.vstack([M_dm1, M_pm1])
    s_combined = np.linalg.svd(combined, compute_uv=False)
    n_invariant = np.sum(s_combined < 1e-6)

    print(f"\n  Fully invariant directions (Δ=1, φ=+1): {n_invariant}")

    # Check uniform scaling
    scaling = duals.flatten()
    scaling_unit = scaling / np.linalg.norm(scaling)
    coords = flat_dirs @ scaling_unit
    proj_norm = np.linalg.norm(coords)
    print(f"\n  Uniform scaling a_i → λa_i:")
    print(f"    Component in flat subspace: {proj_norm:.6f} / 1.000 → {'IS flat' if proj_norm > 0.999 else 'NOT flat'}")
    print(f"    (sys is scale-invariant: c² ∝ λ⁴, vol ∝ λ⁴ in R⁴)")

    # Irreducible decomposition summary
    n_72 = angle_counts.get(72, 0)
    n_144 = angle_counts.get(144, 0)
    print(f"\n  Irreducible decomposition of 15D flat subspace:")
    print(f"    Δ=1, φ=+1 (invariant):   {n_invariant}D")
    print(f"    Δ=1, φ=-1 (q↔p antisym): {angle_counts.get(0, 0) - n_invariant}D")
    print(f"    Δ=e^{{±72°i}} (2D irreps):  {2*n_72}D ({n_72} copies)")
    print(f"    Δ=e^{{±144°i}} (2D irreps): {2*n_144}D ({n_144} copies)")
    print(f"    Total: {n_flat}D")
    n_curv_classes = n_invariant + (angle_counts.get(0, 0) - n_invariant) + n_72 + n_144
    print(f"\n  Curvature classes (directions up to symmetry): ≤{n_curv_classes}")
    print()

def plot_random_histogram(curvatures):
    """Histogram of curvatures from random directions."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.hist(curvatures, bins=20, color='#2ca02c', edgecolor='k', linewidth=0.5)
    ax.axvline(0, color='#d62728', linewidth=1.5, linestyle='--', label=r"$r = 0$")
    ax.set_xlabel(r"Curvature $r(\varepsilon)$")
    ax.set_ylabel("Count")
    ax.legend()
    fig.savefig(EXPERIMENT_DIR / "second_order_random_hist.png")
    plt.close(fig)
    print(f"  Wrote second_order_random_hist.png")

# ─── Main ────────────────────────────────────────────────────────────────────

def main():
    base = load_base()
    curves = load_curves()

    print()

    # Part A: Phase C LP
    lp_feasible = phase_c_lp(base)

    # Part B: Curvature analysis (SVD basis)
    dir_curvatures, dir_curvature_arrays = analyze_curvatures(base, curves)

    # Part C: Random directions + symmetry
    random_rows = load_random()
    random_curvatures = None
    if random_rows:
        random_curvatures = analyze_random(random_rows)
        symmetry_decomposition(base)

    # Figures
    print("--- Figures ---")
    plot_curves(base, curves, dir_curvatures)
    plot_curvatures(dir_curvatures)
    write_curvature_table(dir_curvatures, dir_curvature_arrays)
    if random_curvatures is not None:
        plot_random_histogram(random_curvatures)

    # Summary
    print()
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"  LP feasible (0 ∈ conv): {lp_feasible}")
    print(f"  Gradient rank: {base['rank']} / 40")
    print(f"  Flat directions: {base['n_flat_directions']}")
    all_neg_basis = all(c < 0 for c in dir_curvatures.values())
    print(f"  All basis curvatures negative: {all_neg_basis}")
    if random_curvatures is not None:
        all_neg_random = all(c < -1e-6 for c in random_curvatures)
        print(f"  All random curvatures negative: {all_neg_random} ({len(random_curvatures)} samples)")
        worst = max(random_curvatures)
        print(f"  Worst random curvature: {worst:+.4e}")
    if all_neg_basis:
        print()
        print("  CONCLUSION: HKO2024 satisfies:")
        print("  1. First-order necessary condition: 0 ∈ conv(subdifferential)")
        print("  2. All 15 SVD basis directions have negative curvature")
        if random_curvatures and all(c < -1e-6 for c in random_curvatures):
            print(f"  3. All {len(random_curvatures)} random directions have negative curvature (worst: {worst:+.4e})")
            print("  → Strong numerical evidence for negative definiteness of the Hessian")
            print("    on the 15D flat subspace, supporting local maximality of sys.")


if __name__ == "__main__":
    main()
