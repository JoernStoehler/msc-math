#!/usr/bin/env python3
"""
Analyze sys-optimization Phase 1-2 results: sensitivity analysis and gradient steps.

Goal: Visualize gradient structure and sys improvement from targeted height changes.
Input:
  - experiments/sys-optimization/sys-optimization-sensitivity.jsonl
  - experiments/sys-optimization/sys-optimization-steps.jsonl
Output:
  - experiments/sys-optimization/sys_optimization_gradient_hist.png
  - experiments/sys-optimization/sys_optimization_favorable.png
  - experiments/sys-optimization/sys_optimization_improvement.png
  - experiments/sys-optimization/sys_optimization_stats.tex
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
SENSITIVITY_PATH = EXPERIMENT_DIR / "sys-optimization-sensitivity.jsonl"
STEPS_PATH = EXPERIMENT_DIR / "sys-optimization-steps.jsonl"


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print("Run: cargo run --bin sys_optimization --release", file=sys.stderr)
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


# =============================================================================
# Figure 1: Gradient magnitude histogram
# =============================================================================

def plot_gradient_histogram(sens_rows: list[dict], output_path: Path) -> None:
    """Histogram of d(sys)/d(log h_k) = h_k * d(sys)/d(h_k), signed."""
    all_grads = []
    for r in sens_rows:
        heights = r["heights"]
        for k, ds in enumerate(r["d_sys"]):
            if ds is not None and np.isfinite(ds) and abs(ds) > 1e-15:
                all_grads.append(heights[k] * ds)

    if not all_grads:
        print("WARNING: no gradient data to plot", file=sys.stderr)
        return

    all_grads = np.array(all_grads)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 4.5))

    # Signed histogram (linear scale)
    ax1.hist(all_grads, bins=50, color="#3b6ea8", alpha=0.75, edgecolor="white")
    ax1.axvline(x=0, color="black", linewidth=0.8, alpha=0.5)
    ax1.set_xlabel(r"$\partial\,\mathrm{sys}\,/\,\partial\log h_k$")
    ax1.set_ylabel("Count")
    ax1.set_title("Logarithmic sensitivity (linear)")
    ax1.grid(True, alpha=0.3)

    # Magnitude on log scale
    abs_grads = np.abs(all_grads)
    log_grads = np.log10(abs_grads)
    ax2.hist(log_grads, bins=50, color="#3b6ea8", alpha=0.75, edgecolor="white")
    ax2.set_xlabel(r"$\log_{10}\,|\partial\,\mathrm{sys}\,/\,\partial\log h_k|$")
    ax2.set_ylabel("Count")
    ax2.set_title("Logarithmic sensitivity (log magnitude)")
    ax2.grid(True, alpha=0.3)

    fig.tight_layout()
    fig.savefig(output_path, dpi=150)
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 2: Favorable facets by facet count
# =============================================================================

def plot_favorable_facets(sens_rows: list[dict], output_path: Path) -> None:
    """Fraction of facets with favorable gradient direction, by facet count."""
    by_f = defaultdict(lambda: {"favorable": 0, "total": 0})
    for r in sens_rows:
        f = r["facet_count"]
        by_f[f]["favorable"] += r["n_favorable"]
        by_f[f]["total"] += f

    f_vals = sorted(by_f.keys())
    fractions = [by_f[f]["favorable"] / by_f[f]["total"] for f in f_vals]

    fig, ax = plt.subplots(figsize=(8, 4.5))
    ax.bar(f_vals, fractions, color="#3b6ea8", alpha=0.8, edgecolor="white")
    ax.axhline(y=1.0, color="gray", linestyle=":", alpha=0.5)
    ax.set_xlabel("Facet count F")
    ax.set_ylabel("Fraction of favorable facets")
    ax.set_title("Facets with non-zero gradient direction")
    ax.set_ylim(0, 1.05)
    ax.set_xticks(f_vals)
    ax.grid(True, alpha=0.3, axis="y")

    # Annotate counts
    for f, frac in zip(f_vals, fractions):
        n = by_f[f]["total"]
        ax.annotate(f"n={n}", (f, frac + 0.02), ha="center", fontsize=8, color="gray")

    fig.tight_layout()
    fig.savefig(output_path, dpi=150)
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 3: Improvement scatter
# =============================================================================

def plot_improvement(steps_rows: list[dict], output_path: Path) -> None:
    """Scatter: sys_before vs best sys_after per polytope."""
    # Group by polytope name, pick best step
    by_name: dict[str, dict] = {}
    for r in steps_rows:
        if not r["construction_ok"]:
            continue
        name = r["name"]
        if name not in by_name or r["new_sys"] > by_name[name]["new_sys"]:
            by_name[name] = r

    if not by_name:
        print("WARNING: no step data to plot", file=sys.stderr)
        return

    old_sys = np.array([r["old_sys"] for r in by_name.values()])
    new_sys = np.array([r["new_sys"] for r in by_name.values()])
    valid = np.isfinite(old_sys) & np.isfinite(new_sys)
    old_sys = old_sys[valid]
    new_sys = new_sys[valid]

    fig, ax = plt.subplots(figsize=(7, 7))

    # Reference lines
    lim = max(old_sys.max(), new_sys.max()) * 1.1
    ax.plot([0, lim], [0, lim], "k--", alpha=0.3, label="No improvement")
    ax.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.5, label="sys = 1")
    ax.axvline(x=1.0, color="#c0392b", linestyle="--", alpha=0.5)

    # Color by improvement
    improved = new_sys > old_sys
    ax.scatter(
        old_sys[improved], new_sys[improved],
        alpha=0.6, s=25, color="#27ae60", label=f"Improved ({improved.sum()})",
        zorder=3,
    )
    ax.scatter(
        old_sys[~improved], new_sys[~improved],
        alpha=0.6, s=25, color="#c0392b", label=f"Not improved ({(~improved).sum()})",
        zorder=3,
    )

    ax.set_xlabel("sys (before gradient step)")
    ax.set_ylabel("sys (after best gradient step)")
    ax.set_title("Sys improvement from single gradient step")
    ax.set_xlim(0, lim)
    ax.set_ylim(0, lim)
    ax.set_aspect("equal")
    ax.legend(loc="upper left")
    ax.grid(True, alpha=0.3)

    fig.tight_layout()
    fig.savefig(output_path, dpi=150)
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Stats table (LaTeX)
# =============================================================================

def write_stats_table(
    sens_rows: list[dict],
    steps_rows: list[dict],
    output_path: Path,
) -> None:
    """Write summary statistics as a LaTeX table."""
    n_polytopes = len(sens_rows)
    n_with_gradient = sum(1 for r in sens_rows if r["gradient_norm"] > 1e-10)

    # Collect sys values
    sys_before = [r["sys"] for r in sens_rows]
    gradient_norms = [r["gradient_norm"] for r in sens_rows]

    # Best step per polytope
    by_name: dict[str, dict] = {}
    for r in steps_rows:
        if not r["construction_ok"] or not np.isfinite(r["new_sys"]):
            continue
        name = r["name"]
        if name not in by_name or r["new_sys"] > by_name[name]["new_sys"]:
            by_name[name] = r

    best_steps = list(by_name.values())
    n_improved = sum(1 for r in best_steps if r["new_sys"] > r["old_sys"] + 1e-10)
    delta_sys_vals = [r["new_sys"] - r["old_sys"] for r in best_steps]
    new_sys_vals = [r["new_sys"] for r in best_steps]

    max_sys_before = max(sys_before) if sys_before else 0
    max_sys_after = max(new_sys_vals) if new_sys_vals else 0
    mean_delta = np.mean(delta_sys_vals) if delta_sys_vals else 0
    median_delta = np.median(delta_sys_vals) if delta_sys_vals else 0

    lines = [
        r"\begin{tabular}{lr}",
        r"\toprule",
        r"Statistic & Value \\",
        r"\midrule",
        rf"Polytopes analyzed & {n_polytopes} \\",
        rf"Polytopes with $\|\nabla \mathrm{{sys}}\| > 0$ & {n_with_gradient} \\",
        rf"Polytopes improved by gradient step & {n_improved} \\",
        rf"Best sys (before) & {max_sys_before:.4f} \\",
        rf"Best sys (after step) & {max_sys_after:.4f} \\",
        rf"Mean $\Delta$sys (best step) & {mean_delta:.4f} \\",
        rf"Median $\Delta$sys (best step) & {median_delta:.4f} \\",
        rf"Mean $\|\nabla \mathrm{{sys}}\|$ & {np.mean(gradient_norms):.4f} \\",
        r"\bottomrule",
        r"\end{tabular}",
    ]
    text = "\n".join(lines) + "\n"
    output_path.write_text(text)
    print(f"Saved: {output_path}")


# =============================================================================
# Main
# =============================================================================

def main() -> None:
    sens_rows = load_jsonl(SENSITIVITY_PATH)
    steps_rows = load_jsonl(STEPS_PATH)

    print(f"Loaded {len(sens_rows)} sensitivity rows, {len(steps_rows)} step rows\n")

    plot_gradient_histogram(
        sens_rows, EXPERIMENT_DIR / "sys_optimization_gradient_hist.png"
    )
    plot_favorable_facets(
        sens_rows, EXPERIMENT_DIR / "sys_optimization_favorable.png"
    )
    plot_improvement(
        steps_rows, EXPERIMENT_DIR / "sys_optimization_improvement.png"
    )
    write_stats_table(
        sens_rows, steps_rows, EXPERIMENT_DIR / "sys_optimization_stats.tex"
    )


if __name__ == "__main__":
    main()
