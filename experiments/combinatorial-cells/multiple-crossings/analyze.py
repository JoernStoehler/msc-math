# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: Analyze multi-boundary sweep results.
Input Artifacts: experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl
Output Artifacts: experiments/combinatorial-cells/multiple-crossings/*.png

Note: The original combined analyze.py did not produce sweep-specific figures.
This script provides summary statistics from the sweep data. Figures can be
added as needed.
"""

import json
import sys
from collections import defaultdict
from pathlib import Path
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent.parent))
from figure_config import (
    setup, FIGSIZE_SINGLE,
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


sweep = load_jsonl("combinatorial-boundaries-sweep.jsonl")

print(f"Loaded: {len(sweep)} sweep rows")

# ============================================================================
# Summary statistics
# ============================================================================

print("\n=== Summary Statistics ===")

if sweep:
    n_boundaries = [r["n_boundaries"] for r in sweep]
    print(f"Sweep rows: {len(sweep)}")
    print(f"Boundaries crossed: median={np.median(n_boundaries):.0f}, "
          f"mean={np.mean(n_boundaries):.1f}, max={max(n_boundaries)}")

    # By F
    by_f = defaultdict(list)
    for r in sweep:
        by_f[r["facet_count"]].append(r["n_boundaries"])

    print("\nBoundaries by F:")
    for f in sorted(by_f.keys()):
        vals = by_f[f]
        print(f"  F={f}: median={np.median(vals):.0f}, "
              f"mean={np.mean(vals):.1f}, max={max(vals)}")

    # Failure rate
    n_failed = sum(1 for r in sweep if r["ended_by_failure"])
    print(f"\nConstruction failures: {n_failed}/{len(sweep)} "
          f"({100 * n_failed / len(sweep):.1f}%)")

    # sys improvement along gradient
    grad_sweeps = [r for r in sweep if r["direction_type"] == "gradient"]
    if grad_sweeps:
        n_improved = sum(1 for r in grad_sweeps
                        if r["sys_values"] and r["sys_values"][-1] is not None
                        and not np.isnan(r["sys_values"][-1])
                        and r["sys_values"][-1] > r["sys_start"])
        n_valid = sum(1 for r in grad_sweeps
                     if r["sys_values"] and r["sys_values"][-1] is not None
                     and not np.isnan(r["sys_values"][-1]))
        if n_valid > 0:
            print(f"\nGradient sweeps with sys improvement: {n_improved}/{n_valid} "
                  f"({100 * n_improved / n_valid:.0f}%)")

    # Event type distribution
    all_events = []
    for r in sweep:
        all_events.extend(r["event_types"])
    if all_events:
        event_counts = defaultdict(int)
        for e in all_events:
            event_counts[e] += 1
        print("\nEvent types in sweeps:")
        total = sum(event_counts.values())
        for et in sorted(event_counts.keys()):
            print(f"  {et}: {event_counts[et]} ({100 * event_counts[et] / total:.1f}%)")

print("\nDone.")
