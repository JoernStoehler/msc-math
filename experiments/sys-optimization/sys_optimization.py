#!/usr/bin/env python3
"""
Analyze sys-optimization results: sensitivity analysis and gradient steps.

Compares height-only (h) vs joint height-normal (h,n) gradient steps.

Input:
  - experiments/sys-optimization/sys-optimization-sensitivity.jsonl
  - experiments/sys-optimization/sys-optimization-steps.jsonl
Output:
  - experiments/sys-optimization/sys_optimization_gradient_hist.png
  - experiments/sys-optimization/sys_optimization_gradient_comparison.png
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
    """Load JSONL, skipping truncated lines (from interrupted experiments)."""
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print("Run: cargo run --bin sys_optimization --release", file=sys.stderr)
        sys.exit(1)
    rows = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError:
                pass  # truncated last line from interrupted run
    return rows


# =============================================================================
# Figure 1: Gradient magnitude histogram (height sensitivities)
# =============================================================================

def plot_gradient_histogram(sens_rows: list[dict], output_path: Path) -> None:
    """Histogram of d(sys)/d(log h_k) = h_k * d(sys)/d(h_k), signed."""
    all_grads = []
    for r in sens_rows:
        heights = r["heights"]
        for k, ds in enumerate(r["d_sys_h"]):
            if ds is not None and np.isfinite(ds) and abs(ds) > 1e-15:
                all_grads.append(heights[k] * ds)

    if not all_grads:
        print("WARNING: no gradient data to plot", file=sys.stderr)
        return

    all_grads = np.array(all_grads)

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 4.5))

    # Signed histogram (linear scale), bins centered on zero
    extent = max(abs(all_grads.min()), abs(all_grads.max()))
    bin_edges = np.linspace(-extent, extent, 51)
    ax1.hist(all_grads, bins=bin_edges, color="#3b6ea8", alpha=0.75, edgecolor="white")
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
# Figure 2: Gradient comparison — height vs normal components
# =============================================================================

def plot_gradient_comparison(sens_rows: list[dict], output_path: Path) -> None:
    """Scatter: |∇_h sys| vs |∇_n sys| per polytope, colored by facet count."""
    grad_h = np.array([r["gradient_norm_h"] for r in sens_rows])
    grad_n = np.array([r["gradient_norm_n"] for r in sens_rows])
    facet_counts = np.array([r["facet_count"] for r in sens_rows])

    # Filter out zero gradients for log scale
    valid = (grad_h > 1e-15) & (grad_n > 1e-15)
    if valid.sum() == 0:
        print("WARNING: no gradient comparison data", file=sys.stderr)
        return

    gh = grad_h[valid]
    gn = grad_n[valid]
    fc = facet_counts[valid]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(13, 5.5))

    # Left: log-log scatter of |∇_h| vs |∇_n|
    scatter = ax1.scatter(gh, gn, c=fc, cmap="viridis", alpha=0.7, s=30, zorder=3)
    cbar = fig.colorbar(scatter, ax=ax1)
    cbar.set_label("Facet count")

    # Diagonal reference
    lo = min(gh.min(), gn.min()) * 0.5
    hi = max(gh.max(), gn.max()) * 2
    ax1.plot([lo, hi], [lo, hi], "k--", alpha=0.3, label=r"$|\nabla_n| = |\nabla_h|$")
    ax1.set_xscale("log")
    ax1.set_yscale("log")
    ax1.set_xlabel(r"$\|\nabla_h\,\mathrm{sys}\|$")
    ax1.set_ylabel(r"$\|\nabla_n\,\mathrm{sys}\|$")
    ax1.set_title(r"Height vs normal gradient magnitudes")
    ax1.legend(loc="lower right")
    ax1.grid(True, alpha=0.3, which="both")

    # Right: t_max comparison
    t_h = np.array([r["t_max_h"] for r in sens_rows])[valid]
    t_hn = np.array([r["t_max_hn"] for r in sens_rows])[valid]
    t_valid = (t_h > 1e-15) & (t_hn > 1e-15)
    if t_valid.sum() > 0:
        scatter2 = ax2.scatter(
            t_h[t_valid], t_hn[t_valid],
            c=fc[t_valid], cmap="viridis", alpha=0.7, s=30, zorder=3,
        )
        lo2 = min(t_hn[t_valid].min(), 1e-4) * 0.5
        hi2 = max(t_h[t_valid].max(), 1) * 2
        ax2.plot([lo2, hi2], [lo2, hi2], "k--", alpha=0.3)
        ax2.set_xscale("log")
        ax2.set_yscale("log")
    ax2.set_xlabel(r"$t_{\max}$ (height only)")
    ax2.set_ylabel(r"$t_{\max}$ (height + normal)")
    ax2.set_title(r"Step size bounds: $h$ vs $(h,n)$")
    ax2.grid(True, alpha=0.3, which="both")

    fig.tight_layout()
    fig.savefig(output_path, dpi=150)
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 3: h-only vs (h,n) improvement comparison
# =============================================================================

def plot_improvement(steps_rows: list[dict], output_path: Path) -> None:
    """Scatter comparing best h-only vs best (h,n) improvement per polytope."""
    # Group best step per (polytope, step_type)
    best: dict[tuple[str, str], dict] = {}
    for r in steps_rows:
        if not r["construction_ok"]:
            continue
        key = (r["name"], r["step_type"])
        if key not in best or r["new_sys"] > best[key]["new_sys"]:
            best[key] = r

    # Match polytopes that have both step types
    names = set()
    for (name, stype) in best:
        if stype == "h_only":
            if (name, "h_n") in best:
                names.add(name)

    if not names:
        print("WARNING: no matched step pairs to compare", file=sys.stderr)
        return

    names = sorted(names)
    delta_h = np.array([best[(n, "h_only")]["delta_sys"] for n in names])
    delta_hn = np.array([best[(n, "h_n")]["delta_sys"] for n in names])
    old_sys = np.array([best[(n, "h_only")]["old_sys"] for n in names])

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(13, 5.5))

    # Left: delta_sys comparison
    above = delta_hn > delta_h
    ax1.scatter(
        delta_h[above], delta_hn[above],
        alpha=0.6, s=25, color="#27ae60",
        label=f"$(h,n)$ better ({above.sum()})", zorder=3,
    )
    ax1.scatter(
        delta_h[~above], delta_hn[~above],
        alpha=0.6, s=25, color="#c0392b",
        label=f"$h$ better ({(~above).sum()})", zorder=3,
    )
    lo = min(delta_h.min(), delta_hn.min())
    hi = max(delta_h.max(), delta_hn.max())
    margin = (hi - lo) * 0.05
    ax1.plot([lo - margin, hi + margin], [lo - margin, hi + margin], "k--", alpha=0.3)
    ax1.axhline(y=0, color="gray", linewidth=0.5, alpha=0.5)
    ax1.axvline(x=0, color="gray", linewidth=0.5, alpha=0.5)
    ax1.set_xlabel(r"$\Delta\mathrm{sys}$ (height only)")
    ax1.set_ylabel(r"$\Delta\mathrm{sys}$ (height + normal)")
    ax1.set_title(r"Improvement comparison: $h$ vs $(h,n)$")
    ax1.legend(loc="upper left")
    ax1.grid(True, alpha=0.3)

    # Right: new_sys after best step of either type
    new_sys_h = np.array([best[(n, "h_only")]["new_sys"] for n in names])
    new_sys_hn = np.array([best[(n, "h_n")]["new_sys"] for n in names])
    best_new = np.maximum(new_sys_h, new_sys_hn)

    ax2.scatter(old_sys, best_new, alpha=0.6, s=25, color="#3b6ea8", zorder=3)
    lim = max(old_sys.max(), best_new.max()) * 1.1
    ax2.plot([0, lim], [0, lim], "k--", alpha=0.3, label="No improvement")
    ax2.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.5, label="sys = 1")
    ax2.axvline(x=1.0, color="#c0392b", linestyle="--", alpha=0.5)
    ax2.set_xlabel("sys (before)")
    ax2.set_ylabel("sys (after best step)")
    ax2.set_title("Best achievable sys (either step type)")
    ax2.set_xlim(0, lim)
    ax2.set_ylim(0, lim)
    ax2.set_aspect("equal")
    ax2.legend(loc="upper left")
    ax2.grid(True, alpha=0.3)

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

    # Gradient norms
    gradient_h = [r["gradient_norm_h"] for r in sens_rows]
    gradient_n = [r["gradient_norm_n"] for r in sens_rows]
    gradient_hn = [r["gradient_norm_hn"] for r in sens_rows]
    n_with_gradient = sum(1 for g in gradient_hn if g > 1e-10)

    # Sys values
    sys_before = [r["sys"] for r in sens_rows]
    max_sys_before = max(sys_before) if sys_before else 0

    # Best step per (polytope, type)
    best: dict[tuple[str, str], dict] = {}
    for r in steps_rows:
        if not r["construction_ok"] or not np.isfinite(r["new_sys"]):
            continue
        key = (r["name"], r["step_type"])
        if key not in best or r["new_sys"] > best[key]["new_sys"]:
            best[key] = r

    # h-only stats
    h_steps = [v for (_, t), v in best.items() if t == "h_only"]
    n_improved_h = sum(1 for r in h_steps if r["delta_sys"] > 1e-10)
    delta_h = [r["delta_sys"] for r in h_steps]
    new_sys_h = [r["new_sys"] for r in h_steps]

    # (h,n) stats
    hn_steps = [v for (_, t), v in best.items() if t == "h_n"]
    n_improved_hn = sum(1 for r in hn_steps if r["delta_sys"] > 1e-10)
    delta_hn = [r["delta_sys"] for r in hn_steps]
    new_sys_hn = [r["new_sys"] for r in hn_steps]

    # Best overall
    all_new = new_sys_h + new_sys_hn
    max_sys_after = max(all_new) if all_new else 0

    lines = [
        r"\begin{tabular}{lr}",
        r"\toprule",
        r"Statistic & Value \\",
        r"\midrule",
        rf"Polytopes analyzed & {n_polytopes} \\",
        rf"Polytopes with $\|\nabla_{{(h,n)}} \mathrm{{sys}}\| > 0$ & {n_with_gradient} \\",
        r"\midrule",
        rf"Mean $\|\nabla_h \mathrm{{sys}}\|$ & {np.mean(gradient_h):.4f} \\",
        rf"Mean $\|\nabla_n \mathrm{{sys}}\|$ & {np.mean(gradient_n):.4f} \\",
        rf"Mean $\|\nabla_{{(h,n)}} \mathrm{{sys}}\|$ & {np.mean(gradient_hn):.4f} \\",
        r"\midrule",
        rf"Improved by $h$-only step & {n_improved_h}/{len(h_steps)} \\",
        rf"Improved by $(h,n)$ step & {n_improved_hn}/{len(hn_steps)} \\",
        rf"Mean $\Delta\mathrm{{sys}}$ ($h$-only) & {np.mean(delta_h):.4f} \\",
        rf"Mean $\Delta\mathrm{{sys}}$ ($(h,n)$) & {np.mean(delta_hn):.4f} \\",
        r"\midrule",
        rf"Best sys (before) & {max_sys_before:.4f} \\",
        rf"Best sys (after any step) & {max_sys_after:.4f} \\",
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
    plot_gradient_comparison(
        sens_rows, EXPERIMENT_DIR / "sys_optimization_gradient_comparison.png"
    )
    plot_improvement(
        steps_rows, EXPERIMENT_DIR / "sys_optimization_improvement.png"
    )
    write_stats_table(
        sens_rows, steps_rows, EXPERIMENT_DIR / "sys_optimization_stats.tex"
    )


if __name__ == "__main__":
    main()
