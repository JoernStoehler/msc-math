"""
Goal: Analyze dense perturbation sweep around HKO2024 in Lagrangian product space.
Input: crates/exp-hko-local-maximum/lagrangian-boundary/lagrangian-search.jsonl (per-sample data)
       crates/exp-hko-local-maximum/lagrangian-boundary/lagrangian-search-levels.jsonl (per-level summary)
Output: crates/exp-hko-local-maximum/lagrangian-boundary/lagrangian_search_fraction.png (fraction sys>1 vs epsilon)
        crates/exp-hko-local-maximum/lagrangian-boundary/lagrangian_search_sys_vs_eps.png (sys distribution per epsilon)
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import (
    FIGSIZE_SINGLE,
    LINE_WIDTH,
    MARKER_SIZE,
    SCATTER_SIZE,
    setup,
)

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent


def load_levels():
    """Load per-level summary data."""
    rows = []
    with open(EXPERIMENT_DIR / "lagrangian-search-levels.jsonl") as f:
        for line in f:
            rows.append(json.loads(line))
    return rows


def load_samples():
    """Load per-sample data."""
    rows = []
    with open(EXPERIMENT_DIR / "lagrangian-search.jsonl") as f:
        for line in f:
            rows.append(json.loads(line))
    return rows


def plot_fraction_curve(levels):
    """Plot fraction of samples with sys > 1 vs perturbation epsilon."""
    epsilons = [r["epsilon"] for r in levels]
    fracs = [r["frac_above_1"] for r in levels]
    n_accepted = [r["n_accepted"] for r in levels]

    # Wilson score 95% confidence interval
    ci_lo = []
    ci_hi = []
    z = 1.96
    for frac, n in zip(fracs, n_accepted):
        if n == 0:
            ci_lo.append(0.0)
            ci_hi.append(0.0)
            continue
        denom = 1 + z**2 / n
        centre = (frac + z**2 / (2 * n)) / denom
        spread = z * np.sqrt((frac * (1 - frac) + z**2 / (4 * n)) / n) / denom
        ci_lo.append(max(0.0, centre - spread))
        ci_hi.append(min(1.0, centre + spread))

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.plot(
        epsilons,
        fracs,
        "o-",
        color="C0",
        markersize=MARKER_SIZE,
        linewidth=LINE_WIDTH,
        zorder=3,
    )
    ax.fill_between(
        epsilons, ci_lo, ci_hi, alpha=0.2, color="C0", label="95% Wilson CI"
    )
    ax.axhline(y=0.5, color="gray", linestyle="--", alpha=0.5, linewidth=0.8)
    ax.set_xlabel(r"Perturbation $\varepsilon$ (per component)")
    ax.set_ylabel(r"Fraction with $\mathrm{sys} > 1$")
    ax.set_xscale("log")
    ax.set_ylim(-0.05, 1.05)
    ax.legend(loc="upper right")

    out = EXPERIMENT_DIR / "lagrangian_search_fraction.png"
    fig.savefig(out)
    plt.close(fig)
    print(f"Wrote {out}")


def plot_sys_vs_epsilon(samples):
    """Plot sys distribution per epsilon level as strip plot with summary stats."""
    # Group by epsilon (skip base row)
    groups = {}
    for row in samples:
        if row["is_base"]:
            continue
        eps = row["epsilon"]
        groups.setdefault(eps, []).append(row["sys"])

    epsilons = sorted(groups.keys())
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    # Strip plot: jitter x around epsilon, plot sys
    for eps in epsilons:
        sys_vals = np.array(groups[eps])
        # Subsample for visual clarity if too many points
        if len(sys_vals) > 200:
            idx = np.random.default_rng(42).choice(len(sys_vals), 200, replace=False)
            sys_plot = sys_vals[idx]
        else:
            sys_plot = sys_vals
        jitter = np.random.default_rng(42).uniform(-0.1, 0.1, len(sys_plot))
        x = np.log10(eps) + jitter * 0.05
        ax.scatter(x, sys_plot, s=SCATTER_SIZE * 0.3, alpha=0.3, color="C0", zorder=2)
        # Median line
        med = np.median(sys_vals)
        ax.plot(
            [np.log10(eps) - 0.03, np.log10(eps) + 0.03],
            [med, med],
            color="C1",
            linewidth=2,
            zorder=4,
        )

    ax.axhline(y=1.0, color="red", linestyle="--", alpha=0.6, linewidth=0.8)
    ax.set_xlabel(r"Perturbation $\varepsilon$ (per component)")
    ax.set_ylabel(r"Systolic ratio $\mathrm{sys}$")
    ax.set_xticks([np.log10(e) for e in epsilons])
    ax.set_xticklabels([f"{e}" for e in epsilons], rotation=45, ha="right")

    # Mark HKO base value
    ax.axhline(
        y=1.047214, color="C2", linestyle=":", alpha=0.5, linewidth=0.8, label="HKO2024"
    )
    ax.legend(loc="lower left")

    out = EXPERIMENT_DIR / "lagrangian_search_sys_vs_eps.png"
    fig.savefig(out)
    plt.close(fig)
    print(f"Wrote {out}")


def load_probes():
    """Load directional probe data."""
    path = EXPERIMENT_DIR / "lagrangian-probe.jsonl"
    if not path.exists():
        return None
    rows = []
    with open(path) as f:
        for line in f:
            rows.append(json.loads(line))
    return rows


def plot_radius_histogram(probes):
    """Plot histogram of boundary radii from directional probing."""
    radii = np.array([p["radius"] for p in probes if p["success"]])
    if len(radii) == 0:
        return

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.hist(radii, bins=40, color="C0", alpha=0.7, edgecolor="C0")
    ax.axvline(
        x=np.median(radii),
        color="C1",
        linestyle="-",
        linewidth=LINE_WIDTH,
        label=f"median = {np.median(radii):.4f}",
    )
    ax.axvline(
        x=np.mean(radii),
        color="C3",
        linestyle="--",
        linewidth=LINE_WIDTH,
        label=f"mean = {np.mean(radii):.4f}",
    )
    ax.set_xlabel(r"Boundary radius $r(\mathbf{u})$ (L2 norm)")
    ax.set_ylabel("Count")
    ax.legend()

    out = EXPERIMENT_DIR / "lagrangian_probe_radii.png"
    fig.savefig(out)
    plt.close(fig)
    print(f"Wrote {out}")


def print_summary_table(levels):
    """Print a summary table for the logbook."""
    print("\n## Summary table\n")
    print(
        f"{'epsilon':>8} {'accepted':>8} {'attempts':>8} {'acc%':>8} "
        f"{'sys>1':>6} {'frac%':>8} {'sys_min':>8} {'sys_mean':>8} "
        f"{'sys_max':>8} {'sys_std':>8}"
    )
    for r in levels:
        print(
            f"{r['epsilon']:>8.2f} {r['n_accepted']:>8d} {r['n_attempts']:>8d} "
            f"{r['accept_rate']*100:>7.1f}% {r['n_above_1']:>6d} "
            f"{r['frac_above_1']*100:>7.1f}% {r['sys_min']:>8.4f} "
            f"{r['sys_mean']:>8.4f} {r['sys_max']:>8.4f} {r['sys_std']:>8.4f}"
        )


if __name__ == "__main__":
    levels = load_levels()
    samples = load_samples()

    plot_fraction_curve(levels)
    plot_sys_vs_epsilon(samples)
    print_summary_table(levels)

    probes = load_probes()
    if probes is not None:
        plot_radius_histogram(probes)
