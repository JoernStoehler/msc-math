# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Gradient analysis experiment: figures and analysis.

Goal: Visualize sensitivity structure and gradient ascent at HKO2024.
Input Artifacts: hko-neighborhood-sensitivity.jsonl, hko-neighborhood-ascent.jsonl
Output Artifacts: hko-neighborhood-gradient.png, hko-neighborhood-orbits.png
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

def experiments_dir() -> Path:
    for parent in Path(__file__).resolve().parents:
        if (parent / "figure_config.py").exists():
            return parent
    raise RuntimeError("could not find experiments/figure_config.py")


sys.path.insert(0, str(experiments_dir()))
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
    """Bar chart of dellsys/dellh_k for HKO2024."""
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
    fig.savefig(out)
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
    fig.savefig(out)
    plt.close()
    print(f"  Wrote {out}")


def main():
    print("Gradient analysis: generating figures\n")

    # Load data
    sens_rows = load_jsonl("hko-neighborhood-sensitivity.jsonl")
    ascent_rows = load_jsonl("hko-neighborhood-ascent.jsonl")

    if not sens_rows:
        print("ERROR: No sensitivity data. Run `cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis` first.")
        sys.exit(1)

    sens = sens_rows[0]

    # Print summary
    print("Sensitivity data:")
    print(f"  sys = {sens['sys']:.10f}")
    print(f"  capacity = {sens['capacity']:.10f}")
    print(f"  volume = {sens['volume']:.10f}")
    print(f"  |grad_sys_h| = {sens['gradient_norm_h']:.6e}")
    print(f"  |grad_sys_n| = {sens['gradient_norm_n']:.6e}")
    print(f"  |grad_sys_hn| = {sens['gradient_norm_hn']:.6e}")
    print(f"  Valid orbits: {sens['n_valid_orbits']}")
    print(f"  Near-optimal: {sens['n_near_optimal']}")

    # All d_sys_h are negative → HKO2024 is a local max in height space
    d_sys_h = sens["d_sys_h"]
    all_negative = all(d < 0 for d in d_sys_h)
    print(f"\n  All dellsys/dellh_k < 0: {all_negative}")
    if all_negative:
        print("  -> Increasing any h_k DECREASES sys (normals fixed)")
        print("  -> Decreasing any h_k would shrink K (heights bounded below)")
        print(f"  -> |grad_sys_n| = {sens['gradient_norm_n']:.4e} (nonzero -- not a critical point)")
        print("  -> HKO2024 is a local max in h-space; normal gradient is nonzero")
    print()

    # Ascent summary
    if ascent_rows:
        last = ascent_rows[-1]
        print("Gradient ascent:")
        print(f"  Iterations: {len(ascent_rows)}")
        print(f"  Final sys: {last['sys_after']:.10f}")
        print(f"  delta_sys: {last['sys_after'] - sens['sys']:.6e}")
        print()

    # Generate figures
    print("Generating figures...")
    fig_gradient(sens)
    fig_orbits(sens)

    print("\nDone.")


if __name__ == "__main__":
    main()
