# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Facet-splitting experiment: figures and analysis.

Goal: Visualize facet-splitting results (F=10 -> F=11) for HKO2024.
Input: hko-neighborhood-splitting.jsonl
Output: hko-neighborhood-splitting.png
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

# Add parent for figure_config
sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_DUAL

setup()

SCRIPT_DIR = Path(__file__).resolve().parent


def load_jsonl(name: str) -> list[dict]:
    """Load a JSONL file, returning list of dicts."""
    path = SCRIPT_DIR / name
    if not path.exists():
        print(f"WARNING: {path} not found")
        return []
    rows = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                try:
                    rows.append(json.loads(line))
                except json.JSONDecodeError:
                    print(f"  WARNING: skipping truncated line in {name}")
                    continue
    return rows


def fig_splitting(splitting_rows: list[dict]) -> None:
    """Facet-splitting results: delta_sys vs angular offset."""
    if not splitting_rows:
        print("  No splitting data, skipping splitting figure")
        return

    # Filter to successful constructions
    ok_rows = [r for r in splitting_rows if r.get("construction_ok", True) and np.isfinite(r.get("delta_sys", float("nan")))]
    if not ok_rows:
        print("  No successful splits, skipping splitting figure")
        return

    # Separate by type
    facet_rows = [r for r in ok_rows if r["source_facet"] < 100]
    other_rows = [r for r in ok_rows if r["source_facet"] >= 100]

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    # Left: delta_sys vs angular offset
    ax = axes[0]
    for eps_val in sorted(set(r["epsilon"] for r in facet_rows)):
        eps_rows = [r for r in facet_rows if r["epsilon"] == eps_val]
        angles = [r["angular_offset"] for r in eps_rows]
        deltas = [r["delta_sys"] for r in eps_rows]
        ax.scatter(angles, deltas, s=10, alpha=0.6, label=f"near-facet eps={eps_val:.0e}")
    # Include mixed/control rows
    if other_rows:
        angles = [r["angular_offset"] for r in other_rows]
        deltas = [r["delta_sys"] for r in other_rows]
        ax.scatter(angles, deltas, s=10, alpha=0.4, marker="x", color="gray", label="mixed/control")

    ax.axhline(y=0, color="red", linewidth=1, linestyle="--")
    ax.set_xlabel("Angular offset from facet normal (rad)")
    ax.set_ylabel(r"$\Delta\,\mathrm{sys}$")
    ax.set_title(r"$\Delta\,\mathrm{sys}$ vs angle")
    ax.legend(fontsize=7)

    # Right: delta_sys histogram
    ax = axes[1]
    all_deltas = [r["delta_sys"] for r in ok_rows]
    ax.hist(all_deltas, bins=40, edgecolor="black", linewidth=0.3, alpha=0.7)
    ax.axvline(x=0, color="red", linewidth=1, linestyle="--")
    ax.set_xlabel(r"$\Delta\,\mathrm{sys}$")
    ax.set_ylabel("Count")
    ax.set_title(f"Distribution (n={len(ok_rows)})")

    # Add annotation about whether any improvement was found
    n_positive = sum(1 for d in all_deltas if d > 0)
    n_negative = sum(1 for d in all_deltas if d < 0)
    n_zero = sum(1 for d in all_deltas if d == 0)
    ax.text(
        0.95, 0.95,
        f"+: {n_positive}\n-: {n_negative}\n0: {n_zero}",
        transform=ax.transAxes,
        ha="right", va="top",
        fontsize=8,
        bbox=dict(boxstyle="round,pad=0.3", facecolor="lightyellow", alpha=0.8),
    )

    plt.tight_layout()
    out = SCRIPT_DIR / "hko-neighborhood-splitting.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close()
    print(f"  Wrote {out}")


def main():
    print("Facet-splitting: generating figures\n")

    # Load data
    splitting_rows = load_jsonl("hko-neighborhood-splitting.jsonl")

    if not splitting_rows:
        print("ERROR: No splitting data. Run `cargo run -p exp-hko-local-maximum --release --bin hko-facet-splitting` first.")
        sys.exit(1)

    # Splitting summary
    ok = [r for r in splitting_rows if r.get("construction_ok", True) and np.isfinite(r.get("delta_sys", float("nan")))]
    deltas = [r["delta_sys"] for r in ok]
    print("Facet-splitting:")
    print(f"  Total directions: {len(splitting_rows)}")
    print(f"  Successful: {len(ok)}")
    if deltas:
        print(f"  max delta_sys: {max(deltas):.6e}")
        print(f"  min delta_sys: {min(deltas):.6e}")
        print(f"  All delta_sys <= 0: {all(d <= 0 for d in deltas)}")
        n_near_zero = sum(1 for d in deltas if abs(d) < 1e-8)
        print(f"  Near-zero (|delta_sys| < 1e-8): {n_near_zero} of {len(deltas)}")
    print()

    # Generate figure
    print("Generating figures...")
    fig_splitting(splitting_rows)

    print("\nDone.")


if __name__ == "__main__":
    main()
