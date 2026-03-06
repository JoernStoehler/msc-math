#!/usr/bin/env python3
"""
Extended analysis of gradient ascent trajectories.

Goal: Understand convergence quality, density shifts, and step size dynamics.
Input: experiments/gradient-descent/gradient-descent.jsonl
Output:
  - gradient_descent_density.png    (KDE: starting vs final, general vs lagrangian)
  - gradient_descent_convergence.png (final gradient norms vs final sys)
  - gradient_descent_stepsize.png   (step size and fraction evolution by iteration)
"""

import json
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import gaussian_kde

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "gradient-descent.jsonl"


def load_raw():
    """Load all JSONL rows and group by polytope name."""
    rows = []
    with open(DATA_PATH) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))

    if not rows:
        print(f"No data in {DATA_PATH}. Run: cd experiments/ && cargo run --release --bin gradient_descent")
        return None, None

    by_name = defaultdict(list)
    for row in rows:
        by_name[row["name"]].append(row)

    # Sort each polytope's iterations
    for name in by_name:
        by_name[name].sort(key=lambda r: r["iteration"])

    return rows, by_name


def plot_density(by_name):
    """KDE density: 4 overlaid curves for (general/lagrangian) x (starting/final)."""
    fig, ax = plt.subplots(1, 1, figsize=(10, 6))

    groups = {
        "General starting": [],
        "General final": [],
        "Lagrangian starting": [],
        "Lagrangian final": [],
    }

    for name, iters in by_name.items():
        ptype = iters[0]["polytope_type"]
        s = iters[0]["starting_sys"]
        f = iters[-1]["sys_after"]
        if not (np.isfinite(s) and np.isfinite(f)):
            continue
        if ptype == "general":
            groups["General starting"].append(s)
            groups["General final"].append(f)
        else:
            groups["Lagrangian starting"].append(s)
            groups["Lagrangian final"].append(f)

    x = np.linspace(0, 1.05, 500)
    styles = {
        "General starting": ("steelblue", "--", 1.0),
        "General final": ("steelblue", "-", 2.0),
        "Lagrangian starting": ("coral", "--", 1.0),
        "Lagrangian final": ("coral", "-", 2.0),
    }

    for label, vals in groups.items():
        color, ls, lw = styles[label]
        kde = gaussian_kde(vals, bw_method=0.12)
        kde_vals = kde(x)
        ax.plot(x, kde_vals, color=color, linestyle=ls, linewidth=lw, label=label)

    ax.axvline(x=1.0, color="red", linestyle=":", linewidth=1, alpha=0.5, label="sys = 1")
    ax.set_xlabel("sys")
    ax.set_ylabel("Density")
    ax.set_title("Distribution shift under gradient ascent (F = 10)")
    ax.legend()
    ax.set_xlim(0, 1.05)

    out = EXPERIMENT_DIR / "gradient_descent_density.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved {out}")


def plot_convergence(by_name):
    """Final gradient norms vs final sys, colored by convergence reason."""
    fig, axes = plt.subplots(1, 2, figsize=(14, 6))

    for ax, (grad_key, grad_label) in zip(axes, [
        ("gradient_norm_h", "Height gradient ||grad_h||"),
        ("gradient_norm_n", "Normal gradient ||grad_n||"),
    ]):
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
                final_grad.append(last[grad_key])
                hit_maxiter.append(len(iters) >= 20)

            final_sys = np.array(final_sys)
            final_grad = np.array(final_grad)
            hit_maxiter = np.array(hit_maxiter)

            # Converged (delta < threshold before max iters)
            conv = ~hit_maxiter
            ax.scatter(
                final_sys[conv], final_grad[conv],
                alpha=0.3, color=color, marker=marker, s=12,
                label=f"{ptype} (converged)" if ptype == "general" else f"{ptype} (conv.)",
            )
            # Hit max iterations
            if hit_maxiter.any():
                ax.scatter(
                    final_sys[hit_maxiter], final_grad[hit_maxiter],
                    alpha=0.7, color=color, marker="x", s=30,
                    label=f"{ptype} (max iter)",
                )

        ax.set_xlabel("Final sys")
        ax.set_ylabel(grad_label)
        ax.set_yscale("log")
        ax.legend(fontsize=8)

    axes[0].set_title("Height gradient at termination")
    axes[1].set_title("Normal gradient at termination")
    fig.suptitle("Gradient norms at termination vs final sys (F = 10)", fontsize=13)
    fig.tight_layout()

    out = EXPERIMENT_DIR / "gradient_descent_convergence.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved {out}")


def plot_stepsize(rows, by_name):
    """Step size evolution: median t_actual and fraction choice by iteration."""
    fig, axes = plt.subplots(1, 3, figsize=(16, 5))

    # Panel 1: median t_actual by iteration (log scale)
    ax = axes[0]
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
        counts = [len(by_iter[i]) for i in iters_sorted]

        ax.plot(iters_sorted, medians, color=color, marker=".", markersize=4, label=f"{ptype} median")
        ax.fill_between(iters_sorted, p25, p75, color=color, alpha=0.15)

    ax.set_yscale("log")
    ax.set_xlabel("Iteration")
    ax.set_ylabel("Step size t_actual")
    ax.set_title("Step size shrinks exponentially")
    ax.legend(fontsize=8)

    # Panel 2: fraction of 0.95 chosen per iteration
    ax = axes[1]
    for ptype, color in [("general", "steelblue"), ("lagrangian", "coral")]:
        by_iter = defaultdict(list)
        for name, iters in by_name.items():
            if ptype == "general" and iters[0]["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and iters[0]["polytope_type"] == "general":
                continue
            for row in iters:
                by_iter[row["iteration"]].append(row["t_fraction"])

        iters_sorted = sorted(by_iter.keys())
        frac_95 = [np.mean([1 if f == 0.95 else 0 for f in by_iter[i]]) for i in iters_sorted]
        counts = [len(by_iter[i]) for i in iters_sorted]

        ax.plot(iters_sorted, frac_95, color=color, marker=".", markersize=4, label=ptype)

    ax.set_xlabel("Iteration")
    ax.set_ylabel("Fraction choosing t = 0.95 * t_max")
    ax.set_title("Line search consistently picks max fraction")
    ax.set_ylim(0, 1.05)
    ax.legend(fontsize=8)

    # Panel 3: polytope count (survival curve)
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
    ax.legend(fontsize=8)

    fig.suptitle("Step size dynamics (F = 10)", fontsize=13, y=1.02)
    fig.tight_layout()

    out = EXPERIMENT_DIR / "gradient_descent_stepsize.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved {out}")


def print_convergence_summary(by_name):
    """Print convergence quality statistics."""
    print("\n" + "=" * 80)
    print("CONVERGENCE QUALITY SUMMARY")
    print("=" * 80)

    for ptype in ["general", "lagrangian"]:
        final_gh = []
        final_gn = []
        final_delta = []
        n_maxiter = 0
        n_total = 0

        for name, iters in by_name.items():
            if ptype == "general" and iters[0]["polytope_type"] != "general":
                continue
            if ptype == "lagrangian" and iters[0]["polytope_type"] == "general":
                continue
            last = iters[-1]
            final_gh.append(last["gradient_norm_h"])
            final_gn.append(last["gradient_norm_n"])
            final_delta.append(last["delta_sys"])
            if len(iters) >= 20:
                n_maxiter += 1
            n_total += 1

        gh = np.array(final_gh)
        gn = np.array(final_gn)
        delta = np.array(final_delta)

        print(f"\n{ptype} (N={n_total}, hit max_iter={n_maxiter}):")
        print(f"  Final gradient norms:")
        print(f"    ||grad_h||: median={np.median(gh):.3e}, mean={np.mean(gh):.3e}")
        print(f"    ||grad_n||: median={np.median(gn):.3e}, mean={np.mean(gn):.3e}")
        print(f"  Final delta_sys:")
        print(f"    median={np.median(delta):.3e}, P90={np.percentile(delta, 90):.3e}")
        print(f"    < 1e-6: {np.sum(delta < 1e-6)} ({100*np.mean(delta < 1e-6):.0f}%)")

    # Key interpretation
    print("\n" + "-" * 80)
    print("INTERPRETATION:")
    print("  Gradient norms are O(1) at termination — NOT near-zero local maxima.")
    print("  The algorithm stops because t_max shrinks (combinatorial type boundary),")
    print("  not because the gradient vanishes. Polytopes are step-bound-limited.")
    print("=" * 80)


def main():
    if not DATA_PATH.exists():
        print(f"Data not found: {DATA_PATH}")
        print("Run: cd experiments/ && cargo run --release --bin gradient_descent")
        return

    rows, by_name = load_raw()
    if rows is None:
        return

    print(f"Loaded {len(by_name)} polytope trajectories, {len(rows)} iteration rows.")

    plot_density(by_name)
    plot_convergence(by_name)
    plot_stepsize(rows, by_name)
    print_convergence_summary(by_name)


if __name__ == "__main__":
    main()
