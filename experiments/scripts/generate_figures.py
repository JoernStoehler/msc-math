#!/usr/bin/env python3
"""
Generate publication-ready figures from polytope datasets.

Goal: Produce publication-ready plots of systolic ratios, capacity vs facet count,
      and acceptance rates from the polytope dataset.
Input: experiments/data/polytopes.jsonl, experiments/data/acceptance.jsonl
Output: experiments/figures/sys_histogram.png,
        experiments/figures/facet_vs_capacity.png,
        experiments/figures/acceptance_rates.png
"""

import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
DATA_DIR = REPO_ROOT / "experiments" / "data"
FIGURES_DIR = REPO_ROOT / "experiments" / "figures"


def load_polytopes():
    rows = []
    with open(DATA_DIR / "polytopes.jsonl") as f:
        for line in f:
            rows.append(json.loads(line))
    return rows


def plot_sys_histogram(rows):
    """Histogram of systolic ratios for random polytopes, colored by facet count."""
    random_rows = [r for r in rows if r["source"] == "random" and r["sys"] is not None]
    facet_counts = sorted(set(r["facet_count"] for r in random_rows))

    fig, ax = plt.subplots(figsize=(8, 5))

    for fc in facet_counts:
        vals = [r["sys"] for r in random_rows if r["facet_count"] == fc]
        ax.hist(vals, bins=20, alpha=0.6, label=f"F={fc}")  # 20 bins for visual clarity at typical dataset sizes (~50 per facet count)

    # Mark pentagon
    pentagon = [r for r in rows if r["source"] == "hko_pentagon"]
    if pentagon:
        ax.axvline(pentagon[0]["sys"], color="red", linestyle="--", linewidth=2, label="HK-O pentagon")

    # Mark Viterbo threshold
    ax.axvline(1.0, color="black", linestyle=":", linewidth=1.5, label="sys = 1 (Viterbo)")

    ax.set_xlabel("Systolic ratio sys(K)")
    ax.set_ylabel("Count")
    ax.set_title(f"Systolic ratios of {len(random_rows)} random 4D polytopes (F=5-{max(facet_counts)})")
    ax.legend()
    fig.tight_layout()
    fig.savefig(FIGURES_DIR / "sys_histogram.png", dpi=150)
    plt.close(fig)
    print(f"  sys_histogram.png: {len(random_rows)} random polytopes")


def plot_facet_vs_capacity(rows):
    """Two-panel figure: (left) facet count vs systolic ratio, (right) facet count vs capacity computation time."""
    valid = [r for r in rows if r["capacity"] is not None and r["sys"] is not None]

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 5))

    # Left: facet count vs sys
    random_vals = [(r["facet_count"], r["sys"]) for r in valid if r["source"] == "random"]
    known_vals = [(r["facet_count"], r["sys"], r["source"]) for r in valid if r["source"] != "random"]

    if random_vals:
        x, y = zip(*random_vals)
        ax1.scatter(x, y, alpha=0.4, s=20, label="random")
    for fx, fy, name in known_vals:
        ax1.scatter(fx, fy, marker="*", s=100, zorder=5, label=name)

    ax1.axhline(1.0, color="black", linestyle=":", linewidth=1)
    ax1.set_xlabel("Facet count F")
    ax1.set_ylabel("Systolic ratio sys(K)")
    ax1.set_title("Systolic ratio vs facet count")
    ax1.legend(fontsize=7, loc="upper left")

    # Right: facet count vs capacity computation time
    random_timing = [(r["facet_count"], r["time_capacity_ms"]) for r in valid if r["source"] == "random"]
    if random_timing:
        x, y = zip(*random_timing)
        ax2.scatter(x, y, alpha=0.4, s=20)
    ax2.set_xlabel("Facet count F")
    ax2.set_ylabel("Capacity computation time (ms)")
    ax2.set_title("Computation time vs facet count")
    ax2.set_yscale("log")

    fig.tight_layout()
    fig.savefig(FIGURES_DIR / "facet_vs_capacity.png", dpi=150)
    plt.close(fig)
    print(f"  facet_vs_capacity.png: {len(valid)} polytopes")


def plot_acceptance_ratios(data_path=None):
    """Plot acceptance rates from sweep data."""
    path = data_path or (DATA_DIR / "acceptance.jsonl")
    if not path.exists():
        print("  acceptance_ratios.png: SKIPPED (no acceptance.jsonl)")
        return

    rows = []
    with open(path) as f:
        for line in f:
            rows.append(json.loads(line))

    fig, ax = plt.subplots(figsize=(8, 5))

    facet_counts = sorted(set(r["facet_count"] for r in rows))
    for fc in facet_counts:
        fc_rows = [r for r in rows if r["facet_count"] == fc]
        h_ranges = [r["h_max"] - r["h_min"] for r in fc_rows]
        rates = [r["acceptance_ratio"] for r in fc_rows]
        ax.plot(h_ranges, rates, marker="o", label=f"F={fc}")

    ax.set_xlabel("Height range (h_max - h_min)")
    ax.set_ylabel("Acceptance rate")
    ax.set_title("Random polytope acceptance rates")
    ax.legend()
    fig.tight_layout()
    fig.savefig(FIGURES_DIR / "acceptance_rates.png", dpi=150)
    plt.close(fig)
    print(f"  acceptance_rates.png: {len(rows)} sweep rows")


def main():
    """Generate all figures, creating output directory if needed."""
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)
    rows = load_polytopes()
    print(f"Loaded {len(rows)} polytopes from {DATA_DIR / 'polytopes.jsonl'}")

    plot_sys_histogram(rows)
    plot_facet_vs_capacity(rows)
    plot_acceptance_ratios()

    print("Done.")


if __name__ == "__main__":
    main()
