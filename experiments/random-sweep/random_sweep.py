#!/usr/bin/env python3
"""
Plot systolic ratio summary for random 4D polytopes by facet count.

Goal: Visualize how sys varies with F using random polytopes and the pruned algorithm.
Input: experiments/random-sweep/random-sweep.jsonl
Output: experiments/random-sweep/random_sweep_sys_vs_f.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "random-sweep.jsonl"
FIGURES_DIR = EXPERIMENT_DIR


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print("Run: cargo run --bin random_sweep --release", file=sys.stderr)
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def compute_stats(rows: list[dict]) -> list[dict]:
    by_f = defaultdict(list)
    for r in rows:
        by_f[r["facet_count"]].append(r["sys"])

    stats = []
    for f in sorted(by_f.keys()):
        sys_vals = np.array(by_f[f], dtype=float)
        median = float(np.median(sys_vals))
        std = float(np.std(sys_vals, ddof=1)) if len(sys_vals) > 1 else 0.0
        stats.append({
            "F": f,
            "N": len(sys_vals),
            "median": median,
            "std": std,
            "values": sys_vals,
        })
    return stats


def plot_summary(stats: list[dict], output_path: Path) -> None:
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    fig, ax = plt.subplots(figsize=(9, 5))

    # Scatter all samples
    for s in stats:
        x = np.full_like(s["values"], s["F"], dtype=float)
        ax.scatter(x, s["values"], alpha=0.35, s=18, color="#3b6ea8")

    # Median with std error bars
    f_vals = [s["F"] for s in stats]
    medians = [s["median"] for s in stats]
    stds = [s["std"] for s in stats]
    ax.errorbar(
        f_vals,
        medians,
        yerr=stds,
        fmt="o-",
        color="#0f4c81",
        linewidth=1.6,
        markersize=5,
        capsize=4,
        label="Median ± std",
    )

    ax.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.7, label="sys = 1")
    ax.set_xlabel("Facet count F")
    ax.set_ylabel("Systolic ratio sys")
    ax.set_title("Random 4D polytopes: systolic ratio vs facet count")
    ax.grid(True, alpha=0.3)
    ax.set_xticks(f_vals)
    ax.legend(loc="best")

    fig.tight_layout()
    fig.savefig(output_path, dpi=150)
    plt.close(fig)
    print(f"Saved: {output_path}")


def main() -> None:
    rows = load_jsonl(DATA_PATH)
    stats = compute_stats(rows)
    plot_summary(stats, FIGURES_DIR / "random_sweep_sys_vs_f.png")


if __name__ == "__main__":
    main()
