#!/usr/bin/env python3
"""
Analyze polytope dataset and produce figures.

Reads JSONL from experiments/data/, saves figures to experiments/figures/.

Requires: matplotlib (pip install matplotlib)
"""
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
DATA_DIR = REPO_ROOT / "experiments" / "data"
FIGURES_DIR = REPO_ROOT / "experiments" / "figures"

POLYTOPE_FILE = DATA_DIR / "polytopes.jsonl"
SWEEP_FILE = DATA_DIR / "acceptance.jsonl"


def load_jsonl(path):
    if not path.exists():
        print(f"File not found: {path}")
        print("Run run_dataset.py first to generate data.")
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def plot_sys_histogram(rows, out_path):
    """Histogram of systolic ratio values."""
    import matplotlib.pyplot as plt

    sys_vals = [r["sys"] for r in rows]

    fig, ax = plt.subplots(figsize=(8, 5))
    ax.hist(sys_vals, bins=30, edgecolor="black", alpha=0.7)
    ax.axvline(x=1.0, color="red", linestyle="--", label="Viterbo bound (sys=1)")
    ax.set_xlabel("Systolic ratio sys(K) = c²/(2·vol)")
    ax.set_ylabel("Count")
    ax.set_title("Distribution of Systolic Ratios")
    ax.legend()
    fig.tight_layout()
    fig.savefig(out_path, dpi=150)
    plt.close(fig)
    print(f"  Saved: {out_path}")


def plot_facet_vs_capacity(rows, out_path):
    """Scatter plot: facet count vs capacity."""
    import matplotlib.pyplot as plt

    facets = [r["facet_count"] for r in rows]
    caps = [r["capacity"] for r in rows]
    sources = [r["source"] for r in rows]

    fig, ax = plt.subplots(figsize=(8, 5))
    # Color by source
    known_mask = [s != "random" for s in sources]
    random_mask = [s == "random" for s in sources]

    ax.scatter(
        [f for f, m in zip(facets, random_mask) if m],
        [c for c, m in zip(caps, random_mask) if m],
        alpha=0.5,
        label="random",
        s=20,
    )
    ax.scatter(
        [f for f, m in zip(facets, known_mask) if m],
        [c for c, m in zip(caps, known_mask) if m],
        marker="^",
        s=80,
        label="known",
        zorder=5,
    )
    ax.set_xlabel("Facet count")
    ax.set_ylabel("Capacity")
    ax.set_title("Facet Count vs Capacity")
    ax.legend()
    fig.tight_layout()
    fig.savefig(out_path, dpi=150)
    plt.close(fig)
    print(f"  Saved: {out_path}")


def plot_acceptance_rates(rows, out_path):
    """Bar chart: acceptance rate by facet count and height range."""
    import matplotlib.pyplot as plt
    import numpy as np

    # Group by h_range
    h_ranges = sorted(set((r["h_min"], r["h_max"]) for r in rows))
    facet_counts = sorted(set(r["facet_count"] for r in rows))

    x = np.arange(len(facet_counts))
    width = 0.8 / len(h_ranges)

    fig, ax = plt.subplots(figsize=(10, 5))
    for i, (hmin, hmax) in enumerate(h_ranges):
        rates = []
        for f in facet_counts:
            match = [r for r in rows if r["facet_count"] == f and r["h_min"] == hmin]
            rates.append(match[0]["acceptance_ratio"] if match else 0.0)
        ax.bar(
            x + i * width - 0.4 + width / 2,
            rates,
            width,
            label=f"h∈[{hmin},{hmax}]",
        )

    ax.set_xlabel("Facet count")
    ax.set_ylabel("Acceptance ratio")
    ax.set_title("Rejection Sampling Acceptance Rates")
    ax.set_xticks(x)
    ax.set_xticklabels(facet_counts)
    ax.legend()
    fig.tight_layout()
    fig.savefig(out_path, dpi=150)
    plt.close(fig)
    print(f"  Saved: {out_path}")


def main():
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    print("Loading polytope dataset...")
    polytopes = load_jsonl(POLYTOPE_FILE)
    print(f"  {len(polytopes)} rows loaded.")

    print("Generating figures...")
    plot_sys_histogram(polytopes, FIGURES_DIR / "sys_histogram.png")
    plot_facet_vs_capacity(polytopes, FIGURES_DIR / "facet_vs_capacity.png")

    if SWEEP_FILE.exists():
        print("Loading acceptance sweep...")
        sweep = load_jsonl(SWEEP_FILE)
        print(f"  {len(sweep)} rows loaded.")
        plot_acceptance_rates(sweep, FIGURES_DIR / "acceptance_rates.png")

    print("\nDone.")


if __name__ == "__main__":
    main()
