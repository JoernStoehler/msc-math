"""
Goal: Validate analytical gradient correctness across 4 experimental phases.
Input:
  - experiments/gradient-correctness/gradient-correctness-q1-generic.jsonl
  - experiments/gradient-correctness/gradient-correctness-q2-nongeneric.jsonl
  - experiments/gradient-correctness/gradient-correctness-q3-degeneracy.jsonl
  - experiments/gradient-correctness/gradient-correctness-q4-redundant.jsonl
Output:
  - experiments/gradient-correctness/gc_q1_step_sweep.png
  - experiments/gradient-correctness/gc_q1_dimension.png
  - experiments/gradient-correctness/gc_q2_nongeneric.png
  - experiments/gradient-correctness/gc_q3_gap_vs_error.png
  - experiments/gradient-correctness/gc_q3_orbit_switching.png
  - experiments/gradient-correctness/gc_q4_delta_vs_error.png
  - experiments/gradient-correctness/gc_summary.tex
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.lines import Line2D
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from figure_config import (
    setup,
    FIGSIZE_SINGLE,
    FIGSIZE_DUAL,
    FIGSIZE_TRIPLE,
    FONT_SIZE_SMALL,
    SCATTER_SIZE,
)

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent

TARGETS = ["capacity", "volume", "sys"]
TARGET_LABELS = {"capacity": r"$\partial c / \partial a_k$",
                 "volume": r"$\partial \mathrm{vol} / \partial a_k$",
                 "sys": r"$\partial \mathrm{sys} / \partial a_k$"}
TARGET_COLORS = {"capacity": "C0", "volume": "C1", "sys": "C2"}


def load_jsonl(name):
    path = EXPERIMENT_DIR / name
    if not path.exists():
        print(f"Warning: {path} not found, skipping")
        return []
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


# ============================================================================
# Q1: FD step-size sweep and dimension scaling
# ============================================================================

def plot_q1_step_sweep(data):
    """V-curve: log(eps) vs log(max_rel_error), one line per F, 3 panels."""
    if not data:
        return

    fig, axes = plt.subplots(1, 3, figsize=FIGSIZE_TRIPLE, sharey=True)
    facet_counts = sorted(set(r["facet_count"] for r in data))
    cmap = plt.cm.viridis(np.linspace(0.1, 0.9, len(facet_counts)))

    for ax, target in zip(axes, TARGETS):
        target_data = [r for r in data if r["target"] == target]
        for fi, fc in enumerate(facet_counts):
            fc_data = [r for r in target_data if r["facet_count"] == fc]
            # Group by epsilon, compute median error
            eps_vals = sorted(set(r["fd_epsilon"] for r in fc_data))
            medians = []
            for eps in eps_vals:
                errs = [r["max_rel_error"] for r in fc_data if r["fd_epsilon"] == eps]
                medians.append(np.median(errs))
            ax.plot(eps_vals, medians, marker="o", color=cmap[fi],
                    label=f"F={fc}", markersize=3)

        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_xlabel(r"FD step size $\varepsilon$")
        ax.set_title(TARGET_LABELS[target])
        ax.invert_xaxis()

    axes[0].set_ylabel("Median max relative error")
    axes[-1].legend(fontsize=FONT_SIZE_SMALL, loc="upper left")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "gc_q1_step_sweep.png")
    plt.close(fig)
    print("Saved gc_q1_step_sweep.png")


def plot_q1_dimension(data):
    """Error at sweet-spot eps vs F, box plot, 3 panels."""
    if not data:
        return

    sweet_eps = 1e-5
    eps_tol = 0.5  # relative tolerance for matching epsilon
    sweet_data = [r for r in data
                  if abs(r["fd_epsilon"] - sweet_eps) / sweet_eps < eps_tol]

    fig, axes = plt.subplots(1, 3, figsize=FIGSIZE_TRIPLE, sharey=True)
    facet_counts = sorted(set(r["facet_count"] for r in sweet_data))

    for ax, target in zip(axes, TARGETS):
        target_data = [r for r in sweet_data if r["target"] == target]
        box_data = []
        for fc in facet_counts:
            errs = [r["max_rel_error"] for r in target_data if r["facet_count"] == fc]
            box_data.append(errs)

        bp = ax.boxplot(box_data, tick_labels=[str(f) for f in facet_counts],
                        patch_artist=True)
        for patch in bp["boxes"]:
            patch.set_facecolor(TARGET_COLORS[target])
            patch.set_alpha(0.4)

        ax.set_yscale("log")
        ax.set_xlabel("Facet count F")
        ax.set_title(TARGET_LABELS[target])

    axes[0].set_ylabel(r"Max relative error at $\varepsilon = 10^{-5}$")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "gc_q1_dimension.png")
    plt.close(fig)
    print("Saved gc_q1_dimension.png")


# ============================================================================
# Q2: Non-generic geometry
# ============================================================================

def plot_q2_nongeneric(q1_data, q2_data):
    """Error comparison: generic (Q1 at sweet-spot) vs non-generic (Q2) types."""
    if not q2_data:
        return

    sweet_eps = 1e-5
    eps_tol = 0.5
    q1_sweet = [r for r in q1_data
                if abs(r["fd_epsilon"] - sweet_eps) / sweet_eps < eps_tol]

    classes = sorted(set(r["polytope_class"] for r in q2_data))
    all_classes = ["random"] + classes
    class_labels = {
        "random": "Generic\n(Q1)",
        "lagrangian_regular": "LP\nregular",
        "lagrangian_rotated": "LP\nrotated",
        "lagrangian_random": "LP\nrandom",
    }

    fig, axes = plt.subplots(1, 3, figsize=FIGSIZE_TRIPLE, sharey=True)

    for ax, target in zip(axes, TARGETS):
        box_data = []
        labels = []
        for cls in all_classes:
            if cls == "random":
                errs = [r["max_rel_error"] for r in q1_sweet if r["target"] == target]
            else:
                errs = [r["max_rel_error"] for r in q2_data
                        if r["target"] == target and r["polytope_class"] == cls]
            if errs:
                box_data.append(errs)
                labels.append(class_labels.get(cls, cls))

        if box_data:
            bp = ax.boxplot(box_data, tick_labels=labels, patch_artist=True)
            for patch in bp["boxes"]:
                patch.set_facecolor(TARGET_COLORS[target])
                patch.set_alpha(0.4)

        ax.set_yscale("log")
        ax.set_title(TARGET_LABELS[target])
        ax.tick_params(axis="x", labelsize=FONT_SIZE_SMALL - 1)

    axes[0].set_ylabel("Max relative error")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "gc_q2_nongeneric.png")
    plt.close(fig)
    print("Saved gc_q2_nongeneric.png")


# ============================================================================
# Q3: Near-degeneracy
# ============================================================================

def plot_q3_gap_vs_error(data):
    """Scatter: log(action_gap) vs log(max_rel_error) for capacity and sys."""
    if not data:
        return

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL, sharey=True)

    for ax, target in zip(axes, ["capacity", "sys"]):
        target_data = [r for r in data
                       if r["target"] == target and r["action_gap"] is not None]
        gaps = [r["action_gap"] for r in target_data]
        errs = [r["max_rel_error"] for r in target_data]
        switched = [r.get("orbit_switched_in_fd", False) for r in target_data]

        # Color by orbit switching
        colors = ["C3" if s else TARGET_COLORS[target] for s in switched]
        ax.scatter(gaps, errs, c=colors, s=SCATTER_SIZE, alpha=0.7, edgecolors="none")

        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_xlabel("Action gap (2nd best − best)")
        ax.set_title(TARGET_LABELS[target])

        # Legend for orbit switching
        handles = [
            Line2D([0], [0], marker="o", color="w", markerfacecolor=TARGET_COLORS[target],
                   markersize=5, label="No switch"),
            Line2D([0], [0], marker="o", color="w", markerfacecolor="C3",
                   markersize=5, label="Orbit switched"),
        ]
        ax.legend(handles=handles, fontsize=FONT_SIZE_SMALL)

    axes[0].set_ylabel("Max relative error")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "gc_q3_gap_vs_error.png")
    plt.close(fig)
    print("Saved gc_q3_gap_vs_error.png")


def plot_q3_orbit_switching(data):
    """Fraction of polytopes with orbit switching, binned by action gap."""
    if not data:
        return

    # Only look at capacity target (orbit switching is per-polytope, same for cap and sys)
    cap_data = [r for r in data
                if r["target"] == "capacity" and r["action_gap"] is not None]
    if not cap_data:
        return

    bin_edges = [0, 1e-4, 1e-2, 1e-1, float("inf")]
    bin_labels = [r"$< 10^{-4}$", r"$10^{-4}$–$10^{-2}$",
                  r"$10^{-2}$–$10^{-1}$", r"$> 10^{-1}$"]

    fractions = []
    counts = []
    for i in range(len(bin_edges) - 1):
        lo, hi = bin_edges[i], bin_edges[i + 1]
        bin_rows = [r for r in cap_data if lo <= r["action_gap"] < hi]
        n = len(bin_rows)
        if n > 0:
            switched = sum(1 for r in bin_rows if r.get("orbit_switched_in_fd", False))
            fractions.append(switched / n)
        else:
            fractions.append(0)
        counts.append(n)

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    x = range(len(bin_labels))
    bars = ax.bar(x, fractions, color="C3", alpha=0.7)

    # Annotate with counts
    for bar, count in zip(bars, counts):
        ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 0.02,
                f"n={count}", ha="center", fontsize=FONT_SIZE_SMALL)

    ax.set_xticks(list(x))
    ax.set_xticklabels(bin_labels)
    ax.set_xlabel("Action gap bin")
    ax.set_ylabel("Fraction with orbit switching in FD")
    ax.set_ylim(0, 1.1)
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "gc_q3_orbit_switching.png")
    plt.close(fig)
    print("Saved gc_q3_orbit_switching.png")


# ============================================================================
# Q4: Barely-cutting facets
# ============================================================================

def plot_q4_delta_vs_error(data):
    """Error vs log(delta), lines for cap/vol/sys."""
    if not data:
        return

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    for target in TARGETS:
        target_data = [r for r in data
                       if r["target"] == target and r["barely_cutting_delta"] is not None]
        deltas = sorted(set(r["barely_cutting_delta"] for r in target_data))
        medians = []
        p25 = []
        p75 = []
        for d in deltas:
            errs = [r["max_rel_error"] for r in target_data if r["barely_cutting_delta"] == d]
            medians.append(np.median(errs))
            p25.append(np.percentile(errs, 25))
            p75.append(np.percentile(errs, 75))

        ax.plot(deltas, medians, marker="o", color=TARGET_COLORS[target],
                label=TARGET_LABELS[target], markersize=4)
        ax.fill_between(deltas, p25, p75, color=TARGET_COLORS[target], alpha=0.15)

    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel(r"Barely-cutting $\delta$")
    ax.set_ylabel("Median max relative error")
    ax.legend()
    ax.invert_xaxis()
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "gc_q4_delta_vs_error.png")
    plt.close(fig)
    print("Saved gc_q4_delta_vs_error.png")


# ============================================================================
# Summary table
# ============================================================================

def write_summary_table(q1, q2, q3, q4):
    """Write LaTeX summary table with median and P95 errors per phase x target."""
    sweet_eps = 1e-5
    eps_tol = 0.5

    phases = [
        ("Q1 generic", [r for r in q1 if abs(r["fd_epsilon"] - sweet_eps) / sweet_eps < eps_tol]),
        ("Q2 non-generic", q2),
        ("Q3 near-degenerate", q3),
        ("Q4 barely-cutting", q4),
    ]

    lines = []
    lines.append(r"\begin{tabular}{l l r r r}")
    lines.append(r"\toprule")
    lines.append(r"Phase & Target & Median & P95 & Max \\")
    lines.append(r"\midrule")

    for phase_name, phase_data in phases:
        if not phase_data:
            continue
        for target in TARGETS:
            td = [r["max_rel_error"] for r in phase_data if r["target"] == target]
            if not td:
                continue
            med = np.median(td)
            p95 = np.percentile(td, 95)
            mx = np.max(td)
            lines.append(
                f"{phase_name} & {target} & "
                f"{med:.2e} & {p95:.2e} & {mx:.2e} \\\\"
            )
        lines.append(r"\midrule")

    # Remove last midrule, replace with bottomrule
    if lines[-1] == r"\midrule":
        lines[-1] = r"\bottomrule"
    lines.append(r"\end{tabular}")

    path = EXPERIMENT_DIR / "gc_summary.tex"
    with open(path, "w") as f:
        f.write("\n".join(lines) + "\n")
    print(f"Saved gc_summary.tex")

    # Also print to stdout
    print("\nSummary (max_rel_error):")
    print(f"{'Phase':<20} {'Target':<10} {'Median':>10} {'P95':>10} {'Max':>10}")
    print("-" * 62)
    for phase_name, phase_data in phases:
        if not phase_data:
            continue
        for target in TARGETS:
            td = [r["max_rel_error"] for r in phase_data if r["target"] == target]
            if not td:
                continue
            print(f"{phase_name:<20} {target:<10} {np.median(td):>10.2e} "
                  f"{np.percentile(td, 95):>10.2e} {np.max(td):>10.2e}")


# ============================================================================
# Main
# ============================================================================

def main():
    q1 = load_jsonl("gradient-correctness-q1-generic.jsonl")
    q2 = load_jsonl("gradient-correctness-q2-nongeneric.jsonl")
    q3 = load_jsonl("gradient-correctness-q3-degeneracy.jsonl")
    q4 = load_jsonl("gradient-correctness-q4-redundant.jsonl")

    print(f"Loaded: Q1={len(q1)}, Q2={len(q2)}, Q3={len(q3)}, Q4={len(q4)} rows\n")

    plot_q1_step_sweep(q1)
    plot_q1_dimension(q1)
    plot_q2_nongeneric(q1, q2)
    plot_q3_gap_vs_error(q3)
    plot_q3_orbit_switching(q3)
    plot_q4_delta_vs_error(q4)
    write_summary_table(q1, q2, q3, q4)


if __name__ == "__main__":
    main()
