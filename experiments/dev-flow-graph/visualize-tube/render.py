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
from figure_config import FONT_SIZE_SMALL, TEXT_WIDTH, setup

setup()

import matplotlib.pyplot as plt
from matplotlib.patches import FancyArrowPatch, Patch, Polygon
from matplotlib.lines import Line2D
from mpl_toolkits.mplot3d.art3d import Poly3DCollection


FACE_COLOR = "#b7c9e2"
TUBE_COLORS = {
    "start": "#d62728",
    "intermediate": "#ff7f0e",
    "end": "#2ca02c",
}
FIXED_COLOR = "#111111"
PROJECTION_CLIP_RADIUS = 3.5
PROJECTION_FACE_CLIP_RADIUS = 6.0


def main() -> None:
    args = parse_args()
    with args.input.open() as file:
        data = json.load(file)

    if args.layout == "sequence":
        render_sequence(data, args.output)
        return
    if args.layout == "projection":
        render_projection(data, args.output)
        return

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


def render_sequence(data, output):
    tube = data.get("closed_tube")
    if not tube:
        raise ValueError("sequence layout requires a nonempty closed tube")

    panels = tube["intermediate_polygons"]
    fixed_points = tube["fixed_points_on_faces"]
    if len(panels) != len(fixed_points):
        raise ValueError("tube panels and propagated fixed points must have equal length")
    if len(panels) > 7:
        raise ValueError("thesis sequence layout supports at most seven visited sections")

    faces_by_pair = {tuple(face["pair"]): face for face in data["all_two_faces"]}
    columns = 3 if len(panels) <= 6 else 4
    fig, axes = plt.subplots(2, columns, figsize=(TEXT_WIDTH, 4.5))
    axes = axes.ravel()
    panel_axes = (
        [axes[0], axes[1], axes[2], axes[5], axes[4], axes[3]]
        if len(panels) == 6
        else list(axes[: len(panels)])
    )
    shared_start_return_points = tube_focus_points(panels[0], fixed_points[0])
    shared_start_return_points.extend(tube_focus_points(panels[-1], fixed_points[-1]))

    for index, (panel, fixed, ax) in enumerate(zip(panels, fixed_points, panel_axes)):
        pair = tuple(panel["pair"])
        if tuple(fixed["pair"]) != pair:
            raise ValueError(f"panel/fixed-point pair mismatch at step {index}")
        face = faces_by_pair.get(pair)
        if face is None:
            raise ValueError(f"missing ambient facet-pair section {pair}")

        draw_polygon_or_points(
            ax, face["vertices"], FACE_COLOR, 0.35, FACE_COLOR, "full section"
        )
        color = TUBE_COLORS["intermediate"]
        draw_polygon_or_points(
            ax,
            panel["polygon"]["vertices"],
            color,
            0.55,
            color,
            "surviving tube",
            linewidth=2.0,
        )
        point = fixed.get("point")
        if point is not None:
            ax.scatter([point[0]], [point[1]], s=30, c=FIXED_COLOR, marker="x", zorder=5)

        role = panel["role"]
        role_label = "start" if role == "start" else "return" if role == "end" else f"step {index}"
        ax.set_title(rf"{role_label}: $P_{{{pair[0]}{pair[1]}}}$")
        if index in (0, len(panels) - 1):
            set_axis_limits_to_points(ax, shared_start_return_points, pad_fraction=0.2)
        else:
            set_axis_limits_to_tube(ax, panel, fixed)
        ax.set_aspect("equal", adjustable="box")
        ax.set_xticks([])
        ax.set_yticks([])
        ax.grid(False)

        inset = ax.inset_axes([0.72, 0.04, 0.26, 0.26])
        draw_polygon_or_points(
            inset, face["vertices"], FACE_COLOR, 0.35, FACE_COLOR, "full section"
        )
        draw_polygon_or_points(
            inset,
            panel["polygon"]["vertices"],
            color,
            0.7,
            color,
            "surviving tube",
            linewidth=1.0,
        )
        set_axis_limits(inset, face, [panel], [fixed])
        inset.set_aspect("equal", adjustable="box")
        inset.set_xticks([])
        inset.set_yticks([])
        inset.set_facecolor("white")

    for ax in axes:
        if ax not in panel_axes:
            ax.axis("off")

    fig.tight_layout(rect=(0.0, 0.12, 1.0, 0.94), w_pad=2.2, h_pad=2.0)
    fig.canvas.draw()
    for index, (source, target) in enumerate(zip(panel_axes, panel_axes[1:])):
        source_box = source.get_position()
        target_box = target.get_position()
        if len(panels) == 6:
            if index < 2:
                start = (source_box.x1 + 0.004, 0.5 * (source_box.y0 + source_box.y1))
                end = (target_box.x0 - 0.004, 0.5 * (target_box.y0 + target_box.y1))
            elif index == 2:
                arrow_x = target_box.x1 + 0.012
                start = (arrow_x, source_box.y0 - 0.004)
                end = (arrow_x, target_box.y1 + 0.004)
            else:
                start = (source_box.x0 - 0.004, 0.5 * (source_box.y0 + source_box.y1))
                end = (target_box.x1 + 0.004, 0.5 * (target_box.y0 + target_box.y1))
        elif abs(source_box.y0 - target_box.y0) < 0.02:
            start = (source_box.x1 + 0.004, 0.5 * (source_box.y0 + source_box.y1))
            end = (target_box.x0 - 0.004, 0.5 * (target_box.y0 + target_box.y1))
        else:
            start = (0.5 * (source_box.x0 + source_box.x1), source_box.y0 - 0.004)
            end = (0.5 * (target_box.x0 + target_box.x1), target_box.y1 + 0.004)
        fig.add_artist(
            FancyArrowPatch(
                start,
                end,
                transform=fig.transFigure,
                arrowstyle="-|>",
                mutation_scale=9,
                linewidth=0.9,
                color="#444444",
            )
        )
        pair = panels[index]["pair"]
        label_x = 0.5 * (start[0] + end[0]) + (0.012 if len(panels) == 6 and index == 2 else 0.0)
        fig.text(
            label_x,
            0.5 * (start[1] + end[1]),
            rf"$R_{{{pair[1]}}}$",
            fontsize=FONT_SIZE_SMALL,
            ha="center",
            va="center",
            bbox={"facecolor": "white", "edgecolor": "none", "pad": 0.5},
        )

    fig.legend(
        handles=[
            Patch(facecolor=FACE_COLOR, edgecolor=FACE_COLOR, alpha=0.35, label="full section"),
            Patch(
                facecolor=TUBE_COLORS["intermediate"],
                edgecolor=TUBE_COLORS["intermediate"],
                alpha=0.55,
                label="tube cross-section",
            ),
            Line2D([], [], color=FIXED_COLOR, marker="x", linestyle="None", label="closed orbit"),
        ],
        loc="lower center",
        ncol=3,
        frameon=False,
        bbox_to_anchor=(0.5, 0.01),
    )

    sigma = ",".join(str(entry) for entry in data["sigma"])
    fig.suptitle(rf"Closed tube for $\sigma=({sigma})$")
    output.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(output, facecolor="white", transparent=False)
    plt.close(fig)
    print(output)


def render_projection(data, output):
    vertices = np.asarray(data["vertices_f64"], dtype=float)
    orbit = data.get("closed_orbit")
    if orbit is None:
        raise ValueError("projection layout requires a solved closed orbit")
    breakpoints = np.asarray(orbit["breakpoints"], dtype=float)
    if len(breakpoints) != len(orbit["facets"]):
        raise ValueError("closed orbit must have one breakpoint per facet segment")

    projected_edge_points = []
    for first, second in data["edges"]:
        curve = projected_segment(vertices[first], vertices[second])
        if len(curve) < 2:
            continue
        projected_edge_points.append(curve)

    projected_orbit_points = []
    for index, start in enumerate(breakpoints):
        end = breakpoints[(index + 1) % len(breakpoints)]
        curve = projected_segment(start, end)
        if len(curve) < 2:
            continue
        projected_orbit_points.append(curve)

    projected_faces = []
    for index, face in enumerate(data["two_faces"]):
        points = project_points(vertices[face["vertices"]])
        if len(points) != len(face["vertices"]):
            continue
        projected_faces.append((order_polygon_3d(points), index))

    all_points = projected_edge_points + projected_orbit_points
    if not all_points:
        raise ValueError("stereographic projection produced no visible geometry")

    fig = plt.figure(figsize=(TEXT_WIDTH, 2.9))
    cameras = ((20, -58), (27, 28))
    colors = plt.get_cmap("tab20")
    for panel, (elevation, azimuth) in enumerate(cameras, start=1):
        ax = fig.add_subplot(1, 2, panel, projection="3d")
        for polygon, face_index in projected_faces:
            face_color = colors(face_index % 20)
            ax.add_collection3d(
                Poly3DCollection(
                    [polygon],
                    facecolor=face_color,
                    edgecolor=face_color,
                    linewidth=0.35,
                    alpha=0.22,
                )
            )
        for curve in projected_edge_points:
            ax.plot(
                curve[:, 0],
                curve[:, 1],
                curve[:, 2],
                color="#5f6469",
                alpha=0.72,
                linewidth=0.8,
            )
        for curve in projected_orbit_points:
            ax.plot(curve[:, 0], curve[:, 1], curve[:, 2], color="#6f2dbd", linewidth=2.7)
        set_equal_3d_limits(ax, np.concatenate(all_points))
        ax.view_init(elev=elevation, azim=azimuth)
        ax.set_axis_off()
        ax.text2D(0.04, 0.94, f"view {panel}", transform=ax.transAxes, fontsize=FONT_SIZE_SMALL)
    fig.subplots_adjust(left=0.0, right=1.0, bottom=0.0, top=1.0, wspace=-0.04)
    output.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(output, facecolor="white", transparent=False)
    plt.close(fig)
    print(output)


def projected_segment(start, end, samples=48):
    parameters = np.linspace(0.0, 1.0, samples)
    points = np.outer(1.0 - parameters, start) + np.outer(parameters, end)
    norms = np.linalg.norm(points, axis=1)
    valid = norms > 1e-12
    sphere = points[valid] / norms[valid, None]
    denominators = 1.0 - sphere[:, 3]
    visible = np.abs(denominators) > 0.04
    sphere = sphere[visible]
    denominators = denominators[visible]
    if len(sphere) == 0:
        return np.empty((0, 3))
    projected = sphere[:, :3] / denominators[:, None]
    return projected[np.linalg.norm(projected, axis=1) <= PROJECTION_CLIP_RADIUS]


def project_points(points):
    points = np.asarray(points, dtype=float)
    norms = np.linalg.norm(points, axis=1)
    if np.any(norms <= 1e-12):
        return np.empty((0, 3))
    sphere = points / norms[:, None]
    denominators = 1.0 - sphere[:, 3]
    if np.any(np.abs(denominators) <= 0.04):
        return np.empty((0, 3))
    projected = sphere[:, :3] / denominators[:, None]
    if np.any(np.linalg.norm(projected, axis=1) > PROJECTION_FACE_CLIP_RADIUS):
        return np.empty((0, 3))
    return projected


def order_polygon_3d(points):
    center = points.mean(axis=0)
    _, _, basis = np.linalg.svd(points - center, full_matrices=False)
    first = (points - center) @ basis[0]
    second = (points - center) @ basis[1]
    angles = np.arctan2(second, first)
    return points[np.argsort(angles)]


def set_equal_3d_limits(ax, points):
    center = 0.5 * (points.min(axis=0) + points.max(axis=0))
    radius = 0.38 * np.max(points.max(axis=0) - points.min(axis=0))
    radius = max(radius, 1e-6)
    ax.set_xlim(center[0] - radius, center[0] + radius)
    ax.set_ylim(center[1] - radius, center[1] + radius)
    ax.set_zlim(center[2] - radius, center[2] + radius)


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


def draw_polygon_or_points(ax, vertices, facecolor, alpha, edgecolor, label, linewidth=1.0):
    points = np.array(vertices, dtype=float)
    if len(points) == 0:
        return
    if len(points) >= 3:
        ordered = order_polygon_vertices(points)
        patch = Polygon(
            ordered,
            closed=True,
            facecolor=facecolor,
            edgecolor=edgecolor,
            alpha=alpha,
            linewidth=linewidth,
        )
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


def set_axis_limits_to_tube(ax, tube_face, fixed_point):
    set_axis_limits_to_points(ax, tube_focus_points(tube_face, fixed_point), pad_fraction=0.2)


def tube_focus_points(tube_face, fixed_point):
    points = list(tube_face["polygon"]["vertices"])
    if fixed_point.get("point") is not None:
        points.append(fixed_point["point"])
    return points


def set_axis_limits_to_points(ax, points, pad_fraction):
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
    pad = pad_fraction * span + 1e-6
    ax.set_xlim(mins[0] - pad[0], maxs[0] + pad[0])
    ax.set_ylim(mins[1] - pad[1], maxs[1] + pad[1])


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, default=EXPERIMENT_DIR / "tube-visualization.json")
    parser.add_argument("--output", type=Path, default=EXPERIMENT_DIR / "tube-visualization.png")
    parser.add_argument("--layout", choices=("grid", "sequence", "projection"), default="grid")
    return parser.parse_args()


if __name__ == "__main__":
    main()
