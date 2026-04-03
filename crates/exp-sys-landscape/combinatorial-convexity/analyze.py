"""
Goal: Analyze cell convexity testing results.
Input: crates/exp-sys-landscape/combinatorial-convexity/combinatorial-boundaries-convexity.jsonl
Output: crates/exp-sys-landscape/combinatorial-convexity/*.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent.parent))
from figure_config import (
    setup, FIGSIZE_DUAL,
    FONT_SIZE_SMALL,
)
setup()

import matplotlib.pyplot as plt

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


convexity = load_jsonl("combinatorial-boundaries-convexity.jsonl")

print(f"Loaded: {len(convexity)} convexity rows")

# ============================================================================
# Figure 1: Convexity failure rates
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
# Summary statistics
# ============================================================================

print("\n=== Summary Statistics ===")

if ok_conv:
    omega_fail_rate = 100 * (1 - sum(1 for r in ok_conv if r["midpoint_same_omega_signs"]) / len(ok_conv))
    trans_fail_rate = 100 * (1 - sum(1 for r in ok_conv if r["midpoint_same_transitions"]) / len(ok_conv))
    incidence_fail_rate = 100 * (1 - sum(1 for r in ok_conv if r["midpoint_same_incidence"]) / len(ok_conv))
    print(f"Convexity (midpoint test, n={len(ok_conv)}):")
    print(f"  Incidence failure:    {incidence_fail_rate:.1f}%")
    print(f"  omega_0 sign failure: {omega_fail_rate:.1f}%")
    print(f"  Transition failure:   {trans_fail_rate:.1f}%")

print("\nDone.")
