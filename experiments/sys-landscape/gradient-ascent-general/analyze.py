#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: Assess whether gradient ascent + escape rounds push sys above
      sys(HKO2024) = 1.0472 on general (non-Lagrangian) F=10 polytopes,
      and build a distribution of ascent endpoints. Bayesian update on
      the conjecture that no hit exists uses 3/N upper credible bound.
Input Artifacts:
  - experiments/sys-landscape/gradient-ascent-general/data/*.jsonl (per-seed summaries)
  - experiments/sys-landscape/datascience/produce/ascent.jsonl (bounded local fallback)
       Preference order: licca.jsonl > licca-shard-*.jsonl (legacy architecture-A) > datascience/produce/ascent.jsonl.
Output Artifacts:
  - gradient_ascent_general_distribution.png   (final sys histogram; linear)
  - gradient_ascent_general_tail.png           (final sys histogram; log-y tail)
  - gradient_ascent_general_improvement.png    (starting vs final sys scatter)
  - gradient_ascent_general_strategy.png       (final sys by winning strategy)
  - gradient_ascent_general_escape.png         (escape success rates)
  - gradient_ascent_general_convergence.png    (iteration count by type)
  - stdout: per-category summary + high-sys bucket counts + Bayesian bound
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
DATA_DIR = EXPERIMENT_DIR / "data"
LEGACY_SUMMARY_PATH = EXPERIMENT_DIR.parent / "datascience/produce/ascent.jsonl"
HKO_SYS = 1.0472
HIGH_SYS_THRESHOLDS = [0.95, 0.99, 1.00, HKO_SYS]

CATEGORY_COLORS = {"general": "#2196F3"}
STRATEGY_COLORS = {"within_cell": "#9E9E9E", "overshoot": "#E91E63", "wiggle": "#00BCD4", "none": "#BDBDBD"}


def pick_jsonl_files() -> list[Path]:
    """Prefer architecture-B licca.jsonl, then legacy architecture-A shards,
    then the pre-refactor legacy file.

    Returns all files matching the highest-priority tier that has data. This
    keeps the analyzer stable across the local smoke / LICCA production
    lifecycle described in the logbook. The legacy `licca-shard-*.jsonl` tier
    is retained so old committed architecture-A data still loads after merge;
    the current `job.sh` does not produce shard files, and the current
    `job-smoke.sh` writes temp outputs outside `data/`.
    """
    if DATA_DIR.exists():
        licca = DATA_DIR / "licca.jsonl"
        if licca.exists():
            return [licca]
        shards = sorted(DATA_DIR.glob("licca-shard-*.jsonl"))
        if shards:
            return shards
    if LEGACY_SUMMARY_PATH.exists():
        return [LEGACY_SUMMARY_PATH]
    print(
        f"ERROR: no data in {DATA_DIR} or at {LEGACY_SUMMARY_PATH}. "
        "From the repository root, see experiments/sys-landscape/gradient-ascent-general/job.sh 'How to run'.",
        file=sys.stderr,
    )
    sys.exit(1)


def load_summaries(files: list[Path]):
    """Load per-seed summary data from one or more JSONL files.

    Malformed lines are skipped: a partial write from a crashed LICCA job
    (tail-truncated last row, or interleaved bytes from a concurrent rayon
    writer) must not derail the analyzer.
    """
    rows = []
    for path in files:
        with open(path) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    rows.append(json.loads(line))
                except json.JSONDecodeError:
                    continue
    return rows


def summary_table(data):
    """Print summary statistics by polytope category."""
    by_cat = defaultdict(list)
    for row in data:
        cat = row["polytope_type"]
        by_cat[cat].append(row)

    print(f"{'Category':<14} {'N':>4} {'Mean sys':>9} {'Max sys':>9} {'P90 sys':>9} {'Mean Δ':>9} {'Escapes':>8}")
    print("-" * 72)
    for cat in ["general"]:
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
    """Histogram of final sys values (linear y-scale)."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    finals = np.array([r["final_sys"] for r in data], dtype=float)
    if finals.size:
        n_bins = 40 if finals.size >= 200 else 15
        ax.hist(
            finals, bins=n_bins, alpha=0.7,
            color=CATEGORY_COLORS["general"], edgecolor="white",
        )

    ax.axvline(x=1.0, color="red", linestyle="--", linewidth=1, label=r"$\mathrm{sys} = 1$")
    ax.axvline(x=HKO_SYS, color="#2d6a4f", linestyle="-", linewidth=1, label=r"$\mathrm{sys}(K_{\mathrm{HKO}})$")
    ax.set_xlabel(r"Final $\mathrm{sys}(K)$")
    ax.set_ylabel("Count")
    ax.legend()

    path = EXPERIMENT_DIR / "gradient_ascent_general_distribution.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_tail(data):
    """Log-y histogram of final sys values; makes the tail near sys=1 visible."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    finals = np.array([r["final_sys"] for r in data], dtype=float)
    if finals.size:
        n_bins = 60 if finals.size >= 500 else 25
        ax.hist(
            finals, bins=n_bins, alpha=0.7,
            color=CATEGORY_COLORS["general"], edgecolor="white",
        )

    ax.axvline(x=1.0, color="red", linestyle="--", linewidth=1, label=r"$\mathrm{sys} = 1$")
    ax.axvline(x=HKO_SYS, color="#2d6a4f", linestyle="-", linewidth=1, label=r"$\mathrm{sys}(K_{\mathrm{HKO}})$")
    ax.set_yscale("log")
    ax.set_xlabel(r"Final $\mathrm{sys}(K)$")
    ax.set_ylabel(r"Count (log)")
    ax.legend()

    path = EXPERIMENT_DIR / "gradient_ascent_general_tail.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_improvement(data):
    """Scatter: starting sys vs final sys."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    colors = CATEGORY_COLORS

    for cat in ["general"]:
        rows = [r for r in data if r["polytope_type"] == cat]
        if rows:
            starts = [r["starting_sys"] for r in rows]
            finals = [r["final_sys"] for r in rows]
            ax.scatter(starts, finals, s=SCATTER_SIZE, alpha=0.7, label=cat, color=colors[cat])

    upper = max(max(r["starting_sys"] for r in data), max(r["final_sys"] for r in data))
    lims = [0, upper * 1.05]
    ax.plot(lims, lims, "k--", linewidth=0.8, alpha=0.5, label=r"$y = x$")
    ax.axhline(y=1.0, color="red", linestyle="--", linewidth=0.8, alpha=0.5)

    ax.set_xlabel(r"Starting $\mathrm{sys}(K)$")
    ax.set_ylabel(r"Final $\mathrm{sys}(K)$")
    ax.legend()

    path = EXPERIMENT_DIR / "gradient_ascent_general_improvement.png"
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

    path = EXPERIMENT_DIR / "gradient_ascent_general_strategy.png"
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

    path = EXPERIMENT_DIR / "gradient_ascent_general_escape.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def plot_convergence(data):
    """Box plot of total gradient iterations by polytope category."""
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    categories = ["general"]
    cat_data = {c: [] for c in categories}
    for row in data:
        cat = row["polytope_type"]
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

    path = EXPERIMENT_DIR / "gradient_ascent_general_convergence.png"
    fig.savefig(path)
    plt.close(fig)
    print(f"Saved {path}")


def main():
    files = pick_jsonl_files()
    print(f"Using {len(files)} data file(s):")
    for p in files:
        print(f"  {p.relative_to(EXPERIMENT_DIR)}")
    data = load_summaries(files)
    if not data:
        print("No data rows found.")
        return

    print(f"\nLoaded {len(data)} seeds\n")

    summary_table(data)
    print()

    plot_distribution(data)
    plot_tail(data)
    plot_improvement(data)
    plot_strategy(data)
    plot_escape(data)
    plot_convergence(data)

    finals = np.array([r["final_sys"] for r in data], dtype=float)
    best = max(data, key=lambda r: r["final_sys"])
    print(f"\nBest sys: {best['final_sys']:.6f} ({best['name']})")
    print(f"\nHigh-sys bucket counts (N = {len(data)}):")
    counts = {thr: int((finals > thr).sum()) for thr in HIGH_SYS_THRESHOLDS}
    for thr in HIGH_SYS_THRESHOLDS:
        print(f"  sys > {thr:.4f}: {counts[thr]}/{len(data)}")

    if counts[HKO_SYS] == 0 and len(data) > 0:
        # Under a uniform Beta(1,1) prior, the posterior after 0 hits in N
        # trials is Beta(1, N+1). The exact 95% upper quantile is
        # 1 - 0.05 ** (1 / (N+1)); the rule-of-three approximation 3/N is
        # accurate to <1% for N >= 100 and is what we print for readability.
        n = len(data)
        exact_bound = 1.0 - 0.05 ** (1.0 / (n + 1))
        rule_of_three = 3.0 / n
        print(
            f"\n0 hits above sys(HKO) in N={n}. "
            f"95% upper credible bound on hit density p: "
            f"~{rule_of_three:.2e} (rule of three); "
            f"exact Beta(1,{n+1}) quantile: {exact_bound:.2e}."
        )


if __name__ == "__main__":
    main()
