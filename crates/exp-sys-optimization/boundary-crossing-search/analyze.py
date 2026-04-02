#!/usr/bin/env python3
"""
Analyze sys-search results: gradient ascent with boundary-crossing strategies.

Goal: Assess whether boundary-crossing (overshoot, wiggle) improves sys beyond
      within-cell gradient ascent, and compare strategies.
Input: experiments/sys-search/sys-search.jsonl (per-seed summaries)
       experiments/sys-search/sys-search-trace.jsonl (per-iteration trace)
Output:
  - sys_search_distribution.png   (final sys histogram by polytope type)
  - sys_search_improvement.png    (starting vs final sys scatter)
  - sys_search_strategy.png       (final sys by winning strategy)
  - sys_search_escape.png         (escape success rates)
  - sys_search_convergence.png    (iteration count by type)
  - stdout: summary table
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE, SCATTER_SIZE
setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
SUMMARY_PATH = EXPERIMENT_DIR / "sys-search.jsonl"
TRACE_PATH = EXPERIMENT_DIR / "sys-search-trace.jsonl"

CATEGORY_COLORS = {"general": "#2196F3", "lagrangian": "#4CAF50", "warm": "#FF9800"}
STRATEGY_COLORS = {"within_cell": "#9E9E9E", "overshoot": "#E91E63", "wiggle": "#00BCD4", "none": "#BDBDBD"}


def load_summaries():
    """Load per-seed summary data."""
    rows = []
    with open(SUMMARY_PATH) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def load_trace():
    """Load per-iteration trace data."""
    rows = []
    if not TRACE_PATH.exists():
        return rows
    with open(TRACE_PATH) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def classify_type(ptype):
    """Map polytope_type to display category."""
    if ptype.startswith("warm_"):
        return "warm"
    elif ptype == "general":
        return "general"
    else:
        return "lagrangian"


def summary_table(data):
    """Print summary statistics by polytope category."""
    by_cat = defaultdict(list)
    for row in data:
        cat = classify_type(row["polytope_type"])
        by_cat[cat].append(row)

    print(f"{'Category':<14} {'N':>4} {'Mean sys':>9} {'Max sys':>9} {'P90 sys':>9} {'Mean Δ':>9} {'Escapes':>8}")
    print("-" * 72)
    for cat in ["general", "lagrangian", "warm"]:
        if cat not in by_cat:
            continue
        rows = by_cat[cat]
        finals = [r["final_sys"] for r in rows]
        deltas = [r["total_delta"] for r in rows]
        escapes = sum(1 for r in rows if r["best_strategy"] not in ("within_cell", "none"))
        n = len(rows)
        print(
            f"{cat:<14} {n:>4} {np.mean(finals):>9.4f} {np.max(finals):>9.4f} "
            f"{np.percentile(finals, 90):>9.4f} {np.mean(deltas):>9.4f} {escapes:>5}/{n}"
        )


def plot_distribution(data):
    """Histogram of final sys values by polytope category."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    colors = CATEGORY_COLORS

    for cat in ["general", "lagrangian", "warm"]:
        finals = [r["final_sys"] for r in data if classify_type(r["polytope_type"]) == cat]
        if finals:
            ax.hist(finals, bins=15, alpha=0.6, label=cat, color=colors[cat], edgecolor="white")

    ax.axvline(x=1.0, color="red", linestyle="--", linewidth=1, label=r"$\mathrm{sys} = 1$")
    ax.set_xlabel(r"Final $\mathrm{sys}(K)$")
    ax.set_ylabel("Count")
    ax.yaxis.set_major_locator(plt.MaxNLocator(integer=True))
    ax.legend()

    path = EXPERIMENT_DIR / "sys_search_distribution.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_improvement(data):
    """Scatter: starting sys vs final sys."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    colors = CATEGORY_COLORS

    for cat in ["general", "lagrangian", "warm"]:
        rows = [r for r in data if classify_type(r["polytope_type"]) == cat]
        if rows:
            starts = [r["starting_sys"] for r in rows]
            finals = [r["final_sys"] for r in rows]
            ax.scatter(starts, finals, s=SCATTER_SIZE, alpha=0.7, label=cat, color=colors[cat])

    lims = [0, max(r["final_sys"] for r in data) * 1.05]
    ax.plot(lims, lims, "k--", linewidth=0.8, alpha=0.5, label=r"$y = x$")
    ax.axhline(y=1.0, color="red", linestyle="--", linewidth=0.8, alpha=0.5)

    ax.set_xlabel(r"Starting $\mathrm{sys}(K)$")
    ax.set_ylabel(r"Final $\mathrm{sys}(K)$")
    ax.legend()

    path = EXPERIMENT_DIR / "sys_search_improvement.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_strategy(data):
    """Box plot of final sys by winning strategy."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    strategies = ["within_cell", "overshoot", "wiggle", "none"]
    strategy_data = {s: [] for s in strategies}
    for row in data:
        s = row["best_strategy"]
        if s in strategy_data:
            strategy_data[s].append(row["final_sys"])

    # Filter to strategies that have data
    plot_strategies = [s for s in strategies if strategy_data[s]]
    plot_data = [strategy_data[s] for s in plot_strategies]

    if plot_data:
        bp = ax.boxplot(plot_data, tick_labels=plot_strategies, patch_artist=True)
        colors = STRATEGY_COLORS
        for patch, strat in zip(bp["boxes"], plot_strategies):
            patch.set_facecolor(colors.get(strat, "#9E9E9E"))
            patch.set_alpha(0.6)

    ax.axhline(y=1.0, color="red", linestyle="--", linewidth=0.8, alpha=0.5)
    ax.set_ylabel(r"Final $\mathrm{sys}(K)$")
    ax.set_xlabel("Winning strategy")

    path = EXPERIMENT_DIR / "sys_search_strategy.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_escape(data):
    """Bar chart: fraction of seeds where each strategy improved sys."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    n_total = len(data)
    n_within = sum(1 for r in data if r["best_strategy"] == "within_cell")
    n_overshoot = sum(1 for r in data if r["best_strategy"] == "overshoot")
    n_wiggle = sum(1 for r in data if r["best_strategy"] == "wiggle")
    n_none = sum(1 for r in data if r["best_strategy"] == "none")

    labels = ["within_cell", "overshoot", "wiggle", "none"]
    counts = [n_within, n_overshoot, n_wiggle, n_none]
    fracs = [c / n_total if n_total > 0 else 0 for c in counts]
    colors = [STRATEGY_COLORS[s] for s in labels]

    bars = ax.bar(labels, fracs, color=colors, alpha=0.7, edgecolor="white")
    for bar, count in zip(bars, counts):
        ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 0.01,
                str(count), ha="center", va="bottom")

    ax.set_ylabel("Fraction of seeds")
    ax.set_xlabel("Winning strategy")
    ax.set_ylim(0, 1.05)

    path = EXPERIMENT_DIR / "sys_search_escape.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_convergence(data):
    """Box plot of total gradient iterations by polytope category."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    categories = ["general", "lagrangian", "warm"]
    cat_data = {c: [] for c in categories}
    for row in data:
        cat = classify_type(row["polytope_type"])
        cat_data[cat].append(row["n_gradient_iters_total"])

    plot_cats = [c for c in categories if cat_data[c]]
    plot_data = [cat_data[c] for c in plot_cats]

    if plot_data:
        colors_map = CATEGORY_COLORS
        bp = ax.boxplot(plot_data, tick_labels=plot_cats, patch_artist=True)
        for patch, cat in zip(bp["boxes"], plot_cats):
            patch.set_facecolor(colors_map.get(cat, "#9E9E9E"))
            patch.set_alpha(0.6)

    ax.set_ylabel("Total gradient iterations")
    ax.set_xlabel("Polytope category")

    path = EXPERIMENT_DIR / "sys_search_convergence.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def main():
    if not SUMMARY_PATH.exists():
        print(f"No data at {SUMMARY_PATH}. Run: cd experiments/ && cargo run --release --bin sys_search")
        return

    data = load_summaries()
    if not data:
        print("No data rows found.")
        return

    print(f"\nLoaded {len(data)} seeds from {SUMMARY_PATH}\n")

    summary_table(data)
    print()

    plot_distribution(data)
    plot_improvement(data)
    plot_strategy(data)
    plot_escape(data)
    plot_convergence(data)

    # Overall stats
    best = max(data, key=lambda r: r["final_sys"])
    print(f"\nBest sys: {best['final_sys']:.6f} ({best['name']})")
    print(f"Seeds with sys > 0.9: {sum(1 for r in data if r['final_sys'] > 0.9)}/{len(data)}")
    print(f"Seeds with sys > 1.0: {sum(1 for r in data if r['final_sys'] > 1.0)}/{len(data)}")


if __name__ == "__main__":
    main()
