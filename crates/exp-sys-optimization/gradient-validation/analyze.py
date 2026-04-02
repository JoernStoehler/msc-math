"""
Goal: Analyze first-order prediction test for gradient correctness.
Input:
  - experiments/gradient-correctness/gradient-correctness-q1-generic.jsonl
  - experiments/gradient-correctness/gradient-correctness-q2-nongeneric.jsonl
  - experiments/gradient-correctness/gradient-correctness-q3-degeneracy.jsonl
  - experiments/gradient-correctness/gradient-correctness-q4-redundant.jsonl
  - experiments/gradient-correctness/gradient-correctness-q5-subdiff.jsonl
  - experiments/gradient-correctness/gradient-correctness-q5b-symmetric.jsonl
Output:
  - experiments/gradient-correctness/gc_convergence.png    (Q1 log-log convergence)
  - experiments/gradient-correctness/gc_slopes.png         (slope distributions Q1+Q2)
  - experiments/gradient-correctness/gc_q3_gap.png         (Q3 action gap vs slope)
  - experiments/gradient-correctness/gc_q4_delta.png       (Q4 delta vs slope)
  - experiments/gradient-correctness/gc_q5_convergence.png (Q5 subdiff vs single-orbit convergence)
  - experiments/gradient-correctness/gc_q5_switching.png   (Q5 orbit switching rate vs t and gap)
  - experiments/gradient-correctness/gc_q5b_boundary.png   (Q5b subdiff at exact switching boundaries)
  - experiments/gradient-correctness/gc_summary.tex        (summary table)
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
    FIGSIZE_DUAL,
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
            x0 = t_sorted[-1]  # log_t at largest t (≈ -1)
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
# Figure 3: Q3 — action gap vs fitted slope
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
# Figure 4: Q4 — delta vs fitted slope
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
    boundaries, D_d c = min_i(g_i · d). Slope 2 = correct directional derivative
    with C² per-orbit remainder. Slope 1 = wrong prediction (O(t) error).
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
# Summary table
# ============================================================================

def write_summary(q1, q2, q3, q4):
    """Print and save summary statistics of fitted slopes per phase x target."""
    phases = [
        ("Q1 generic", q1),
        ("Q2 non-generic", q2),
        ("Q3 near-degenerate", q3),
        ("Q4 barely-cutting", q4),
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
    q3 = load_jsonl("gradient-correctness-q3-degeneracy.jsonl")
    q4 = load_jsonl("gradient-correctness-q4-redundant.jsonl")
    q5 = load_jsonl("gradient-correctness-q5-subdiff.jsonl")
    q5b = load_jsonl("gradient-correctness-q5b-symmetric.jsonl")

    print(f"Loaded: Q1={len(q1)}, Q2={len(q2)}, Q3={len(q3)}, Q4={len(q4)}, Q5={len(q5)}, Q5b={len(q5b)} rows")

    plot_convergence(q1, "gc_convergence.png")
    plot_slopes(q1, q2, "gc_slopes.png")
    plot_q3_gap(q3, "gc_q3_gap.png")
    plot_q4_delta(q4, "gc_q4_delta.png")
    plot_q5_convergence(q5, "gc_q5_convergence.png")
    plot_q5_switching(q5, "gc_q5_switching.png")
    plot_q5b_boundary(q5b, "gc_q5b_boundary.png")
    write_summary(q1, q2, q3, q4)


if __name__ == "__main__":
    main()
