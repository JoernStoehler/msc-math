"""
Goal: Analyze combinatorial boundary events and produce figures.
Input: experiments/combinatorial-boundaries/combinatorial-boundaries-{anatomy,crossing,gradient}.jsonl
Output: experiments/combinatorial-boundaries/*.png
"""

import json
import sys
from pathlib import Path
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent))
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
    "sparse_random": "#4CAF50",
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
gradient = load_jsonl("combinatorial-boundaries-gradient.jsonl")

print(f"Loaded: {len(anatomy)} anatomy, {len(crossing)} crossing, {len(gradient)} gradient rows")

# ============================================================================
# Figure 1: Event type breakdown
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
ax.set_title("Boundary event types (first boundary per direction)")
fig.savefig(EXPERIMENT_DIR / "boundary_event_types.png")
plt.close()
print("  boundary_event_types.png")

# ============================================================================
# Figure 2: t_max distribution by event type
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
# Figure 3: t_max by direction type (box plot)
# ============================================================================

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
dir_types = ["gradient", "neg_gradient", "dense_random", "sparse_random"]
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
# Figure 4: t_max vs facet count
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
# Figure 5: sys continuity scatter
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
    ax.set_title(r"$\mathrm{sys}$ continuity at incidence-flip boundaries")
    ax.legend()
    fig.savefig(EXPERIMENT_DIR / "boundary_sys_continuity.png")
    plt.close()
    print("  boundary_sys_continuity.png")

# ============================================================================
# Figure 6: |delta_sys| histogram
# ============================================================================

if ok_crossing:
    delta = [abs(row["delta_sys"]) for row in ok_crossing
             if abs(row["delta_sys"]) < 1e10]
    if delta:
        fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
        ax.hist(delta, bins=50, color="#2196F3", alpha=0.7)
        ax.set_xlabel(r"$|\Delta \mathrm{sys}|$")
        ax.set_ylabel("Count")
        ax.set_title(r"$|\Delta \mathrm{sys}|$ at first boundary")
        ax.axvline(np.median(delta), color="red", ls="--", lw=1,
                   label=f"Median: {np.median(delta):.2e}")
        ax.legend()
        fig.savefig(EXPERIMENT_DIR / "boundary_delta_sys.png")
        plt.close()
        print("  boundary_delta_sys.png")

# ============================================================================
# Figure 7: Orbit switch rate (bar chart by event type)
# ============================================================================

# Since orbit_changed = 0 for all rows, this figure would be trivial.
# Record the finding instead of producing an empty chart.
orbit_switch_count = sum(1 for row in ok_crossing if row["orbit_changed"])
print(f"\n  Orbit switches at first boundary: {orbit_switch_count}/{len(ok_crossing)} "
      f"({100 * orbit_switch_count / max(len(ok_crossing), 1):.1f}%)")

# ============================================================================
# Figure 8: Gradient angle change
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
# Figure 9: CDF of t_max by direction type
# ============================================================================

fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
for dt in ["gradient", "neg_gradient", "random"]:
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
# Figure 10: Crossing success rate by F
# ============================================================================

success_by_f = {}
total_by_f = {}
for row in crossing:
    f = row["facet_count"]
    total_by_f[f] = total_by_f.get(f, 0) + 1
    if row["construction_ok_after"]:
        success_by_f[f] = success_by_f.get(f, 0) + 1

if total_by_f:
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    f_vals = sorted(total_by_f.keys())
    rates = [100 * success_by_f.get(f, 0) / total_by_f[f] for f in f_vals]
    ax.bar(f_vals, rates, color="#4CAF50", alpha=0.7)
    ax.set_xlabel("Facet count " + r"$F$")
    ax.set_ylabel("Success rate (%)")
    ax.set_title("Crossing evaluation success rate by " + r"$F$")
    for f, r in zip(f_vals, rates):
        ax.text(f, r + 1, f"{r:.0f}%", ha="center", fontsize=FONT_SIZE_SMALL)
    fig.savefig(EXPERIMENT_DIR / "boundary_crossing_success.png")
    plt.close()
    print("  boundary_crossing_success.png")

# ============================================================================
# Summary statistics
# ============================================================================

print("\n=== Summary Statistics ===")
print(f"Polytopes analyzed: {len(set(row['polytope_name'] for row in anatomy))}")
print(f"Total directions probed: {len(anatomy)}")
print(f"Event types: {dict(sorted(event_counts.items()))}")

tmax_real = [row["t_max"] for row in anatomy
             if row["event_type"] not in ("unbounded",)]
if tmax_real:
    print(f"t_max (non-unbounded): min={min(tmax_real):.4f}, "
          f"median={np.median(tmax_real):.4f}, max={max(tmax_real):.4f}")

print(f"\nCrossing evaluation: {len(ok_crossing)}/{len(crossing)} succeeded")
if ok_crossing:
    abs_ds = [abs(row["delta_sys"]) for row in ok_crossing]
    print(f"|delta_sys|: max={max(abs_ds):.2e}")
    print(f"Orbit switches: {orbit_switch_count}")

if gradient:
    print(f"\nGradient rows: {len(gradient)}")
    angles_all = [row["gradient_angle_change_deg"] for row in gradient
                  if not np.isnan(row["gradient_angle_change_deg"])]
    if angles_all:
        print(f"Gradient angle change: max={max(angles_all):.4f} deg")

print("\nDone.")
