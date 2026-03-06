#!/usr/bin/env python3
"""
Analyze gradient ascent results on F=10 polytopes.

Goal: Visualize gradient ascent outcomes and diagnose convergence behavior.
Input: experiments/gradient-descent/gradient-descent.jsonl
Output:
  - gradient_descent_scatter.png      (starting vs final sys, by class)
  - gradient_descent_convergence.png  (gradient norms + step size dynamics)
"""

import json
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

# Consistent font sizes for \includegraphics[width=\textwidth] figures
plt.rcParams.update({
    "axes.labelsize": 14,
    "axes.titlesize": 14,
    "xtick.labelsize": 11,
    "ytick.labelsize": 11,
    "legend.fontsize": 11,
    "figure.titlesize": 15,
})

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "gradient-descent.jsonl"


def load_data():
    """Load JSONL, group by polytope name, compute per-polytope summaries."""
    rows = []
    with open(DATA_PATH) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))

    if not rows:
        print(f"No data in {DATA_PATH}. Run: cd experiments/ && cargo run --release --bin gradient_descent")
        return None, None, None

    by_name = defaultdict(list)
    for row in rows:
        by_name[row["name"]].append(row)

    for name in by_name:
        by_name[name].sort(key=lambda r: r["iteration"])

    summaries = []
    for name, iterations in by_name.items():
        first = iterations[0]
        last = iterations[-1]
        summaries.append({
            "name": name,
            "polytope_type": first["polytope_type"],
            "starting_sys": first["starting_sys"],
            "final_sys": last["sys_after"],
            "total_delta": last["sys_after"] - first["starting_sys"],
            "iterations": len(iterations),
        })

    return rows, by_name, summaries


def plot_scatter(summaries):
    """Scatter: starting sys vs final sys, colored by polytope class."""
    fig, ax = plt.subplots(1, 1, figsize=(8, 8))

    general = [s for s in summaries if s["polytope_type"] == "general"]
    lagrangian = [s for s in summaries if s["polytope_type"] != "general"]

    for group, label, color, marker in [
        (general, "General", "steelblue", "o"),
        (lagrangian, "Lagrangian", "coral", "^"),
    ]:
        x = [s["starting_sys"] for s in group if np.isfinite(s["final_sys"])]
        y = [s["final_sys"] for s in group if np.isfinite(s["final_sys"])]
        ax.scatter(x, y, alpha=0.4, label=label, color=color, marker=marker, s=15)

    lims = [0, max(ax.get_xlim()[1], ax.get_ylim()[1])]
    ax.plot(lims, lims, "k--", alpha=0.3, linewidth=0.5)
    ax.axhline(y=1.0, color="red", linestyle="--", linewidth=1, alpha=0.5, label="sys = 1")

    ax.set_xlabel("Starting sys")
    ax.set_ylabel("Final sys (after gradient ascent)")
    ax.set_title("Gradient ascent improvement (F = 10)")
    ax.legend()
    ax.set_aspect("equal")

    out = EXPERIMENT_DIR / "gradient_descent_scatter.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved {out}")


def plot_convergence(rows, by_name):
    """Combined figure: gradient norms at termination + step size dynamics."""
    fig, axes = plt.subplots(1, 3, figsize=(16, 5.5))

    # Panel 1: height gradient norm vs final sys
    ax = axes[0]
    for ptype, color, marker in [
        ("general", "steelblue", "o"),
        ("lagrangian", "coral", "^"),
    ]:
        final_sys = []
        final_grad = []
        hit_maxiter = []

        for name, iters in by_name.items():
            if ptype == "general" and iters[0]["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and iters[0]["polytope_type"] == "general":
                continue
            last = iters[-1]
            final_sys.append(last["sys_after"])
            final_grad.append(last["gradient_norm_h"])
            hit_maxiter.append(len(iters) >= 20)

        final_sys = np.array(final_sys)
        final_grad = np.array(final_grad)
        hit_maxiter = np.array(hit_maxiter)

        conv = ~hit_maxiter
        ax.scatter(final_sys[conv], final_grad[conv],
                   alpha=0.3, color=color, marker=marker, s=12,
                   label=f"{ptype} (conv.)")
        if hit_maxiter.any():
            ax.scatter(final_sys[hit_maxiter], final_grad[hit_maxiter],
                       alpha=0.7, color=color, marker="x", s=30,
                       label=f"{ptype} (max iter)")

    ax.set_xlabel("Final sys")
    ax.set_ylabel(r"$\|\nabla_h\,\mathrm{sys}\|$ at termination")
    ax.set_yscale("log")
    ax.set_title(r"Residual gradient $\|\nabla_h\,\mathrm{sys}\|$")
    ax.legend()

    # Panel 2: step size evolution (median with IQR)
    ax = axes[1]
    for ptype, color in [("general", "steelblue"), ("lagrangian", "coral")]:
        by_iter = defaultdict(list)
        for name, iters in by_name.items():
            if ptype == "general" and iters[0]["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and iters[0]["polytope_type"] == "general":
                continue
            for row in iters:
                by_iter[row["iteration"]].append(row["t_actual"])

        iters_sorted = sorted(by_iter.keys())
        medians = [np.median(by_iter[i]) for i in iters_sorted]
        p25 = [np.percentile(by_iter[i], 25) for i in iters_sorted]
        p75 = [np.percentile(by_iter[i], 75) for i in iters_sorted]

        ax.plot(iters_sorted, medians, color=color, marker=".", markersize=4,
                label=f"{ptype} median")
        ax.fill_between(iters_sorted, p25, p75, color=color, alpha=0.15)

    ax.set_yscale("log")
    ax.set_xlabel("Iteration")
    ax.set_ylabel("Step size $t$")
    ax.set_title("Step size shrinks exponentially")
    ax.legend()

    # Panel 3: survival curve (active polytopes per iteration)
    ax = axes[2]
    for ptype, color in [("general", "steelblue"), ("lagrangian", "coral")]:
        by_iter = defaultdict(int)
        for name, iters in by_name.items():
            if ptype == "general" and iters[0]["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and iters[0]["polytope_type"] == "general":
                continue
            for row in iters:
                by_iter[row["iteration"]] += 1

        iters_sorted = sorted(by_iter.keys())
        counts = [by_iter[i] for i in iters_sorted]
        ax.plot(iters_sorted, counts, color=color, marker=".", markersize=4, label=ptype)

    ax.set_xlabel("Iteration")
    ax.set_ylabel("Active polytopes")
    ax.set_title("Convergence survival curve")
    ax.legend()

    fig.suptitle("Convergence diagnostics (F = 10)", y=1.02)
    fig.tight_layout()

    out = EXPERIMENT_DIR / "gradient_descent_convergence.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved {out}")


def print_summary_table(summaries):
    """Print summary statistics by polytope type."""
    types = sorted(set(s["polytope_type"] for s in summaries))

    print("\n" + "=" * 80)
    print(f"{'Type':<20} {'N':>5} {'Mean sys':>10} {'Max sys':>10} {'P90 sys':>10} {'Mean Δ':>10}")
    print("-" * 80)

    for t in types:
        group = [s for s in summaries if s["polytope_type"] == t and np.isfinite(s["final_sys"])]
        if not group:
            continue
        finals = [s["final_sys"] for s in group]
        deltas = [s["total_delta"] for s in group]
        print(
            f"{t:<20} {len(group):>5} {np.mean(finals):>10.3f} {np.max(finals):>10.3f} "
            f"{np.percentile(finals, 90):>10.3f} {np.mean(deltas):>10.3f}"
        )

    print("=" * 80)

    # Any sys > 1?
    above_one = [s for s in summaries if np.isfinite(s["final_sys"]) and s["final_sys"] > 1.0]
    if above_one:
        print(f"\n*** COUNTEREXAMPLES FOUND: {len(above_one)} polytopes with sys > 1 ***")
        for s in sorted(above_one, key=lambda x: -x["final_sys"]):
            print(f"  {s['name']:<30} type={s['polytope_type']:<15} sys = {s['final_sys']:.10f}")
    else:
        print("\nNo polytopes achieved sys > 1.")

    # Convergence quality (reproduces claims in .tex)
    print("\nConvergence quality:")
    for ptype in ["general", "lagrangian"]:
        n_total = 0
        for s in summaries:
            if ptype == "general" and s["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and s["polytope_type"] == "general":
                continue
            n_total += 1
        print(f"  {ptype}: {n_total} polytopes")


def print_convergence_stats(by_name, rows):
    """Print statistics cited in the .tex: correlation, gradient norms, fraction choice."""
    # Collect final gradient norms and sys for correlation
    final_sys_all = []
    final_grad_h_all = []
    for ptype in ["general", "lagrangian"]:
        final_sys = []
        final_gh = []
        final_gn = []
        for name, iters in by_name.items():
            if ptype == "general" and iters[0]["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and iters[0]["polytope_type"] == "general":
                continue
            last = iters[-1]
            final_sys.append(last["sys_after"])
            final_gh.append(last["gradient_norm_h"])
            final_gn.append(last["gradient_norm_n"])
        final_sys_all.extend(final_sys)
        final_grad_h_all.extend(final_gh)
        gh = np.array(final_gh)
        gn = np.array(final_gn)
        print(f"\n  {ptype}:")
        print(f"    median ||grad_h||: {np.median(gh):.2f}")
        print(f"    median ||grad_n||: {np.median(gn):.1f}")

    r = np.corrcoef(final_sys_all, final_grad_h_all)[0, 1]
    print(f"\n  Correlation(final_sys, ||grad_h||): r = {r:.2f}")

    # Fraction choosing 0.95 * t_max
    n_095 = sum(1 for row in rows if row["t_fraction"] == 0.95)
    print(f"  Fraction choosing 0.95*t_max: {n_095}/{len(rows)} = {100*n_095/len(rows):.1f}%")


def main():
    if not DATA_PATH.exists():
        print(f"Data not found: {DATA_PATH}")
        print("Run: cd experiments/ && cargo run --release --bin gradient_descent")
        return

    rows, by_name, summaries = load_data()
    if rows is None:
        return

    print(f"Loaded {len(summaries)} polytope trajectories, {len(rows)} iteration rows.")
    plot_scatter(summaries)
    plot_convergence(rows, by_name)
    print_summary_table(summaries)
    print_convergence_stats(by_name, rows)


if __name__ == "__main__":
    main()
