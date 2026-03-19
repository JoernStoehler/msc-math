#!/usr/bin/env python3
"""
Analyze the omega-obstacle hypothesis: do small ω₀ values correlate with high systolic ratios?

Goal: Test whether near-Lagrangian 2-faces (small |ω₀(n_i, n_j)|) between adjacent
facets help create high systolic ratios.

Input: experiments/omega-obstacle/omega-obstacle.jsonl
Output: experiments/omega-obstacle/omega_obstacle_*.png (multiple figures)
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from figure_config import setup, FIGSIZE_SINGLE, FIGSIZE_DUAL
setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "omega-obstacle.jsonl"

# Color palette for facet counts
F_COLORS = {
    5: "#1f77b4",
    6: "#ff7f0e",
    7: "#2ca02c",
    8: "#d62728",
    9: "#9467bd",
    10: "#8c564b",
}
KNOWN_MARKER = "D"  # diamond for known polytopes


def load_data():
    """Load JSONL dataset."""
    if not DATA_PATH.exists():
        raise FileNotFoundError(
            f"Data not found: {DATA_PATH}\n"
            "Run: cd experiments/ && cargo run --bin omega_obstacle --release"
        )
    rows = []
    with open(DATA_PATH) as f:
        for line in f:
            rows.append(json.loads(line))
    return rows


def split_data(rows):
    """Split into random polytopes and known/reference polytopes."""
    random_rows = [r for r in rows if r["source"].startswith("random_")]
    known_rows = [r for r in rows if not r["source"].startswith("random_")]
    return random_rows, known_rows


def fig1_ridge_omega_abs_min_vs_sys(random_rows, known_rows):
    """Scatter: min |ω| over ridge-adjacent pairs vs sys. (Jörn's ideas 1+2)"""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    for f_count in sorted(F_COLORS.keys()):
        subset = [r for r in random_rows if r["facet_count"] == f_count]
        if not subset:
            continue
        x = [r["ridge_omega_abs_min"] for r in subset]
        y = [r["sys"] for r in subset]
        ax.scatter(x, y, c=F_COLORS[f_count], alpha=0.5, s=15,
                   label=f"F={f_count}")

    # Known polytopes
    for r in known_rows:
        ax.scatter(r["ridge_omega_abs_min"], r["sys"], marker=KNOWN_MARKER,
                   c="black", s=80, zorder=5, edgecolors="gold", linewidths=1.5)
        ax.annotate(r["source"], (r["ridge_omega_abs_min"], r["sys"]),
                    fontsize=7, ha="left", va="bottom", xytext=(4, 4),
                    textcoords="offset points")

    ax.set_xlabel("min |ω₀(nᵢ, nⱼ)| over ridge-adjacent pairs")
    ax.set_ylabel("sys = c² / (2V)")
    ax.set_title("Ridge ω abs-min vs systolic ratio")
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_ridge_min_vs_sys.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_ridge_min_vs_sys.png")


def fig2_orbit_omega_min_vs_sys(random_rows, known_rows):
    """Scatter: min ω over orbit transitions vs sys. (orbit-specific)"""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    for f_count in sorted(F_COLORS.keys()):
        subset = [r for r in random_rows if r["facet_count"] == f_count]
        if not subset:
            continue
        x = [r["orbit_omega_min"] for r in subset]
        y = [r["sys"] for r in subset]
        ax.scatter(x, y, c=F_COLORS[f_count], alpha=0.5, s=15,
                   label=f"F={f_count}")

    for r in known_rows:
        ax.scatter(r["orbit_omega_min"], r["sys"], marker=KNOWN_MARKER,
                   c="black", s=80, zorder=5, edgecolors="gold", linewidths=1.5)
        ax.annotate(r["source"], (r["orbit_omega_min"], r["sys"]),
                    fontsize=7, ha="left", va="bottom", xytext=(4, 4),
                    textcoords="offset points")

    ax.set_xlabel("min ω₀(n_from, n_to) over orbit transitions")
    ax.set_ylabel("sys = c² / (2V)")
    ax.set_title("Orbit ω min vs systolic ratio")
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_orbit_min_vs_sys.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_orbit_min_vs_sys.png")


def fig3_orbit_vs_nonorbit_omega(random_rows):
    """Box plot: orbit ω distribution vs non-orbit ridge ω distribution."""
    orbit_omegas_all = []
    nonorbit_omegas_all = []

    for r in random_rows:
        orbit_set = set(map(tuple, zip(r["orbit_facets"],
                                        r["orbit_facets"][1:] + [r["orbit_facets"][0]])))
        for triple in r["ridge_omegas"]:
            i, j = int(triple[0]), int(triple[1])
            w_abs = abs(triple[2])
            if (i, j) in orbit_set or (j, i) in orbit_set:
                orbit_omegas_all.append(w_abs)
            else:
                nonorbit_omegas_all.append(w_abs)

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    bp = ax.boxplot([orbit_omegas_all, nonorbit_omegas_all],
                    labels=["Orbit ridges", "Non-orbit ridges"],
                    patch_artist=True)
    bp["boxes"][0].set_facecolor("#d62728")
    bp["boxes"][1].set_facecolor("#1f77b4")

    ax.set_ylabel("|ω₀(nᵢ, nⱼ)|")
    ax.set_title("Ridge |ω| distribution: orbit vs non-orbit")
    ax.grid(True, alpha=0.3, axis="y")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_orbit_vs_nonorbit.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_orbit_vs_nonorbit.png")

    # Print stats
    print(f"    Orbit ridges:     median={np.median(orbit_omegas_all):.4f}, "
          f"mean={np.mean(orbit_omegas_all):.4f}, n={len(orbit_omegas_all)}")
    print(f"    Non-orbit ridges: median={np.median(nonorbit_omegas_all):.4f}, "
          f"mean={np.mean(nonorbit_omegas_all):.4f}, n={len(nonorbit_omegas_all)}")


def fig4_orbit_omega_mean_vs_sys(random_rows, known_rows):
    """Scatter: mean orbit ω vs sys."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    for f_count in sorted(F_COLORS.keys()):
        subset = [r for r in random_rows if r["facet_count"] == f_count]
        if not subset:
            continue
        x = [r["orbit_omega_mean"] for r in subset]
        y = [r["sys"] for r in subset]
        ax.scatter(x, y, c=F_COLORS[f_count], alpha=0.5, s=15,
                   label=f"F={f_count}")

    for r in known_rows:
        ax.scatter(r["orbit_omega_mean"], r["sys"], marker=KNOWN_MARKER,
                   c="black", s=80, zorder=5, edgecolors="gold", linewidths=1.5)
        ax.annotate(r["source"], (r["orbit_omega_mean"], r["sys"]),
                    fontsize=7, ha="left", va="bottom", xytext=(4, 4),
                    textcoords="offset points")

    ax.set_xlabel("mean ω₀ over orbit transitions")
    ax.set_ylabel("sys = c² / (2V)")
    ax.set_title("Orbit ω mean vs systolic ratio")
    ax.legend(fontsize=8)
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_orbit_mean_vs_sys.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_orbit_mean_vs_sys.png")


def fig5_gradient_dots_orbit(random_rows):
    """Histogram of dot products ⟨∇sys, ∇ω⟩ for orbit facets."""
    dots_orbit = []
    dots_nonorbit = []

    for r in random_rows:
        for gd in r["gradient_dots"]:
            if gd["grad_sys_norm"] < 1e-15:
                continue  # skip degenerate
            if gd["k_on_orbit"]:
                dots_orbit.append(gd["dot"])
            else:
                dots_nonorbit.append(gd["dot"])

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    # Orbit facets
    if dots_orbit:
        ax = axes[0]
        ax.hist(dots_orbit, bins=80, color="#d62728", alpha=0.7, edgecolor="black",
                linewidth=0.3)
        ax.axvline(0, color="black", linewidth=1, linestyle="--")
        median = np.median(dots_orbit)
        ax.axvline(median, color="blue", linewidth=1.5, linestyle="-",
                   label=f"median={median:.4f}")
        frac_neg = np.mean(np.array(dots_orbit) < 0)
        ax.set_title(f"Orbit facets: ⟨∇sys, ∇ω⟩\n({frac_neg:.0%} negative)")
        ax.set_xlabel("dot product ⟨∇sys, ∇ω⟩")
        ax.set_ylabel("count")
        ax.legend(fontsize=8)

    # Non-orbit facets
    if dots_nonorbit:
        ax = axes[1]
        ax.hist(dots_nonorbit, bins=80, color="#1f77b4", alpha=0.7,
                edgecolor="black", linewidth=0.3)
        ax.axvline(0, color="black", linewidth=1, linestyle="--")
        median = np.median(dots_nonorbit)
        ax.axvline(median, color="blue", linewidth=1.5, linestyle="-",
                   label=f"median={median:.4f}")
        frac_neg = np.mean(np.array(dots_nonorbit) < 0)
        ax.set_title(f"Non-orbit facets: ⟨∇sys, ∇ω⟩\n({frac_neg:.0%} negative)")
        ax.set_xlabel("dot product ⟨∇sys, ∇ω⟩")
        ax.set_ylabel("count")
        ax.legend(fontsize=8)

    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_gradient_dots.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_gradient_dots.png")
    print(f"    Orbit facets: n={len(dots_orbit)}, "
          f"median={np.median(dots_orbit):.6f}, "
          f"frac_neg={np.mean(np.array(dots_orbit) < 0):.2%}")
    if dots_nonorbit:
        print(f"    Non-orbit:    n={len(dots_nonorbit)}, "
              f"median={np.median(dots_nonorbit):.6f}, "
              f"frac_neg={np.mean(np.array(dots_nonorbit) < 0):.2%}")


def fig6_gradient_dots_by_orbit_neighbor(random_rows):
    """Histogram split: i on orbit vs i not on orbit (orbit-consecutive neighbor)."""
    dots_i_orbit = []
    dots_i_nonorbit = []

    for r in random_rows:
        for gd in r["gradient_dots"]:
            if not gd["k_on_orbit"] or gd["grad_sys_norm"] < 1e-15:
                continue
            if gd["i_on_orbit"]:
                dots_i_orbit.append(gd["dot"])
            else:
                dots_i_nonorbit.append(gd["dot"])

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    for ax, data, label, color in [
        (axes[0], dots_i_orbit, "Neighbor i on orbit", "#d62728"),
        (axes[1], dots_i_nonorbit, "Neighbor i NOT on orbit", "#2ca02c"),
    ]:
        if not data:
            ax.set_title(f"{label}: no data")
            continue
        ax.hist(data, bins=60, color=color, alpha=0.7, edgecolor="black",
                linewidth=0.3)
        ax.axvline(0, color="black", linewidth=1, linestyle="--")
        median = np.median(data)
        ax.axvline(median, color="blue", linewidth=1.5, linestyle="-",
                   label=f"median={median:.4f}")
        frac_neg = np.mean(np.array(data) < 0)
        short_label = "i on orbit" if "on orbit" in label and "NOT" not in label else "i not on orbit"
        ax.set_title(f"{short_label}\n({frac_neg:.0%} neg)")
        ax.set_xlabel("dot product ⟨∇sys, ∇ω⟩")
        ax.set_ylabel("count")
        ax.legend(fontsize=8)

    fig.suptitle(r"Orbit facet $k$: $\langle\nabla\mathrm{sys}, \nabla\omega\rangle$ by neighbor type", y=1.02)
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_gradient_neighbor_split.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_gradient_neighbor_split.png")


def fig7_omega_vs_dot(random_rows):
    """Scatter: |ω(n_k, n_i)| vs dot product (orbit facets only)."""
    omegas = []
    dots = []
    for r in random_rows:
        for gd in r["gradient_dots"]:
            if gd["k_on_orbit"] and gd["grad_sys_norm"] > 1e-15:
                omegas.append(abs(gd["omega"]))
                dots.append(gd["dot"])

    if not omegas:
        return

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.scatter(omegas, dots, alpha=0.15, s=5, c="#d62728")
    ax.axhline(0, color="black", linewidth=1, linestyle="--")
    ax.set_xlabel(r"$|\omega_0(n_k, n_i)|$")
    ax.set_ylabel(r"$\langle\nabla_{n_k}\mathrm{sys},\;\nabla_{n_k}\omega(n_k, n_i)\rangle$")
    ax.set_title("Orbit facets: |ω| vs gradient dot product")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "omega_obstacle_omega_vs_dot.png")
    plt.close(fig)
    print("  Saved: omega_obstacle_omega_vs_dot.png")


def print_summary(random_rows, known_rows):
    """Print dataset summary statistics."""
    print("\n=== Dataset summary ===")
    print(f"Random polytopes: {len(random_rows)}")
    print(f"Known polytopes:  {len(known_rows)}")

    sys_vals = [r["sys"] for r in random_rows]
    print(f"sys range: [{min(sys_vals):.4f}, {max(sys_vals):.4f}]")
    print(f"sys median: {np.median(sys_vals):.4f}")

    for f_count in sorted(F_COLORS.keys()):
        subset = [r for r in random_rows if r["facet_count"] == f_count]
        if subset:
            sys_f = [r["sys"] for r in subset]
            print(f"  F={f_count}: n={len(subset)}, "
                  f"sys=[{min(sys_f):.4f}, {max(sys_f):.4f}], "
                  f"median={np.median(sys_f):.4f}")

    for r in known_rows:
        print(f"  {r['source']}: sys={r['sys']:.6f}, "
              f"orbit_omega_min={r['orbit_omega_min']:.6f}, "
              f"ridge_omega_abs_min={r['ridge_omega_abs_min']:.6f}")

    # Correlation
    print("\n=== Correlations (Spearman) ===")
    from scipy.stats import spearmanr
    sys_arr = np.array([r["sys"] for r in random_rows])
    for feature_name, feature_fn in [
        ("ridge_omega_abs_min", lambda r: r["ridge_omega_abs_min"]),
        ("orbit_omega_min", lambda r: r["orbit_omega_min"]),
        ("orbit_omega_mean", lambda r: r["orbit_omega_mean"]),
        ("orbit_length", lambda r: r["orbit_length"]),
    ]:
        vals = np.array([feature_fn(r) for r in random_rows])
        rho, pval = spearmanr(vals, sys_arr)
        print(f"  {feature_name:25s}: rho={rho:+.4f}, p={pval:.2e}")


def main():
    print(f"Loading data from {DATA_PATH}")
    rows = load_data()
    random_rows, known_rows = split_data(rows)

    print_summary(random_rows, known_rows)

    print("\n=== Generating figures ===")
    fig1_ridge_omega_abs_min_vs_sys(random_rows, known_rows)
    fig2_orbit_omega_min_vs_sys(random_rows, known_rows)
    fig3_orbit_vs_nonorbit_omega(random_rows)
    fig4_orbit_omega_mean_vs_sys(random_rows, known_rows)
    fig5_gradient_dots_orbit(random_rows)
    fig6_gradient_dots_by_orbit_neighbor(random_rows)
    fig7_omega_vs_dot(random_rows)

    print("\nDone.")


if __name__ == "__main__":
    main()
