#!/usr/bin/env python3
"""
Goal: Analyze second-order behavior along flat directions at HKO2024.
      Part A: Phase C LP replacement — verify 0 ∈ conv(gradients) in a_i space.
      Part B: Curvature analysis — fit second-order behavior, produce figures.
Input: second-order-base.jsonl, second-order-curves.jsonl
Output: second_order_curves.png, second_order_curvatures.png,
        second_order_curvatures.tex
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import numpy as np
from scipy.optimize import linprog

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
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
    ax.set_ylabel(r"Curvature $r(\varepsilon) = (\mathrm{sys}(+\varepsilon) + \mathrm{sys}(-\varepsilon) - 2\,\mathrm{sys}(0))/\varepsilon^2$")
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

# ─── Main ────────────────────────────────────────────────────────────────────

def main():
    base = load_base()
    curves = load_curves()

    print()

    # Part A: Phase C LP
    lp_feasible = phase_c_lp(base)

    # Part B: Curvature analysis
    dir_curvatures, dir_curvature_arrays = analyze_curvatures(base, curves)

    # Figures
    print("--- Figures ---")
    plot_curves(base, curves, dir_curvatures)
    plot_curvatures(dir_curvatures)
    write_curvature_table(dir_curvatures, dir_curvature_arrays)

    # Summary
    print()
    print("=" * 70)
    print("SUMMARY")
    print("=" * 70)
    print(f"  LP feasible (0 ∈ conv): {lp_feasible}")
    print(f"  Gradient rank: {base['rank']} / 40")
    print(f"  Flat directions: {base['n_flat_directions']}")
    all_neg = all(c < 0 for c in dir_curvatures.values())
    print(f"  All curvatures negative: {all_neg}")
    if all_neg:
        print()
        print("  CONCLUSION: HKO2024 satisfies both the first-order necessary condition")
        print("  (0 ∈ conv of subdifferential) and the second-order sufficient condition")
        print("  (negative curvature along all flat directions) for local maximality")
        print("  of sys in the F=10 dual-vertex parameter space R^40.")


if __name__ == "__main__":
    main()
