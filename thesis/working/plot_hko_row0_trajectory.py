#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""Plot one HKO witness-entry trajectory in the q- and p-planes."""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parents[1]
sys.path.insert(0, str(REPO_ROOT / "experiments"))

from figure_config import DPI, FIGSIZE_DUAL, FONT_SIZE_SMALL, setup  # noqa: E402

WITNESS_PATH = (
    REPO_ROOT
    / "experiments/hko-local-maximum/theorem/feasible-section-certificate/"
    / "witness.json"
)
PNG_PATH = SCRIPT_DIR / "hko-row0-trajectory-projections.png"

PATH_COLOR = "#1f5aa6"
FIXED_COLOR = "#d55e00"
PENTAGON_COLOR = "#666666"
POINT_COLOR = "#202020"


def hko_dual_vertices() -> np.ndarray:
    t = math.tan(math.pi / 5)
    sqrt5 = math.sqrt(5)
    alpha = (3 - sqrt5) / 2
    beta = t * (1 + sqrt5) / 2
    sec36 = sqrt5 - 1
    return np.array(
        [
            [1, t, 0, 0],
            [-alpha, beta, 0, 0],
            [-sec36, 0, 0, 0],
            [-alpha, -beta, 0, 0],
            [1, -t, 0, 0],
            [0, 0, t, -1],
            [0, 0, beta, alpha],
            [0, 0, 0, sec36],
            [0, 0, -beta, alpha],
            [0, 0, -t, -1],
        ],
        dtype=float,
    )


def reeb_vector(dual: np.ndarray) -> np.ndarray:
    return 2.0 * np.array([-dual[2], -dual[3], dual[0], dual[1]], dtype=float)


def polygon_vertices_from_facets(normals: np.ndarray) -> np.ndarray:
    vertices = []
    for index in range(len(normals)):
        matrix = np.array([normals[index], normals[(index + 1) % len(normals)]])
        vertices.append(np.linalg.solve(matrix, np.ones(2)))
    return np.array(vertices, dtype=float)


def recover_breakpoints(
    duals: np.ndarray, sigma: list[int], beta: np.ndarray, action: float
) -> np.ndarray:
    dwell_times = action * beta
    displacements = [np.zeros(4)]
    for dwell_time, facet in zip(dwell_times, sigma):
        displacements.append(displacements[-1] + dwell_time * reeb_vector(duals[facet]))
    displacements = np.array(displacements)

    matrix = []
    rhs = []
    for word_index, facet in enumerate(sigma):
        dual = duals[facet]
        matrix.append(dual)
        rhs.append(1.0 - dual.dot(displacements[word_index]))
    matrix = np.array(matrix)
    rhs = np.array(rhs)
    base_point, residuals, _rank, _singular_values = np.linalg.lstsq(
        matrix, rhs, rcond=None
    )
    breakpoints = base_point + displacements

    closure_error = np.linalg.norm(breakpoints[-1] - breakpoints[0])
    on_facet_error = np.max(np.abs(matrix @ base_point - rhs))
    inside_violation = max(dual.dot(point) - 1.0 for dual in duals for point in breakpoints)
    least_square_residual = residuals[0] if residuals.size else 0.0
    assert closure_error < 1e-10, closure_error
    assert on_facet_error < 1e-10, on_facet_error
    assert inside_violation < 1e-10, inside_violation
    assert least_square_residual < 1e-20, least_square_residual
    return breakpoints


def draw_panel(
    ax: plt.Axes,
    *,
    title: str,
    pentagon: np.ndarray,
    facet_labels: list[int],
    path: np.ndarray,
    fixed_indices: list[int],
) -> None:
    closed_pentagon = np.vstack([pentagon, pentagon[0]])
    ax.fill(pentagon[:, 0], pentagon[:, 1], color="#f8f8f8", zorder=0)
    ax.plot(closed_pentagon[:, 0], closed_pentagon[:, 1], color=PENTAGON_COLOR, linewidth=1.2)

    center = pentagon.mean(axis=0)
    for index, facet in enumerate(facet_labels):
        midpoint = (pentagon[index] + pentagon[(index + 1) % len(pentagon)]) / 2
        label_point = midpoint + 0.13 * (midpoint - center)
        ax.text(
            label_point[0],
            label_point[1],
            str(facet),
            color="#555555",
            ha="center",
            va="center",
            fontsize=FONT_SIZE_SMALL,
        )

    for word_index, (start, end) in enumerate(zip(path[:-1], path[1:])):
        fixed = word_index in fixed_indices
        color = FIXED_COLOR if fixed else PATH_COLOR
        linewidth = 2.2 if fixed else 1.5
        if np.linalg.norm(end - start) > 1e-9:
            ax.plot(
                [start[0], end[0]],
                [start[1], end[1]],
                color=color,
                linewidth=linewidth,
                solid_capstyle="round",
                zorder=2,
            )
        else:
            ax.scatter(
                [start[0]],
                [start[1]],
                s=38 if fixed else 20,
                facecolors="white",
                edgecolors=color,
                linewidths=linewidth,
                zorder=3,
            )

    distinct_points = []
    for point in path[:-1]:
        if not any(np.linalg.norm(point - old) < 1e-9 for old in distinct_points):
            distinct_points.append(point)
    distinct_points = np.array(distinct_points)
    ax.scatter(distinct_points[:, 0], distinct_points[:, 1], s=12, color=POINT_COLOR, zorder=4)

    all_points = np.vstack([pentagon, path])
    min_xy = all_points.min(axis=0)
    max_xy = all_points.max(axis=0)
    pad = 0.10 * max(max_xy - min_xy)
    ax.set_xlim(min_xy[0] - pad, max_xy[0] + pad)
    ax.set_ylim(min_xy[1] - pad, max_xy[1] + pad)
    ax.set_aspect("equal", adjustable="box")
    ax.set_xticks([])
    ax.set_yticks([])
    ax.set_title(title)
    ax.grid(False)
    for spine in ax.spines.values():
        spine.set_visible(False)


def row_breakpoints(row: dict, duals: np.ndarray) -> np.ndarray:
    sigma = list(row["sigma"])
    beta = np.array(row["beta_f64"], dtype=float)
    action = float(row["action_f64"])
    return recover_breakpoints(duals, sigma, beta, action)


def main() -> None:
    setup()
    with WITNESS_PATH.open() as handle:
        row = json.load(handle)["entries"][0]

    sigma = list(row["sigma"])
    fixed_indices = list(row["fixed_beta_indices"])
    duals = hko_dual_vertices()
    breakpoints = row_breakpoints(row, duals)
    q_pentagon = polygon_vertices_from_facets(duals[:5, :2])
    p_pentagon = polygon_vertices_from_facets(duals[5:, 2:])

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)
    draw_panel(
        axes[0],
        title=r"$q$-projection",
        pentagon=q_pentagon,
        facet_labels=[0, 1, 2, 3, 4],
        path=breakpoints[:, :2],
        fixed_indices=fixed_indices,
    )
    draw_panel(
        axes[1],
        title=r"$p$-projection",
        pentagon=p_pentagon,
        facet_labels=[5, 6, 7, 8, 9],
        path=breakpoints[:, 2:],
        fixed_indices=fixed_indices,
    )
    fig.subplots_adjust(left=0.02, right=0.98, top=0.88, bottom=0.04, wspace=0.12)
    fig.savefig(PNG_PATH, dpi=DPI)
    print(f"wrote {PNG_PATH}")


if __name__ == "__main__":
    main()
