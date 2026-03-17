"""
HKO-Neighborhood experiment: figures and analysis.

Goal: Visualize local maximality of HKO2024 via sensitivity data, gradient
      ascent trajectory, and facet-splitting results.
Input: hko-neighborhood-{sensitivity,ascent,splitting}.jsonl
Output: hko-neighborhood-{gradient,orbits,splitting}.png
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

# Add parent for figure_config
sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
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


def fig_gradient(sens: dict) -> None:
    """Bar chart of ∂sys/∂h_k for HKO2024."""
    d_sys_h = sens["d_sys_h"]
    f = len(d_sys_h)
    facet_labels = [f"$h_{{{k}}}$" for k in range(f)]

    # Split into Q-space (0-4) and P-space (5-9) facets
    colors = ["#2196F3"] * 5 + ["#FF9800"] * 5  # blue=Q, orange=P

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL, gridspec_kw={"width_ratios": [1, 1]})

    # Left: height derivatives
    ax = axes[0]
    ax.bar(range(f), d_sys_h, color=colors, edgecolor="black", linewidth=0.5)
    ax.set_xticks(range(f))
    ax.set_xticklabels(facet_labels, fontsize=7)
    ax.set_ylabel(r"$\partial \mathrm{sys} / \partial h_k$")
    ax.set_title("Height sensitivity")
    ax.axhline(y=0, color="black", linewidth=0.5)

    # Add Q/P labels
    ax.text(2, ax.get_ylim()[0] * 0.9, "Q-space", ha="center", fontsize=7, color="#1565C0")
    ax.text(7, ax.get_ylim()[0] * 0.9, "P-space", ha="center", fontsize=7, color="#E65100")

    # Right: normal derivatives (show norms)
    d_sys_n = sens["d_sys_n"]
    norms_n = [np.linalg.norm(v) for v in d_sys_n]

    ax = axes[1]
    ax.bar(range(f), norms_n, color=colors, edgecolor="black", linewidth=0.5)
    ax.set_xticks(range(f))
    ax.set_xticklabels(facet_labels, fontsize=7)
    ax.set_ylabel(r"$\|\partial \mathrm{sys} / \partial n_k\|$")
    ax.set_title("Normal sensitivity (norms)")

    fig.suptitle(
        r"HKO2024 sensitivity: $\mathrm{sys}$ = " f"{sens['sys']:.6f}, "
        r"$|\nabla\,\mathrm{sys}|$ = " f"{sens['gradient_norm_hn']:.4f}",
    )
    plt.tight_layout()
    out = SCRIPT_DIR / "hko-neighborhood-gradient.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close()
    print(f"  Wrote {out}")


def fig_orbits(sens: dict) -> None:
    """Near-optimal orbit action distribution."""
    orbits = sens["orbits"]
    if not orbits:
        print("  No orbit data, skipping orbit figure")
        return

    actions = [o["action"] for o in orbits]
    gaps = [o["relative_gap"] for o in orbits]
    best = actions[0]

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)

    # Left: action deviation from best (shows spread is < machine epsilon)
    ax = axes[0]
    deviations = [a - best for a in actions]
    ax.plot(range(len(deviations)), deviations, "o", markersize=3)
    ax.axhline(y=0, color="red", linewidth=0.5, linestyle="--")
    ax.set_xlabel("Orbit index")
    ax.set_ylabel(rf"Action $-\;{best:.6f}$")
    ax.set_title(f"{len(orbits)} near-optimal orbits")
    ax.ticklabel_format(axis="y", style="scientific", scilimits=(-3, 3))

    # Right: gap distribution
    ax = axes[1]
    # All gaps are essentially zero (machine precision)
    ax.semilogy(range(len(gaps)), [max(g, 1e-16) for g in gaps], "o", markersize=3)
    ax.set_xlabel("Orbit index")
    ax.set_ylabel("Relative gap")
    ax.set_title("Gap from best action")
    ax.axhline(y=1e-14, color="gray", linewidth=0.5, linestyle=":", label=r"$10^{-14}$ (machine $\varepsilon$)")
    ax.legend(fontsize=7)

    fig.suptitle(
        f"HKO2024 orbit structure: {len(orbits)} orbits, all at action ≈ {best:.4f}"
    )
    plt.tight_layout()
    out = SCRIPT_DIR / "hko-neighborhood-orbits.png"
    fig.savefig(out, dpi=150, bbox_inches="tight")
    plt.close()
    print(f"  Wrote {out}")


def fig_splitting(splitting_rows: list[dict]) -> None:
    """Facet-splitting results: Δsys vs angular offset."""
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

    # Left: Δsys vs angular offset
    ax = axes[0]
    for eps_val in sorted(set(r["epsilon"] for r in facet_rows)):
        eps_rows = [r for r in facet_rows if r["epsilon"] == eps_val]
        angles = [r["angular_offset"] for r in eps_rows]
        deltas = [r["delta_sys"] for r in eps_rows]
        ax.scatter(angles, deltas, s=10, alpha=0.6, label=f"near-facet ε={eps_val:.0e}")
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

    # Right: Δsys histogram
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
        f"+: {n_positive}\n−: {n_negative}\n0: {n_zero}",
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
    print("HKO-Neighborhood: generating figures\n")

    # Load data
    sens_rows = load_jsonl("hko-neighborhood-sensitivity.jsonl")
    ascent_rows = load_jsonl("hko-neighborhood-ascent.jsonl")
    splitting_rows = load_jsonl("hko-neighborhood-splitting.jsonl")

    if not sens_rows:
        print("ERROR: No sensitivity data. Run `cargo run --bin hko_neighborhood --release` first.")
        sys.exit(1)

    sens = sens_rows[0]

    # Print summary
    print("Sensitivity data:")
    print(f"  sys = {sens['sys']:.10f}")
    print(f"  capacity = {sens['capacity']:.10f}")
    print(f"  volume = {sens['volume']:.10f}")
    print(f"  |∇sys_h| = {sens['gradient_norm_h']:.6e}")
    print(f"  |∇sys_n| = {sens['gradient_norm_n']:.6e}")
    print(f"  |∇sys_hn| = {sens['gradient_norm_hn']:.6e}")
    print(f"  Valid orbits: {sens['n_valid_orbits']}")
    print(f"  Near-optimal: {sens['n_near_optimal']}")

    # All d_sys_h are negative → HKO2024 is a local max in height space
    d_sys_h = sens["d_sys_h"]
    all_negative = all(d < 0 for d in d_sys_h)
    print(f"\n  All ∂sys/∂h_k < 0: {all_negative}")
    if all_negative:
        print("  → Increasing any h_k DECREASES sys (normals fixed)")
        print("  → Decreasing any h_k would shrink K (heights bounded below)")
        print(f"  → |∇sys_n| = {sens['gradient_norm_n']:.4e} (nonzero — not a critical point)")
        print("  → HKO2024 is a local max in h-space; normal gradient is nonzero")
    print()

    # Ascent summary
    if ascent_rows:
        last = ascent_rows[-1]
        print("Gradient ascent:")
        print(f"  Iterations: {len(ascent_rows)}")
        print(f"  Final sys: {last['sys_after']:.10f}")
        print(f"  Δsys: {last['sys_after'] - sens['sys']:.6e}")
        print()

    # Splitting summary
    if splitting_rows:
        ok = [r for r in splitting_rows if r.get("construction_ok", True) and np.isfinite(r.get("delta_sys", float("nan")))]
        deltas = [r["delta_sys"] for r in ok]
        print("Facet-splitting:")
        print(f"  Total directions: {len(splitting_rows)}")
        print(f"  Successful: {len(ok)}")
        if deltas:
            print(f"  max Δsys: {max(deltas):.6e}")
            print(f"  min Δsys: {min(deltas):.6e}")
            print(f"  All Δsys ≤ 0: {all(d <= 0 for d in deltas)}")
            n_near_zero = sum(1 for d in deltas if abs(d) < 1e-8)
            print(f"  Near-zero (|Δsys| < 1e-8): {n_near_zero} of {len(deltas)}")
        print()

    # Generate figures
    print("Generating figures...")
    fig_gradient(sens)
    fig_orbits(sens)
    fig_splitting(splitting_rows)

    print("\nDone.")


if __name__ == "__main__":
    main()
