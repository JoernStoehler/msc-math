#!/usr/bin/env python3
"""
Analyze UNKNOWN admissibility predicate prevalence across datasets.

Goal: Determine whether the EHZ capacity algorithm ever produces UNKNOWN
      (inconclusive) admissibility verdicts on our polytope datasets.
Input: experiments/unknown-predicates/unknown-predicates.jsonl
Output: experiments/unknown-predicates/unknown_predicates_beta_min.png
        Summary statistics to stdout
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_DUAL
setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_FILE = EXPERIMENT_DIR / "unknown-predicates.jsonl"
FIG_BETA_MIN = EXPERIMENT_DIR / "unknown_predicates_beta_min.png"


def load_data(path: Path) -> list[dict]:
    if not path.exists():
        print(
            f"File not found: {path}\n"
            "Run: cd experiments/ && cargo run --bin unknown_predicates --release",
            file=sys.stderr,
        )
        sys.exit(1)
    rows = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def main():
    rows = load_data(DATA_FILE)
    n = len(rows)

    # Split by dataset
    random_rows = [r for r in rows if r["dataset"] == "random-sweep"]
    lagrangian_rows = [r for r in rows if r["dataset"] == "lagrangian-products"]

    # Count UNKNOWNs
    random_unknowns = [r for r in random_rows if r["has_unknown"]]
    lagrangian_unknowns = [r for r in lagrangian_rows if r["has_unknown"]]
    total_unknowns = len(random_unknowns) + len(lagrangian_unknowns)

    # Summary
    print("=" * 60)
    print("UNKNOWN Predicate Analysis — Phase 1 Results")
    print("=" * 60)
    print()
    print(f"{'Dataset':<25} {'Total':>6} {'UNKNOWNs':>9} {'Rate':>8}")
    print("-" * 50)
    print(
        f"{'random-sweep':<25} {len(random_rows):>6} "
        f"{len(random_unknowns):>9} "
        f"{len(random_unknowns)/max(len(random_rows),1)*100:>7.1f}%"
    )
    print(
        f"{'lagrangian-products':<25} {len(lagrangian_rows):>6} "
        f"{len(lagrangian_unknowns):>9} "
        f"{len(lagrangian_unknowns)/max(len(lagrangian_rows),1)*100:>7.1f}%"
    )
    print("-" * 50)
    print(
        f"{'TOTAL':<25} {n:>6} "
        f"{total_unknowns:>9} "
        f"{total_unknowns/max(n,1)*100:>7.1f}%"
    )
    print()

    # Beta_min statistics
    all_beta_min = np.array([r["beta_min"] for r in rows])
    print("Beta_min distribution (certified orbit, minimum component):")
    print(f"  min:    {all_beta_min.min():.6e}")
    print(f"  median: {np.median(all_beta_min):.6e}")
    print(f"  mean:   {all_beta_min.mean():.6e}")
    print(f"  max:    {all_beta_min.max():.6e}")
    print()

    # Per-dataset beta_min
    for label, subset in [("random-sweep", random_rows), ("lagrangian-products", lagrangian_rows)]:
        bm = np.array([r["beta_min"] for r in subset])
        if len(bm) > 0:
            print(f"  {label}: min={bm.min():.6e}, median={np.median(bm):.6e}, max={bm.max():.6e}")

    # Numerical gap statistics
    all_gaps = np.array([r["numerical_gap"] for r in rows])
    nonzero_gaps = all_gaps[all_gaps > 0]
    print()
    print(f"Numerical gaps: {len(nonzero_gaps)} nonzero out of {n}")
    if len(nonzero_gaps) > 0:
        print(f"  Nonzero gap range: [{nonzero_gaps.min():.6e}, {nonzero_gaps.max():.6e}]")
        print()
        print("Polytopes with UNKNOWN predicates:")
        for r in rows:
            if r["has_unknown"]:
                print(
                    f"  {r['name']}: gap={r['numerical_gap']:.6e}, "
                    f"cap={r['capacity']:.8f}, cap_unc={r['capacity_uncertain']:.8f}, "
                    f"beta_min={r['beta_min']:.6e}"
                )

    # Conclusion
    print()
    if total_unknowns == 0:
        print("CONCLUSION: No UNKNOWN predicates found.")
        print("The algorithm is empirically exact at f64 precision on this dataset.")
        print("Phase 2 (high-precision re-solve) is NOT needed.")
    else:
        print(f"CONCLUSION: {total_unknowns} UNKNOWN predicate(s) found.")
        print("Phase 2 (high-precision re-solve) IS needed.")

    # -----------------------------------------------------------------------
    # Figure: beta_min distribution (log scale)
    # -----------------------------------------------------------------------
    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    for ax, label, subset in [
        (axes[0], "random-sweep", random_rows),
        (axes[1], "lagrangian-products", lagrangian_rows),
    ]:
        bm = np.array([r["beta_min"] for r in subset])
        if len(bm) == 0:
            continue
        log_bm = np.log10(bm[bm > 0])
        ax.hist(log_bm, bins=30, edgecolor="black", alpha=0.7)
        ax.set_xlabel("log₁₀(β_min)")
        ax.set_ylabel("Count")
        ax.set_title(f"{label} (n={len(subset)})")
        ax.axvline(
            np.log10(1e-12),
            color="red",
            linestyle="--",
            label=r"$\epsilon_\beta = 10^{-12}$",
        )
        ax.legend(fontsize=8)

        # Annotate the near-miss outlier if present
        if log_bm.min() < -8:
            nearest = subset[int(np.argmin(bm))]
            ax.annotate(
                f"{nearest['name']}\nβ_min = {bm.min():.1e}",
                xy=(log_bm.min(), 1),
                xytext=(log_bm.min() + 1.5, max(ax.get_ylim()[1] * 0.6, 3)),
                fontsize=7,
                arrowprops=dict(arrowstyle="->", color="black", lw=0.8),
                bbox=dict(boxstyle="round,pad=0.3", fc="lightyellow", ec="gray", lw=0.5),
            )

    fig.suptitle("Minimum β component of certified orbits", fontsize=13)
    fig.tight_layout()
    fig.savefig(FIG_BETA_MIN)
    print(f"\nFigure saved: {FIG_BETA_MIN}")


if __name__ == "__main__":
    main()
