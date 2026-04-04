#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Plot systolic ratio summary for random Lagrangian products by polygon pair.

Goal: Visualize how sys varies across (k,m) polygon pairs.
Input: crates/exp-sys-landscape/random-product-sample/random-product-sweep.jsonl
Output: crates/exp-sys-landscape/random-product-sample/random_product_sweep_sys_vs_pair.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE
setup()

REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "random-product-sweep.jsonl"
FIGURES_DIR = EXPERIMENT_DIR

PAIRS = [
    (3, 3),
    (3, 4),
    (3, 5),
    (3, 6),
    (4, 4),
    (4, 5),
    (4, 6),
    (5, 5),
    (5, 6),
    (6, 6),
]


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print("Run: cargo run --bin random_product_sweep --release", file=sys.stderr)
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def compute_stats(rows: list[dict]) -> list[dict]:
    by_pair = defaultdict(list)
    for r in rows:
        by_pair[(r["k"], r["m"])].append(r["sys"])

    stats = []
    for pair in PAIRS:
        sys_vals = np.array(by_pair.get(pair, []), dtype=float)
        if sys_vals.size == 0:
            continue
        median = float(np.median(sys_vals))
        std = float(np.std(sys_vals, ddof=1)) if len(sys_vals) > 1 else 0.0
        stats.append({
            "pair": pair,
            "N": len(sys_vals),
            "median": median,
            "std": std,
            "values": sys_vals,
        })
    return stats


def plot_summary(stats: list[dict], output_path: Path) -> None:
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    x_positions = np.arange(len(stats))
    labels = [f"({s['pair'][0]},{s['pair'][1]})" for s in stats]

    for idx, s in enumerate(stats):
        x = np.full_like(s["values"], x_positions[idx], dtype=float)
        ax.scatter(x, s["values"], alpha=0.35, s=18, color="#3b6ea8")

    medians = [s["median"] for s in stats]
    stds = [s["std"] for s in stats]
    ax.errorbar(
        x_positions,
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
    ax.set_xlabel("Polygon pair (k,m)")
    ax.set_ylabel("Systolic ratio sys")
    ax.set_title("Random Lagrangian products: systolic ratio by polygon pair")
    ax.set_xticks(x_positions)
    ax.set_xticklabels(labels)
    ax.legend(loc="best")

    fig.tight_layout()
    fig.savefig(output_path)
    plt.close(fig)
    print(f"Saved: {output_path}")


def main() -> None:
    rows = load_jsonl(DATA_PATH)
    stats = compute_stats(rows)
    plot_summary(stats, FIGURES_DIR / "random_product_sweep_sys_vs_pair.png")


if __name__ == "__main__":
    main()
