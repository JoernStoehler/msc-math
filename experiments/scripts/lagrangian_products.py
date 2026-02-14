#!/usr/bin/env python3
"""
Analyze Lagrangian products of rotated polygons.

Goal: Identify which Lagrangian products have sys > 1, map the sys landscape.
Input: experiments/data/pentagon_sweep.jsonl, polygon_grid.jsonl, random_products.jsonl
Output: experiments/figures/pentagon_sweep.png, polygon_grid.png, random_products.png,
        experiments/figures/summary_table.txt
"""
import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
DATA_DIR = REPO_ROOT / "experiments" / "data"
FIGURES_DIR = REPO_ROOT / "experiments" / "figures"


def load_jsonl(path: Path) -> list[dict]:
    """Load JSONL file into list of dicts."""
    if not path.exists():
        print(f"WARNING: {path} not found, skipping", file=sys.stderr)
        return []
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def plot_pentagon_sweep(data: list[dict], output: Path):
    """Plot sys vs rotation angle for pentagon × R(θ)pentagon."""
    angles = np.array([d["angle_deg"] for d in data])
    sys_vals = np.array([d["sys"] for d in data])
    cap_vals = np.array([d["capacity"] for d in data])

    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8), sharex=True)

    # Top: sys vs angle
    ax1.plot(angles, sys_vals, "b-", linewidth=1.5, label="sys(θ)")
    ax1.axhline(y=1.0, color="r", linestyle="--", alpha=0.7, label="sys = 1 (Viterbo)")
    ax1.set_ylabel("sys = c²/(2·vol)")
    ax1.set_title("Pentagon × R(θ)Pentagon: Systolic Ratio")
    ax1.legend()
    ax1.grid(True, alpha=0.3)

    # Mark maximum
    i_max = np.argmax(sys_vals)
    ax1.annotate(
        f"max sys = {sys_vals[i_max]:.4f}\nθ = {angles[i_max]:.2f}°",
        xy=(angles[i_max], sys_vals[i_max]),
        xytext=(angles[i_max] + 5, sys_vals[i_max] - 0.02),
        arrowprops=dict(arrowstyle="->", color="black"),
        fontsize=9,
    )

    # Shade region where sys > 1
    above_1 = sys_vals > 1.0
    if above_1.any():
        ax1.fill_between(
            angles, 1.0, sys_vals, where=above_1, alpha=0.2, color="red",
            label="sys > 1 region"
        )
        ax1.legend()

    # Mark symmetry lines
    for deg in [36, 72]:
        ax1.axvline(x=deg, color="gray", linestyle=":", alpha=0.5)
    ax1.text(36, ax1.get_ylim()[0], " π/5", fontsize=8, color="gray", va="bottom")
    ax1.text(72, ax1.get_ylim()[0], " 2π/5", fontsize=8, color="gray", va="bottom")

    # Bottom: capacity vs angle
    ax2.plot(angles, cap_vals, "g-", linewidth=1.5)
    ax2.set_xlabel("Rotation angle θ (degrees)")
    ax2.set_ylabel("c_EHZ")
    ax2.set_title("EHZ Capacity")
    ax2.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output, dpi=150)
    plt.close()
    print(f"Saved: {output}")

    # Print summary
    print("\nPentagon sweep summary:")
    print(f"  Points: {len(data)}")
    print(f"  sys range: [{sys_vals.min():.6f}, {sys_vals.max():.6f}]")
    print(f"  Max sys at θ = {angles[i_max]:.2f}°")
    print(f"  sys > 1 count: {above_1.sum()} / {len(data)}")
    if above_1.any():
        angles_above = angles[above_1]
        print(f"  sys > 1 angle range: [{angles_above.min():.2f}°, {angles_above.max():.2f}°]")


def plot_polygon_grid(data: list[dict], output: Path):
    """Plot sys vs angle for all (n,m) polygon pairs."""
    # Group by (n1, n2)
    pairs: dict[tuple[int, int], list[dict]] = {}
    for d in data:
        key = (d["n1"], d["n2"])
        pairs.setdefault(key, []).append(d)

    fig, ax = plt.subplots(figsize=(12, 7))

    colors = plt.cm.tab10(np.linspace(0, 1, len(pairs)))
    max_sys_per_pair = {}

    for i, ((n1, n2), rows) in enumerate(sorted(pairs.items())):
        rows.sort(key=lambda r: r["angle_deg"])
        angles = np.array([r["angle_deg"] for r in rows])
        sys_vals = np.array([r["sys"] for r in rows])

        label = f"({n1},{n2}) F={n1+n2}"
        ax.plot(angles, sys_vals, color=colors[i], linewidth=1.5, label=label)

        i_max = np.argmax(sys_vals)
        max_sys_per_pair[(n1, n2)] = {
            "max_sys": sys_vals[i_max],
            "angle": angles[i_max],
            "facets": n1 + n2,
            "above_1": bool(sys_vals[i_max] > 1.0),
        }

    ax.axhline(y=1.0, color="r", linestyle="--", alpha=0.7, label="sys = 1")
    ax.set_xlabel("Rotation angle θ (degrees)")
    ax.set_ylabel("sys = c²/(2·vol)")
    ax.set_title("Regular n-gon × R(θ) m-gon: Systolic Ratio")
    ax.legend(loc="upper right", fontsize=8)
    ax.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output, dpi=150)
    plt.close()
    print(f"Saved: {output}")

    # Print summary table
    print("\nPolygon grid summary:")
    print(f"  {'Pair':>8} {'F':>3} {'max sys':>10} {'angle':>8} {'sys>1':>6}")
    print(f"  {'-'*8} {'-'*3} {'-'*10} {'-'*8} {'-'*6}")
    for (n1, n2), info in sorted(max_sys_per_pair.items()):
        marker = "  YES" if info["above_1"] else ""
        print(
            f"  ({n1},{n2}){' '*(5-len(f'({n1},{n2})'))} "
            f"{info['facets']:>3} {info['max_sys']:>10.6f} {info['angle']:>7.2f}° {marker}"
        )

    return max_sys_per_pair


def plot_random_products(data: list[dict], output: Path):
    """Histogram of sys for random polygon Lagrangian products."""
    sys_vals = np.array([d["sys"] for d in data])
    facet_counts = np.array([d["facet_count"] for d in data])

    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

    # Left: histogram of sys
    ax1.hist(sys_vals, bins=50, edgecolor="black", alpha=0.7, color="steelblue")
    ax1.axvline(x=1.0, color="r", linestyle="--", alpha=0.7, label="sys = 1")
    ax1.set_xlabel("sys = c²/(2·vol)")
    ax1.set_ylabel("Count")
    ax1.set_title("Random Lagrangian Products: sys Distribution")
    ax1.legend()

    # Right: sys vs facet count
    for fc in sorted(set(facet_counts)):
        mask = facet_counts == fc
        ax2.scatter(
            np.full(mask.sum(), fc)
            + np.random.default_rng(42).uniform(-0.15, 0.15, mask.sum()),
            sys_vals[mask],
            s=10,
            alpha=0.5,
            label=f"F={fc}",
        )
    ax2.axhline(y=1.0, color="r", linestyle="--", alpha=0.7)
    ax2.set_xlabel("Facet count")
    ax2.set_ylabel("sys")
    ax2.set_title("sys vs Facet Count")
    ax2.legend(fontsize=8)

    plt.tight_layout()
    plt.savefig(output, dpi=150)
    plt.close()
    print(f"Saved: {output}")

    # Print summary
    above_1 = sys_vals > 1.0
    print("\nRandom products summary:")
    print(f"  Samples: {len(data)}")
    print(f"  sys range: [{sys_vals.min():.6f}, {sys_vals.max():.6f}]")
    print(f"  sys > 1: {above_1.sum()} / {len(data)} ({100*above_1.mean():.1f}%)")
    print(f"  Mean sys: {sys_vals.mean():.4f}")
    print(f"  Median sys: {np.median(sys_vals):.4f}")


def main():
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    # Pentagon sweep
    pentagon_data = load_jsonl(DATA_DIR / "pentagon_sweep.jsonl")
    if pentagon_data:
        plot_pentagon_sweep(pentagon_data, FIGURES_DIR / "pentagon_sweep.png")

    # Polygon grid
    grid_data = load_jsonl(DATA_DIR / "polygon_grid.jsonl")
    if grid_data:
        plot_polygon_grid(grid_data, FIGURES_DIR / "polygon_grid.png")

    # Random products
    random_data = load_jsonl(DATA_DIR / "random_products.jsonl")
    if random_data:
        plot_random_products(random_data, FIGURES_DIR / "random_products.png")

    # Combined "all" file
    all_data = load_jsonl(DATA_DIR / "lagrangian_all.jsonl")
    if all_data:
        pentagon = [d for d in all_data if d["family"] == "pentagon_sweep"]
        grid = [d for d in all_data if d["family"] == "polygon_grid"]
        random = [d for d in all_data if d["family"] == "random_product"]
        if pentagon:
            plot_pentagon_sweep(pentagon, FIGURES_DIR / "pentagon_sweep.png")
        if grid:
            plot_polygon_grid(grid, FIGURES_DIR / "polygon_grid.png")
        if random:
            plot_random_products(random, FIGURES_DIR / "random_products.png")


if __name__ == "__main__":
    main()
