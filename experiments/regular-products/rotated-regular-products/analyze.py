#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Plot systolic ratio curves for pentagon 5x5 and regular polygon pairs.

Goal: Visualize sys(theta) for the 5x5 pentagon sweep and selected n-gon x m-gon pairs.
Input Artifacts: experiments/regular-products/rotated-regular-products/lagrangian-products-5x5.jsonl,
       experiments/regular-products/rotated-regular-products/lagrangian-products-<n>x<m>-6deg.jsonl
Output Artifacts: experiments/regular-products/rotated-regular-products/lagrangian_products_5x5.png,
        experiments/regular-products/rotated-regular-products/lagrangian_products_7x7.png,
        experiments/regular-products/rotated-regular-products/lagrangian_products_polygon_pairs.png
"""
import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE, TEXT_WIDTH
setup()

REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_DIR = EXPERIMENT_DIR
FIGURES_DIR = EXPERIMENT_DIR

PAIR_FILES = {
    (3, 3): "lagrangian-products-3x3-6deg.jsonl",
    (3, 4): "lagrangian-products-3x4-6deg.jsonl",
    (3, 5): "lagrangian-products-3x5-6deg.jsonl",
    (3, 6): "lagrangian-products-3x6-6deg.jsonl",
    (4, 4): "lagrangian-products-4x4-6deg.jsonl",
    (4, 5): "lagrangian-products-4x5-6deg.jsonl",
    (4, 6): "lagrangian-products-4x6-6deg.jsonl",
    (5, 5): "lagrangian-products-5x5-6deg.jsonl",
    (5, 6): "lagrangian-products-5x6-6deg.jsonl",
    (6, 6): "lagrangian-products-6x6-6deg.jsonl",
}


def load_jsonl(path: Path) -> list[dict]:
    """Load JSONL file into list of dicts."""
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print("Run: cargo run -p exp-regular-products --release --bin regular-rotated-products", file=sys.stderr)
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def plot_sweep(data: list[dict], output: Path):
    """Plot sys vs rotation angle for pentagon x R(theta) pentagon."""
    rows = sorted(data, key=lambda d: d["angle_deg"])
    angles = np.array([d["angle_deg"] for d in rows])
    sys_vals = np.array([d["sys"] for d in rows])

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.plot(angles, sys_vals, color="#2f5aa6", linewidth=2.0, label=r"$\mathrm{sys}(\theta)$")
    ax.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.7, label=r"$\mathrm{sys} = 1$")

    ax.set_xlabel(r"Rotation angle $\theta$ (degrees)")
    ax.set_ylabel(r"$\mathrm{sys} = c^2 / (2\,\mathrm{vol})$")
    ax.set_title(r"Pentagon $\times_L$ $R(\theta)$ Pentagon ($0$–$36$ degrees)")

    ax.axvline(x=18.0, color="#7f8c8d", linestyle=":", alpha=0.7)
    ax.text(18.2, ax.get_ylim()[0], r"$18^\circ$", fontsize=8, color="#7f8c8d", va="bottom")

    ax.legend(loc="best")
    fig.tight_layout()
    fig.savefig(output)
    plt.close(fig)
    print(f"Saved: {output}")

    i_max = int(np.argmax(sys_vals))
    print("\nSweep summary:")
    print(f"  Points: {len(rows)}")
    print(f"  sys range: [{sys_vals.min():.6f}, {sys_vals.max():.6f}]")
    print(f"  Max sys at theta = {angles[i_max]:.1f} deg")


def load_pair_data(data_dir: Path) -> dict[tuple[int, int], list[dict]]:
    data = {}
    for pair, filename in PAIR_FILES.items():
        data[pair] = load_jsonl(data_dir / filename)
    return data


def plot_polygon_pairs(data: dict[tuple[int, int], list[dict]], output: Path):
    pairs = list(PAIR_FILES.keys())
    fig, axes = plt.subplots(2, 5, figsize=(TEXT_WIDTH, 4.5), sharey=True)
    axes = axes.flatten()

    all_sys = []
    for pair in pairs:
        rows = sorted(data[pair], key=lambda d: d["angle_deg"])
        sys_vals = [d["sys"] for d in rows]
        all_sys.extend(sys_vals)

    if not all_sys:
        print("ERROR: no pair data found", file=sys.stderr)
        sys.exit(1)

    y_min = min(all_sys)
    y_max = max(all_sys)
    pad = 0.03 * (y_max - y_min) if y_max > y_min else 0.01

    for i, (ax, pair) in enumerate(zip(axes, pairs)):
        rows = sorted(data[pair], key=lambda d: d["angle_deg"])
        angles = np.array([d["angle_deg"] for d in rows])
        sys_vals = np.array([d["sys"] for d in rows])

        ax.plot(angles, sys_vals, color="#2f5aa6", linewidth=1.5)
        ax.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.6)
        ax.set_title(f"{pair[0]}x{pair[1]}")
        ax.set_ylim(y_min - pad, y_max + pad)
        # Only label bottom row x-axis, left column y-axis
        if i >= 5:
            ax.set_xlabel(r"$\theta$ (deg)")
        else:
            ax.tick_params(labelbottom=False)
        if i % 5 == 0:
            ax.set_ylabel(r"$\mathrm{sys}$")

    fig.suptitle(r"Regular $n$-gon $\times_L$ $R(\theta)$ $m$-gon (6-degree steps)", y=1.02)
    fig.tight_layout()
    fig.savefig(output)
    plt.close(fig)
    print(f"Saved: {output}")


def plot_heptagon_sweep(data: list[dict], output: Path):
    """Plot sys vs rotation angle for heptagon x R(theta) heptagon."""
    rows = sorted(data, key=lambda d: d["angle_deg"])
    angles = np.array([d["angle_deg"] for d in rows])
    sys_vals = np.array([d["sys"] for d in rows])

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.plot(angles, sys_vals, color="#2f5aa6", linewidth=2.0, label=r"$\mathrm{sys}(\theta)$")
    ax.axhline(y=1.0, color="#c0392b", linestyle="--", alpha=0.7, label=r"$\mathrm{sys} = 1$")

    ax.set_xlabel(r"Rotation angle $\theta$ (degrees)")
    ax.set_ylabel(r"$\mathrm{sys} = c^2 / (2\,\mathrm{vol})$")
    ax.set_title(r"Heptagon $\times_L$ $R(\theta)$ Heptagon ($0$–$25.7$ degrees)")

    i_max = int(np.argmax(sys_vals))
    ax.axvline(x=angles[i_max], color="#7f8c8d", linestyle=":", alpha=0.7)
    ax.text(
        angles[i_max] + 0.3, ax.get_ylim()[0],
        rf"${angles[i_max]:.1f}^\circ$", fontsize=8, color="#7f8c8d", va="bottom",
    )

    ax.legend(loc="best")
    fig.tight_layout()
    fig.savefig(output)
    plt.close(fig)
    print(f"Saved: {output}")

    print("\nHeptagon sweep summary:")
    print(f"  Points: {len(rows)}")
    print(f"  sys range: [{sys_vals.min():.6f}, {sys_vals.max():.6f}]")
    print(f"  Max sys at theta = {angles[i_max]:.2f} deg")


def main():
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)
    data = load_jsonl(DATA_DIR / "lagrangian-products-5x5.jsonl")
    plot_sweep(data, FIGURES_DIR / "lagrangian_products_5x5.png")

    data_7x7 = load_jsonl(DATA_DIR / "lagrangian-products-7x7.jsonl")
    plot_heptagon_sweep(data_7x7, FIGURES_DIR / "lagrangian_products_7x7.png")

    pair_data = load_pair_data(DATA_DIR)
    plot_polygon_pairs(pair_data, FIGURES_DIR / "lagrangian_products_polygon_pairs.png")


if __name__ == "__main__":
    main()

# Legacy experiment (commented out for now):
# - Polygon grid sweep
# - Random Lagrangian products
# - Multi-figure plotting pipeline
# See git history for the full script.
