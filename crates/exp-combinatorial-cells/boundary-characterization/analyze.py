"""
Goal: Analyze boundary anatomy, crossing evaluation, and orbit gap.
Input: crates/exp-combinatorial-cells/boundary-characterization/combinatorial-boundaries-{anatomy,crossing}.jsonl
Output: crates/exp-combinatorial-cells/boundary-characterization/*.png
"""

import json
import sys
from pathlib import Path
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent.parent))
from figure_config import (
    setup, FIGSIZE_SINGLE, SCATTER_SIZE,
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


anatomy = load_jsonl("combinatorial-boundaries-anatomy.jsonl")
crossing = load_jsonl("combinatorial-boundaries-crossing.jsonl")

print(f"Loaded: {len(anatomy)} anatomy, {len(crossing)} crossing rows")

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
# Figure 6: CDF of t_max by direction type (anatomy)
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
# Figure 7: Orbit gap distribution (anatomy)
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
# Figure 8: Orbit gap vs orbit switch rate (crossing + anatomy)
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
# Summary statistics
# ============================================================================

print("\n=== Summary Statistics ===")
print(f"Polytopes analyzed: {len(set(row['polytope_name'] for row in anatomy))}")
print(f"Global direction probes: {len(anatomy)}")

# Crossing
if ok_cross:
    orbit_switch_count = sum(1 for r in ok_cross if r["orbit_changed"])
    abs_ds = [abs(r["delta_sys"]) for r in ok_cross]
    print(f"\nCrossing evaluation: {len(ok_cross)}/{len(crossing)} succeeded")
    print(f"  |delta_sys|: max={max(abs_ds):.2e}")
    print(f"  Orbit switches: {orbit_switch_count}/{len(ok_cross)} "
          f"({100 * orbit_switch_count / len(ok_cross):.1f}%)")

# Orbit gap
if unique_gaps:
    print(f"\nOrbit gap ({len(unique_gaps)} polytopes with >=2 orbits):")
    print(f"  min={min(unique_gaps):.6f}, median={np.median(unique_gaps):.4f}, max={max(unique_gaps):.4f}")

print("\nDone.")
