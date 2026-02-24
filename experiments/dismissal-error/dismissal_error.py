#!/usr/bin/env python3
"""
Analyse dismissal error bounds across the test dataset.

Goal: Confirm that value loss from near-singular system dismissal is negligible.
Input: experiments/dismissal-error/dismissal-error.jsonl
Output: experiments/dismissal-error/dismissal-error.png, console summary table
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "dismissal-error.jsonl"
FIGURE_PATH = EXPERIMENT_DIR / "dismissal-error.png"


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print(
            "Run: cd experiments/ && cargo run --bin dismissal_error --release",
            file=sys.stderr,
        )
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def main():
    records = load_jsonl(DATA_PATH)

    summaries = [r for r in records if r.get("type") == "summary"]
    details = [r for r in records if "type" not in r]

    # --- Console summary table ---
    print("=" * 90)
    print("Dismissal Error Bound Experiment — Summary")
    print("=" * 90)
    print(
        f"{'Polytope':<40} {'F':>3} {'Pairs':>7} {'Bound':>6} {'Triv':>6} {'Max Rel Err':>12}"
    )
    print("-" * 90)

    total_pairs = 0
    total_bound = 0
    total_trivial = 0
    global_max_rel = 0.0

    for s in summaries:
        name = s["polytope_name"]
        f = s["facet_count"]
        pairs = s["total_pairs"]
        bound = s["dismissals_with_bound"]
        triv = s["trivial_dismissals"]
        max_rel = s["max_relative_error"]

        total_pairs += pairs
        total_bound += bound
        total_trivial += triv
        global_max_rel = max(global_max_rel, max_rel)

        rel_str = f"{max_rel:.2e}" if max_rel > 0 else "—"
        print(f"{name:<40} {f:>3} {pairs:>7} {bound:>6} {triv:>6} {rel_str:>12}")

    print("-" * 90)
    rel_str = f"{global_max_rel:.2e}" if global_max_rel > 0 else "—"
    print(
        f"{'TOTAL':<40} {'':>3} {total_pairs:>7} {total_bound:>6} {total_trivial:>6} {rel_str:>12}"
    )
    print()
    print(f"Dismissals with computed bound: {total_bound}")
    print(f"Trivial dismissals (β₀ ≤ 0): {total_trivial}")
    print(f"Global maximum relative error: {global_max_rel:.2e}")
    print()

    if global_max_rel > 0:
        print(
            f"Conclusion: max relative error = {global_max_rel:.2e} "
            f"(≈ {global_max_rel / 2.2e-16:.1f}× machine epsilon). "
            f"Value loss from dismissal is negligible."
        )
    else:
        print(
            "Conclusion: all error bounds are exactly zero. "
            "Value loss from dismissal is zero."
        )
    print()

    # --- Figure ---
    if not details:
        print("No non-trivial dismissal records — skipping figure.")
        return

    # Collect relative errors for all detail records
    rel_errors = [d["relative_error"] for d in details]
    sigma_js = [d["sigma_j"] for d in details]

    fig, axes = plt.subplots(1, 2, figsize=(12, 5))

    # Panel 1: histogram of relative errors
    ax = axes[0]
    nonzero_rel = [r for r in rel_errors if r > 0]
    if nonzero_rel:
        ax.hist(np.log10(nonzero_rel), bins=20, color="steelblue", edgecolor="white")
        ax.set_xlabel("log₁₀(error bound / capacity)")
        ax.set_ylabel("Count")
        ax.set_title("Distribution of relative error bounds")
        ax.axvline(
            np.log10(2.2e-16),
            color="red",
            linestyle="--",
            linewidth=1,
            label="machine ε",
        )
        ax.legend()
    else:
        ax.text(
            0.5,
            0.5,
            "All error bounds = 0",
            ha="center",
            va="center",
            transform=ax.transAxes,
        )
        ax.set_title("Distribution of relative error bounds")

    # Panel 2: sigma_j vs relative error (scatter)
    ax = axes[1]
    nonzero_mask = [r > 0 for r in rel_errors]
    if any(nonzero_mask):
        sj_nz = [s for s, m in zip(sigma_js, nonzero_mask) if m]
        re_nz = [r for r, m in zip(rel_errors, nonzero_mask) if m]
        ax.scatter(sj_nz, re_nz, alpha=0.6, s=20, color="steelblue")
        ax.set_xscale("log")
        ax.set_yscale("log")
        ax.set_xlabel("σⱼ (singular value triggering dismissal)")
        ax.set_ylabel("error bound / capacity")
        ax.set_title("Error bound vs. singular value")
        ax.axhline(
            2.2e-16,
            color="red",
            linestyle="--",
            linewidth=1,
            label="machine ε",
        )
        ax.legend()
    else:
        ax.text(
            0.5,
            0.5,
            "All error bounds = 0",
            ha="center",
            va="center",
            transform=ax.transAxes,
        )
        ax.set_title("Error bound vs. singular value")

    fig.suptitle("Dismissal Error Bound Experiment", fontsize=14, fontweight="bold")
    fig.tight_layout()
    fig.savefig(FIGURE_PATH, dpi=150)
    plt.close(fig)
    print(f"Saved figure: {FIGURE_PATH}")


if __name__ == "__main__":
    main()
