#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""
Plot orbit recovery error metrics by facet count.

Goal: Visualize how recovery errors scale with polytope complexity (F).
Input: crates/dev-capacity-validation/orbit-recovery/orbit-recovery.jsonl
Output: crates/dev-capacity-validation/orbit-recovery/orbit_recovery_errors.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt

# Add parent for figure_config
sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_FILE = EXPERIMENT_DIR / "orbit-recovery.jsonl"
OUTPUT_FILE = EXPERIMENT_DIR / "orbit_recovery_errors.png"


def load_data():
    if not DATA_FILE.exists():
        print(f"File not found: {DATA_FILE}")
        print("Run Rust binary first: cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery")
        sys.exit(1)

    rows = []
    with open(DATA_FILE) as f:
        for line in f:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def main():
    setup()
    rows = load_data()

    # Filter to random polytopes only (known have machine-epsilon errors)
    random_rows = [r for r in rows if r["source"] == "random"]

    # Group by facet count
    by_f = defaultdict(list)
    for r in random_rows:
        by_f[r["facet_count"]].append(r)

    facets = sorted(by_f.keys())
    max_closure = [max(r["closure_error"] for r in by_f[f]) for f in facets]
    max_action = [max(r["action_error"] for r in by_f[f]) for f in facets]

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)

    ax.semilogy(facets, max_closure, "o-", label="Closure error")
    ax.semilogy(facets, max_action, "s-", label="Action error")

    # Thresholds
    ax.axhline(1e-6, color="C0", linestyle="--", alpha=0.5, label="Geometric threshold ($10^{-6}$)")
    ax.axhline(1e-5, color="C1", linestyle="--", alpha=0.5, label="Action threshold ($10^{-5}$)")

    ax.set_xlabel("Facet count $F$")
    ax.set_ylabel("Maximum error")
    ax.set_xticks(facets)
    ax.legend()

    fig.savefig(OUTPUT_FILE)
    plt.close(fig)
    print(f"Saved: {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
