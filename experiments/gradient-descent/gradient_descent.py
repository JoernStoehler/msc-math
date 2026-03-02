#!/usr/bin/env python3
"""
Analyze gradient ascent results on F=10 polytopes.

Goal: Visualize the distribution of final sys values, compare general vs Lagrangian products.
Input: experiments/gradient-descent/gradient-descent.jsonl
Output: experiments/gradient-descent/gradient_descent_results.png
        experiments/gradient-descent/gradient_descent_scatter.png
"""

import json
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "gradient-descent.jsonl"


def load_data():
    """Load JSONL and compute per-polytope summaries."""
    rows = []
    with open(DATA_PATH) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))

    if not rows:
        print(f"No data found in {DATA_PATH}. File exists but is empty or corrupt.")
        print("Rerun: cd experiments/ && cargo run --release --bin gradient_descent")
        return None

    # Group by polytope name
    by_name = defaultdict(list)
    for row in rows:
        by_name[row["name"]].append(row)

    summaries = []
    for name, iterations in by_name.items():
        iterations.sort(key=lambda r: r["iteration"])
        first = iterations[0]
        last = iterations[-1]
        summaries.append(
            {
                "name": name,
                "polytope_type": first["polytope_type"],
                "starting_sys": first["starting_sys"],
                "final_sys": last["sys_after"],
                "total_delta": last["sys_after"] - first["starting_sys"],
                "iterations": len(iterations),
            }
        )

    return summaries


def plot_histogram(summaries):
    """Histogram of final sys values, colored by polytope type."""
    fig, ax = plt.subplots(1, 1, figsize=(10, 6))

    # Separate by type
    general = [s for s in summaries if s["polytope_type"] == "general"]
    lagrangian = [s for s in summaries if s["polytope_type"] != "general"]

    gen_sys = [s["final_sys"] for s in general if np.isfinite(s["final_sys"])]
    lag_sys = [s["final_sys"] for s in lagrangian if np.isfinite(s["final_sys"])]

    bins = np.linspace(0, max(max(gen_sys, default=0), max(lag_sys, default=0)) * 1.05, 40)

    ax.hist(gen_sys, bins=bins, alpha=0.6, label=f"General (n={len(gen_sys)})", color="steelblue")
    ax.hist(
        lag_sys, bins=bins, alpha=0.6, label=f"Lagrangian (n={len(lag_sys)})", color="coral"
    )

    ax.axvline(x=1.0, color="red", linestyle="--", linewidth=1.5, label="sys = 1")
    ax.set_xlabel("Final sys after gradient ascent")
    ax.set_ylabel("Count")
    ax.set_title("Distribution of final systolic ratios (F=10)")
    ax.legend()

    out = EXPERIMENT_DIR / "gradient_descent_results.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved {out}")


def plot_scatter(summaries):
    """Scatter: starting sys vs final sys."""
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

    # Diagonal
    lims = [0, max(ax.get_xlim()[1], ax.get_ylim()[1])]
    ax.plot(lims, lims, "k--", alpha=0.3, linewidth=0.5)
    ax.axhline(y=1.0, color="red", linestyle="--", linewidth=1, alpha=0.5, label="sys = 1")

    ax.set_xlabel("Starting sys")
    ax.set_ylabel("Final sys (after gradient ascent)")
    ax.set_title("Gradient ascent improvement (F=10)")
    ax.legend()
    ax.set_aspect("equal")

    out = EXPERIMENT_DIR / "gradient_descent_scatter.png"
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
            f"{t:<20} {len(group):>5} {np.mean(finals):>10.6f} {np.max(finals):>10.6f} "
            f"{np.percentile(finals, 90):>10.6f} {np.mean(deltas):>10.6f}"
        )

    print("=" * 80)

    # Highlight any sys > 0.9
    high = [s for s in summaries if np.isfinite(s["final_sys"]) and s["final_sys"] > 0.9]
    if high:
        print(f"\nPolytopes with final sys > 0.9 ({len(high)}):")
        for s in sorted(high, key=lambda x: -x["final_sys"])[:10]:
            print(
                f"  {s['name']:<30} type={s['polytope_type']:<15} "
                f"sys: {s['starting_sys']:.6f} → {s['final_sys']:.6f}"
            )

    # Any sys > 1?
    above_one = [s for s in summaries if np.isfinite(s["final_sys"]) and s["final_sys"] > 1.0]
    if above_one:
        print(f"\n*** COUNTEREXAMPLES FOUND: {len(above_one)} polytopes with sys > 1 ***")
        for s in sorted(above_one, key=lambda x: -x["final_sys"]):
            print(
                f"  {s['name']:<30} type={s['polytope_type']:<15} "
                f"sys = {s['final_sys']:.10f}"
            )
    else:
        print("\nNo polytopes achieved sys > 1.")


def main():
    if not DATA_PATH.exists():
        print(f"Data not found: {DATA_PATH}")
        print("Run: cd experiments/ && cargo run --release --bin gradient_descent")
        return

    summaries = load_data()
    if summaries is None:
        return

    print(f"Loaded {len(summaries)} polytope trajectories.")
    plot_histogram(summaries)
    plot_scatter(summaries)
    print_summary_table(summaries)


if __name__ == "__main__":
    main()
