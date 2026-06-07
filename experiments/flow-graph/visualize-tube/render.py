# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: Render one flow-graph tube visualization JSON file.
Input Artifacts: JSON from flow-graph-visualize-tube-data.
Output Artifacts: PNG figure.
"""

import argparse
import json
import math
import sys
from pathlib import Path

import numpy as np

EXPERIMENT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(EXPERIMENT_DIR.parent.parent))
from figure_config import FIGSIZE_SQUARE, FONT_SIZE_SMALL, setup

setup()

import matplotlib.pyplot as plt
from matplotlib.patches import Polygon


FACE_COLOR = "#b7c9e2"
TUBE_COLORS = {
    "start": "#d62728",
    "intermediate": "#ff7f0e",
    "end": "#2ca02c",
}
FIXED_COLOR = "#111111"


def main() -> None:
    args = parse_args()
    with args.input.open() as file:
        data = json.load(file)

    tube_faces_by_pair = {}
    fixed_points_by_pair = {}
    tube = data.get("closed_tube")
    if tube:
        for face in tube["intermediate_polygons"]:
            tube_faces_by_pair.setdefault(tuple(face["pair"]), []).append(face)
        for fixed in tube["fixed_points_on_faces"]:
            fixed_points_by_pair.setdefault(tuple(fixed["pair"]), []).append(fixed)

    faces_by_pair = {tuple(face["pair"]): face for face in data["all_two_faces"]}
    facet_count = data["facet_count"]
    cols = facet_count
    rows = facet_count
    fig, axes = plt.subplots(
        rows,
        cols,
        figsize=(1.7 * cols, 1.7 * rows),
        constrained_layout=True,
    )
    axes = np.array(axes).reshape(rows, cols)

    for first in range(facet_count):
        for second in range(facet_count):
            ax = axes[first, second]
            pair = (first, second)
            face = faces_by_pair.get(pair)
            if first == second or face is None:
                ax.axis("off")
                continue
            draw_face_panel(ax, face, pair, tube_faces_by_pair, fixed_points_by_pair)

    if tube:
        fixed = tube["fixed_point"]
        fig.suptitle(
            f"Flow-graph closed tube sigma={data['sigma']}  fixed={fixed['status']}  action={fixed.get('action')}",
            fontsize=10,
        )

    args.output.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(args.output)
    plt.close(fig)
    print(args.output)


def draw_face_panel(ax, face, pair, tube_faces_by_pair, fixed_points_by_pair):
        pair = tuple(face["pair"])
        ax.set_title(f"frame F{pair[0]} ∩ F{pair[1]}", fontsize=FONT_SIZE_SMALL)
        draw_polygon_or_points(ax, face["vertices"], FACE_COLOR, 0.25, FACE_COLOR, "2-face")

        for tube_face in tube_faces_by_pair.get(pair, []):
            color = TUBE_COLORS.get(tube_face["role"], "#9467bd")
            draw_polygon_or_points(
                ax,
                tube_face["polygon"]["vertices"],
                color,
                0.35,
                color,
                tube_face["role"],
            )

        for fixed in fixed_points_by_pair.get(pair, []):
            point = fixed.get("point")
            if point is not None:
                ax.scatter([point[0]], [point[1]], s=28, c=FIXED_COLOR, marker="x", zorder=5)

        set_axis_limits(ax, face, tube_faces_by_pair.get(pair, []), fixed_points_by_pair.get(pair, []))
        ax.set_aspect("equal", adjustable="box")
        ax.tick_params(labelsize=FONT_SIZE_SMALL)


def draw_polygon_or_points(ax, vertices, facecolor, alpha, edgecolor, label):
    points = np.array(vertices, dtype=float)
    if len(points) == 0:
        return
    if len(points) >= 3:
        ordered = order_polygon_vertices(points)
        patch = Polygon(ordered, closed=True, facecolor=facecolor, edgecolor=edgecolor, alpha=alpha, linewidth=1.0)
        ax.add_patch(patch)
    elif len(points) == 2:
        ax.plot(points[:, 0], points[:, 1], color=edgecolor, linewidth=1.5, alpha=0.9)
        ax.scatter(points[:, 0], points[:, 1], s=14, c=edgecolor, alpha=0.9)
    else:
        ax.scatter(points[:, 0], points[:, 1], s=18, c=edgecolor, alpha=0.95)


def order_polygon_vertices(points):
    center = points.mean(axis=0)
    angles = np.arctan2(points[:, 1] - center[1], points[:, 0] - center[0])
    return points[np.argsort(angles)]


def set_axis_limits(ax, face, tube_faces, fixed_points):
    points = []
    points.extend(face["vertices"])
    for tube_face in tube_faces:
        points.extend(tube_face["polygon"]["vertices"])
    for fixed in fixed_points:
        if fixed.get("point") is not None:
            points.append(fixed["point"])
    if not points:
        ax.set_xlim(-1, 1)
        ax.set_ylim(-1, 1)
        return
    arr = np.array(points, dtype=float)
    finite = arr[np.isfinite(arr).all(axis=1)]
    if len(finite) == 0:
        ax.set_xlim(-1, 1)
        ax.set_ylim(-1, 1)
        return
    mins = finite.min(axis=0)
    maxs = finite.max(axis=0)
    span = np.maximum(maxs - mins, 1e-6)
    pad = 0.12 * span + 1e-6
    ax.set_xlim(mins[0] - pad[0], maxs[0] + pad[0])
    ax.set_ylim(mins[1] - pad[1], maxs[1] + pad[1])


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, default=EXPERIMENT_DIR / "tube-visualization.json")
    parser.add_argument("--output", type=Path, default=EXPERIMENT_DIR / "tube-visualization.png")
    return parser.parse_args()


if __name__ == "__main__":
    main()
