"""Render the validated panel summaries as readable PNG and SVG figures.

The producer and analyzer remain dependency-free.  This optional renderer is
run with ``uv run --with matplotlib`` when a raster figure is wanted.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


POLICIES = [
    "inf_normalized_branch_gradient",
    "near_active_box_lp_maximin",
    "candidate_window_box_lp_maximin",
    "single_branch_box_steepest",
]
COLORS = {
    POLICIES[0]: "#1f77b4",
    POLICIES[1]: "#d62728",
    POLICIES[2]: "#2ca02c",
    POLICIES[3]: "#9467bd",
}
LABELS = {
    POLICIES[0]: "branch gradient (L∞)",
    POLICIES[1]: "near-active box LP",
    POLICIES[2]: "candidate-window box LP",
    POLICIES[3]: "single-branch sign box",
}


def render(root: Path) -> None:
    import matplotlib.pyplot as plt

    observed = json.loads((root / "analysis.json").read_text())["observed"]
    starts = sorted({row["start"] for row in observed})
    radii = sorted({float(row["radius"]) for row in observed})
    values = {
        (row["start"], float(row["radius"]), row["policy"]): row["best_gain"]
        for row in observed
    }

    if len(starts) <= 2:
        # The retained role-labelled panel gets one readable comparison axis.
        fig, ax = plt.subplots(figsize=(8.5, 5.2), constrained_layout=True)
        for start in starts:
            short = start[:8]
            for policy in POLICIES:
                ax.plot(
                    radii,
                    [values[(start, r, policy)] for r in radii],
                    marker="o",
                    color=COLORS[policy],
                    linestyle="-" if start == starts[0] else "--",
                    label=f"{LABELS[policy]} ({short})",
                )
        ax.set_xscale("log")
        ax.set_xticks(radii, [f"{r:g}" for r in radii])
        ax.set_xlabel("initial proposal radius")
        ax.set_ylabel("best-so-far gain in sys")
        ax.set_title("Selected fixtures: validated best-so-far gain")
        ax.grid(True, alpha=0.25)
        ax.legend(fontsize=8, ncol=2)
        out = root / "figures" / "selected-gain-comparison"
    else:
        # Six small multiples keep the start identity visible without a dense
        # 324-point trajectory overlay.
        ncols = 3
        nrows = (len(starts) + ncols - 1) // ncols
        fig, axes = plt.subplots(
            nrows, ncols, figsize=(11.5, 6.8), sharex=True, sharey=False,
        )
        axes = list(axes.flat)
        for ax, start in zip(axes, starts):
            for policy in POLICIES:
                ax.plot(
                    radii,
                    [values[(start, r, policy)] for r in radii],
                    marker="o",
                    color=COLORS[policy],
                    label=LABELS[policy],
                )
            ax.set_xscale("log")
            ax.set_xticks(radii, [f"{r:g}" for r in radii])
            ax.set_title(start)
            ax.grid(True, alpha=0.25)
            ax.set_xlabel("radius")
            ax.set_ylabel("best gain")
        for ax in axes[len(starts):]:
            ax.set_visible(False)
        handles, labels = axes[0].get_legend_handles_labels()
        fig.subplots_adjust(top=0.80, bottom=0.09, left=0.06, right=0.99, hspace=0.42, wspace=0.28)
        fig.legend(handles, labels, loc="upper center", bbox_to_anchor=(0.5, 0.94), ncol=3, fontsize=9)
        fig.suptitle("Generic six-start panel: validated best-so-far gain", y=0.995)
        out = root / "figures" / "generic-gain-grid"

    out.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(out.with_suffix(".png"), dpi=180)
    fig.savefig(out.with_suffix(".svg"))
    plt.close(fig)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    render(parser.parse_args().root)
