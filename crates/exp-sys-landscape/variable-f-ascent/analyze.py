# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: Analyze variable-F gradient ascent results (RQ1 + RQ2).
Input: crates/exp-sys-landscape/variable-f-ascent/variable-f-ascent.jsonl
Output: crates/exp-sys-landscape/variable-f-ascent/variable-f-rq1.png
        crates/exp-sys-landscape/variable-f-ascent/variable-f-rq2.png
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

# figure_config.py is at crates/figure_config.py (two levels up from this script)
sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE, SCATTER_SIZE

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "variable-f-ascent.jsonl"


def load_data():
    rows = []
    with open(DATA_PATH) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def analyze_rq1(rows):
    """RQ1: Can F=10 local maxima be improved in F=11 space?"""
    rq1 = [r for r in rows if r["rq"] == "rq1"]
    if not rq1:
        print("No RQ1 data found.")
        return

    # Extract source name (strip rq1_ prefix and _pN suffix)
    for r in rq1:
        parts = r["name"].rsplit("_p", 1)
        r["source"] = parts[0].replace("rq1_", "")

    sources = sorted(set(r["source"] for r in rq1))
    source_sys = {}
    for r in rq1:
        source_sys[r["source"]] = r["starting_sys"]

    # Summary statistics
    n_improved = sum(1 for r in rq1 if r["delta_vs_source"] > 1e-6)
    n_total = len(rq1)
    deltas = [r["delta_vs_source"] for r in rq1]
    active_frac = sum(1 for r in rq1 if r.get("facet_remained_active")) / n_total

    print("=== RQ1: Improving F=10 local maxima in F=11 space ===")
    print(f"Trials: {n_total}")
    print(f"Improved: {n_improved}/{n_total} ({100*n_improved/n_total:.0f}%)")
    print(f"Mean delta: {np.mean(deltas):+.4f}")
    print(f"Max delta: {max(deltas):+.4f}")
    print(f"Min delta: {min(deltas):+.4f}")
    print(f"Added facet remained active: {100*active_frac:.0f}%")
    print()

    for src in sources:
        trials = [r for r in rq1 if r["source"] == src]
        src_s = source_sys[src]
        finals = [r["final_sys"] for r in trials]
        n_imp = sum(1 for r in trials if r["delta_vs_source"] > 1e-6)
        print(f"  {src}: src_sys={src_s:.4f}, "
              f"final=[{min(finals):.4f}, {max(finals):.4f}], "
              f"improved={n_imp}/{len(trials)}")

    # Figure: scatter of (source sys, final sys) with diagonal
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    x = [r["starting_sys"] for r in rq1]
    y = [r["final_sys"] for r in rq1]

    ax.scatter(x, y, s=SCATTER_SIZE, alpha=0.7, zorder=3)

    # Diagonal: final = source (no improvement)
    lo = min(min(x), min(y)) - 0.02
    hi = max(max(x), max(y)) + 0.02
    ax.plot([lo, hi], [lo, hi], "k--", alpha=0.4, linewidth=1, label="no improvement")

    ax.set_xlabel(r"$F\!=\!10$ local maximum $\mathrm{sys}$")
    ax.set_ylabel(r"$F\!=\!11$ ascent final $\mathrm{sys}$")
    ax.set_title(f"RQ1: {n_improved}/{n_total} trials improved over " + r"$F\!=\!10$ local max")
    ax.legend(loc="lower right")
    ax.set_xlim(lo, hi)
    ax.set_ylim(lo, hi)
    ax.set_aspect("equal")

    fig.savefig(EXPERIMENT_DIR / "variable-f-rq1.png")
    plt.close(fig)
    print(f"\nSaved: variable-f-rq1.png")


def analyze_rq2(rows):
    """RQ2: Three-way comparison from random F=10 starts."""
    rq2 = [r for r in rows if r["rq"] == "rq2"]
    if not rq2:
        print("No RQ2 data found.")
        return

    path_a = [r for r in rq2 if r["path"] == "f10_ascent"]
    path_b = [r for r in rq2 if r["path"] == "f10_add_then_f11"]
    path_c = [r for r in rq2 if r["path"] == "random_f11"]

    print("\n=== RQ2: Three-way comparison ===")
    for label, data in [("A: F=10 ascent", path_a),
                         ("B: add+F=11 ascent", path_b),
                         ("C: random F=11", path_c)]:
        if not data:
            print(f"  {label}: no data")
            continue
        finals = [r["final_sys"] for r in data]
        print(f"  {label}: n={len(data)}, "
              f"mean={np.mean(finals):.4f}, "
              f"median={np.median(finals):.4f}, "
              f"max={max(finals):.4f}, "
              f"min={min(finals):.4f}")

    # Paired comparison: for each seed, compare A vs B
    if path_a and path_b:
        # Extract seed indices from names
        a_by_seed = {}
        for r in path_a:
            seed = r["name"].replace("_pathA_f10", "")
            a_by_seed[seed] = r["final_sys"]
        b_by_seed = {}
        for r in path_b:
            seed = r["name"].replace("_pathB_f11add", "")
            b_by_seed[seed] = r["final_sys"]

        common = sorted(set(a_by_seed) & set(b_by_seed))
        if common:
            n_b_wins = sum(1 for s in common if b_by_seed[s] > a_by_seed[s] + 1e-6)
            print(f"\n  Paired A vs B: {len(common)} seeds, "
                  f"B wins {n_b_wins}/{len(common)}")
            diffs = [b_by_seed[s] - a_by_seed[s] for s in common]
            print(f"  Mean(B-A) = {np.mean(diffs):+.4f}, "
                  f"Median(B-A) = {np.median(diffs):+.4f}")

    # Figure: grouped bar chart or box plot
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    plot_data = []
    labels = []
    colors = ["#4878CF", "#6ACC65", "#D65F5F"]
    for label, data, color in [
        (r"A: $F\!=\!10$ ascent", path_a, colors[0]),
        (r"B: add+$F\!=\!11$", path_b, colors[1]),
        (r"C: random $F\!=\!11$", path_c, colors[2]),
    ]:
        if data:
            finals = [r["final_sys"] for r in data]
            plot_data.append(finals)
            labels.append(label)

    if plot_data:
        positions = range(1, len(plot_data) + 1)
        bp = ax.boxplot(plot_data, positions=positions, widths=0.5,
                        patch_artist=True, showmeans=True,
                        meanprops=dict(marker="D", markerfacecolor="white",
                                       markeredgecolor="black", markersize=5))
        for patch, color in zip(bp["boxes"], colors[:len(plot_data)]):
            patch.set_facecolor(color)
            patch.set_alpha(0.6)

        ax.set_xticks(positions)
        ax.set_xticklabels(labels)
        ax.set_ylabel(r"Final $\mathrm{sys}$")
        ax.set_title("RQ2: Three-way comparison")

    fig.savefig(EXPERIMENT_DIR / "variable-f-rq2.png")
    plt.close(fig)
    print(f"\nSaved: variable-f-rq2.png")


def main():
    if not DATA_PATH.exists():
        print(f"No data found at {DATA_PATH}")
        return

    rows = load_data()
    print(f"Loaded {len(rows)} rows from {DATA_PATH.name}\n")

    analyze_rq1(rows)
    analyze_rq2(rows)

    # Overall summary
    all_sys = [r["final_sys"] for r in rows]
    best = max(rows, key=lambda r: r["final_sys"])
    print(f"\n=== Overall ===")
    print(f"Best sys: {best['final_sys']:.6f} ({best['name']}, path={best['path']})")
    if best["final_sys"] > 1.0:
        print("*** VITERBO VIOLATION ***")


if __name__ == "__main__":
    main()
