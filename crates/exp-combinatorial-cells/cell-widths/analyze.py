"""
Goal: Analyze per-facet cell profiling in dual-vertex space.
Input: crates/exp-combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl
Output: crates/exp-combinatorial-cells/cell-widths/*.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent.parent))
from figure_config import (
    setup, FIGSIZE_SINGLE, FIGSIZE_DUAL,
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

print(f"Loaded: {len(profiling)} profiling rows")

# ============================================================================
# Figure 1: Cell width — orbit facets vs non-orbit facets
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
# Figure 2: Cell width by F
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
# Figure 3: Anisotropy histogram
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
# Figure 4: Event type breakdown (per-facet)
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
print(f"Profiling probes: {len(profiling)}")

orbit_tmax_vals = [r["t_max"] for r in profiling if r["facet_in_orbit"] and r["t_max"] < 100]
non_orbit_tmax_vals = [r["t_max"] for r in profiling if not r["facet_in_orbit"] and r["t_max"] < 100]
print(f"\nPer-facet cell width (t_max):")
print(f"  Orbit facets:     median={np.median(orbit_tmax_vals):.4f} "
      f"(n={len(orbit_tmax_vals)})")
print(f"  Non-orbit facets: median={np.median(non_orbit_tmax_vals):.4f} "
      f"(n={len(non_orbit_tmax_vals)})")
print(f"  Anisotropy:       median={np.median(anisotropies):.1f}x "
      f"(max={max(anisotropies):.0f}x)")

print("\nDone.")
