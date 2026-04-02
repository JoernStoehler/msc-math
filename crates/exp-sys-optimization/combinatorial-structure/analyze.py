"""
Goal: Analyze combinatorial boundary events and cell geometry.
Input: crates/exp-sys-optimization/combinatorial-structure/combinatorial-boundaries-{profiling,anatomy,crossing,gradient,convexity}.jsonl
Output: crates/exp-sys-optimization/combinatorial-structure/*.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent.parent))
from figure_config import (
    setup, FIGSIZE_SINGLE, FIGSIZE_DUAL, SCATTER_SIZE,
    FONT_SIZE_SMALL,
)
setup()

import matplotlib.pyplot as plt

# ============================================================================
# Colors
# ============================================================================

EVENT_COLORS = {
    "incidence_flip": "#2196F3",
    "omega_flip": "#FF9800",
    "dual_vertex_degen": "#F44336",
    "unbounded": "#9E9E9E",
}

DIR_COLORS = {
    "gradient": "#E91E63",
    "neg_gradient": "#9C27B0",
    "dense_random": "#2196F3",
    "facet": "#4CAF50",
}

# ============================================================================
# Data loading
# ============================================================================

def load_jsonl(filename):
    path = EXPERIMENT_DIR / filename
    rows = []
    with open(path) as f:
        for line in f:
            if line.strip():
                rows.append(json.loads(line))
    return rows


profiling = load_jsonl("combinatorial-boundaries-profiling.jsonl")
anatomy = load_jsonl("combinatorial-boundaries-anatomy.jsonl")
crossing = load_jsonl("combinatorial-boundaries-crossing.jsonl")
gradient = load_jsonl("combinatorial-boundaries-gradient.jsonl")
convexity = load_jsonl("combinatorial-boundaries-convexity.jsonl")

print(f"Loaded: {len(profiling)} profiling, {len(anatomy)} anatomy, "
      f"{len(crossing)} crossing, {len(gradient)} gradient, {len(convexity)} convexity rows")

# ============================================================================
# Figure 1: Event type breakdown (anatomy, global probes)
# ============================================================================

event_counts = {}
for row in anatomy:
    et = row["event_type"]
    event_counts[et] = event_counts.get(et, 0) + 1

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
labels = sorted(event_counts.keys())
values = [event_counts[l] for l in labels]
colors = [EVENT_COLORS.get(l, "#999") for l in labels]
bars = ax.bar(labels, values, color=colors)
for bar, val in zip(bars, values):
    ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 10,
            f"{val}\n({100 * val / sum(values):.0f}%)",
            ha="center", va="bottom", fontsize=FONT_SIZE_SMALL)
ax.set_ylabel("Count")
ax.set_title("Boundary event types (global probes)")
fig.savefig(EXPERIMENT_DIR / "boundary_event_types.png")
plt.close()
print("  boundary_event_types.png")

# ============================================================================
# Figure 2: t_max distribution by event type (anatomy)
# ============================================================================

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
for et in ["incidence_flip", "omega_flip", "dual_vertex_degen"]:
    vals = [row["t_max"] for row in anatomy if row["event_type"] == et and row["t_max"] < 100]
    if vals:
        ax.hist(vals, bins=50, alpha=0.7, label=et.replace("_", " "),
                color=EVENT_COLORS.get(et, "#999"))
ax.set_xlabel(r"$t_{\max}$")
ax.set_ylabel("Count")
ax.set_title(r"Distribution of $t_{\max}$ (first boundary distance)")
ax.legend()
fig.savefig(EXPERIMENT_DIR / "boundary_tmax_distribution.png")
plt.close()
print("  boundary_tmax_distribution.png")

# ============================================================================
# Figure 3: t_max by direction type (box plot, anatomy)
# ============================================================================

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
dir_types = ["gradient", "neg_gradient", "dense_random"]
data_by_dir = {}
for row in anatomy:
    dt = row["direction_type"]
    if row["event_type"] != "unbounded" and row["t_max"] < 100:
        data_by_dir.setdefault(dt, []).append(row["t_max"])

present = [dt for dt in dir_types if dt in data_by_dir]
box_data = [data_by_dir[dt] for dt in present]
bp = ax.boxplot(box_data, tick_labels=[dt.replace("_", "\n") for dt in present],
                patch_artist=True, showfliers=False)
for patch, dt in zip(bp["boxes"], present):
    patch.set_facecolor(DIR_COLORS.get(dt, "#999"))
    patch.set_alpha(0.7)
ax.set_ylabel(r"$t_{\max}$")
ax.set_title(r"$t_{\max}$ by direction type (outliers hidden)")
fig.savefig(EXPERIMENT_DIR / "boundary_tmax_by_direction.png")
plt.close()
print("  boundary_tmax_by_direction.png")

# ============================================================================
# Figure 4: t_max vs facet count (anatomy)
# ============================================================================

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
tmax_by_f = {}
for row in anatomy:
    if row["event_type"] != "unbounded" and row["t_max"] < 100:
        f = row["facet_count"]
        tmax_by_f.setdefault(f, []).append(row["t_max"])

f_vals = sorted(tmax_by_f.keys())
medians = [np.median(tmax_by_f[f]) for f in f_vals]
q25 = [np.percentile(tmax_by_f[f], 25) for f in f_vals]
q75 = [np.percentile(tmax_by_f[f], 75) for f in f_vals]

ax.plot(f_vals, medians, "o-", color="#2196F3", label="Median")
ax.fill_between(f_vals, q25, q75, alpha=0.2, color="#2196F3", label="IQR")
ax.set_xlabel("Facet count " + r"$F$")
ax.set_ylabel(r"$t_{\max}$")
ax.set_title(r"Boundary distance vs $F$ (first boundary)")
ax.legend()
fig.savefig(EXPERIMENT_DIR / "boundary_tmax_vs_F.png")
plt.close()
print("  boundary_tmax_vs_F.png")

# ============================================================================
# Figure 5: sys continuity scatter (crossing)
# ============================================================================

ok_crossing = [row for row in crossing if row["construction_ok_after"]]

if ok_crossing:
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    sys_b = [row["sys_before"] for row in ok_crossing]
    sys_a = [row["sys_after"] for row in ok_crossing]
    ax.scatter(sys_b, sys_a, s=SCATTER_SIZE, alpha=0.5, c="#2196F3", edgecolors="none")
    lims = [min(min(sys_b), min(sys_a)), max(max(sys_b), max(sys_a))]
    ax.plot(lims, lims, "k--", lw=0.8, alpha=0.5, label="Continuity line")
    ax.set_xlabel(r"$\mathrm{sys}$ before boundary")
    ax.set_ylabel(r"$\mathrm{sys}$ after boundary")
    ax.set_title(r"$\mathrm{sys}$ continuity at boundaries")
    ax.legend()
    fig.savefig(EXPERIMENT_DIR / "boundary_sys_continuity.png")
    plt.close()
    print("  boundary_sys_continuity.png")

# ============================================================================
# Figure 6: Gradient angle change histogram (gradient)
# ============================================================================

if gradient:
    angles = [row["gradient_angle_change_deg"] for row in gradient
              if not np.isnan(row["gradient_angle_change_deg"])]
    if angles:
        fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
        ax.hist(angles, bins=50, color="#E91E63", alpha=0.7)
        ax.set_xlabel("Gradient angle change (degrees)")
        ax.set_ylabel("Count")
        ax.set_title("Gradient direction change at first boundary")
        ax.axvline(np.median(angles), color="red", ls="--", lw=1,
                   label=f"Median: {np.median(angles):.4f}" + r"$^\circ$")
        ax.legend()
        fig.savefig(EXPERIMENT_DIR / "boundary_gradient_angle.png")
        plt.close()
        print("  boundary_gradient_angle.png")

# ============================================================================
# Figure 7: CDF of t_max by direction type (anatomy)
# ============================================================================

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
for dt in ["gradient", "neg_gradient", "dense_random"]:
    vals = sorted(row["t_max"] for row in anatomy
                  if row["direction_type"] == dt
                  and row["event_type"] != "unbounded"
                  and row["t_max"] < 100)
    if vals:
        cdf = np.arange(1, len(vals) + 1) / len(vals)
        ax.plot(vals, cdf, label=dt.replace("_", " "),
                color=DIR_COLORS.get(dt, "#999"))
ax.set_xlabel(r"$t_{\max}$")
ax.set_ylabel("CDF")
ax.set_title("Boundary encounter CDF by direction type")
ax.legend()
ax.set_xlim(0, min(10, ax.get_xlim()[1]))
fig.savefig(EXPERIMENT_DIR / "boundary_density_cdf.png")
plt.close()
print("  boundary_density_cdf.png")

# ============================================================================
# Figure 8: Per-facet cell width — orbit facets vs non-orbit facets (profiling)
# ============================================================================

orbit_tmax = [r["t_max"] for r in profiling if r["facet_in_orbit"] and r["t_max"] < 100]
non_orbit_tmax = [r["t_max"] for r in profiling if not r["facet_in_orbit"] and r["t_max"] < 100]

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
bp = ax.boxplot([orbit_tmax, non_orbit_tmax],
                tick_labels=["Orbit\nfacets", "Non-orbit\nfacets"],
                patch_artist=True, showfliers=False)
bp["boxes"][0].set_facecolor("#E91E63")
bp["boxes"][0].set_alpha(0.7)
bp["boxes"][1].set_facecolor("#2196F3")
bp["boxes"][1].set_alpha(0.7)
ax.set_ylabel(r"$t_{\max}$ (cell width in facet $\mathbb{R}^4$)")
ax.set_title("Cell width: orbit facets vs non-orbit facets")
# Add median annotations
for i, data in enumerate([orbit_tmax, non_orbit_tmax], 1):
    med = np.median(data)
    ax.text(i, med, f"  {med:.3f}", va="center", fontsize=FONT_SIZE_SMALL)
fig.savefig(EXPERIMENT_DIR / "cell_orbit_vs_nonorbit.png")
plt.close()
print("  cell_orbit_vs_nonorbit.png")

# ============================================================================
# Figure 9: Per-facet cell width by F (profiling)
# ============================================================================

fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

for ax_idx, (in_orbit, label, color) in enumerate([
    (True, "Orbit facets", "#E91E63"),
    (False, "Non-orbit facets", "#2196F3"),
]):
    ax = axes[ax_idx]
    tmax_by_f = defaultdict(list)
    for r in profiling:
        if r["facet_in_orbit"] == in_orbit and r["t_max"] < 100:
            tmax_by_f[r["facet_count"]].append(r["t_max"])
    f_vals = sorted(tmax_by_f.keys())
    if f_vals:
        medians = [np.median(tmax_by_f[f]) for f in f_vals]
        q25 = [np.percentile(tmax_by_f[f], 25) for f in f_vals]
        q75 = [np.percentile(tmax_by_f[f], 75) for f in f_vals]
        ax.plot(f_vals, medians, "o-", color=color, label="Median")
        ax.fill_between(f_vals, q25, q75, alpha=0.2, color=color, label="IQR")
    ax.set_xlabel(r"$F$")
    ax.set_ylabel(r"$t_{\max}$")
    ax.set_title(label)
    ax.legend()

fig.suptitle("Cell width by facet count")
fig.tight_layout()
fig.savefig(EXPERIMENT_DIR / "cell_width_by_F.png")
plt.close()
print("  cell_width_by_F.png")

# ============================================================================
# Figure 10: Anisotropy histogram (profiling)
# ============================================================================

facet_groups = defaultdict(list)
for r in profiling:
    if r["t_max"] < 100:
        facet_groups[(r["polytope_name"], r["facet_index"])].append(r["t_max"])

anisotropies = []
for key, vals in facet_groups.items():
    if len(vals) >= 2 and min(vals) > 0:
        anisotropies.append(max(vals) / min(vals))

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
ax.hist(anisotropies, bins=50, color="#4CAF50", alpha=0.7)
ax.axvline(np.median(anisotropies), color="red", ls="--", lw=1,
           label=f"Median: {np.median(anisotropies):.1f}")
ax.set_xlabel(r"Anisotropy (max / min $t_{\max}$ within facet $\mathbb{R}^4$)")
ax.set_ylabel("Count")
ax.set_title("Cell anisotropy per facet")
ax.legend()
fig.savefig(EXPERIMENT_DIR / "cell_anisotropy.png")
plt.close()
print("  cell_anisotropy.png")

# ============================================================================
# Figure 11: Convexity failure rates (convexity)
# ============================================================================

ok_conv = [r for r in convexity if r["midpoint_construction_ok"]]

if ok_conv:
    # By type: same-facet vs cross-facet
    same_facet = [r for r in ok_conv if r["dir1_facet"] == r["dir2_facet"]]
    cross_facet = [r for r in ok_conv if r["dir1_facet"] != r["dir2_facet"]]

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    # Left: overall convexity failure rates (three checks)
    ax = axes[0]
    categories = ["Incidence", r"$\omega_0$ signs", "Transitions"]

    def fail_rate(rows, key):
        return 100 * (1 - sum(1 for r in rows if r[key]) / max(len(rows), 1))

    all_rates = [fail_rate(ok_conv, "midpoint_same_incidence"),
                 fail_rate(ok_conv, "midpoint_same_omega_signs"),
                 fail_rate(ok_conv, "midpoint_same_transitions")]

    bars = ax.bar(categories, all_rates,
                  color=["#2196F3", "#FF9800", "#F44336"], alpha=0.7)
    for bar, rate in zip(bars, all_rates):
        ax.text(bar.get_x() + bar.get_width() / 2, rate + 0.5,
                f"{rate:.1f}%", ha="center", va="bottom", fontsize=FONT_SIZE_SMALL)
    ax.set_ylabel("Failure rate (%)")
    ax.set_title("Midpoint type change rates")

    # Right: transition failure by F
    ax = axes[1]
    conv_by_f = defaultdict(lambda: {"total": 0, "trans_fail": 0, "omega_fail": 0})
    for r in ok_conv:
        f = r["facet_count"]
        conv_by_f[f]["total"] += 1
        if not r["midpoint_same_transitions"]:
            conv_by_f[f]["trans_fail"] += 1
        if not r["midpoint_same_omega_signs"]:
            conv_by_f[f]["omega_fail"] += 1

    f_vals = sorted(conv_by_f.keys())
    trans_rates = [100 * conv_by_f[f]["trans_fail"] / conv_by_f[f]["total"] for f in f_vals]
    omega_rates = [100 * conv_by_f[f]["omega_fail"] / conv_by_f[f]["total"] for f in f_vals]

    ax.plot(f_vals, trans_rates, "o-", color="#F44336", label="Transition matrix")
    ax.plot(f_vals, omega_rates, "s--", color="#FF9800", alpha=0.6, label=r"$\omega_0$ signs")
    ax.set_xlabel(r"$F$")
    ax.set_ylabel("Failure rate (%)")
    ax.set_title("Non-convexity by facet count")
    ax.legend()

    fig.suptitle("Cell convexity testing")
    fig.tight_layout()
    fig.savefig(EXPERIMENT_DIR / "cell_convexity.png")
    plt.close()
    print("  cell_convexity.png")

# ============================================================================
# Figure 12: Orbit gap distribution (anatomy)
# ============================================================================

orbit_gaps = [r["orbit_gap"] for r in anatomy if r["orbit_gap"] is not None]
# Deduplicate per polytope (gap is a polytope property, not per-direction)
polytope_gaps = {}
for r in anatomy:
    if r["orbit_gap"] is not None:
        polytope_gaps[r["polytope_name"]] = r["orbit_gap"]
unique_gaps = list(polytope_gaps.values())

if unique_gaps:
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.hist(unique_gaps, bins=40, color="#9C27B0", alpha=0.7)
    ax.axvline(np.median(unique_gaps), color="red", ls="--", lw=1,
               label=f"Median: {np.median(unique_gaps):.4f}")
    ax.set_xlabel("Orbit gap (second best − best action)")
    ax.set_ylabel("Count")
    ax.set_title(rf"Orbit action gap ({len(unique_gaps)} polytopes with $\geq 2$ orbits)")
    ax.legend()
    fig.savefig(EXPERIMENT_DIR / "orbit_gap_distribution.png")
    plt.close()
    print("  orbit_gap_distribution.png")

# ============================================================================
# Figure 13: Orbit gap vs orbit switch rate (crossing + anatomy)
# ============================================================================

# Match crossing rows to their polytope's orbit gap
polytope_orbit_gap = {}
polytope_n_orbits = {}
for r in anatomy:
    polytope_orbit_gap[r["polytope_name"]] = r["orbit_gap"]
    polytope_n_orbits[r["polytope_name"]] = r["n_valid_orbits"]

ok_cross = [r for r in crossing if r["construction_ok_after"]]
if ok_cross:
    # Bin polytopes by orbit gap into quartiles
    gaps_for_crossing = []
    switches_for_crossing = []
    for r in ok_cross:
        gap = polytope_orbit_gap.get(r["polytope_name"])
        if gap is not None:
            gaps_for_crossing.append(gap)
            switches_for_crossing.append(1 if r["orbit_changed"] else 0)

    if gaps_for_crossing:
        fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
        gaps_arr = np.array(gaps_for_crossing)
        switches_arr = np.array(switches_for_crossing)

        # Bin by gap quartiles
        quartiles = np.percentile(gaps_arr, [25, 50, 75])
        bins = [0, quartiles[0], quartiles[1], quartiles[2], gaps_arr.max() + 1]
        bin_labels = [f"Q1\n≤{quartiles[0]:.3f}", f"Q2\n≤{quartiles[1]:.3f}",
                      f"Q3\n≤{quartiles[2]:.3f}", f"Q4\n>{quartiles[2]:.3f}"]
        rates = []
        for i in range(4):
            mask = (gaps_arr >= bins[i]) & (gaps_arr < bins[i+1])
            if mask.sum() > 0:
                rates.append(100 * switches_arr[mask].mean())
            else:
                rates.append(0)

        bars = ax.bar(range(4), rates, color="#9C27B0", alpha=0.7)
        ax.set_xticks(range(4))
        ax.set_xticklabels(bin_labels)
        for bar, rate in zip(bars, rates):
            if rate > 0:
                ax.text(bar.get_x() + bar.get_width() / 2, rate + 0.5,
                        f"{rate:.1f}%", ha="center", va="bottom", fontsize=FONT_SIZE_SMALL)
        ax.set_xlabel("Orbit gap quartile")
        ax.set_ylabel("Orbit switch rate (%)")
        ax.set_title("Does small orbit gap predict orbit switches?")
        fig.savefig(EXPERIMENT_DIR / "orbit_gap_vs_switch.png")
        plt.close()
        print("  orbit_gap_vs_switch.png")

# ============================================================================
# Figure 14: Gradient-cell alignment (profiling + anatomy)
# ============================================================================

# For each polytope, get the gradient per-facet magnitudes and the per-facet median t_max
polytope_gradient = {}
for r in anatomy:
    if r["direction_type"] == "gradient":
        polytope_gradient[r["polytope_name"]] = r  # one gradient row per polytope

facet_median_tmax = defaultdict(dict)
for r in profiling:
    if r["t_max"] < 100:
        key = (r["polytope_name"], r["facet_index"])
        facet_median_tmax[r["polytope_name"]][r["facet_index"]] = \
            facet_median_tmax[r["polytope_name"]].get(r["facet_index"], [])
        facet_median_tmax[r["polytope_name"]][r["facet_index"]].append(r["t_max"])

# Compute median per facet
for pname in facet_median_tmax:
    for k in facet_median_tmax[pname]:
        vals = facet_median_tmax[pname][k]
        facet_median_tmax[pname][k] = np.median(vals)

# Now correlate: for each polytope + facet, do we have both gradient magnitude and cell width?
# We need the actual gradient components per facet — those aren't in the JSONL directly.
# Instead, use the gradient t_max (boundary distance in gradient direction) as a proxy:
# if gradient points toward narrow cells, gradient t_max should be small when cells are narrow.
# A simpler approach: just correlate per-polytope min cell width with gradient t_max.
grad_tmax = []
min_cell_width = []
for r in anatomy:
    if r["direction_type"] == "gradient" and r["t_max"] < 100:
        pname = r["polytope_name"]
        if pname in facet_median_tmax:
            widths = list(facet_median_tmax[pname].values())
            if widths:
                grad_tmax.append(r["t_max"])
                min_cell_width.append(min(widths))

if grad_tmax and min_cell_width:
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.scatter(min_cell_width, grad_tmax, s=SCATTER_SIZE, alpha=0.5,
               c="#E91E63", edgecolors="none")
    ax.set_xlabel("Min per-facet median cell width")
    ax.set_ylabel(r"Gradient direction $t_{\max}$")
    ax.set_title("Gradient boundary distance vs narrowest cell width")

    # Correlation
    corr = np.corrcoef(min_cell_width, grad_tmax)[0, 1]
    ax.text(0.05, 0.95, f"r = {corr:.3f}", transform=ax.transAxes, va="top",
            fontsize=FONT_SIZE_SMALL)
    fig.savefig(EXPERIMENT_DIR / "gradient_cell_alignment.png")
    plt.close()
    print("  gradient_cell_alignment.png")

# ============================================================================
# Figure 15: Event type breakdown in profiling (per-facet)
# ============================================================================

prof_event_counts = {}
for row in profiling:
    et = row["event_type"]
    prof_event_counts[et] = prof_event_counts.get(et, 0) + 1

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
labels = sorted(prof_event_counts.keys())
values = [prof_event_counts[l] for l in labels]
colors = [EVENT_COLORS.get(l, "#999") for l in labels]
bars = ax.bar(labels, values, color=colors)
for bar, val in zip(bars, values):
    ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 10,
            f"{val}\n({100 * val / sum(values):.0f}%)",
            ha="center", va="bottom", fontsize=FONT_SIZE_SMALL)
ax.set_ylabel("Count")
ax.set_title("Boundary event types (per-facet probes)")
fig.savefig(EXPERIMENT_DIR / "profiling_event_types.png")
plt.close()
print("  profiling_event_types.png")

# ============================================================================
# Summary statistics
# ============================================================================

print("\n=== Summary Statistics ===")
print(f"Polytopes analyzed: {len(set(row['polytope_name'] for row in anatomy))}")
print(f"Profiling probes: {len(profiling)}")
print(f"Global direction probes: {len(anatomy)}")

# Profiling: orbit vs non-orbit
orbit_tmax_vals = [r["t_max"] for r in profiling if r["facet_in_orbit"] and r["t_max"] < 100]
non_orbit_tmax_vals = [r["t_max"] for r in profiling if not r["facet_in_orbit"] and r["t_max"] < 100]
print(f"\nPer-facet cell width (t_max):")
print(f"  Orbit facets:     median={np.median(orbit_tmax_vals):.4f} "
      f"(n={len(orbit_tmax_vals)})")
print(f"  Non-orbit facets: median={np.median(non_orbit_tmax_vals):.4f} "
      f"(n={len(non_orbit_tmax_vals)})")
print(f"  Anisotropy:       median={np.median(anisotropies):.1f}x "
      f"(max={max(anisotropies):.0f}x)")

# Convexity
if ok_conv:
    omega_fail_rate = 100 * (1 - sum(1 for r in ok_conv if r["midpoint_same_omega_signs"]) / len(ok_conv))
    trans_fail_rate = 100 * (1 - sum(1 for r in ok_conv if r["midpoint_same_transitions"]) / len(ok_conv))
    incidence_fail_rate = 100 * (1 - sum(1 for r in ok_conv if r["midpoint_same_incidence"]) / len(ok_conv))
    print(f"\nConvexity (midpoint test, n={len(ok_conv)}):")
    print(f"  Incidence failure:    {incidence_fail_rate:.1f}%")
    print(f"  ω₀ sign failure:      {omega_fail_rate:.1f}%")
    print(f"  Transition failure:   {trans_fail_rate:.1f}%")

# Orbit gap
if unique_gaps:
    print(f"\nOrbit gap ({len(unique_gaps)} polytopes with ≥2 orbits):")
    print(f"  min={min(unique_gaps):.6f}, median={np.median(unique_gaps):.4f}, max={max(unique_gaps):.4f}")

# Crossing
if ok_cross:
    orbit_switch_count = sum(1 for r in ok_cross if r["orbit_changed"])
    abs_ds = [abs(r["delta_sys"]) for r in ok_cross]
    print(f"\nCrossing evaluation: {len(ok_cross)}/{len(crossing)} succeeded")
    print(f"  |delta_sys|: max={max(abs_ds):.2e}")
    print(f"  Orbit switches: {orbit_switch_count}/{len(ok_cross)} "
          f"({100 * orbit_switch_count / len(ok_cross):.1f}%)")

# Gradient
if gradient:
    angles_all = [r["gradient_angle_change_deg"] for r in gradient
                  if not np.isnan(r["gradient_angle_change_deg"])]
    if angles_all:
        print(f"\nGradient angle change: median={np.median(angles_all):.4f}°, "
              f"max={max(angles_all):.1f}°")

print("\nDone.")
