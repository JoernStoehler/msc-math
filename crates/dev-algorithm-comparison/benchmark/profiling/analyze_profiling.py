#!/usr/bin/env python3
"""
Analyze criterion benchmark results for pipeline phase breakdown.

Goal: Show where wall-clock time goes at each facet count (construction vs
      capacity vs volume), and how the balance shifts as F grows.
Input: crates/target/criterion/*/N/new/estimates.json (criterion output)
Output: crates/dev-algorithm-comparison/benchmark/profiling/phase_breakdown.png
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent

sys.path.insert(0, str(REPO_ROOT / "crates"))
from figure_config import setup, FIGSIZE_SINGLE, FIGSIZE_DUAL

setup()

CRITERION_DIR = REPO_ROOT / "crates" / "target" / "criterion"

# Benchmark groups and their display names
GROUPS = {
    "construction": "Construction\n(exact rational)",
    "capacity": "EHZ capacity\n(HK2017 pruned)",
    "volume": "Volume\n(qhull)",
    "transition_matrix": "Transition matrix",
    "kkt_single": "Single KKT solve",
    "pruning_check": "Pruning check",
}

FACET_COUNTS = [5, 6, 7, 8, 9, 10, 11]


def read_criterion_median(group: str, f: int) -> float | None:
    """Read median time in seconds from criterion estimates.json."""
    path = CRITERION_DIR / group / str(f) / "new" / "estimates.json"
    if not path.exists():
        print(f"  Warning: {path} not found", file=sys.stderr)
        return None
    with open(path) as fh:
        data = json.load(fh)
    # Criterion stores times in nanoseconds.
    return data["median"]["point_estimate"] * 1e-9


def read_all_data() -> dict[str, dict[int, float]]:
    """Read median times for all groups and facet counts."""
    results = {}
    for group in GROUPS:
        results[group] = {}
        for f in FACET_COUNTS:
            t = read_criterion_median(group, f)
            if t is not None:
                results[group][f] = t
    return results


def print_summary_table(data: dict[str, dict[int, float]]) -> None:
    """Print a summary table to stdout."""
    print("\nPhase breakdown (median, seconds):")
    print(f"{'F':>3}", end="")
    for group in ["construction", "capacity", "volume"]:
        print(f"  {group:>14}", end="")
    print(f"  {'total':>10}  {'construction%':>14}")

    for f in FACET_COUNTS:
        c = data["construction"].get(f, 0)
        cap = data["capacity"].get(f, 0)
        v = data["volume"].get(f, 0)
        total = c + cap + v
        pct = 100 * c / total if total > 0 else 0
        print(
            f"{f:3d}  {c:14.6f}  {cap:14.6f}  {v:14.6f}  {total:10.6f}  {pct:13.1f}%"
        )


def plot_phase_breakdown(data: dict[str, dict[int, float]]) -> None:
    """Create a two-panel figure: absolute timing (log) and phase proportion."""
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    phases = ["construction", "capacity", "volume"]
    colors = ["#2196F3", "#FF9800", "#4CAF50"]
    markers = ["o", "s", "^"]
    labels = ["Construction", "Capacity", "Volume"]

    # Left panel: absolute times (log scale)
    handles = []
    for phase, color, marker, label in zip(phases, colors, markers, labels):
        times = [data[phase].get(f, np.nan) for f in FACET_COUNTS]
        h, = ax1.semilogy(FACET_COUNTS, [t * 1000 for t in times],
                          color=color, marker=marker, label=label)
        handles.append(h)

    ax1.set_xlabel(r"Facet count $F$")
    ax1.set_ylabel("Time (ms)")
    ax1.set_title("Absolute timing")
    ax1.set_xticks(FACET_COUNTS)

    # Right panel: fraction of total (log scale to show small fractions)
    for phase, color, marker in zip(phases, colors, markers):
        fractions = []
        for f in FACET_COUNTS:
            total = sum(data[p].get(f, 0) for p in phases)
            frac = data[phase].get(f, 0) / total if total > 0 else 0
            fractions.append(100 * frac)
        ax2.semilogy(FACET_COUNTS, fractions, color=color, marker=marker)

    ax2.set_xlabel(r"Facet count $F$")
    ax2.set_ylabel("Fraction of total (%)")
    ax2.set_title("Phase proportion")
    ax2.set_xticks(FACET_COUNTS)
    ax2.set_ylim(0.1, 100)
    ax2.set_yticks([0.1, 1, 10, 100])
    ax2.set_yticklabels(["0.1", "1", "10", "100"])

    # Shared legend between panels
    fig.legend(handles, labels, loc="lower center", ncol=3,
               bbox_to_anchor=(0.5, -0.02))
    fig.tight_layout()
    fig.subplots_adjust(bottom=0.18)

    out_path = EXPERIMENT_DIR / "phase_breakdown.png"
    fig.savefig(out_path)
    print(f"\nSaved: {out_path}")
    plt.close(fig)


def plot_micro_benchmarks(data: dict[str, dict[int, float]]) -> None:
    """Create a figure for the micro-level benchmarks (KKT single, pruning, transition matrix)."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    micro = ["kkt_single", "transition_matrix", "pruning_check"]
    colors = ["#9C27B0", "#607D8B", "#795548"]
    markers = ["D", "v", "x"]
    labels = ["Single KKT solve", "Transition matrix", "Pruning check"]

    # All times in nanoseconds for consistent axis.
    for phase, color, marker, label in zip(micro, colors, markers, labels):
        times_ns = [data[phase].get(f, np.nan) * 1e9 for f in FACET_COUNTS]
        ax.semilogy(FACET_COUNTS, times_ns, color=color, marker=marker, label=label)

    ax.set_xlabel(r"Facet count $F$")
    ax.set_ylabel("Time per call (ns)")
    ax.set_title("Micro-benchmarks (per-call)")
    ax.legend()
    ax.set_xticks(FACET_COUNTS)

    fig.tight_layout()
    out_path = EXPERIMENT_DIR / "micro_benchmarks.png"
    fig.savefig(out_path)
    print(f"Saved: {out_path}")
    plt.close(fig)


if __name__ == "__main__":
    if not CRITERION_DIR.exists():
        print(
            f"Criterion output not found at {CRITERION_DIR}.\n"
            "Run: cd crates && cargo bench --bench profiling",
            file=sys.stderr,
        )
        sys.exit(1)

    data = read_all_data()
    print_summary_table(data)
    plot_phase_breakdown(data)
    plot_micro_benchmarks(data)
