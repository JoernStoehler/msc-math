# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: Analyze subdifferential prediction tests (Q5 orbit-switching, Q5b exact boundaries).
Input Artifacts:
  - experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5-subdiff.jsonl
  - experiments/numerics/gradient/numerics-subdifferential/gradient-correctness-q5b-symmetric.jsonl
Output Artifacts:
  - experiments/numerics/gradient/numerics-subdifferential/gc_q5_convergence.png (Q5 subdiff vs single-orbit convergence)
  - experiments/numerics/gradient/numerics-subdifferential/gc_q5_switching.png   (Q5 orbit switching rate vs t and gap)
  - experiments/numerics/gradient/numerics-subdifferential/gc_q5b_boundary.png   (Q5b subdiff at exact switching boundaries)
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent.parent))
from figure_config import (
    setup,
    FIGSIZE_DUAL,
    FIGSIZE_TRIPLE,
    FONT_SIZE_SMALL,
)

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent

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
# Q5: Subdifferential prediction — convergence and orbit switching
# ============================================================================

Q5_GAP_BINS = [
    (0.0, 1e-5, "tiny"),
    (1e-5, 1e-3, "small"),
    (1e-3, 1e-1, "medium"),
    (1e-1, np.inf, "large"),
]
Q5_GAP_COLORS = {"tiny": "C3", "small": "C2", "medium": "C1", "large": "C0"}


def q5_gap_label(gap):
    """Assign gap bin label."""
    for lo, hi, label in Q5_GAP_BINS:
        if lo <= gap < hi:
            return label
    return "large"


def q5_group_traces(data, residual_field):
    """Group Q5 rows into traces: (polytope_id, dir_idx) -> [(log_t, log_residual)].

    residual_field: 'subdiff_log_residual' or 'single_log_residual'.
    """
    traces = defaultdict(list)
    for r in data:
        key = (r["polytope_id"], r["dir_idx"])
        traces[key].append((r["log_t"], r[residual_field]))
    for key in traces:
        traces[key].sort()
    return traces


def plot_q5_convergence(data, filename="gc_q5_convergence.png"):
    """Two-panel convergence: subdiff prediction (left) vs single-orbit (right).

    Traces colored by action gap bin. Shows whether subdiff prediction
    improves convergence at small gap.
    """
    if not data:
        return

    gap_map = {}
    for r in data:
        key = (r["polytope_id"], r["dir_idx"])
        gap_map[key] = r["action_gap"]

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL, sharey=True)
    panels = [
        ("subdiff_log_residual", r"Subdiff: $\min_i(g_i \cdot d)$"),
        ("single_log_residual", r"Single orbit: $g_{\mathrm{best}} \cdot d$"),
    ]

    for ax, (res_field, title) in zip(axes, panels):
        traces = q5_group_traces(data, res_field)

        # Aggregate by gap bin and t
        by_bin_t = defaultdict(lambda: defaultdict(list))
        for key, pts in traces.items():
            gap_label = q5_gap_label(gap_map.get(key, np.inf))
            for lt, lr in pts:
                by_bin_t[gap_label][lt].append(lr)

        for lo, hi, gap_label in Q5_GAP_BINS:
            if gap_label not in by_bin_t:
                continue
            bt = by_bin_t[gap_label]
            t_sorted = sorted(bt.keys())
            medians = [np.median(bt[lt]) for lt in t_sorted]
            n = sum(len(bt[lt]) for lt in t_sorted) // max(1, len(t_sorted))
            ax.plot(t_sorted, medians, color=Q5_GAP_COLORS[gap_label],
                    linewidth=1.5, label=f"gap {gap_label} (n={n})")

        # Reference slope 2
        if traces:
            all_lt = sorted({lt for pts in traces.values() for lt, _ in pts})
            ref_x = np.array(all_lt)
            # Anchor at largest t using the overall median
            all_lr_at_max = []
            for pts in traces.values():
                for lt, lr in pts:
                    if lt == all_lt[-1]:
                        all_lr_at_max.append(lr)
            if all_lr_at_max:
                y0 = np.median(all_lr_at_max)
                x0 = all_lt[-1]
                ax.plot(ref_x, y0 + 2 * (ref_x - x0), "k--", alpha=0.4,
                        linewidth=1, label=r"Slope 2 ($C^2$)")

        ax.set_xlabel(r"$\log_{10} t$")
        ax.set_title(title)

    axes[0].set_ylabel(r"$\log_{10}$ residual")
    axes[0].legend(fontsize=FONT_SIZE_SMALL - 1, loc="upper left")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


def plot_q5_switching(data, filename="gc_q5_switching.png"):
    """Two-panel: orbit switching rate vs t by gap bin (left),
    and switching rate vs action gap at t=0.1 and t=0.01 (right)."""
    if not data:
        return

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    # Left: switching rate vs t, by gap bin
    ax = axes[0]
    by_bin_t = defaultdict(lambda: defaultdict(lambda: [0, 0]))  # [switched, total]
    for r in data:
        gap_label = q5_gap_label(r["action_gap"])
        lt = r["log_t"]
        by_bin_t[gap_label][lt][1] += 1
        if r["orbit_switched"]:
            by_bin_t[gap_label][lt][0] += 1

    for lo, hi, gap_label in Q5_GAP_BINS:
        if gap_label not in by_bin_t:
            continue
        bt = by_bin_t[gap_label]
        t_sorted = sorted(bt.keys())
        rates = [bt[lt][0] / max(1, bt[lt][1]) for lt in t_sorted]
        ax.plot(t_sorted, rates, marker=".", color=Q5_GAP_COLORS[gap_label],
                linewidth=1.5, label=f"gap {gap_label}")

    ax.set_xlabel(r"$\log_{10} t$")
    ax.set_ylabel("Orbit switching rate")
    ax.set_ylim(-0.05, 1.05)
    ax.legend(fontsize=FONT_SIZE_SMALL - 1)

    # Right: switching rate vs gap at a few t values
    ax = axes[1]
    t_targets = [0.1, 0.01, 0.001]
    t_colors = ["C3", "C1", "C0"]

    for t_val, t_col in zip(t_targets, t_colors):
        rows_at_t = [r for r in data if abs(r["t"] - t_val) < t_val * 0.05]
        if not rows_at_t:
            continue
        # Bin by gap (log-spaced)
        gaps = np.array([r["action_gap"] for r in rows_at_t])
        switched = np.array([r["orbit_switched"] for r in rows_at_t])
        log_gaps = np.log10(np.maximum(gaps, 1e-8))
        bin_edges = np.linspace(log_gaps.min() - 0.1, log_gaps.max() + 0.1, 12)
        bin_centers = 0.5 * (bin_edges[:-1] + bin_edges[1:])
        bin_rates = []
        for j in range(len(bin_edges) - 1):
            mask = (log_gaps >= bin_edges[j]) & (log_gaps < bin_edges[j + 1])
            if mask.sum() > 0:
                bin_rates.append(switched[mask].mean())
            else:
                bin_rates.append(np.nan)
        ax.plot(bin_centers, bin_rates, marker=".", color=t_col,
                linewidth=1.5, label=f"$t={t_val}$")

    ax.set_xlabel(r"$\log_{10}$ action gap")
    ax.set_ylabel("Orbit switching rate")
    ax.set_ylim(-0.05, 1.05)
    ax.legend(fontsize=FONT_SIZE_SMALL - 1)

    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


# ============================================================================
# Q5b: Subdifferential at exact switching boundaries (symmetric polytopes)
# ============================================================================

def q5b_short_name(pid):
    """Human-readable short name for Q5b polytope IDs."""
    if pid.startswith("q5b_lp"):
        parts = pid.replace("q5b_lp", "").split("_")
        return f"LP({parts[0]},{parts[1]})"
    elif pid == "q5b_hko2024":
        return "HKO2024"
    elif pid == "q5b_simplex":
        return "Simplex"
    elif pid == "q5b_hypercube":
        return "Hypercube"
    elif pid.startswith("q5b_gorbit"):
        parts = pid.replace("q5b_gorbit_n", "").split("_")
        return f"G-orbit n={parts[0]}"
    return pid


def plot_q5b_boundary(data, filename="gc_q5b_boundary.png"):
    """Selected panels showing subdiff (blue) and single-orbit (red) convergence
    at exact switching boundaries. One panel per polytope type.

    Key test of [prop:capacity-smoothness-classification](b): at exact switching
    boundaries, D_d c = min_i(g_i . d). Slope 2 = correct directional derivative
    with C^2 per-orbit remainder. Slope 1 = wrong prediction (O(t) error).
    """
    if not data:
        return

    by_poly = defaultdict(list)
    for r in data:
        by_poly[r["polytope_id"]].append(r)

    # Select representative polytopes (skip redundant G-orbits)
    # Priority: LP(3,3), simplex, hko2024, LP(5,5), hypercube, LP(4,4)
    # + one G-orbit n=5 if available
    priority = [
        "q5b_lp3_3", "q5b_simplex", "q5b_hko2024",
        "q5b_lp5_5", "q5b_hypercube", "q5b_lp4_4",
    ]
    # Add first G-orbit n=5 with data
    for pid in sorted(by_poly.keys()):
        if pid.startswith("q5b_gorbit_n5") and pid not in priority:
            priority.append(pid)
            break

    selected = [pid for pid in priority if pid in by_poly]
    n_polys = len(selected)
    if n_polys == 0:
        return

    n_cols = min(n_polys, 4)
    n_rows = (n_polys + n_cols - 1) // n_cols
    fig_w = FIGSIZE_TRIPLE[0]
    fig_h = FIGSIZE_TRIPLE[1] * n_rows
    fig, axes_flat = plt.subplots(n_rows, n_cols, figsize=(fig_w, fig_h),
                                  sharey=True, squeeze=False)
    axes = [axes_flat[r][c] for r in range(n_rows) for c in range(n_cols)]

    for idx, pid in enumerate(selected):
        ax = axes[idx]
        prows = by_poly[pid]
        n_orbits = prows[0]["n_orbits"]
        f_count = prows[0]["facet_count"]

        dirs = sorted(set(r["dir_idx"] for r in prows))

        for di in dirs:
            drows = sorted(
                [r for r in prows if r["dir_idx"] == di], key=lambda r: r["log_t"]
            )
            log_ts = [r["log_t"] for r in drows]
            sub_lr = [r["subdiff_log_residual"] for r in drows]
            sin_lr = [r["single_log_residual"] for r in drows]

            label_sub = r"$\min_i(g_i \cdot d)$" if di == 0 and idx == 0 else None
            label_sin = r"$g_{\mathrm{best}} \cdot d$" if di == 0 and idx == 0 else None

            ax.plot(log_ts, sub_lr, color="C0", alpha=0.4, linewidth=1,
                    label=label_sub)
            ax.plot(log_ts, sin_lr, color="C3", alpha=0.4, linewidth=1,
                    label=label_sin)

        # Reference slopes
        all_lt = sorted(set(r["log_t"] for r in prows))
        lr_at_max = [r["subdiff_log_residual"] for r in prows
                     if r["log_t"] == all_lt[-1]]
        if lr_at_max and all_lt:
            y0 = np.median(lr_at_max)
            x0 = all_lt[-1]
            ref_x = np.array(all_lt)
            ax.plot(ref_x, y0 + 2 * (ref_x - x0), "k--", alpha=0.4,
                    linewidth=1, label="Slope 2" if idx == 0 else None)
            ax.plot(ref_x, y0 + 1 * (ref_x - x0), "k:", alpha=0.4,
                    linewidth=1, label="Slope 1" if idx == 0 else None)

        short = q5b_short_name(pid)
        ax.set_title(f"{short}: F={f_count}, {n_orbits} tied",
                     fontsize=FONT_SIZE_SMALL)
        ax.set_xlabel(r"$\log_{10} t$")

    # Hide unused panels
    for idx in range(n_polys, len(axes)):
        axes[idx].set_visible(False)

    axes[0].set_ylabel(r"$\log_{10}$ residual")
    if n_rows > 1:
        axes[n_cols].set_ylabel(r"$\log_{10}$ residual")
    axes[0].legend(fontsize=FONT_SIZE_SMALL - 1, loc="upper left")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


# ============================================================================
# Main
# ============================================================================

def main():
    q5 = load_jsonl("gradient-correctness-q5-subdiff.jsonl")
    q5b = load_jsonl("gradient-correctness-q5b-symmetric.jsonl")

    print(f"Loaded: Q5={len(q5)}, Q5b={len(q5b)} rows")

    plot_q5_convergence(q5, "gc_q5_convergence.png")
    plot_q5_switching(q5, "gc_q5_switching.png")
    plot_q5b_boundary(q5b, "gc_q5b_boundary.png")


if __name__ == "__main__":
    main()
