"""
Goal: Analyze gradient correctness edge cases (Q3 near-degeneracy, Q4 barely-cutting).
Input:
  - crates/dev-gradient/numerics-edge-cases/gradient-correctness-q3-degeneracy.jsonl
  - crates/dev-gradient/numerics-edge-cases/gradient-correctness-q4-redundant.jsonl
Output:
  - crates/dev-gradient/numerics-edge-cases/gc_q3_gap.png    (Q3 action gap vs slope)
  - crates/dev-gradient/numerics-edge-cases/gc_q4_delta.png  (Q4 delta vs slope)
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import (
    setup,
    FIGSIZE_SINGLE,
    FIGSIZE_TRIPLE,
    FONT_SIZE_SMALL,
    SCATTER_SIZE,
)

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent

TARGETS = ["capacity", "volume", "sys"]
TARGET_LABELS = {
    "capacity": r"$c_{\mathrm{EHZ}}$",
    "volume": r"$\mathrm{vol}$",
    "sys": r"$\mathrm{sys}$",
}
TARGET_COLORS = {"capacity": "C0", "volume": "C1", "sys": "C2"}

# Fit slope over t in [1e-4, 1e-1] (log_t in [-4, -1]) to avoid the
# floating-point cancellation region at small t (volume and sys degrade
# below t ~ 1e-4 due to f(a+td)-f(a) cancellation).
SLOPE_FIT_LOG_T_RANGE = (-4.0, -1.0)


def load_jsonl(name):
    """Load JSONL file, return list of dicts. Returns [] if file missing."""
    path = EXPERIMENT_DIR / name
    if not path.exists():
        print(f"Warning: {path} not found, skipping")
        return []
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def group_traces(data):
    """Group rows into traces: (polytope_id, dir_idx, target) -> [(log_t, log_residual)]."""
    traces = defaultdict(list)
    for r in data:
        key = (r["polytope_id"], r["dir_idx"], r["target"])
        traces[key].append((r["log_t"], r["log_residual"]))
    for key in traces:
        traces[key].sort()
    return traces


def fit_slope(points, log_t_range=SLOPE_FIT_LOG_T_RANGE):
    """Fit slope of log(residual) vs log(t) within a t range.

    Returns (slope, r_squared). The slope should be ~2 for C^2 functions
    (quadratic Taylor remainder), ~1 for C^1 not C^2, ~0 at non-differentiable points.
    Returns (nan, nan) if fewer than 3 points in range.
    """
    x = np.array([p[0] for p in points])
    y = np.array([p[1] for p in points])
    mask = (x >= log_t_range[0]) & (x <= log_t_range[1]) & (y > -250)
    x, y = x[mask], y[mask]
    if len(x) < 3:
        return np.nan, np.nan
    coeffs = np.polyfit(x, y, 1)
    y_pred = np.polyval(coeffs, x)
    ss_res = np.sum((y - y_pred) ** 2)
    ss_tot = np.sum((y - np.mean(y)) ** 2)
    r2 = 1 - ss_res / ss_tot if ss_tot > 0 else 0.0
    return coeffs[0], r2


# ============================================================================
# Figure 1: Q3 — action gap vs fitted slope
# ============================================================================

def plot_q3_gap(data, filename="gc_q3_gap.png"):
    """Scatter: action gap vs fitted slope. Shows whether near-degeneracy
    affects gradient quality."""
    if not data:
        return

    traces = group_traces(data)
    gap_map = {}
    for r in data:
        if r["action_gap"] is not None:
            key = (r["polytope_id"], r["dir_idx"], r["target"])
            gap_map[key] = r["action_gap"]

    fig, axes = plt.subplots(1, 3, figsize=FIGSIZE_TRIPLE, sharey=True)

    for ax, target in zip(axes, TARGETS):
        gaps = []
        slopes = []
        for key, pts in traces.items():
            if key[2] != target or key not in gap_map:
                continue
            slope, r2 = fit_slope(pts)
            if not np.isnan(slope):
                gaps.append(gap_map[key])
                slopes.append(slope)

        if gaps:
            ax.scatter(gaps, slopes, s=SCATTER_SIZE, alpha=0.5,
                       color=TARGET_COLORS[target], edgecolors="none")
            ax.axhline(2.0, color="k", linestyle="--", alpha=0.3)
            ax.set_xscale("log")
            ax.set_xlabel("Action gap")
            ax.set_title(TARGET_LABELS[target])

    axes[0].set_ylabel("Fitted log-log slope")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


# ============================================================================
# Figure 2: Q4 — delta vs fitted slope
# ============================================================================

def plot_q4_delta(data, filename="gc_q4_delta.png"):
    """Median slope vs barely-cutting delta, one line per target."""
    if not data:
        return

    traces = group_traces(data)
    delta_map = {}
    for r in data:
        if r["barely_cutting_delta"] is not None:
            key = (r["polytope_id"], r["dir_idx"], r["target"])
            delta_map[key] = r["barely_cutting_delta"]

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    for target in TARGETS:
        by_delta = defaultdict(list)
        for key, pts in traces.items():
            if key[2] != target or key not in delta_map:
                continue
            slope, r2 = fit_slope(pts)
            if not np.isnan(slope):
                by_delta[delta_map[key]].append(slope)

        if by_delta:
            ds = sorted(by_delta.keys())
            medians = [np.median(by_delta[d]) for d in ds]
            ax.plot(ds, medians, marker="o", color=TARGET_COLORS[target],
                    label=TARGET_LABELS[target])

    ax.axhline(2.0, color="k", linestyle="--", alpha=0.3, label=r"Slope $= 2$")
    ax.set_xscale("log")
    ax.set_xlabel(r"Barely-cutting $\delta$")
    ax.set_ylabel("Median fitted slope")
    ax.legend(fontsize=FONT_SIZE_SMALL)
    ax.invert_xaxis()
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


# ============================================================================
# Main
# ============================================================================

def main():
    q3 = load_jsonl("gradient-correctness-q3-degeneracy.jsonl")
    q4 = load_jsonl("gradient-correctness-q4-redundant.jsonl")

    print(f"Loaded: Q3={len(q3)}, Q4={len(q4)} rows")

    plot_q3_gap(q3, "gc_q3_gap.png")
    plot_q4_delta(q4, "gc_q4_delta.png")


if __name__ == "__main__":
    main()
