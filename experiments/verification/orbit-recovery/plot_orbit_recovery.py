#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""
Goal: Plot worst trusted-minimum recovery errors by facet count.
Input: experiments/verification/orbit-recovery/orbit-recovery.jsonl
Output: experiments/verification/orbit-recovery/orbit_recovery_errors.png
"""

import json
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import FIGSIZE_SINGLE, setup

EXPERIMENT_DIR = Path(__file__).resolve().parent
def parse_summary_path():
    smoke = len(sys.argv) > 1 and sys.argv[1] == "--smoke"
    if len(sys.argv) > 2 or (len(sys.argv) == 2 and not smoke):
        print("Usage: uv run plot_orbit_recovery.py [--smoke]")
        sys.exit(2)
    prefix = "smoke-" if smoke else ""
    return (
        smoke,
        EXPERIMENT_DIR / f"{prefix}orbit-recovery.jsonl",
        EXPERIMENT_DIR / f"{prefix}orbit_recovery_errors.png",
    )


def load_rows(path):
    if not path.exists():
        print(f"File not found: {path}")
        print(
            "Run Rust binary first: cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery"
            + (" -- --full" if "smoke-" not in path.name else "")
        )
        sys.exit(1)

    rows = []
    with open(path) as f:
        for line in f:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def main():
    smoke, data_file, output_file = parse_summary_path()
    rows = load_rows(data_file)

    setup()
    ok_rows = [row for row in rows if row["status"] == "ok"]
    by_facet_count = defaultdict(list)
    for row in ok_rows:
        by_facet_count[row["facet_count"]].append(row)

    facets = sorted(by_facet_count)
    max_closure = [
        max(row["worst_closure_error"] for row in by_facet_count[facet]) for facet in facets
    ]
    max_action = [
        max(row["worst_action_error"] for row in by_facet_count[facet]) for facet in facets
    ]

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.semilogy(facets, max_closure, "o-", label="Worst closure error")
    ax.semilogy(facets, max_action, "s-", label="Worst action error")
    ax.axhline(1e-6, color="C0", linestyle="--", alpha=0.5, label=r"Geometry threshold ($10^{-6}$)")
    ax.axhline(1e-5, color="C1", linestyle="--", alpha=0.5, label=r"Action threshold ($10^{-5}$)")
    ax.set_xlabel(r"Facet count $F$")
    ax.set_ylabel("Maximum error")
    ax.set_xticks(facets)
    ax.set_title("Trusted-minimum recovery errors" + (" (smoke)" if smoke else ""))
    ax.legend()
    fig.savefig(output_file)
    plt.close(fig)
    print(f"Saved: {output_file}")


if __name__ == "__main__":
    main()
