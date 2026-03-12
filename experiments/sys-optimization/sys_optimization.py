#!/usr/bin/env python3
"""
Analyze sys-optimization results: sensitivity analysis, gradient steps, iteration.

Compares height-only (h) vs joint height-normal (h,n) gradient steps.

Input:
  - experiments/sys-optimization/sys-optimization-sensitivity.jsonl
  - experiments/sys-optimization/sys-optimization-steps.jsonl
  - experiments/sys-optimization/sys-optimization-iterations.jsonl
Output:
  - experiments/sys-optimization/sys_optimization_gradient_hist.png
  - experiments/sys-optimization/sys_optimization_gradient_comparison.png
  - experiments/sys-optimization/sys_optimization_improvement.png
  - experiments/sys-optimization/sys_optimization_convergence.png
  - experiments/sys-optimization/sys_optimization_iteration_summary.png
  - experiments/sys-optimization/sys_optimization_validity.png
  - experiments/sys-optimization/sys_optimization_stats.tex
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

# Consistent figure style for thesis embedding.
# Body text is ~11pt; figures are scaled to \textwidth (~5.5in),
# so axis labels at 9pt and titles at 10pt are readable.
plt.rcParams.update({
    "font.size": 9,
    "axes.titlesize": 10,
    "axes.labelsize": 9,
    "xtick.labelsize": 8,
    "ytick.labelsize": 8,
    "legend.fontsize": 8,
    "figure.titlesize": 10,
})

EXPERIMENT_DIR = Path(__file__).resolve().parent
SENSITIVITY_PATH = EXPERIMENT_DIR / "sys-optimization-sensitivity.jsonl"
STEPS_PATH = EXPERIMENT_DIR / "sys-optimization-steps.jsonl"
ITERATIONS_PATH = EXPERIMENT_DIR / "sys-optimization-iterations.jsonl"
VALIDITY_PATH = EXPERIMENT_DIR / "sys-optimization-validity.jsonl"


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

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(5.4, 2.8))

    # Signed histogram (linear scale), bins centered on zero
    extent = max(abs(all_grads.min()), abs(all_grads.max()))
    n_bins = 50
    bin_width = 2 * extent / n_bins
    bin_edges = np.arange(-extent - bin_width / 2, extent + bin_width, bin_width)
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
    fig.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 2: Gradient comparison — height vs normal components
# =============================================================================

def plot_gradient_comparison(sens_rows: list[dict], output_path: Path) -> None:
    """Predicted max improvement and step bounds: h-only vs (h,n)."""
    grad_h = np.array([r["gradient_norm_h"] for r in sens_rows])
    grad_hn = np.array([r["gradient_norm_hn"] for r in sens_rows])
    t_h = np.array([r["t_max_h"] for r in sens_rows])
    t_hn = np.array([r["t_max_hn"] for r in sens_rows])
    facet_counts = np.array([r["facet_count"] for r in sens_rows])

    # Predicted max Δsys = t_max * ||∇sys|| (dimensionless)
    pred_h = t_h * grad_h
    pred_hn = t_hn * grad_hn

    # Filter to polytopes with nonzero predictions
    valid = (pred_h > 1e-15) & (pred_hn > 1e-15)
    if valid.sum() == 0:
        print("WARNING: no gradient comparison data", file=sys.stderr)
        return

    fc = facet_counts[valid]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(5.4, 3.0))

    # Left: predicted max Δsys comparison
    scatter = ax1.scatter(
        pred_h[valid], pred_hn[valid],
        c=fc, cmap="viridis", alpha=0.7, s=30, zorder=3,
    )
    cbar = fig.colorbar(scatter, ax=ax1)
    cbar.set_label("Facet count")

    lo = min(pred_h[valid].min(), pred_hn[valid].min()) * 0.5
    hi = max(pred_h[valid].max(), pred_hn[valid].max()) * 2
    ax1.plot([lo, hi], [lo, hi], "k--", alpha=0.3, label="Equal")
    ax1.set_xscale("log")
    ax1.set_yscale("log")
    ax1.set_xlabel(r"$t_{\max}^{(h)} \cdot \|\nabla_h\,\mathrm{sys}\|$")
    ax1.set_ylabel(r"$t_{\max}^{(h,n)} \cdot \|\nabla_{(h,n)}\,\mathrm{sys}\|$")
    ax1.set_title(r"Predicted max $\Delta\mathrm{sys}$")
    ax1.legend(loc="lower right")
    ax1.grid(True, alpha=0.3, which="both")

    # Right: t_max comparison
    t_valid = valid & (t_h > 1e-15) & (t_hn > 1e-15)
    if t_valid.sum() > 0:
        scatter2 = ax2.scatter(
            t_h[t_valid], t_hn[t_valid],
            c=facet_counts[t_valid], cmap="viridis", alpha=0.7, s=30, zorder=3,
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
    fig.savefig(output_path, dpi=150, bbox_inches='tight')
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

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(5.4, 3.0))

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
    fig.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 4: Convergence trajectories (Phase 3)
# =============================================================================

def plot_convergence(iter_rows: list[dict], output_path: Path) -> None:
    """Line plots of sys vs iteration for each polytope, colored by facet count."""
    # Group by polytope name
    by_name: dict[str, list[dict]] = defaultdict(list)
    for r in iter_rows:
        by_name[r["name"]].append(r)

    if not by_name:
        print("WARNING: no iteration data to plot", file=sys.stderr)
        return

    # Sort each trajectory by iteration
    for name in by_name:
        by_name[name].sort(key=lambda r: r["iteration"])

    # Get facet count per polytope for coloring
    facet_count = {}
    for name, rows in by_name.items():
        facet_count[name] = rows[0]["facet_count"]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(5.4, 3.0))

    # Left: sys trajectories colored by facet count
    names_sorted = sorted(by_name.keys(), key=lambda n: facet_count[n])
    cmap = plt.cm.viridis
    fc_values = sorted(set(facet_count.values()))
    vmin = min(fc_values)
    vmax = max(fc_values)
    norm = plt.Normalize(vmin=vmin, vmax=vmax)

    for name in names_sorted:
        rows = by_name[name]
        iters = [0] + [r["iteration"] + 1 for r in rows]
        sys_vals = [rows[0]["sys_before"]] + [r["sys_after"] for r in rows]
        color = cmap(norm(facet_count[name]))
        ax1.plot(iters, sys_vals, alpha=0.4, linewidth=0.8, color=color)

    sm = plt.cm.ScalarMappable(cmap=cmap, norm=norm)
    cbar = fig.colorbar(sm, ax=ax1)
    cbar.set_label("Facet count")
    ax1.set_xlabel("Iteration")
    ax1.set_ylabel("sys")
    ax1.set_title("Convergence trajectories")
    ax1.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.4, label="sys = 1")
    ax1.legend(loc="lower right")
    ax1.grid(True, alpha=0.3)

    # Right: delta_sys per iteration (all polytopes overlaid)
    for name in names_sorted:
        rows = by_name[name]
        iters = [r["iteration"] + 1 for r in rows]
        deltas = [r["delta_sys"] for r in rows]
        color = cmap(norm(facet_count[name]))
        ax2.plot(iters, deltas, alpha=0.4, linewidth=0.8, color=color)

    ax2.set_xlabel("Iteration")
    ax2.set_ylabel(r"$\Delta\mathrm{sys}$ per step")
    ax2.set_title("Per-step improvement")
    ax2.set_yscale("symlog", linthresh=1e-6)
    ax2.axhline(y=0, color="black", linewidth=0.5, alpha=0.5)
    ax2.grid(True, alpha=0.3)

    fig.tight_layout()
    fig.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 5: Iteration summary (Phase 3)
# =============================================================================

def plot_iteration_summary(iter_rows: list[dict], output_path: Path) -> None:
    """Iteration count histogram + starting vs final sys scatter."""
    by_name: dict[str, list[dict]] = defaultdict(list)
    for r in iter_rows:
        by_name[r["name"]].append(r)

    if not by_name:
        print("WARNING: no iteration data to plot", file=sys.stderr)
        return

    for name in by_name:
        by_name[name].sort(key=lambda r: r["iteration"])

    # Per-polytope summary
    names = sorted(by_name.keys())
    n_iters = np.array([len(by_name[n]) for n in names])
    starting = np.array([by_name[n][0]["starting_sys"] for n in names])
    final = np.array([by_name[n][-1]["sys_after"] for n in names])
    cumulative = final - starting

    # Step type counts
    h_count = sum(1 for r in iter_rows if r["step_type"] == "h_only")
    hn_count = sum(1 for r in iter_rows if r["step_type"] == "h_n")

    fig, axes = plt.subplots(1, 3, figsize=(5.4, 2.5))

    # Left: iteration count histogram
    ax = axes[0]
    max_iter = n_iters.max()
    bins = np.arange(0.5, max_iter + 1.5, 1)
    ax.hist(n_iters, bins=bins, color="#3b6ea8", alpha=0.75, edgecolor="white")
    ax.set_xlabel("Iterations to convergence")
    ax.set_ylabel("Count")
    ax.set_title(f"Iteration counts (mean {n_iters.mean():.1f})")
    ax.grid(True, alpha=0.3)

    # Middle: starting vs final sys
    ax = axes[1]
    ax.scatter(starting, final, alpha=0.6, s=25, color="#3b6ea8", zorder=3)
    lim = max(starting.max(), final.max()) * 1.1
    ax.plot([0, lim], [0, lim], "k--", alpha=0.3, label="No improvement")
    ax.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.4, label="sys = 1")
    ax.set_xlabel("sys (initial)")
    ax.set_ylabel("sys (after iteration)")
    ax.set_title("Iterative improvement")
    ax.set_xlim(0, lim)
    ax.set_ylim(0, lim)
    ax.set_aspect("equal")
    ax.legend(loc="upper left")
    ax.grid(True, alpha=0.3)

    # Right: step type pie chart
    ax = axes[2]
    ax.bar(
        ["$h$-only", "$(h,n)$"],
        [h_count, hn_count],
        color=["#3b6ea8", "#27ae60"],
        alpha=0.75,
        edgecolor="white",
    )
    ax.set_ylabel("Steps taken")
    ax.set_title(f"Step type usage ({h_count + hn_count} total)")
    ax.grid(True, alpha=0.3, axis="y")

    fig.tight_layout()
    fig.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Figure 6: Gradient validity testing (Phase 4)
# =============================================================================

def plot_validity(val_rows: list[dict], output_path: Path) -> None:
    """Prediction error vs step fraction, step bound conservativeness."""
    if not val_rows:
        print("WARNING: no validity data to plot", file=sys.stderr)
        return

    ok_rows = [r for r in val_rows if r["construction_ok"]]
    if not ok_rows:
        print("WARNING: no successful validity evaluations", file=sys.stderr)
        return

    fig, axes = plt.subplots(1, 3, figsize=(5.4, 2.5))

    # --- Left: prediction error vs t/t_max by direction type ---
    ax = axes[0]
    fracs = sorted(set(r["t_fraction"] for r in ok_rows))
    colors = {"gradient_h": "#c0392b", "gradient_hn": "#8e44ad", "random": "#3b6ea8"}
    labels = {"gradient_h": r"$\nabla_h$", "gradient_hn": r"$\nabla_{(h,n)}$", "random": "Random"}

    for dtype in ["gradient_h", "gradient_hn", "random"]:
        subset = [r for r in ok_rows if r["direction_type"] == dtype]
        if not subset:
            continue
        medians = []
        q25s = []
        q75s = []
        valid_fracs = []
        for frac in fracs:
            errs = [
                r["relative_error"]
                for r in subset
                if r["t_fraction"] == frac and np.isfinite(r["relative_error"])
            ]
            if len(errs) < 3:
                continue
            earr = np.array(errs)
            valid_fracs.append(frac)
            medians.append(np.median(earr))
            q25s.append(np.percentile(earr, 25))
            q75s.append(np.percentile(earr, 75))

        if valid_fracs:
            vf = np.array(valid_fracs)
            med = np.array(medians)
            ax.plot(vf, med, color=colors[dtype], label=labels[dtype], linewidth=1.5)
            ax.fill_between(
                vf, q25s, q75s, color=colors[dtype], alpha=0.15,
            )

    ax.axvline(x=1.0, color="black", linestyle="--", alpha=0.4, label=r"$t = t_{\max}$")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel(r"$t\,/\,t_{\max}$")
    ax.set_ylabel("Relative prediction error")
    ax.set_title("Gradient prediction accuracy")
    ax.legend(loc="upper left")
    ax.grid(True, alpha=0.3, which="both")

    # --- Middle: construction success rate beyond t_max ---
    ax = axes[1]
    beyond_fracs = sorted(set(r["t_fraction"] for r in val_rows if r["beyond_t_max"]))
    if beyond_fracs:
        success_rates = []
        type_preserved_rates = []
        for frac in beyond_fracs:
            subset = [r for r in val_rows if r["t_fraction"] == frac and r["beyond_t_max"]]
            n_total = len(subset)
            n_ok = sum(1 for r in subset if r["construction_ok"])
            n_type_ok = sum(
                1 for r in subset if r["construction_ok"] and not r["vertex_count_changed"]
            )
            success_rates.append(100 * n_ok / max(n_total, 1))
            type_preserved_rates.append(100 * n_type_ok / max(n_total, 1))

        x = np.arange(len(beyond_fracs))
        width = 0.35
        ax.bar(
            x - width / 2, success_rates, width,
            label="Construction OK", color="#3b6ea8", alpha=0.75,
        )
        ax.bar(
            x + width / 2, type_preserved_rates, width,
            label="Type preserved", color="#27ae60", alpha=0.75,
        )
        ax.set_xticks(x)
        ax.set_xticklabels([f"{f:.0f}×" for f in beyond_fracs])
        ax.set_xlabel(r"Step size ($\times\,t_{\max}$)")
        ax.set_ylabel("Success rate (%)")
        ax.set_title("Step bound conservativeness")
        ax.legend()
        ax.grid(True, alpha=0.3, axis="y")

    # --- Right: validity radius distribution ---
    ax = axes[2]
    # For each (polytope, direction), find largest t_fraction with rel error < 20%
    by_key: dict[tuple[str, str, int], list[dict]] = defaultdict(list)
    for r in ok_rows:
        key = (r["name"], r["direction_type"], r["direction_index"])
        by_key[key].append(r)

    validity_radii = {"gradient_h": [], "gradient_hn": [], "random": []}
    for key, rows in by_key.items():
        rows_sorted = sorted(rows, key=lambda r: r["t_fraction"])
        max_valid_frac = 0.0
        for r in rows_sorted:
            if np.isfinite(r["relative_error"]) and r["relative_error"] < 0.2:
                max_valid_frac = r["t_fraction"]
        dtype = key[1]
        if dtype in validity_radii:
            validity_radii[dtype].append(max_valid_frac)

    all_radii = []
    all_labels = []
    all_colors = []
    for dtype in ["gradient_h", "gradient_hn", "random"]:
        if validity_radii[dtype]:
            all_radii.append(np.array(validity_radii[dtype]))
            all_labels.append(labels[dtype])
            all_colors.append(colors[dtype])

    if all_radii:
        ax.hist(
            all_radii, bins=15, label=all_labels, color=all_colors,
            alpha=0.6, edgecolor="white",
        )
        ax.axvline(x=1.0, color="black", linestyle="--", alpha=0.4)
        ax.set_xlabel(r"Validity radius ($t\,/\,t_{\max}$)")
        ax.set_ylabel("Count")
        ax.set_title("Where gradient breaks down")
        ax.legend()
        ax.grid(True, alpha=0.3)

    fig.tight_layout()
    fig.savefig(output_path, dpi=150, bbox_inches='tight')
    plt.close(fig)
    print(f"Saved: {output_path}")


# =============================================================================
# Stats table (LaTeX)
# =============================================================================

def write_stats_table(
    sens_rows: list[dict],
    steps_rows: list[dict],
    iter_rows: list[dict],
    val_rows: list[dict],
    output_path: Path,
) -> None:
    """Write summary statistics as a LaTeX table."""
    n_polytopes = len(sens_rows)

    # Predicted max Δsys per step type (dimensionless: t_max * ||∇sys||)
    pred_h = [r["t_max_h"] * r["gradient_norm_h"] for r in sens_rows]
    pred_hn = [r["t_max_hn"] * r["gradient_norm_hn"] for r in sens_rows]
    n_with_gradient = sum(1 for p in pred_hn if p > 1e-10)

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

    # Best overall (single step)
    all_new = new_sys_h + new_sys_hn
    max_sys_single = max(all_new) if all_new else 0

    # Phase 3: iteration stats
    by_name: dict[str, list[dict]] = defaultdict(list)
    for r in iter_rows:
        by_name[r["name"]].append(r)
    for name in by_name:
        by_name[name].sort(key=lambda r: r["iteration"])

    n_iterated = len(by_name)
    iter_counts = [len(rows) for rows in by_name.values()]
    mean_iters = np.mean(iter_counts) if iter_counts else 0
    max_iters_used = max(iter_counts) if iter_counts else 0

    # Terminated: last step had delta < threshold (converged)
    # Note: len(rows) < 20 was removed — max iterations is 15, so len(rows) < 20
    # is always true and would make n_terminated always equal n_iterated.
    n_terminated = sum(
        1 for rows in by_name.values()
        if rows[-1]["delta_sys"] < 1e-6
    )

    # Final sys values after iteration
    final_sys = [rows[-1]["sys_after"] for rows in by_name.values()]
    max_sys_iterated = max(final_sys) if final_sys else 0
    cumulative_deltas = [rows[-1]["cumulative_delta"] for rows in by_name.values()]
    mean_cumulative = np.mean(cumulative_deltas) if cumulative_deltas else 0

    # Step type breakdown
    h_steps_iter = sum(1 for r in iter_rows if r["step_type"] == "h_only")
    hn_steps_iter = sum(1 for r in iter_rows if r["step_type"] == "h_n")

    # Phase 4: validity stats
    val_ok = [r for r in val_rows if r["construction_ok"]]
    n_val_total = len(val_rows)
    n_val_ok = len(val_ok)

    # Median relative error at t/t_max = 0.25 (within step bound)
    small_step_errs = [
        r["relative_error"] for r in val_ok
        if abs(r["t_fraction"] - 0.25) < 0.01 and np.isfinite(r["relative_error"])
    ]
    median_err_025 = np.median(small_step_errs) if small_step_errs else float("nan")

    # Median relative error at t/t_max = 1.0 (at step bound)
    bound_errs = [
        r["relative_error"] for r in val_ok
        if abs(r["t_fraction"] - 1.0) < 0.01 and np.isfinite(r["relative_error"])
    ]
    median_err_10 = np.median(bound_errs) if bound_errs else float("nan")

    # Beyond-t_max success rate at 2× and 5×
    beyond_2x = [r for r in val_rows if abs(r["t_fraction"] - 2.0) < 0.01]
    beyond_5x = [r for r in val_rows if abs(r["t_fraction"] - 5.0) < 0.01]
    rate_2x = 100 * sum(1 for r in beyond_2x if r["construction_ok"]) / max(len(beyond_2x), 1)
    rate_5x = 100 * sum(1 for r in beyond_5x if r["construction_ok"]) / max(len(beyond_5x), 1)

    lines = [
        r"\begin{tabular}{lr}",
        r"\toprule",
        r"Statistic & Value \\",
        r"\midrule",
        rf"Polytopes analyzed & {n_polytopes} \\",
        rf"Polytopes with $\|\nabla_{{(h,n)}} \mathrm{{sys}}\| > 0$ & {n_with_gradient} \\",
        r"\midrule",
        rf"Mean predicted max $\Delta\mathrm{{sys}}$ ($h$-only) & {np.mean(pred_h):.4f} \\",
        rf"Mean predicted max $\Delta\mathrm{{sys}}$ ($(h,n)$) & {np.mean(pred_hn):.4f} \\",
        r"\midrule",
        rf"Improved by $h$-only step & {n_improved_h}/{len(h_steps)} \\",
        rf"Improved by $(h,n)$ step & {n_improved_hn}/{len(hn_steps)} \\",
        rf"Mean $\Delta\mathrm{{sys}}$ ($h$-only) & {np.mean(delta_h):.4f} \\",
        rf"Mean $\Delta\mathrm{{sys}}$ ($(h,n)$) & {np.mean(delta_hn):.4f} \\",
        rf"Best sys (single step) & {max_sys_single:.4f} \\",
        r"\midrule",
        rf"Polytopes iterated & {n_iterated} \\",
        rf"Mean iterations & {mean_iters:.1f} \\",
        rf"Terminated ($\Delta < 10^{{-6}}$) & {n_terminated}/{n_iterated} \\",
        rf"Steps: $h$-only / $(h,n)$ & {h_steps_iter} / {hn_steps_iter} \\",
        rf"Mean cumulative $\Delta\mathrm{{sys}}$ & {mean_cumulative:.4f} \\",
        r"\midrule",
        rf"Validity evaluations & {n_val_ok}/{n_val_total} OK \\",
        rf"Median rel.\ error at $0.25\,t_{{\max}}$ & {median_err_025:.4f} \\",
        rf"Median rel.\ error at $t_{{\max}}$ & {median_err_10:.4f} \\",
        rf"Construction OK at $2\times t_{{\max}}$ & {rate_2x:.0f}\% \\",
        rf"Construction OK at $5\times t_{{\max}}$ & {rate_5x:.0f}\% \\",
        r"\midrule",
        rf"Best sys (before) & {max_sys_before:.4f} \\",
        rf"Best sys (after iteration) & {max_sys_iterated:.4f} \\",
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
    iter_rows = load_jsonl(ITERATIONS_PATH)
    val_rows = load_jsonl(VALIDITY_PATH)

    print(
        f"Loaded {len(sens_rows)} sensitivity rows, "
        f"{len(steps_rows)} step rows, "
        f"{len(iter_rows)} iteration rows, "
        f"{len(val_rows)} validity rows\n"
    )

    plot_gradient_histogram(
        sens_rows, EXPERIMENT_DIR / "sys_optimization_gradient_hist.png"
    )
    plot_gradient_comparison(
        sens_rows, EXPERIMENT_DIR / "sys_optimization_gradient_comparison.png"
    )
    plot_improvement(
        steps_rows, EXPERIMENT_DIR / "sys_optimization_improvement.png"
    )
    plot_convergence(
        iter_rows, EXPERIMENT_DIR / "sys_optimization_convergence.png"
    )
    plot_iteration_summary(
        iter_rows, EXPERIMENT_DIR / "sys_optimization_iteration_summary.png"
    )
    plot_validity(
        val_rows, EXPERIMENT_DIR / "sys_optimization_validity.png"
    )
    write_stats_table(
        sens_rows, steps_rows, iter_rows, val_rows,
        EXPERIMENT_DIR / "sys_optimization_stats.tex",
    )


if __name__ == "__main__":
    main()
