#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: summarize the pentagon-rotation minima surface and render the sigma-level
branch plots needed for the 3-bounce exclusion.
Input Artifacts:
  - preferred smoke minima run: experiments/sys-landscape/pentagon-rotation-formula/smoke-theta-sweep.jsonl
  - canonical minima refresh: experiments/sys-landscape/pentagon-rotation-formula/theta-sweep.jsonl
  - preferred smoke branch run: experiments/sys-landscape/pentagon-rotation-formula/smoke-three-bounce-branches.jsonl
  - canonical branch refresh: experiments/sys-landscape/pentagon-rotation-formula/three-bounce-branches.jsonl
Output Artifacts:
  - experiments/sys-landscape/pentagon-rotation-formula/three_bounce_branch_actions.png
  - experiments/sys-landscape/pentagon-rotation-formula/signature_state_table_full.png
  - experiments/sys-landscape/pentagon-rotation-formula/signature_state_table_competitive.png
  - experiments/sys-landscape/pentagon-rotation-formula/labeled_pentagons_theta.png
  - experiments/sys-landscape/pentagon-rotation-formula/trajectory_projections_theta14.png
  - experiments/sys-landscape/pentagon-rotation-formula/trajectory_projections_theta14_affine.png
  - experiments/sys-landscape/pentagon-rotation-formula/signature_legend.txt
"""

import json
import math
import sys
from string import ascii_uppercase
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.axes import Axes
from matplotlib.colors import BoundaryNorm, ListedColormap
from matplotlib.patches import Arc, Patch
from matplotlib.transforms import blended_transform_factory

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import FIGSIZE_DUAL, FIGSIZE_SINGLE, FIGSIZE_SQUARE, setup

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
MINIMA_SMOKE_DATA_PATH = EXPERIMENT_DIR / "smoke-theta-sweep.jsonl"
MINIMA_CANONICAL_DATA_PATH = EXPERIMENT_DIR / "theta-sweep.jsonl"
BRANCH_SMOKE_DATA_PATH = EXPERIMENT_DIR / "smoke-three-bounce-branches.jsonl"
BRANCH_CANONICAL_DATA_PATH = EXPERIMENT_DIR / "three-bounce-branches.jsonl"

BRANCH_FIGURE_PATH = EXPERIMENT_DIR / "three_bounce_branch_actions.png"
FULL_TABLE_PATH = EXPERIMENT_DIR / "signature_state_table_full.png"
COMPETITIVE_TABLE_PATH = EXPERIMENT_DIR / "signature_state_table_competitive.png"
PENTAGON_LABEL_FIGURE_PATH = EXPERIMENT_DIR / "labeled_pentagons_theta.png"
TRAJECTORY_PROJECTION_FIGURE_PATH = (
    EXPERIMENT_DIR / "trajectory_projections_theta14.png"
)
AFFINE_TRAJECTORY_PROJECTION_FIGURE_PATH = (
    EXPERIMENT_DIR / "trajectory_projections_theta14_affine.png"
)
SIGNATURE_LEGEND_PATH = EXPERIMENT_DIR / "signature_legend.txt"

HALF_DOMAIN_MAX_DEG = 18.0
HIGHLIGHT_COUNT = 6
TOKEN_SLOT_WIDTH = 2
GENERIC_THETA_DEG = 14.0
STATE_COLORS = {
    "absent": "#ffffff",
    "admissible": "#dbe6f2",
    "minimal": "#1f5aa6",
}
EDGE_COLORS = ["#0072B2", "#E69F00", "#009E73", "#CC79A7", "#D55E00"]
PENTAGON_EDGE_COLOR = "#7a7a7a"
PENTAGON_REFERENCE_COLOR = "#d0d0d0"
PENTAGON_LABEL_COLOR = "#3d3d3d"
PENTAGON_LABEL_BOX = "#f7f7f7"
TRAJECTORY_COLORS = {"minimum": "#1f5aa6"}


def preferred_path(smoke_path: Path, canonical_path: Path) -> Path:
    if smoke_path.exists():
        return smoke_path
    return canonical_path


def minima_data_path() -> Path:
    return preferred_path(MINIMA_SMOKE_DATA_PATH, MINIMA_CANONICAL_DATA_PATH)


def branch_data_path() -> Path:
    return preferred_path(BRANCH_SMOKE_DATA_PATH, BRANCH_CANONICAL_DATA_PATH)


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        raise FileNotFoundError(path)
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def load_minima_rows() -> list[dict]:
    return load_jsonl(minima_data_path())


def load_branch_rows() -> list[dict]:
    return load_jsonl(branch_data_path())


def conjectured_sys(theta: float) -> float:
    base = (5.0 + 2.0 * math.sqrt(5.0)) / 10.0
    folded = min(theta, math.pi / 5.0 - theta)
    return base / (math.cos(folded) ** 2)


def normalize_block(block: dict) -> tuple:
    if block["kind"] == "edge":
        return ("E", block["facets_rel"][0])
    return ("V", tuple(block["facets_rel"]))


def rotate_label(label: tuple, shift: int) -> tuple:
    kind, value = label
    if kind == "E":
        return ("E", (value + shift) % 5)
    a, b = value
    return ("V", tuple(sorted(((a + shift) % 5, (b + shift) % 5))))


def canonical_cycle(labels: list[tuple]) -> tuple:
    n = len(labels)
    rotations = [tuple(labels[i:] + labels[:i]) for i in range(n)]
    reversed_labels = list(reversed(labels))
    reverse_rotations = [
        tuple(reversed_labels[i:] + reversed_labels[:i]) for i in range(n)
    ]
    return min(rotations + reverse_rotations)


def orbit_signature(orbit: dict) -> tuple:
    q_labels = [normalize_block(block) for block in orbit["q_blocks"]]
    p_labels = [normalize_block(block) for block in orbit["p_blocks"]]
    representatives = []
    for shift in range(5):
        q_rot = [rotate_label(label, shift) for label in q_labels]
        p_rot = [rotate_label(label, shift) for label in p_labels]
        representatives.append((canonical_cycle(q_rot), canonical_cycle(p_rot)))
    return min(representatives)


def signature_bounce_count(signature: tuple) -> int:
    q_cycle, _ = signature
    return len(q_cycle)


def signature_sort_key(signature: tuple) -> tuple:
    return (signature_bounce_count(signature), format_signature(signature))


def format_block_token(label: tuple) -> str:
    kind, value = label
    if kind == "E":
        return str(value)
    a, b = value
    return f"{a}{b}"


def format_cycle(cycle: tuple) -> str:
    return "-".join(format_block_token(label) for label in cycle)


def format_signature(signature: tuple) -> str:
    q_cycle, p_cycle = signature
    return f"Q:{format_cycle(q_cycle)}|P:{format_cycle(p_cycle)}"


def format_cycle_aligned(cycle: tuple, slot_width: int = TOKEN_SLOT_WIDTH) -> str:
    tokens = [format_block_token(label) for label in cycle]
    return "-".join(f"{token:>{slot_width}}" for token in tokens)


def cycle_widths(signatures: list[tuple]) -> tuple[int, int]:
    q_width = max(len(format_cycle_aligned(q_cycle)) for q_cycle, _ in signatures)
    p_width = max(len(format_cycle_aligned(p_cycle)) for _, p_cycle in signatures)
    return q_width, p_width


def format_signature_aligned(
    signature: tuple, q_width: int | None = None, p_width: int | None = None
) -> str:
    q_cycle, p_cycle = signature
    q = format_cycle_aligned(q_cycle)
    p = format_cycle_aligned(p_cycle)
    if q_width is None:
        q_width = len(q)
    if p_width is None:
        p_width = len(p)
    return f"Q:{q:<{q_width}}  P:{p:<{p_width}}"


def orbit_raw_signature(orbit: dict) -> tuple:
    q_labels = tuple(normalize_block(block) for block in orbit["q_blocks"])
    p_labels = tuple(normalize_block(block) for block in orbit["p_blocks"])
    return (q_labels, p_labels)


def regular_pentagon_normals_and_heights() -> tuple[np.ndarray, np.ndarray]:
    n = 5
    inradius = math.cos(math.pi / n)
    normals = np.array(
        [
            [
                math.cos(math.pi / 2.0 + 2.0 * math.pi * k / n),
                math.sin(math.pi / 2.0 + 2.0 * math.pi * k / n),
            ]
            for k in range(n)
        ],
        dtype=float,
    )
    heights = np.full(n, inradius, dtype=float)
    return normals, heights


def polygon_vertices_from_hrep(normals: np.ndarray, heights: np.ndarray) -> np.ndarray:
    vertices = []
    n = len(normals)
    for i in range(n):
        j = (i + 1) % n
        mat = np.array([normals[i], normals[j]], dtype=float)
        rhs = np.array([heights[i], heights[j]], dtype=float)
        vertices.append(np.linalg.solve(mat, rhs))
    return np.array(vertices, dtype=float)


def rotate_points(points: np.ndarray, theta: float) -> np.ndarray:
    c, s = math.cos(theta), math.sin(theta)
    rot = np.array([[c, -s], [s, c]], dtype=float)
    return points @ rot.T


def lighten_color(hex_color: str, blend: float = 0.78) -> tuple[float, float, float]:
    rgb = np.array(plt.matplotlib.colors.to_rgb(hex_color))
    return tuple((1.0 - blend) * rgb + blend * np.ones(3))


def dual_vertices_for_theta(theta_deg: float) -> np.ndarray:
    normals_q, heights_q = regular_pentagon_normals_and_heights()
    theta = math.radians(theta_deg)
    normals_p = rotate_points(normals_q, theta)
    dual_q = np.column_stack(
        [
            normals_q[:, 0] / heights_q,
            normals_q[:, 1] / heights_q,
            np.zeros(5),
            np.zeros(5),
        ]
    )
    dual_p = np.column_stack(
        [
            np.zeros(5),
            np.zeros(5),
            normals_p[:, 0] / heights_q,
            normals_p[:, 1] / heights_q,
        ]
    )
    return np.vstack([dual_q, dual_p])


def recover_breakpoints(orbit: dict, theta_deg: float) -> np.ndarray:
    duals = dual_vertices_for_theta(theta_deg)
    sigma = orbit["sigma"]
    beta = np.array(orbit["beta"], dtype=float)
    capacity = float(orbit["action"])
    dwell_times = capacity * beta
    reeb_vectors = np.array(
        [
            2.0
            * np.array(
                [
                    -duals[facet, 2],
                    -duals[facet, 3],
                    duals[facet, 0],
                    duals[facet, 1],
                ],
                dtype=float,
            )
            for facet in sigma
        ]
    )

    displacements = [np.zeros(4, dtype=float)]
    for tau, reeb in zip(dwell_times, reeb_vectors):
        displacements.append(displacements[-1] + tau * reeb)
    displacements = np.array(displacements, dtype=float)

    mat = np.array([duals[facet] for facet in sigma], dtype=float)
    rhs = np.array(
        [1.0 - duals[facet].dot(displacements[k]) for k, facet in enumerate(sigma)],
        dtype=float,
    )
    base_point, *_ = np.linalg.lstsq(mat, rhs, rcond=None)
    breakpoints = base_point + displacements

    max_violation = max(
        float(dual.dot(point) - 1.0) for point in breakpoints for dual in duals
    )
    closure_error = float(np.linalg.norm(breakpoints[-1] - breakpoints[0]))
    if max_violation > 1e-7 or closure_error > 1e-7:
        raise RuntimeError(
            f"recovered orbit failed validation: max_violation={max_violation:.2e}, "
            f"closure_error={closure_error:.2e}"
        )
    return breakpoints


def projected_path(breakpoints: np.ndarray, coord_slice: slice, tol: float = 1e-9) -> np.ndarray:
    points = breakpoints[:, coord_slice]
    compressed = [points[0]]
    for point in points[1:]:
        if np.linalg.norm(point - compressed[-1]) > tol:
            compressed.append(point)
    if np.linalg.norm(compressed[0] - compressed[-1]) > tol:
        compressed.append(compressed[0])
    return np.array(compressed, dtype=float)


def segment_motion_type(
    start: np.ndarray, end: np.ndarray, tol: float = 1e-9
) -> str:
    dq = np.linalg.norm(end[:2] - start[:2])
    dp = np.linalg.norm(end[2:] - start[2:])
    if dq <= tol and dp > tol:
        return "Q"
    if dp <= tol and dq > tol:
        return "P"
    raise RuntimeError(f"unexpected segment type: dq={dq:.3e}, dp={dp:.3e}")


def compress_affine_cycle(breakpoints: np.ndarray) -> np.ndarray:
    if len(breakpoints) < 2:
        raise RuntimeError("cannot compress an empty orbit")

    segment_types = [
        segment_motion_type(start, end)
        for start, end in zip(breakpoints[:-1], breakpoints[1:])
    ]
    affine_points = [breakpoints[0]]
    current_type = segment_types[0]
    for index, segment_type in enumerate(segment_types):
        if segment_type != current_type:
            affine_points.append(breakpoints[index])
            current_type = segment_type
    affine_points.append(breakpoints[-1])
    return np.array(affine_points, dtype=float)


def filter_half_domain(rows: list[dict]) -> list[dict]:
    return [row for row in rows if row["angle_deg"] <= HALF_DOMAIN_MAX_DEG + 1e-12]


def summarize_minima(rows: list[dict]) -> list[tuple[float, list[tuple]]]:
    summary = []
    for row in rows:
        signatures = sorted({orbit_signature(orbit) for orbit in row["tied_orbits"]})
        summary.append((row["angle_deg"], signatures))
    return summary


def collect_signatures(minima_rows: list[dict], branch_rows: list[dict]) -> list[tuple]:
    signatures = set()
    for row in minima_rows:
        for orbit in row["tied_orbits"]:
            signatures.add(orbit_signature(orbit))
    for row in branch_rows:
        for orbit in row["admissible_three_bounce_orbits"]:
            signatures.add(orbit_signature(orbit))

    return sorted(signatures, key=signature_sort_key)


def write_signature_legend(
    signatures: list[tuple], minima_rows: list[dict], branch_rows: list[dict]
) -> None:
    q_width, p_width = cycle_widths(signatures)
    minima_presence = defaultdict(list)
    for row in minima_rows:
        angle = row["angle_deg"]
        for signature in {orbit_signature(orbit) for orbit in row["tied_orbits"]}:
            minima_presence[signature].append(angle)

    branch_presence = defaultdict(list)
    for row in branch_rows:
        angle = row["angle_deg"]
        for signature in {
            orbit_signature(orbit) for orbit in row["admissible_three_bounce_orbits"]
        }:
            branch_presence[signature].append(angle)

    header_lines = [
        "Canonical affine signature catalog",
        "",
        "Compact P5 notation",
        "  - k means the facet/edge e_k of the reference regular pentagon",
        "  - ij means the vertex e_i \\cap e_j",
        "  - compact form: Q:0-23-1|P:0-2-3",
        "  - aligned render: Q: 0-23- 1  P: 0- 2- 3",
        "",
        "Normalization used to compare across theta",
        "  - this is an affine support signature, not the full raw sigma",
        "  - q- and p-cycles are recorded separately as cyclic block lists",
        "  - cyclic starting point and reversal are forgotten inside each q/p cycle",
        "  - a common relabeling i -> i + k (mod 5) of both q and p labels is also forgotten",
        "",
        "Fields",
        "  - minimal angles: sampled theta values where a minimizing orbit has this signature",
        "  - admissible angles: sampled theta values where an admissible 3-bounce orbit has this signature",
    ]

    lines = []
    for signature in signatures:
        q_cycle, p_cycle = signature
        lines.append(
            "\n".join(
                [
                    format_signature_aligned(signature, q_width, p_width),
                    f"  bounces: {signature_bounce_count(signature)}",
                    f"  q blocks: {format_cycle(q_cycle)}",
                    f"  p blocks: {format_cycle(p_cycle)}",
                    f"  minimal angles (deg): {minima_presence.get(signature, [])}",
                    f"  admissible angles (deg): {branch_presence.get(signature, [])}",
                    f"  raw: {signature!r}",
                ]
            )
        )
    SIGNATURE_LEGEND_PATH.write_text(
        "\n".join(header_lines) + "\n\n" + "\n\n".join(lines) + "\n"
    )
    print(f"Saved: {SIGNATURE_LEGEND_PATH}")


def branch_curves_by_signature(branch_rows: list[dict]) -> dict[tuple, list[tuple[float, float]]]:
    branch_curves = defaultdict(list)
    for row in branch_rows:
        angle_deg = row["angle_deg"]
        grouped = defaultdict(list)
        for orbit in row["admissible_three_bounce_orbits"]:
            signature = orbit_signature(orbit)
            grouped[signature].append(orbit["action"])
        for signature, actions in grouped.items():
            branch_curves[signature].append((angle_deg, min(actions)))
    return branch_curves


def highlighted_branch_signatures(
    minima_rows: list[dict], branch_rows: list[dict]
) -> list[tuple]:
    branch_curves = branch_curves_by_signature(branch_rows)
    angle_to_minimum = {row["angle_deg"]: row["capacity"] for row in minima_rows}
    best_gaps = []
    for signature, points in branch_curves.items():
        points = sorted(points)
        open_points = [
            (angle, action)
            for angle, action in points
            if angle < HALF_DOMAIN_MAX_DEG - 1e-12
        ]
        open_gap = min(
            (action - angle_to_minimum[angle] for angle, action in open_points),
            default=float("inf"),
        )
        overall_gap = min(action - angle_to_minimum[angle] for angle, action in points)
        best_gaps.append((open_gap, overall_gap, signature))
    return [signature for _, _, signature in sorted(best_gaps)[:HIGHLIGHT_COUNT]]


def representative_orbits_for_theta(
    theta_deg: float, minima_rows: list[dict], branch_rows: list[dict]
) -> list[tuple[str, dict]]:
    minima_row = next(row for row in minima_rows if abs(row["angle_deg"] - theta_deg) < 1e-12)
    branch_row = next(row for row in branch_rows if abs(row["angle_deg"] - theta_deg) < 1e-12)

    minimum_orbit = min(
        minima_row["tied_orbits"],
        key=lambda orbit: (orbit["action"], format_signature(orbit_signature(orbit))),
    )

    best_three_by_signature = {}
    for orbit in branch_row["admissible_three_bounce_orbits"]:
        signature = orbit_signature(orbit)
        candidate_key = (orbit["action"], orbit["sigma"])
        current = best_three_by_signature.get(signature)
        if current is None or candidate_key < (current["action"], current["sigma"]):
            best_three_by_signature[signature] = orbit

    competitor = min(
        best_three_by_signature.values(),
        key=lambda orbit: (orbit["action"], format_signature(orbit_signature(orbit))),
    )

    return [("minimum", minimum_orbit), ("competitor", competitor)]


def minimum_orbit_for_theta(theta_deg: float, minima_rows: list[dict]) -> dict:
    minima_row = next(row for row in minima_rows if abs(row["angle_deg"] - theta_deg) < 1e-12)
    return min(
        minima_row["tied_orbits"],
        key=lambda orbit: (orbit["action"], format_signature(orbit_signature(orbit))),
    )


def plot_three_bounce_branches(
    minima_rows: list[dict], branch_rows: list[dict]
) -> None:
    branch_curves = branch_curves_by_signature(branch_rows)
    minima_angles = np.array([row["angle_deg"] for row in minima_rows], dtype=float)
    minima_actions = np.array([row["capacity"] for row in minima_rows], dtype=float)
    highlighted_list = highlighted_branch_signatures(minima_rows, branch_rows)
    highlighted = set(highlighted_list)
    q_width, p_width = cycle_widths(highlighted_list)
    highlight_colors = ["#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e", "#e6ab02"]

    fig, ax = plt.subplots(figsize=FIGSIZE_SQUARE)
    for signature, points in sorted(branch_curves.items(), key=lambda item: signature_sort_key(item[0])):
        points = sorted(points)
        x = np.array([angle for angle, _ in points], dtype=float)
        y = np.array([action for _, action in points], dtype=float)
        if signature in highlighted:
            idx = highlighted_list.index(signature)
            ax.plot(
                x,
                y,
                color=highlight_colors[idx % len(highlight_colors)],
                linewidth=1.8,
                label=format_signature_aligned(signature, q_width, p_width),
            )
        else:
            ax.plot(x, y, color="#b0b0b0", linewidth=0.9, alpha=0.7)

    ax.plot(
        minima_angles,
        minima_actions,
        color="black",
        linewidth=2.0,
        label=format_signature_aligned(
            ((("E", 0), ("V", (2, 3))), (("E", 0), ("V", (2, 3)))),
            q_width,
            p_width,
        ),
        zorder=5,
    )
    competitive_y_values = minima_actions.tolist()
    for signature in highlighted_list:
        competitive_y_values.extend(action for _, action in branch_curves[signature])
    all_branch_actions = np.array(
        [action for points in branch_curves.values() for _, action in points],
        dtype=float,
    )
    y_min = min(competitive_y_values)
    y_max = max(
        max(competitive_y_values),
        float(np.quantile(all_branch_actions, 0.92)),
    )
    y_pad = 0.08 * (y_max - y_min)
    ax.set_ylim(y_min - y_pad, y_max + y_pad)
    ax.set_xlabel(r"Rotation angle $\theta$ (degrees)")
    ax.set_ylabel(r"Capacity / action")
    ax.set_title(r"Admissible $3$-bounce signature actions on $0^\circ$--$18^\circ$")
    legend = ax.legend(loc="upper left")
    for text in legend.get_texts():
        text.set_fontfamily("monospace")
    fig.tight_layout()
    fig.savefig(BRANCH_FIGURE_PATH)
    plt.close(fig)
    print(f"Saved: {BRANCH_FIGURE_PATH}")


def plot_signature_table(
    signatures: list[tuple],
    minima_rows: list[dict],
    branch_rows: list[dict],
    output_path: Path,
    title: str,
) -> None:
    minima_summary = summarize_minima(minima_rows)
    branch_summary = []
    for row in branch_rows:
        row_signatures = sorted(
            {orbit_signature(orbit) for orbit in row["admissible_three_bounce_orbits"]}
        )
        branch_summary.append((row["angle_deg"], row_signatures))
    angles = [angle for angle, _ in minima_summary]
    minima_by_angle = {angle: set(signatures) for angle, signatures in minima_summary}
    branch_by_angle = {angle: set(signatures) for angle, signatures in branch_summary}

    matrix = []
    for signature in signatures:
        row = []
        for angle in angles:
            state = 0
            if signature in branch_by_angle.get(angle, set()):
                state = 1
            if signature in minima_by_angle.get(angle, set()):
                state = 2
            row.append(state)
        matrix.append(row)

    fig = plt.figure(figsize=FIGSIZE_SQUARE)
    grid = fig.add_gridspec(
        nrows=2,
        ncols=2,
        height_ratios=[0.55, 4.85],
        width_ratios=[1.08, 1.92],
        hspace=0.02,
        wspace=0.02,
    )
    legend_ax = fig.add_subplot(grid[0, :])
    label_ax = fig.add_subplot(grid[1, 0])
    heatmap_ax = fig.add_subplot(grid[1, 1])

    cmap = ListedColormap(
        [
            STATE_COLORS["absent"],
            STATE_COLORS["admissible"],
            STATE_COLORS["minimal"],
        ]
    )
    norm = BoundaryNorm([-0.5, 0.5, 1.5, 2.5], cmap.N)
    heatmap_ax.imshow(
        matrix,
        aspect="auto",
        interpolation="nearest",
        cmap=cmap,
        norm=norm,
    )
    heatmap_ax.grid(False)
    heatmap_ax.set_xlabel(r"Rotation angle $\theta$ (degrees)")
    heatmap_ax.set_title(title)
    heatmap_ax.set_xticks(range(len(angles)))
    heatmap_ax.set_xticklabels(
        [f"{angle:.1f}" if index % 8 == 0 else "" for index, angle in enumerate(angles)]
    )
    heatmap_ax.set_yticks([])
    heatmap_ax.set_xticks(np.arange(-0.5, len(angles), 1.0), minor=True)
    heatmap_ax.set_yticks(np.arange(-0.5, len(signatures), 1.0), minor=True)
    heatmap_ax.grid(which="minor", color="#ececec", linewidth=0.25)
    heatmap_ax.tick_params(which="minor", bottom=False, left=False)
    for spine in heatmap_ax.spines.values():
        spine.set_color("#b8b8b8")
        spine.set_linewidth(0.35)

    label_ax.set_xlim(0.0, 1.0)
    label_ax.set_ylim(len(signatures) - 0.5, -0.5)
    label_ax.axis("off")
    text_transform = blended_transform_factory(label_ax.transAxes, label_ax.transData)
    label_ax.text(
        0.00,
        1.01,
        "Q",
        transform=label_ax.transAxes,
        ha="left",
        va="bottom",
        fontfamily="monospace",
        fontsize=8.0,
        color="#7a2e00",
    )
    label_ax.text(
        0.56,
        1.01,
        "P",
        transform=label_ax.transAxes,
        ha="left",
        va="bottom",
        fontfamily="monospace",
        fontsize=8.0,
        color="#0b4f8a",
    )
    for row_index, signature in enumerate(signatures):
        q_cycle, p_cycle = signature
        label_ax.text(
            0.00,
            row_index,
            format_cycle_aligned(q_cycle),
            transform=text_transform,
            va="center",
            ha="left",
            fontfamily="monospace",
            fontsize=7.2,
        )
        label_ax.text(
            0.56,
            row_index,
            format_cycle_aligned(p_cycle),
            transform=text_transform,
            va="center",
            ha="left",
            fontfamily="monospace",
            fontsize=7.2,
        )

    legend_ax.axis("off")
    handles = [
        Patch(facecolor=STATE_COLORS["absent"], edgecolor="#909090", label="absent"),
        Patch(facecolor=STATE_COLORS["admissible"], edgecolor="#909090", label="admissible"),
        Patch(facecolor=STATE_COLORS["minimal"], edgecolor=STATE_COLORS["minimal"], label="minimal"),
    ]
    legend_ax.legend(
        handles=handles,
        loc="center",
        ncols=3,
        frameon=False,
        handlelength=1.4,
        columnspacing=1.5,
    )

    fig.subplots_adjust(left=0.08, right=0.98, bottom=0.10, top=0.95)
    fig.savefig(output_path)
    plt.close(fig)
    print(f"Saved: {output_path}")


def plot_minimum_signature_table(
    minima_rows: list[dict], branch_rows: list[dict]
) -> None:
    signatures = sorted(
        {
            signature
            for angle, row_signatures in summarize_minima(minima_rows)
            for signature in row_signatures
        }
        | {
            signature
            for row in branch_rows
            for signature in {
                orbit_signature(orbit) for orbit in row["admissible_three_bounce_orbits"]
            }
        },
        key=signature_sort_key,
    )
    plot_signature_table(
        signatures,
        minima_rows,
        branch_rows,
        FULL_TABLE_PATH,
        r"All signature states on $0^\circ$--$18^\circ$",
    )


def plot_focus_signature_table(
    minima_rows: list[dict], branch_rows: list[dict]
) -> None:
    minimal_signatures = {
        signature
        for _, row_signatures in summarize_minima(minima_rows)
        for signature in row_signatures
    }
    focus_signatures = sorted(
        minimal_signatures | set(highlighted_branch_signatures(minima_rows, branch_rows)),
        key=signature_sort_key,
    )
    plot_signature_table(
        focus_signatures,
        minima_rows,
        branch_rows,
        COMPETITIVE_TABLE_PATH,
        r"Competitive signature states on $0^\circ$--$18^\circ$",
    )


def draw_labeled_pentagon_panel(
    ax: Axes,
    vertices: np.ndarray,
    panel_normals: np.ndarray,
    title: str,
    reference_vertices: np.ndarray | None = None,
) -> None:
    if reference_vertices is not None:
        reference_closed = np.vstack([reference_vertices, reference_vertices[0]])
        ax.plot(
            reference_closed[:, 0],
            reference_closed[:, 1],
            color=PENTAGON_REFERENCE_COLOR,
            linewidth=1.0,
            linestyle=(0, (3, 2)),
            zorder=1,
        )

    for k in range(5):
        prev_vertex = vertices[(k - 1) % 5]
        vertex = vertices[k]
        ax.plot(
            [prev_vertex[0], vertex[0]],
            [prev_vertex[1], vertex[1]],
            color=PENTAGON_EDGE_COLOR,
            linewidth=1.4,
            zorder=3,
        )

    for k in range(5):
        vertex = vertices[k]
        radial = vertex / np.linalg.norm(vertex)
        ax.text(
            vertex[0] + 0.13 * radial[0],
            vertex[1] + 0.13 * radial[1],
            f"{k}{(k + 1) % 5}",
            ha="center",
            va="center",
            fontsize=7.8,
            fontfamily="monospace",
            color=PENTAGON_LABEL_COLOR,
            bbox={"facecolor": PENTAGON_LABEL_BOX, "edgecolor": "none", "pad": 0.2, "alpha": 0.95},
            zorder=4,
        )

        prev_vertex = vertices[(k - 1) % 5]
        midpoint = 0.5 * (prev_vertex + vertex)
        normal = panel_normals[k] / np.linalg.norm(panel_normals[k])
        ax.text(
            midpoint[0] + 0.17 * normal[0],
            midpoint[1] + 0.17 * normal[1],
            f"e{k}",
            ha="center",
            va="center",
            fontsize=7.8,
            color=PENTAGON_LABEL_COLOR,
            bbox={
                "facecolor": PENTAGON_LABEL_BOX,
                "edgecolor": "none",
                "pad": 0.15,
                "alpha": 0.95,
            },
            zorder=4,
        )

    ax.set_title(title)
    ax.set_aspect("equal")
    ax.set_xlim(-1.65, 1.65)
    ax.set_ylim(-1.55, 1.55)
    ax.axis("off")


def draw_theta_marker(
    ax: Axes, reference_normals: np.ndarray, rotated_normals: np.ndarray
) -> None:
    ref_direction = reference_normals[0] / np.linalg.norm(reference_normals[0])
    rot_direction = rotated_normals[0] / np.linalg.norm(rotated_normals[0])
    ray_radius = 0.72
    ax.plot(
        [0.0, ray_radius * ref_direction[0]],
        [0.0, ray_radius * ref_direction[1]],
        color="#8f8f8f",
        linewidth=1.0,
        linestyle=(0, (2, 2)),
        zorder=2,
    )
    ax.plot(
        [0.0, ray_radius * rot_direction[0]],
        [0.0, ray_radius * rot_direction[1]],
        color="#1f5aa6",
        linewidth=1.2,
        zorder=2,
    )
    theta1 = math.degrees(math.atan2(ref_direction[1], ref_direction[0]))
    theta2 = theta1 + GENERIC_THETA_DEG
    ax.add_patch(
        Arc(
            (0.0, 0.0),
            0.78,
            0.78,
            angle=0.0,
            theta1=theta1,
            theta2=theta2,
            color="#1f5aa6",
            linewidth=1.0,
            zorder=2,
        )
    )
    mid_angle = math.radians(theta1 + 0.5 * GENERIC_THETA_DEG)
    ax.text(
        0.53 * math.cos(mid_angle),
        0.53 * math.sin(mid_angle),
        rf"$\theta={GENERIC_THETA_DEG:.0f}^\circ$",
        color="#1f5aa6",
        ha="center",
        va="center",
        fontsize=8.5,
        bbox={"facecolor": "white", "edgecolor": "none", "pad": 0.15, "alpha": 0.9},
        zorder=4,
    )


def plot_labeled_pentagons() -> None:
    normals, heights = regular_pentagon_normals_and_heights()
    vertices_q = polygon_vertices_from_hrep(normals, heights)
    theta = math.radians(GENERIC_THETA_DEG)
    normals_p = rotate_points(normals, theta)
    vertices_p = rotate_points(vertices_q, theta)

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)
    draw_labeled_pentagon_panel(axes[0], vertices_q, normals, "Q factor")
    draw_labeled_pentagon_panel(
        axes[1],
        vertices_p,
        normals_p,
        rf"P factor $= R(\theta)P_5$",
        reference_vertices=vertices_q,
    )
    draw_theta_marker(axes[1], normals, normals_p)

    fig.tight_layout()
    fig.savefig(PENTAGON_LABEL_FIGURE_PATH)
    plt.close(fig)
    print(f"Saved: {PENTAGON_LABEL_FIGURE_PATH}")


def plot_trajectory_projections(minima_rows: list[dict]) -> None:
    normals, heights = regular_pentagon_normals_and_heights()
    vertices_q = polygon_vertices_from_hrep(normals, heights)
    theta = math.radians(GENERIC_THETA_DEG)
    normals_p = rotate_points(normals, theta)
    vertices_p = rotate_points(vertices_q, theta)
    orbit = minimum_orbit_for_theta(GENERIC_THETA_DEG, minima_rows)
    raw_signature = orbit_raw_signature(orbit)
    q_width, p_width = cycle_widths([raw_signature])

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)
    draw_labeled_pentagon_panel(axes[0], vertices_q, normals, "Q projection")
    draw_labeled_pentagon_panel(
        axes[1],
        vertices_p,
        normals_p,
        "P projection",
        reference_vertices=vertices_q,
    )
    draw_theta_marker(axes[1], normals, normals_p)

    breakpoints = recover_breakpoints(orbit, GENERIC_THETA_DEG)
    q_path = projected_path(breakpoints, slice(0, 2))
    p_path = projected_path(breakpoints, slice(2, 4))
    color = TRAJECTORY_COLORS["minimum"]

    for ax, path in zip(axes, (q_path, p_path)):
        ax.plot(
            path[:, 0],
            path[:, 1],
            color=color,
            linewidth=2.4,
            marker="o",
            markersize=4.0,
            markerfacecolor="white",
            markeredgewidth=0.9,
            zorder=5,
        )
        ax.scatter(
            [path[0, 0]],
            [path[0, 1]],
            color=color,
            s=28,
            zorder=6,
        )

    state_points = breakpoints[:-1]
    state_labels = list(ascii_uppercase[: len(state_points)])
    for ax, coord_slice in zip(axes, (slice(0, 2), slice(2, 4))):
        projected_states = state_points[:, coord_slice]
        groups: list[tuple[np.ndarray, list[str]]] = []
        for label, point in zip(state_labels, projected_states):
            for index, (group_point, group_labels) in enumerate(groups):
                if np.linalg.norm(point - group_point) < 1e-9:
                    groups[index] = (group_point, group_labels + [label])
                    break
            else:
                groups.append((point, [label]))

        for point, labels in groups:
            radial = point / max(np.linalg.norm(point), 1e-9)
            ax.text(
                point[0] + 0.10 * radial[0],
                point[1] + 0.10 * radial[1],
                ",".join(labels),
                ha="center",
                va="center",
                fontfamily="monospace",
                fontsize=8.3,
                color=color,
                bbox={"facecolor": "white", "edgecolor": "none", "pad": 0.15, "alpha": 0.92},
                zorder=7,
            )

    fig.suptitle(
        rf"Raw recovered orbit projections at $\theta={GENERIC_THETA_DEG:.0f}^\circ$",
        y=0.98,
    )
    fig.text(
        0.5,
        0.075,
        f"Absolute support blocks: {format_signature_aligned(raw_signature, q_width, p_width)}",
        ha="center",
        va="center",
        fontfamily="monospace",
        fontsize=9.0,
    )
    fig.text(
        0.5,
        0.035,
        "Sequence: A -> B -> C -> D -> E -> F -> A",
        ha="center",
        va="center",
        fontfamily="monospace",
        fontsize=8.4,
        color=color,
    )

    fig.subplots_adjust(bottom=0.18, top=0.90, wspace=0.18)
    fig.savefig(TRAJECTORY_PROJECTION_FIGURE_PATH)
    plt.close(fig)
    print(f"Saved: {TRAJECTORY_PROJECTION_FIGURE_PATH}")


def plot_affine_trajectory_projections(minima_rows: list[dict]) -> None:
    normals, heights = regular_pentagon_normals_and_heights()
    vertices_q = polygon_vertices_from_hrep(normals, heights)
    theta = math.radians(GENERIC_THETA_DEG)
    normals_p = rotate_points(normals, theta)
    vertices_p = rotate_points(vertices_q, theta)
    orbit = minimum_orbit_for_theta(GENERIC_THETA_DEG, minima_rows)
    raw_signature = orbit_raw_signature(orbit)
    q_width, p_width = cycle_widths([raw_signature])

    fig, axes = plt.subplots(1, 2, figsize=FIGSIZE_DUAL)
    draw_labeled_pentagon_panel(axes[0], vertices_q, normals, "Q projection")
    draw_labeled_pentagon_panel(
        axes[1],
        vertices_p,
        normals_p,
        "P projection",
        reference_vertices=vertices_q,
    )
    draw_theta_marker(axes[1], normals, normals_p)

    affine_breakpoints = compress_affine_cycle(
        recover_breakpoints(orbit, GENERIC_THETA_DEG)
    )
    q_path = affine_breakpoints[:, :2]
    p_path = affine_breakpoints[:, 2:]
    color = TRAJECTORY_COLORS["minimum"]

    for ax, path in zip(axes, (q_path, p_path)):
        ax.plot(
            path[:, 0],
            path[:, 1],
            color=color,
            linewidth=2.6,
            marker="o",
            markersize=4.2,
            markerfacecolor="white",
            markeredgewidth=0.9,
            zorder=5,
        )
        ax.scatter(
            [path[0, 0]],
            [path[0, 1]],
            color=color,
            s=28,
            zorder=6,
        )

    state_labels = list(ascii_uppercase[: len(affine_breakpoints) - 1])
    for ax, path in zip(axes, (q_path[:-1], p_path[:-1])):
        groups: list[tuple[np.ndarray, list[str]]] = []
        for label, point in zip(state_labels, path):
            for index, (group_point, group_labels) in enumerate(groups):
                if np.linalg.norm(point - group_point) < 1e-9:
                    groups[index] = (group_point, group_labels + [label])
                    break
            else:
                groups.append((point, [label]))

        for point, labels in groups:
            radial = point / max(np.linalg.norm(point), 1e-9)
            ax.text(
                point[0] + 0.10 * radial[0],
                point[1] + 0.10 * radial[1],
                ",".join(labels),
                ha="center",
                va="center",
                fontfamily="monospace",
                fontsize=8.4,
                color=color,
                bbox={"facecolor": "white", "edgecolor": "none", "pad": 0.15, "alpha": 0.92},
                zorder=7,
            )

    fig.suptitle(
        rf"Affine / convex-mixture orbit projections at $\theta={GENERIC_THETA_DEG:.0f}^\circ$",
        y=0.98,
    )
    fig.text(
        0.5,
        0.075,
        f"Absolute support blocks: {format_signature_aligned(raw_signature, q_width, p_width)}",
        ha="center",
        va="center",
        fontfamily="monospace",
        fontsize=9.0,
    )
    fig.text(
        0.5,
        0.035,
        "Alternating affine cycle: A -> B -> C -> D -> A",
        ha="center",
        va="center",
        fontfamily="monospace",
        fontsize=8.4,
        color=color,
    )

    fig.subplots_adjust(bottom=0.18, top=0.90, wspace=0.18)
    fig.savefig(AFFINE_TRAJECTORY_PROJECTION_FIGURE_PATH)
    plt.close(fig)
    print(f"Saved: {AFFINE_TRAJECTORY_PROJECTION_FIGURE_PATH}")


def print_text_summary(minima_rows: list[dict], branch_rows: list[dict]) -> None:
    print(f"minima_data_path={minima_data_path().name}")
    print(f"branch_data_path={branch_data_path().name}")
    print(f"rows={len(minima_rows)}")

    max_err = 0.0
    switch_rows = []
    previous_signatures = None
    for row in minima_rows:
        theta = row["angle_rad"]
        predicted = conjectured_sys(theta)
        err = abs(predicted - row["sys"])
        max_err = max(max_err, err)

        signatures = sorted({orbit_signature(orbit) for orbit in row["tied_orbits"]})
        if signatures != previous_signatures:
            switch_rows.append((row["angle_deg"], signatures))
            previous_signatures = signatures

    print(f"max_formula_error={max_err:.3e}")
    print("affine_switch_rows:")
    for angle_deg, signatures in switch_rows:
        formatted = [format_signature(signature) for signature in signatures]
        print(f"  theta={angle_deg:6.2f} deg signatures={formatted}")

    midpoint = next(row for row in minima_rows if abs(row["angle_deg"] - 18.0) < 1e-12)
    bounce_counts = {}
    for orbit in midpoint["tied_orbits"]:
        bounce_counts[orbit["bounces"]] = bounce_counts.get(orbit["bounces"], 0) + 1
    print(
        "midpoint_summary:",
        {
            "angle_deg": midpoint["angle_deg"],
            "n_tied_orbits": midpoint["n_tied_orbits"],
            "bounce_counts": bounce_counts,
        },
    )

    branch_midpoint = next(row for row in branch_rows if abs(row["angle_deg"] - 18.0) < 1e-12)
    print(
        "three_bounce_midpoint_summary:",
        {
            "angle_deg": branch_midpoint["angle_deg"],
            "n_three_bounce_sigmas_examined": branch_midpoint["n_three_bounce_sigmas_examined"],
            "n_admissible_three_bounce_orbits": branch_midpoint["n_admissible_three_bounce_orbits"],
        },
    )


def main() -> None:
    try:
        minima_rows = filter_half_domain(load_minima_rows())
        branch_rows = filter_half_domain(load_branch_rows())
    except FileNotFoundError as err:
        missing = Path(err.args[0])
        print(f"ERROR: data file not found: {missing}", file=sys.stderr)
        if missing == branch_data_path():
            print(
                "Run: cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula -- --three-bounce-branches",
                file=sys.stderr,
            )
        else:
            print(
                "Run: cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula",
                file=sys.stderr,
            )
        sys.exit(1)

    signatures = collect_signatures(minima_rows, branch_rows)
    print_text_summary(minima_rows, branch_rows)
    write_signature_legend(signatures, minima_rows, branch_rows)
    plot_three_bounce_branches(minima_rows, branch_rows)
    plot_minimum_signature_table(minima_rows, branch_rows)
    plot_focus_signature_table(minima_rows, branch_rows)
    plot_labeled_pentagons()
    plot_trajectory_projections(minima_rows)
    plot_affine_trajectory_projections(minima_rows)


if __name__ == "__main__":
    main()
