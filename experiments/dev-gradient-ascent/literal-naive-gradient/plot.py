# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""Plot literal unconditional branch-gradient trajectories."""

from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt


OWNER = Path(__file__).resolve().parent
ARTIFACTS = OWNER / "artifacts"
FIGURES = OWNER / "figures"
ETAS = [1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0]
COLORS = plt.get_cmap("viridis")([0.05, 0.23, 0.41, 0.59, 0.77, 0.95])


def eta_label(eta: float) -> str:
    return f"$\\eta=10^{{{round(__import__('math').log10(eta))}}}$"


def file_label(eta: float) -> str:
    return {
        1e-5: "1e-5",
        1e-4: "1e-4",
        1e-3: "1e-3",
        1e-2: "1e-2",
        1e-1: "1e-1",
        1.0: "1",
    }[eta]


def read_rows(eta: float) -> list[dict]:
    path = ARTIFACTS / f"trajectory-eta-{file_label(eta)}.jsonl"
    with path.open() as stream:
        return [json.loads(line) for line in stream]


def main() -> None:
    FIGURES.mkdir(exist_ok=True)
    plt.rcParams.update(
        {
            "font.size": 9,
            "axes.grid": True,
            "grid.alpha": 0.25,
            "legend.fontsize": 8,
            "figure.dpi": 160,
            "savefig.dpi": 200,
        }
    )
    fig, (raw_ax, best_ax) = plt.subplots(1, 2, figsize=(10.2, 3.8), sharex=True)

    for eta, color in zip(ETAS, COLORS, strict=True):
        rows = read_rows(eta)
        valid = [row for row in rows if row["sys"] is not None]
        iterations = [row["iteration"] for row in valid]
        systems = [row["sys"] for row in valid]
        best = [row["best_sys"] for row in valid]
        raw_ax.plot(iterations, systems, color=color, label=eta_label(eta))
        best_ax.plot(iterations, best, color=color, label=eta_label(eta))
        failure = next((row for row in rows if row["failure"]), None)
        if failure is not None:
            for ax in (raw_ax, best_ax):
                ax.scatter(
                    failure["iteration"],
                    valid[-1]["sys"],
                    marker="x",
                    color=color,
                    s=42,
                    linewidth=2,
                    zorder=5,
                )
            raw_ax.annotate(
                "invalid geometry",
                (failure["iteration"], valid[-1]["sys"]),
                xytext=(8, -20),
                textcoords="offset points",
                color=color,
                fontsize=8,
            )

    for ax in (raw_ax, best_ax):
        ax.axvline(20, color="0.35", linestyle="--", linewidth=1)
        ax.set_xlabel("Iteration")
        ax.set_xlim(0, 100)
    raw_ax.set_ylabel("Full $\\mathrm{sys}(a)$")
    raw_ax.set_title("Raw unconditional trajectories")
    best_ax.set_ylabel("Best $\\mathrm{sys}$ seen so far")
    best_ax.set_title("Best-so-far trajectories")
    best_ax.legend(ncol=2, loc="lower right")
    fig.suptitle("Literal branch-gradient ascent: one common $F=6$ start")
    fig.tight_layout()

    for extension in ("png", "pdf"):
        fig.savefig(FIGURES / f"iteration-vs-sys.{extension}", bbox_inches="tight")
    plt.close(fig)


if __name__ == "__main__":
    main()
