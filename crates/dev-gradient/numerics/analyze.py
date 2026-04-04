"""
Goal: Analyze first-order prediction test for gradient correctness (Q1 generic, Q2 non-generic).
Input:
  - crates/dev-gradient/numerics/gradient-correctness-q1-generic.jsonl
  - crates/dev-gradient/numerics/gradient-correctness-q2-nongeneric.jsonl
Output:
  - crates/dev-gradient/numerics/gc_convergence.png    (Q1 log-log convergence)
  - crates/dev-gradient/numerics/gc_slopes.png         (slope distributions Q1+Q2)
  - crates/dev-gradient/numerics/gc_summary.tex        (summary table)
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
    FIGSIZE_TRIPLE,
    FONT_SIZE_SMALL,
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


def compute_all_slopes(data, r2_threshold=0.5):
    """Compute fitted slopes for all traces, filtered by R^2.

    Returns dict: target -> list of slopes.
    """
    traces = group_traces(data)
    result = defaultdict(list)
    for key, pts in traces.items():
        target = key[2]
        slope, r2 = fit_slope(pts)
        if not np.isnan(slope) and r2 > r2_threshold:
            result[target].append(slope)
    return result


# ============================================================================
# Figure 1: Q1 Convergence (the main result)
# ============================================================================

def plot_convergence(data, filename="gc_convergence.png"):
    """Log-log plot of residual vs t, showing convergence rate.

    One panel per target. Thin gray lines = individual traces,
    thick colored line = median, shaded band = IQR.
    Reference lines at slope 1 and slope 2.
    """
    if not data:
        return
    traces = group_traces(data)

    fig, axes = plt.subplots(1, 3, figsize=FIGSIZE_TRIPLE, sharey=True)

    for ax, target in zip(axes, TARGETS):
        target_traces = {k: v for k, v in traces.items() if k[2] == target}

        # Aggregate: for each log_t, collect all log_residual values
        by_t = defaultdict(list)
        for pts in target_traces.values():
            for lt, lr in pts:
                by_t[lt].append(lr)

        t_sorted = sorted(by_t.keys())
        median_r = [np.median(by_t[lt]) for lt in t_sorted]
        q25 = [np.percentile(by_t[lt], 25) for lt in t_sorted]
        q75 = [np.percentile(by_t[lt], 75) for lt in t_sorted]

        # Individual traces (subsample for readability)
        trace_list = list(target_traces.values())
        rng = np.random.default_rng(42)
        n_show = min(100, len(trace_list))
        if len(trace_list) > n_show:
            indices = rng.choice(len(trace_list), n_show, replace=False)
        else:
            indices = range(len(trace_list))
        for idx in indices:
            pts = trace_list[idx]
            ax.plot([p[0] for p in pts], [p[1] for p in pts],
                    color="gray", alpha=0.05, linewidth=0.5)

        # Median + IQR
        ax.plot(t_sorted, median_r, color=TARGET_COLORS[target], linewidth=2,
                label="Median")
        ax.fill_between(t_sorted, q25, q75, color=TARGET_COLORS[target], alpha=0.2)

        # Reference slopes anchored at the largest t (rightmost point)
        if t_sorted and median_r:
            x0 = t_sorted[-1]  # log_t at largest t (approx -1)
            y0 = median_r[-1]  # median residual there
            ref_x = np.array(t_sorted)
            ax.plot(ref_x, y0 + 2 * (ref_x - x0), "k--", alpha=0.4,
                    linewidth=1, label=r"Slope 2 ($C^2$)")
            ax.plot(ref_x, y0 + 1 * (ref_x - x0), "k:", alpha=0.4,
                    linewidth=1, label="Slope 1")

        ax.set_xlabel(r"$\log_{10} t$")
        ax.set_title(TARGET_LABELS[target])

    axes[0].set_ylabel(r"$\log_{10} |f(a{+}td) - f(a) - t \, g \cdot d|$")
    axes[0].legend(fontsize=FONT_SIZE_SMALL, loc="upper left")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


# ============================================================================
# Figure 2: Slope distributions (Q1 + Q2)
# ============================================================================

def plot_slopes(q1, q2, filename="gc_slopes.png"):
    """Histogram of fitted log-log slopes, one panel per target."""
    fig, axes = plt.subplots(1, 3, figsize=FIGSIZE_TRIPLE, sharey=True)

    for ax, target in zip(axes, TARGETS):
        slopes_q1 = compute_all_slopes(q1).get(target, []) if q1 else []
        slopes_q2 = compute_all_slopes(q2).get(target, []) if q2 else []

        bins = np.linspace(0, 3, 31)
        if slopes_q1:
            ax.hist(slopes_q1, bins=bins, alpha=0.6, color="C0",
                    label=f"Q1 generic (n={len(slopes_q1)})")
        if slopes_q2:
            ax.hist(slopes_q2, bins=bins, alpha=0.6, color="C1",
                    label=f"Q2 non-generic (n={len(slopes_q2)})")

        ax.axvline(2.0, color="k", linestyle="--", alpha=0.5, label=r"Slope $= 2$")
        ax.set_xlabel("Fitted log-log slope")
        ax.set_title(TARGET_LABELS[target])
        ax.legend(fontsize=FONT_SIZE_SMALL - 1)

    axes[0].set_ylabel("Count")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / filename)
    plt.close(fig)
    print(f"Saved {filename}")


# ============================================================================
# Summary table
# ============================================================================

def write_summary(q1, q2):
    """Print and save summary statistics of fitted slopes per phase x target."""
    phases = [
        ("Q1 generic", q1),
        ("Q2 non-generic", q2),
    ]

    print(f"\n{'Phase':<20} {'Target':<10} {'Median':>8} {'P25':>8} {'P75':>8} {'n':>6}")
    print("-" * 56)

    tex_lines = []
    tex_lines.append(r"\begin{tabular}{l l r r r}")
    tex_lines.append(r"\toprule")
    tex_lines.append(r"Phase & Target & Median slope & [P25, P75] & $n$ \\")
    tex_lines.append(r"\midrule")

    for phase_name, phase_data in phases:
        if not phase_data:
            continue
        slopes_by_target = compute_all_slopes(phase_data)
        for target in TARGETS:
            slopes = slopes_by_target.get(target, [])
            if not slopes:
                continue
            med = np.median(slopes)
            p25 = np.percentile(slopes, 25)
            p75 = np.percentile(slopes, 75)
            n = len(slopes)
            print(f"{phase_name:<20} {target:<10} {med:>8.2f} {p25:>8.2f} {p75:>8.2f} {n:>6}")
            tex_lines.append(
                f"{phase_name} & {target} & {med:.2f} & [{p25:.2f}, {p75:.2f}] & {n} \\\\"
            )
        tex_lines.append(r"\midrule")

    if tex_lines[-1] == r"\midrule":
        tex_lines[-1] = r"\bottomrule"
    tex_lines.append(r"\end{tabular}")

    path = EXPERIMENT_DIR / "gc_summary.tex"
    with open(path, "w") as f:
        f.write("\n".join(tex_lines) + "\n")
    print(f"\nSaved gc_summary.tex")


# ============================================================================
# Main
# ============================================================================

def main():
    q1 = load_jsonl("gradient-correctness-q1-generic.jsonl")
    q2 = load_jsonl("gradient-correctness-q2-nongeneric.jsonl")

    print(f"Loaded: Q1={len(q1)}, Q2={len(q2)} rows")

    plot_convergence(q1, "gc_convergence.png")
    plot_slopes(q1, q2, "gc_slopes.png")
    write_summary(q1, q2)


if __name__ == "__main__":
    main()
