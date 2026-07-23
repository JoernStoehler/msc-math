#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: render the sampled enumerated KKT-branch landscape, or explicitly invoke
the retained legacy minima/three-bounce analysis.
Input Artifacts:
  - required explicit KKT landscape path in `landscape` mode
  - committed minima sweep: experiments/regular-products/pentagon-rotation-empirics/theta-sweep.jsonl
  - required explicit minima and branch paths in `legacy` mode
Output Artifacts:
  - experiments/regular-products/pentagon-rotation-empirics/enumerated_kkt_branch_landscape.png
  - experiments/regular-products/pentagon-rotation-empirics/enumerated_kkt_branch_landscape.pdf
  - experiments/regular-products/pentagon-rotation-empirics/enumerated_kkt_branch_landscape_raw.png
  - experiments/regular-products/pentagon-rotation-empirics/enumerated_kkt_branch_landscape_grouped.png
  - experiments/regular-products/pentagon-rotation-empirics/kkt_branch_sampled_classification.png
  - experiments/regular-products/pentagon-rotation-empirics/kkt_branch_sampled_classification.pdf
  - experiments/regular-products/pentagon-rotation-empirics/kkt-branch-analysis.json
Legacy outputs:
  - experiments/regular-products/pentagon-rotation-empirics/three_bounce_branch_actions.png
  - experiments/regular-products/pentagon-rotation-empirics/signature_state_table_full.png
  - experiments/regular-products/pentagon-rotation-empirics/signature_state_table_competitive.png
  - experiments/regular-products/pentagon-rotation-empirics/labeled_pentagons_theta.png
  - experiments/regular-products/pentagon-rotation-empirics/trajectory_projections_theta14.png
  - experiments/regular-products/pentagon-rotation-empirics/trajectory_projections_theta14_affine.png
  - experiments/regular-products/pentagon-rotation-empirics/signature_legend.txt
Related Viewer Artifacts:
  - producer: experiments/regular-products/pentagon-rotation-empirics/build_interactive_orbit_viewer.py
  - dataset: experiments/regular-products/pentagon-rotation-empirics/minimum_orbit_projection_dataset.jsonl
  - html: experiments/regular-products/pentagon-rotation-empirics/minimum_orbit_projection_viewer.html
"""

import argparse
import hashlib
import json
import math
import sys
from string import ascii_uppercase
from collections import Counter, defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.axes import Axes
from matplotlib.colors import BoundaryNorm, ListedColormap
from matplotlib.lines import Line2D
from matplotlib.patches import Arc, Patch
from matplotlib.transforms import blended_transform_factory

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import FIGSIZE_DUAL, FIGSIZE_SQUARE, TEXT_WIDTH, setup

setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
LANDSCAPE_SELECTED_FIGURE_PATH = EXPERIMENT_DIR / "enumerated_kkt_branch_landscape.png"
LANDSCAPE_SELECTED_PDF_PATH = EXPERIMENT_DIR / "enumerated_kkt_branch_landscape.pdf"
LANDSCAPE_RAW_FIGURE_PATH = EXPERIMENT_DIR / "enumerated_kkt_branch_landscape_raw.png"
LANDSCAPE_GROUPED_FIGURE_PATH = (
    EXPERIMENT_DIR / "enumerated_kkt_branch_landscape_grouped.png"
)
LANDSCAPE_CLASSIFICATION_FIGURE_PATH = (
    EXPERIMENT_DIR / "kkt_branch_sampled_classification.png"
)
LANDSCAPE_CLASSIFICATION_PDF_PATH = (
    EXPERIMENT_DIR / "kkt_branch_sampled_classification.pdf"
)
LANDSCAPE_ANALYSIS_PATH = EXPERIMENT_DIR / "kkt-branch-analysis.json"

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

LANDSCAPE_SCHEMA = "pentagon-kkt-branch-landscape-v1"
LANDSCAPE_STATUSES = (
    "admissible",
    "numerically_inadmissible",
    "indeterminate",
    "solve_failure",
)
PRESENCE_CLASSES = (
    "no_admissible_sample",
    "admissible_at_every_sample",
    "one_contiguous_sampled_run",
    "multiple_sampled_runs",
)
ACTIVE_SIGMA = (3, 8, 1, 0, 5, 6)
WORKED_COMPETITOR_SIGMA = (0, 5, 3, 8, 1, 7)
GROUP_ACTION_DECIMALS = 10


def figure_save_kwargs(path: Path) -> dict:
    if path.suffix.lower() == ".png":
        return {"dpi": 300}
    if path.suffix.lower() == ".pdf":
        return {"metadata": {"CreationDate": None, "ModDate": None}}
    return {}


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        raise FileNotFoundError(path)
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def load_minima_rows(path: Path) -> list[dict]:
    return load_jsonl(path)


def load_branch_rows(path: Path) -> list[dict]:
    return load_jsonl(path)


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


def print_text_summary(
    minima_rows: list[dict],
    branch_rows: list[dict],
    minima_path: Path,
    branch_path: Path,
) -> None:
    print(f"minima_data_path={minima_path}")
    print(f"branch_data_path={branch_path}")
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


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_landscape(
    path: Path, allow_spike: bool
) -> tuple[dict, list[dict], dict]:
    records = load_jsonl(path)
    metadata_records = [row for row in records if row.get("record_type") == "metadata"]
    branches = [row for row in records if row.get("record_type") == "branch"]
    summary_records = [
        row for row in records if row.get("record_type") == "run_summary"
    ]
    require(len(metadata_records) == 1, "landscape must contain one metadata record")
    require(len(summary_records) == 1, "landscape must contain one run_summary record")
    require(
        len(records) == len(branches) + 2,
        "landscape contains an unknown record type",
    )
    metadata = metadata_records[0]
    summary = summary_records[0]
    require(metadata.get("schema_version") == LANDSCAPE_SCHEMA, "unexpected schema")
    require(summary.get("completed") is True, "landscape run is incomplete")
    require(
        metadata.get("epistemic_role")
        == "sampled numerical explanation only; not a proof input",
        "landscape epistemic role is missing or changed",
    )
    run_kind = metadata.get("run_kind")
    require(
        run_kind in ("canonical", "three-angle-spike"),
        f"unknown landscape run kind: {run_kind}",
    )
    if run_kind == "three-angle-spike":
        require(
            allow_spike,
            "publication analysis refuses a spike artifact; pass the canonical artifact",
        )

    angles = metadata.get("sample_angles_deg", [])
    require(metadata.get("theta_domain_deg") == [0.0, 18.0], "unexpected theta domain")
    if run_kind == "canonical":
        expected_angles = [0.25 * index for index in range(73)]
        require(
            angles == expected_angles,
            "canonical landscape must use the exact 73-angle 0.25-degree grid",
        )
        require(
            metadata.get("canonical_step_deg") == 0.25,
            "canonical landscape must declare the 0.25-degree step",
        )
    else:
        require(
            angles == [0.0, 9.0, 18.0],
            "spike landscape must use exactly 0, 9, and 18 degrees",
        )
        require(
            metadata.get("canonical_step_deg") is None,
            "spike landscape must not declare a canonical step",
        )
    require(
        metadata.get("frozen_universe_count") == 3340,
        "frozen universe is not the expected 3340 raw words",
    )
    require(len(branches) == 3340, "branch row count does not match frozen universe")
    require(
        metadata.get("generic_angle_set_checks_equal") is True,
        "generic-angle Rust universe check did not pass",
    )

    branches.sort(key=lambda branch: branch["raw_sigma_id"])
    require(
        [branch["raw_sigma_id"] for branch in branches] == list(range(len(branches))),
        "raw sigma IDs must be consecutive and unique",
    )
    sigma_tuples = [tuple(branch["sigma"]) for branch in branches]
    require(len(set(sigma_tuples)) == len(branches), "raw sigma words are not unique")
    universe_payload = json.dumps(
        [branch["sigma"] for branch in branches], separators=(",", ":")
    ).encode()
    # The retained digest is an advisory staleness cue. Consecutive IDs,
    # uniqueness, counts, and branch semantics above remain blocking.
    if hashlib.sha256(universe_payload).hexdigest() != metadata.get("frozen_universe_sha256"):
        print(
            "warning: frozen-universe bytes differ from retained metadata; "
            "continuing with semantic checks. Reassess retained interpretation "
            "before treating this run as equivalent.",
            file=sys.stderr,
        )

    recomputed_statuses = Counter()
    recomputed_block_counts = Counter()
    for branch in branches:
        require(branch.get("block_count") in (2, 3), "unexpected block count")
        recomputed_block_counts[branch["block_count"]] += 1
        samples = branch.get("samples", [])
        require(
            len(samples) == len(angles),
            f"raw sigma {branch['raw_sigma_id']} does not retain every sample",
        )
        for sample in samples:
            status = sample.get("status")
            require(status in LANDSCAPE_STATUSES, f"unknown solve status {status}")
            recomputed_statuses[status] += 1
            if status in ("admissible", "indeterminate"):
                require(sample.get("action") is not None, "numerical outcome lacks action")

    require(
        dict(recomputed_block_counts)
        == {int(key): value for key, value in metadata["frozen_block_counts"].items()},
        "block-count metadata mismatch",
    )
    expected_outcomes = len(branches) * len(angles)
    require(summary.get("expected_outcomes") == expected_outcomes, "expected outcome mismatch")
    require(summary.get("retained_outcomes") == expected_outcomes, "retained outcome mismatch")
    require(summary.get("branch_rows") == len(branches), "summary branch row mismatch")
    require(summary.get("sample_angles") == len(angles), "summary sample-angle mismatch")
    summary_statuses = Counter(summary.get("status_counts", {}))
    require(recomputed_statuses == summary_statuses, "summary status counts mismatch")

    producer_path = EXPERIMENT_DIR.parents[2] / metadata["producer_source"]
    require(producer_path.exists(), f"producer source is missing: {producer_path}")
    if sha256(producer_path) != metadata.get("producer_source_sha256"):
        print(
            "warning: current producer source differs from the retained input "
            "artifact; continuing with semantic checks. Reassess retained "
            "interpretation before treating this run as equivalent.",
            file=sys.stderr,
        )
    return metadata, branches, summary


def presence_class(samples: list[dict]) -> str:
    flags = [sample["status"] == "admissible" for sample in samples]
    count = sum(flags)
    if count == 0:
        return "no_admissible_sample"
    if count == len(flags):
        return "admissible_at_every_sample"
    runs = sum(flag and (index == 0 or not flags[index - 1]) for index, flag in enumerate(flags))
    if runs == 1:
        return "one_contiguous_sampled_run"
    return "multiple_sampled_runs"


def profile_key(branch: dict) -> tuple:
    profile = []
    for sample in branch["samples"]:
        status = sample["status"]
        action = sample.get("action")
        profile.append(
            (
                status,
                round(float(action), GROUP_ACTION_DECIMALS)
                if action is not None
                else None,
            )
        )
    return (branch["block_count"], tuple(profile))


def profile_groups(branches: list[dict]) -> list[dict]:
    grouped = defaultdict(list)
    for branch in branches:
        grouped[profile_key(branch)].append(branch)
    groups = []
    for members in grouped.values():
        members.sort(key=lambda branch: tuple(branch["sigma"]))
        groups.append(
            {
                "block_count": members[0]["block_count"],
                "representative": members[0],
                "member_raw_sigma_ids": [member["raw_sigma_id"] for member in members],
                "raw_multiplicity": len(members),
            }
        )
    groups.sort(
        key=lambda group: (
            group["block_count"],
            tuple(group["representative"]["sigma"]),
        )
    )
    return groups


def sampled_classification(branches: list[dict]) -> dict:
    result = {
        "presence": {
            str(block): {category: 0 for category in PRESENCE_CLASSES}
            for block in (2, 3, "all")
        },
        "endpoint_status": {
            endpoint: {
                str(block): {status: 0 for status in LANDSCAPE_STATUSES}
                for block in (2, 3, "all")
            }
            for endpoint in ("theta_0", "theta_pi_over_10")
        },
    }
    for branch in branches:
        block = str(branch["block_count"])
        category = presence_class(branch["samples"])
        result["presence"][block][category] += 1
        result["presence"]["all"][category] += 1
        for endpoint, index in (("theta_0", 0), ("theta_pi_over_10", -1)):
            status = branch["samples"][index]["status"]
            result["endpoint_status"][endpoint][block][status] += 1
            result["endpoint_status"][endpoint]["all"][status] += 1
    return result


def branch_by_sigma(branches: list[dict], sigma: tuple[int, ...]) -> dict:
    try:
        return next(branch for branch in branches if tuple(branch["sigma"]) == sigma)
    except StopIteration as error:
        raise ValueError(f"required raw sigma is absent: {sigma}") from error


def plot_branch_series(
    ax: Axes,
    angles: np.ndarray,
    branch: dict,
    display_floor: float,
    cutoff: float,
    *,
    color: str,
    linewidth: float,
    alpha: float,
    zorder: float,
    rasterized: bool,
) -> None:
    values = np.array(
        [
            sample.get("action", np.nan)
            if sample["status"] == "admissible"
            else np.nan
            for sample in branch["samples"]
        ],
        dtype=float,
    )
    ax.plot(
        angles,
        values,
        color=color,
        linewidth=linewidth,
        alpha=alpha,
        zorder=zorder,
        rasterized=rasterized,
    )

    continuation_crossings = []
    for left_index in range(len(angles) - 1):
        left_sample = branch["samples"][left_index]
        right_sample = branch["samples"][left_index + 1]
        if (
            left_sample["status"] != "admissible"
            or right_sample["status"] != "admissible"
        ):
            continue
        left_action = float(left_sample["action"])
        right_action = float(right_sample["action"])
        if not (
            left_action <= cutoff < right_action
            or right_action <= cutoff < left_action
        ):
            continue
        fraction = (cutoff - left_action) / (right_action - left_action)
        continuation_crossings.append(
            float(angles[left_index] + fraction * (angles[left_index + 1] - angles[left_index]))
        )
    if continuation_crossings:
        marker_y = cutoff - 0.018 * (cutoff - display_floor)
        ax.scatter(
            continuation_crossings,
            [marker_y] * len(continuation_crossings),
            s=34,
            marker="^",
            facecolors="white",
            edgecolors="#444444",
            linewidths=0.9,
            alpha=max(alpha, 0.85),
            zorder=7.0,
            clip_on=True,
            rasterized=rasterized,
        )


def plot_landscape(
    metadata: dict,
    branches: list[dict],
    groups: list[dict],
    *,
    grouped: bool,
    output_paths: list[Path],
) -> None:
    angles = np.array(metadata["sample_angles_deg"], dtype=float)
    cutoff = float(metadata["display_action_cutoff"])
    active = branch_by_sigma(branches, ACTIVE_SIGMA)
    competitor = branch_by_sigma(branches, WORKED_COMPETITOR_SIGMA)
    highlighted_ids = {active["raw_sigma_id"], competitor["raw_sigma_id"]}
    finite_actions = [
        float(sample["action"])
        for branch in branches
        for sample in branch["samples"]
        if sample["status"] == "admissible"
        and sample.get("action") is not None
        and sample["action"] <= cutoff
    ]
    require(finite_actions, "no admissible actions fall below the display cutoff")
    y_min = min(finite_actions)
    y_min -= 0.035 * (cutoff - y_min)

    fig = plt.figure(figsize=(TEXT_WIDTH, 5.7))
    grid = fig.add_gridspec(2, 1, hspace=0.24)
    action_axes = [fig.add_subplot(grid[index, 0]) for index in range(2)]

    if grouped:
        for group in groups:
            representative = group["representative"]
            if highlighted_ids.intersection(group["member_raw_sigma_ids"]):
                continue
            plot_branch_series(
                action_axes[group["block_count"] - 2],
                angles,
                representative,
                y_min,
                cutoff,
                color="#858585",
                linewidth=0.72,
                alpha=0.52,
                zorder=0.5,
                rasterized=False,
            )
    else:
        for branch in branches:
            if branch["raw_sigma_id"] in highlighted_ids:
                continue
            plot_branch_series(
                action_axes[branch["block_count"] - 2],
                angles,
                branch,
                y_min,
                cutoff,
                color="#777777",
                linewidth=0.34,
                alpha=0.055,
                zorder=0.4,
                rasterized=True,
            )

    plot_branch_series(
        action_axes[0],
        angles,
        active,
        y_min,
        cutoff,
        color="#111111",
        linewidth=2.0,
        alpha=1.0,
        zorder=5.0,
        rasterized=False,
    )
    plot_branch_series(
        action_axes[1],
        angles,
        competitor,
        y_min,
        cutoff,
        color="#1769aa",
        linewidth=1.8,
        alpha=1.0,
        zorder=5.0,
        rasterized=False,
    )

    raw_counts = Counter(branch["block_count"] for branch in branches)
    group_counts = Counter(group["block_count"] for group in groups)
    for block_count, ax in zip((2, 3), action_axes):
        ax.set_ylim(y_min, cutoff)
        ax.axhline(cutoff, color="#555555", linewidth=0.7, linestyle=(0, (3, 2)))
        ax.set_ylabel("action")
        ax.set_title(
            (
                f"{block_count}-block words: {group_counts[block_count]:,} distinct sampled profiles"
                f" from {raw_counts[block_count]:,} raw words"
                if grouped
                else f"{block_count}-block — {raw_counts[block_count]:,} raw curves / raw words"
            ),
            loc="left",
            fontsize=8.8,
            pad=5.0,
        )
        ax.tick_params(labelbottom=False)
        ax.grid(axis="x", visible=False)
    action_axes[1].set_xlabel(r"rotation angle $\theta$")
    action_axes[1].set_xticks([0.0, 4.5, 9.0, 13.5, 18.0])
    action_axes[1].set_xticklabels(
        [r"$0$", r"$\pi/40$", r"$\pi/20$", r"$3\pi/40$", r"$\pi/10$"]
    )
    action_axes[1].tick_params(labelbottom=True)

    legend_handles = [
        Line2D([0], [0], color="#111111", linewidth=2.0, label="active branch"),
        Line2D(
            [0],
            [0],
            color="#1769aa",
            linewidth=1.8,
            label=r"example competitor $\tau$ (3 blocks)",
        ),
        Line2D(
            [0],
            [0],
            color="#858585",
            linewidth=0.72,
            label="other distinct sampled profiles"
            if grouped
            else "other raw branches",
        ),
        Line2D(
            [0],
            [0],
            marker="^",
            color="none",
            markeredgecolor="#444444",
            markerfacecolor="white",
            markersize=5.8,
            label="continues above cutoff",
        ),
    ]
    fig.legend(
        handles=legend_handles,
        loc="upper center",
        bbox_to_anchor=(0.5, 0.948),
        ncols=2,
        frameon=False,
        fontsize=7.5,
    )
    fig.suptitle("Sampled enumerated KKT-branch landscape", y=0.995, fontsize=11.0)
    fig.subplots_adjust(left=0.13, right=0.98, bottom=0.075, top=0.855)
    for output_path in output_paths:
        fig.savefig(output_path, **figure_save_kwargs(output_path))
        print(f"Saved: {output_path}")
    plt.close(fig)


def plot_sampled_classification(
    metadata: dict, classification: dict, output_paths: list[Path]
) -> None:
    presence_labels = {
        "no_admissible_sample": "no admissible\nsample",
        "admissible_at_every_sample": "admissible at\nevery sample",
        "one_contiguous_sampled_run": "one contiguous\nsampled run",
        "multiple_sampled_runs": "multiple\nsampled runs",
    }
    status_labels = {
        "admissible": "admissible",
        "numerically_inadmissible": "numerically\ninadmissible",
        "indeterminate": "indeterminate",
        "solve_failure": "solve failure",
    }
    columns = ["2-block", "3-block", "all"]
    keys = ["2", "3", "all"]
    presence_values = [
        [classification["presence"][key][category] for key in keys]
        for category in PRESENCE_CLASSES
    ]
    endpoint_rows = []
    endpoint_values = []
    for endpoint, endpoint_label in (
        ("theta_0", r"$0$"),
        ("theta_pi_over_10", r"$\pi/10$"),
    ):
        for status in LANDSCAPE_STATUSES:
            endpoint_rows.append(f"{endpoint_label}: {status_labels[status]}")
            endpoint_values.append(
                [classification["endpoint_status"][endpoint][key][status] for key in keys]
            )

    fig, axes = plt.subplots(
        2,
        1,
        figsize=(TEXT_WIDTH, 4.8),
        gridspec_kw={"height_ratios": [0.9, 1.7]},
    )
    for ax in axes:
        ax.axis("off")
    presence_table = axes[0].table(
        cellText=presence_values,
        rowLabels=[presence_labels[category] for category in PRESENCE_CLASSES],
        colLabels=columns,
        loc="center",
        cellLoc="right",
        colLoc="right",
    )
    endpoint_table = axes[1].table(
        cellText=endpoint_values,
        rowLabels=endpoint_rows,
        colLabels=columns,
        loc="center",
        cellLoc="right",
        colLoc="right",
    )
    for table in (presence_table, endpoint_table):
        table.auto_set_font_size(False)
        table.set_fontsize(6.7)
        table.scale(1.0, 1.23)
        for (row, _column), cell in table.get_celld().items():
            cell.set_linewidth(0.35)
            if row == 0:
                cell.set_facecolor("#e9edf2")
                cell.set_text_props(weight="bold")
    axes[0].set_title("Sampled presence class", fontsize=9.0, pad=0)
    axes[1].set_title("Endpoint solve status (reported separately)", fontsize=9.0, pad=0)
    fig.suptitle(
        f"Raw-word classification on the {len(metadata['sample_angles_deg'])}-angle grid",
        y=0.99,
        fontsize=10.5,
    )
    fig.text(
        0.5,
        0.015,
        "Sampled classes are not claims about exact feasibility intervals or isolated specializations.",
        ha="center",
        fontsize=6.7,
    )
    fig.subplots_adjust(left=0.31, right=0.98, bottom=0.07, top=0.91, hspace=0.18)
    for output_path in output_paths:
        fig.savefig(output_path, **figure_save_kwargs(output_path))
        print(f"Saved: {output_path}")
    plt.close(fig)


def write_landscape_analysis(
    input_path: Path,
    metadata: dict,
    branches: list[dict],
    summary: dict,
    groups: list[dict],
    classification: dict,
) -> None:
    active = branch_by_sigma(branches, ACTIVE_SIGMA)
    competitor = branch_by_sigma(branches, WORKED_COMPETITOR_SIGMA)
    group_counts = Counter(group["block_count"] for group in groups)
    group_multiplicities = [group["raw_multiplicity"] for group in groups]
    repo_root = EXPERIMENT_DIR.parents[2]

    def relative(path: Path) -> str:
        return str(path.relative_to(repo_root))

    output_paths = [
        LANDSCAPE_SELECTED_FIGURE_PATH,
        LANDSCAPE_SELECTED_PDF_PATH,
        LANDSCAPE_RAW_FIGURE_PATH,
        LANDSCAPE_GROUPED_FIGURE_PATH,
        LANDSCAPE_CLASSIFICATION_FIGURE_PATH,
        LANDSCAPE_CLASSIFICATION_PDF_PATH,
    ]
    report = {
        "schema_version": "pentagon-kkt-branch-analysis-v1",
        "analyzer": {
            "source": relative(Path(__file__).resolve()),
            "source_sha256": sha256(Path(__file__).resolve()),
            "publication_input_is_explicit": True,
            "group_action_decimals": GROUP_ACTION_DECIMALS,
        },
        "input": {
            "path": str(input_path),
            "sha256": sha256(input_path),
            "producer_source_sha256": metadata["producer_source_sha256"],
            "run_kind": metadata["run_kind"],
            "sample_angles_deg": metadata["sample_angles_deg"],
            "frozen_universe_count": metadata["frozen_universe_count"],
            "frozen_universe_sha256": metadata["frozen_universe_sha256"],
            "status_counts": {
                status: summary.get("status_counts", {}).get(status, 0)
                for status in LANDSCAPE_STATUSES
            },
            "display_action_cutoff": metadata["display_action_cutoff"],
            "numerical_contract": metadata["numerical_contract"],
            "exact_family_comparison": metadata["exact_family_comparison"],
        },
        "classification": classification,
        "grouping": {
            "semantics": f"raw words are grouped only when their block count, all 73 sampled statuses, and all sampled actions rounded to {GROUP_ACTION_DECIMALS} decimal places agree; the plotted profile follows the lexicographically smallest member across all 73 angles, never a pointwise minimum or splice",
            "curve_semantics": "each gray curve in the grouped landscape is one fixed raw representative across all sampled angles; curves are never pointwise aggregated, minimized, or spliced",
            "multiplicity_encoding": "the publication plot uses a uniform gray line for each distinct sampled profile; raw-word multiplicities are reported by the panel totals and remain available in this report",
            "raw_word_count": len(branches),
            "whole_profile_group_count": len(groups),
            "group_count_by_block": {str(key): value for key, value in group_counts.items()},
            "largest_raw_multiplicity": max(group_multiplicities),
            "singleton_groups": sum(value == 1 for value in group_multiplicities),
            "raw_multiplicity_preserved": True,
        },
        "highlighted_raw_sigmas": {
            "semantics": "the active label comes from the theorem source and the worked-competitor label from the chapter calculation; the numerical action-bound diagnostics are not used to establish ordering or ties",
            "active": {
                "sigma": list(ACTIVE_SIGMA),
                "raw_sigma_id": active["raw_sigma_id"],
                "samples": active["samples"],
            },
            "worked_competitor": {
                "sigma": list(WORKED_COMPETITOR_SIGMA),
                "raw_sigma_id": competitor["raw_sigma_id"],
                "samples": competitor["samples"],
            },
        },
        "selected_view": {
            "path": relative(LANDSCAPE_SELECTED_FIGURE_PATH),
            "vector_path": relative(LANDSCAPE_SELECTED_PDF_PATH),
            "kind": "whole-profile grouped landscape",
            "reason": "it draws each distinct whole-grid sampled profile once, reports both profile and raw-word totals, and leaves sampled feasibility classification to the adjacent table; the raw-line render is retained for comparison",
            "panel_counts": {
                str(block): {
                    "sampled_profile_groups": group_counts[block],
                    "raw_words": sum(
                        branch["block_count"] == block for branch in branches
                    ),
                }
                for block in (2, 3)
            },
            "curve_semantics": f"raw words are collapsed only when all 73 sampled statuses agree and all sampled actions agree after rounding to {GROUP_ACTION_DECIMALS} decimal places; one member's whole 73-angle sequence is drawn for each such group, with no pointwise aggregation or splicing",
            "cutoff_encoding": "an upward hollow triangle just inside the action-6 boundary marks an interpolated crossing where adjacent admissible samples continue above the display window; a line break means the raw word was not numerically admissible at the missing samples",
            "status_marker_placement": "indeterminate and failure statuses are classified in the adjacent table rather than overplotted on the action curves",
            "publication_formats": {
                "png_dpi": 300,
                "pdf_is_vector": True,
            },
        },
        "candidate_views": {
            "raw": relative(LANDSCAPE_RAW_FIGURE_PATH),
            "whole_profile_grouped": relative(LANDSCAPE_GROUPED_FIGURE_PATH),
            "sampled_classification": relative(LANDSCAPE_CLASSIFICATION_FIGURE_PATH),
            "sampled_classification_vector": relative(
                LANDSCAPE_CLASSIFICATION_PDF_PATH
            ),
        },
        "output_sha256": {relative(path): sha256(path) for path in output_paths},
        "allowed_interpretation": "The sampled numerical grid illustrates the scale and competition of the enumerated two-/three-block KKT family and sampled appearance or disappearance of admissible branches.",
        "prohibited_interpretation": [
            "the figure proves the lower envelope or theorem",
            "the sampled presence classes are exact interval-topology classifications",
            "the figure displays every billiard orbit",
            "the grid excludes narrow or isolated specialization-only components",
            "the shared action_lower/action_upper diagnostics certify ordering or ties",
        ],
    }
    LANDSCAPE_ANALYSIS_PATH.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(f"Saved: {LANDSCAPE_ANALYSIS_PATH}")


def run_landscape_analysis(input_path: Path, allow_spike: bool, validate_only: bool) -> None:
    metadata, branches, summary = load_landscape(input_path, allow_spike)
    print(
        "landscape_validated:",
        {
            "input": str(input_path),
            "run_kind": metadata["run_kind"],
            "raw_words": len(branches),
            "sample_angles": len(metadata["sample_angles_deg"]),
            "outcomes": summary["retained_outcomes"],
            "status_counts": {
                status: summary.get("status_counts", {}).get(status, 0)
                for status in LANDSCAPE_STATUSES
            },
        },
    )
    if validate_only:
        return
    require(
        metadata["run_kind"] == "canonical",
        "figure generation is allowed only for the canonical artifact",
    )
    groups = profile_groups(branches)
    classification = sampled_classification(branches)
    plot_landscape(
        metadata,
        branches,
        groups,
        grouped=False,
        output_paths=[LANDSCAPE_RAW_FIGURE_PATH],
    )
    plot_landscape(
        metadata,
        branches,
        groups,
        grouped=True,
        output_paths=[
            LANDSCAPE_GROUPED_FIGURE_PATH,
            LANDSCAPE_SELECTED_FIGURE_PATH,
            LANDSCAPE_SELECTED_PDF_PATH,
        ],
    )
    plot_sampled_classification(
        metadata,
        classification,
        [LANDSCAPE_CLASSIFICATION_FIGURE_PATH, LANDSCAPE_CLASSIFICATION_PDF_PATH],
    )
    write_landscape_analysis(
        input_path,
        metadata,
        branches,
        summary,
        groups,
        classification,
    )


def run_legacy_analysis(minima_path: Path, branch_path: Path) -> None:
    minima_rows = filter_half_domain(load_minima_rows(minima_path))
    branch_rows = filter_half_domain(load_branch_rows(branch_path))
    signatures = collect_signatures(minima_rows, branch_rows)
    print_text_summary(minima_rows, branch_rows, minima_path, branch_path)
    write_signature_legend(signatures, minima_rows, branch_rows)
    plot_three_bounce_branches(minima_rows, branch_rows)
    plot_minimum_signature_table(minima_rows, branch_rows)
    plot_focus_signature_table(minima_rows, branch_rows)
    plot_labeled_pentagons()
    plot_trajectory_projections(minima_rows)
    plot_affine_trajectory_projections(minima_rows)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Analyze an explicitly named pentagon-rotation artifact."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    landscape = subparsers.add_parser(
        "landscape", help="validate and render a frozen KKT-branch landscape"
    )
    landscape.add_argument("--input", required=True, type=Path)
    landscape.add_argument(
        "--allow-spike",
        action="store_true",
        help="allow a spike artifact for validation only",
    )
    landscape.add_argument("--validate-only", action="store_true")
    legacy = subparsers.add_parser(
        "legacy", help="regenerate the retained minima/three-bounce figures"
    )
    legacy.add_argument("--minima-input", required=True, type=Path)
    legacy.add_argument("--branch-input", required=True, type=Path)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    try:
        if args.command == "landscape":
            require(
                not args.allow_spike or args.validate_only,
                "--allow-spike may be used only with --validate-only",
            )
            run_landscape_analysis(args.input, args.allow_spike, args.validate_only)
        else:
            run_legacy_analysis(args.minima_input, args.branch_input)
    except (FileNotFoundError, ValueError, KeyError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
