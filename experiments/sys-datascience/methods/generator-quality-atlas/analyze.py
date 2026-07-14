#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///

"""Build a side-count-stratified quality atlas for planar polygon generators."""

from __future__ import annotations

import argparse
from collections import defaultdict
from dataclasses import dataclass
import csv
import hashlib
import json
import math
from pathlib import Path
from typing import Any, Iterable

import numpy as np


HERE = Path(__file__).resolve().parent
SCHEMA = "factor-shape-row-v1"
SMALL_SAMPLE = 5


@dataclass(frozen=True)
class Shape:
    sample_id: str
    law: str
    side_count: int
    vertices: np.ndarray
    support: np.ndarray
    steiner_center: np.ndarray
    steiner_grid_error: float
    original_area: float
    row: dict[str, Any]


def polygon_area(vertices: np.ndarray) -> float:
    return float(
        0.5
        * np.sum(
            vertices[:, 0] * np.roll(vertices[:, 1], -1)
            - vertices[:, 1] * np.roll(vertices[:, 0], -1)
        )
    )


def support_values(vertices: np.ndarray, angles: np.ndarray) -> np.ndarray:
    directions = np.column_stack((np.cos(angles), np.sin(angles)))
    return np.max(vertices @ directions.T, axis=0)


def numerical_steiner_center(vertices: np.ndarray, grid_size: int) -> np.ndarray:
    """Uniform trapezoidal approximation to (1/pi) int h(theta) u(theta)dtheta."""
    angles = 2.0 * math.pi * np.arange(grid_size, dtype=float) / grid_size
    directions = np.column_stack((np.cos(angles), np.sin(angles)))
    support = np.max(vertices @ directions.T, axis=0)
    return (2.0 / grid_size) * np.sum(support[:, None] * directions, axis=0)


def polygon_steiner_center(vertices: np.ndarray) -> np.ndarray:
    """Exact polygon formula for the Steiner point, up to floating-point error."""
    incoming = vertices - np.roll(vertices, 1, axis=0)
    outgoing = np.roll(vertices, -1, axis=0) - vertices
    cross = incoming[:, 0] * outgoing[:, 1] - incoming[:, 1] * outgoing[:, 0]
    dot = np.sum(incoming * outgoing, axis=1)
    exterior_angles = np.arctan2(cross, dot)
    if np.any(exterior_angles <= 0.0):
        raise ValueError("Steiner point requires strictly positive exterior angles")
    return np.sum(exterior_angles[:, None] * vertices, axis=0) / (2.0 * math.pi)


def validate_vertices(raw: Any, side_count: int, context: str) -> np.ndarray:
    try:
        vertices = np.asarray(raw, dtype=float)
    except (TypeError, ValueError) as exc:
        raise ValueError(f"{context}: vertices are not finite numeric pairs") from exc
    if vertices.shape != (side_count, 2):
        raise ValueError(
            f"{context}: vertices shape {vertices.shape} disagrees with "
            f"side_count={side_count}"
        )
    if side_count < 3:
        raise ValueError(f"{context}: side_count must be at least 3")
    if not np.all(np.isfinite(vertices)):
        raise ValueError(f"{context}: vertices contain a non-finite coordinate")

    differences = vertices[:, None, :] - vertices[None, :, :]
    diameter = float(np.max(np.linalg.norm(differences, axis=2)))
    if not math.isfinite(diameter) or diameter <= 0.0:
        raise ValueError(f"{context}: polygon has zero diameter")
    edge_a = np.roll(vertices, -1, axis=0) - vertices
    edge_b = np.roll(vertices, -2, axis=0) - np.roll(vertices, -1, axis=0)
    turns = edge_a[:, 0] * edge_b[:, 1] - edge_a[:, 1] * edge_b[:, 0]
    turn_tolerance = 1e-12 * diameter * diameter
    if np.any(turns <= turn_tolerance):
        raise ValueError(
            f"{context}: vertices must be strictly convex, cyclic, and CCW; "
            f"minimum consecutive turn={float(np.min(turns)):.6g}"
        )
    area = polygon_area(vertices)
    if area <= 1e-12 * diameter * diameter:
        raise ValueError(
            f"{context}: polygon area is non-positive or numerically degenerate "
            f"(area={area:.6g}, diameter={diameter:.6g})"
        )
    return vertices


def standardize_row(
    row: dict[str, Any], support_grid: int, steiner_grid: int, line_number: int
) -> Shape:
    context = f"line {line_number}"
    if row.get("schema") != SCHEMA:
        raise ValueError(
            f"{context}: expected schema={SCHEMA!r}, got {row.get('schema')!r}"
        )
    sample_id = row.get("sample_id")
    law = row.get("population", row.get("law"))
    side_count = row.get("side_count")
    if not isinstance(sample_id, str) or not sample_id:
        raise ValueError(f"{context}: sample_id must be a nonempty string")
    if not isinstance(law, str) or not law:
        raise ValueError(f"{context}: population/law must be a nonempty string")
    if not isinstance(side_count, int) or isinstance(side_count, bool):
        raise ValueError(f"{context}: side_count must be an integer")
    raw_vertices = row.get("vertices_ccw", row.get("vertices"))
    vertices = validate_vertices(raw_vertices, side_count, context)
    area = polygon_area(vertices)
    center = polygon_steiner_center(vertices)
    grid_center = numerical_steiner_center(vertices, steiner_grid)
    normalized = (vertices - center) / math.sqrt(area)
    normalized_area = polygon_area(normalized)
    if not math.isclose(normalized_area, 1.0, rel_tol=1e-10, abs_tol=1e-12):
        raise ValueError(
            f"{context}: area normalization failed (area={normalized_area:.17g})"
        )
    angles = 2.0 * math.pi * np.arange(support_grid, dtype=float) / support_grid
    support = support_values(normalized, angles)
    if not np.all(np.isfinite(support)) or float(np.min(support)) <= 1e-10:
        raise ValueError(
            f"{context}: centered support grid is degenerate "
            f"(minimum={float(np.min(support)):.6g})"
        )
    return Shape(
        sample_id=sample_id,
        law=law,
        side_count=side_count,
        vertices=normalized,
        support=support,
        steiner_center=center,
        steiner_grid_error=float(np.linalg.norm(grid_center - center)),
        original_area=area,
        row=row,
    )


def load_shapes(path: Path, support_grid: int, steiner_grid: int) -> list[Shape]:
    shapes: list[Shape] = []
    seen: set[str] = set()
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError as exc:
                raise ValueError(f"line {line_number}: invalid JSON: {exc}") from exc
            if not isinstance(row, dict):
                raise ValueError(f"line {line_number}: JSON value must be an object")
            shape = standardize_row(row, support_grid, steiner_grid, line_number)
            if shape.sample_id in seen:
                raise ValueError(f"line {line_number}: duplicate sample_id={shape.sample_id!r}")
            seen.add(shape.sample_id)
            shapes.append(shape)
    if not shapes:
        raise ValueError(f"{path}: no nonempty rows")
    return shapes


def rotation_metrics(left: np.ndarray, right: np.ndarray) -> tuple[float, float, int]:
    """Grid-only helper used for explicit sampled-rotation regression checks."""
    if left.shape != right.shape or left.ndim != 1:
        raise ValueError("support vectors must be one-dimensional and equally sized")
    candidates = np.stack([np.roll(right, shift) for shift in range(len(right))])
    absolute = np.abs(candidates - left[None, :])
    l2_by_shift = np.sqrt(np.mean(absolute * absolute, axis=1))
    best_l2_shift = int(np.argmin(l2_by_shift))
    linf = float(np.min(np.max(absolute, axis=1)))
    return float(l2_by_shift[best_l2_shift]), linf, best_l2_shift


def _golden_minimum(function: Any, left: float, right: float) -> tuple[float, float]:
    ratio = (math.sqrt(5.0) - 1.0) / 2.0
    x1 = right - ratio * (right - left)
    x2 = left + ratio * (right - left)
    f1 = float(function(x1))
    f2 = float(function(x2))
    for _ in range(40):
        if f1 <= f2:
            right, x2, f2 = x2, x1, f1
            x1 = right - ratio * (right - left)
            f1 = float(function(x1))
        else:
            left, x1, f1 = x1, x2, f2
            x2 = left + ratio * (right - left)
            f2 = float(function(x2))
    return (x1, f1) if f1 <= f2 else (x2, f2)


def rotated_support(shape: Shape, angles: np.ndarray, rotation: float) -> np.ndarray:
    # h_{R_alpha K}(theta) = h_K(theta-alpha).
    return support_values(shape.vertices, angles - rotation)


def shape_rotation_metrics(
    left: Shape, right: Shape
) -> tuple[float, float, float, int]:
    """Continuously refine rotation after a global grid scan; never reflect."""
    grid_size = len(left.support)
    if len(right.support) != grid_size:
        raise ValueError("support grids disagree")
    angles = 2.0 * math.pi * np.arange(grid_size, dtype=float) / grid_size
    step = 2.0 * math.pi / grid_size

    def errors(rotation: float) -> tuple[float, float]:
        right_forward = rotated_support(right, angles, rotation)
        left_reverse = rotated_support(left, angles, -rotation)
        forward = np.abs(left.support - right_forward)
        reverse = np.abs(right.support - left_reverse)
        l2_squared = 0.5 * (float(np.mean(forward * forward)) + float(np.mean(reverse * reverse)))
        linf = max(float(np.max(forward)), float(np.max(reverse)))
        return l2_squared, linf

    coarse_rotations = np.stack(
        [np.roll(right.support, index) for index in range(grid_size)]
    )
    coarse_absolute = np.abs(coarse_rotations - left.support[None, :])
    coarse = np.mean(coarse_absolute * coarse_absolute, axis=1)
    best_coarse = float(np.min(coarse))
    tie_tolerance = max(1e-14, 1e-9 * max(1.0, best_coarse))
    tie_count = int(np.sum(coarse <= best_coarse + tie_tolerance))
    candidate_indices = np.argsort(coarse)[: min(2, grid_size)]
    l2_candidates = []
    for index in candidate_indices:
        angle, value = _golden_minimum(
            lambda rotation: errors(rotation)[0],
            float(index) * step - step,
            float(index) * step + step,
        )
        l2_candidates.append((value, angle))
    best_l2_squared, best_angle = min(l2_candidates, key=lambda item: (item[0], item[1]))

    linf_coarse = np.max(coarse_absolute, axis=1)
    linf_candidates = []
    for index in np.argsort(linf_coarse)[: min(2, grid_size)]:
        angle, value = _golden_minimum(
            lambda rotation: errors(rotation)[1],
            float(index) * step - step,
            float(index) * step + step,
        )
        linf_candidates.append((value, angle))
    best_linf, _ = min(linf_candidates, key=lambda item: (item[0], item[1]))
    return math.sqrt(max(0.0, best_l2_squared)), float(best_linf), float(best_angle), tie_count


def distance_matrices(shapes: list[Shape]) -> tuple[np.ndarray, np.ndarray]:
    count = len(shapes)
    l2 = np.zeros((count, count), dtype=float)
    linf = np.zeros((count, count), dtype=float)
    for i in range(count):
        for j in range(i + 1, count):
            l2_value, linf_value, _, _ = shape_rotation_metrics(shapes[i], shapes[j])
            l2[i, j] = l2[j, i] = l2_value
            linf[i, j] = linf[j, i] = linf_value
    return l2, linf


def finite_or_none(value: float) -> float | None:
    return float(value) if math.isfinite(float(value)) else None


def effective_dimension(distance: np.ndarray) -> dict[str, float | int | None]:
    count = len(distance)
    if count < 3:
        return {
            "participation_ratio": None,
            "positive_eigenvalues": 0,
            "negative_eigenmass_fraction": None,
        }
    centering = np.eye(count) - np.ones((count, count)) / count
    gram = -0.5 * centering @ (distance * distance) @ centering
    eigenvalues = np.linalg.eigvalsh(gram)
    scale = max(float(np.max(np.abs(eigenvalues))), 1.0)
    positive = eigenvalues[eigenvalues > 1e-12 * scale]
    negative = eigenvalues[eigenvalues < -1e-12 * scale]
    participation = (
        float(np.sum(positive) ** 2 / np.sum(positive * positive))
        if len(positive)
        else None
    )
    total_mass = float(np.sum(np.abs(eigenvalues)))
    negative_fraction = (
        float(np.sum(np.abs(negative)) / total_mass) if total_mass > 0 else 0.0
    )
    return {
        "participation_ratio": participation,
        "positive_eigenvalues": int(len(positive)),
        "negative_eigenmass_fraction": negative_fraction,
    }


def within_metrics(distance: np.ndarray, duplicate_tolerance: float) -> dict[str, Any]:
    count = len(distance)
    if count < 2:
        return {
            "pair_count": 0,
            "pairwise_mean": None,
            "pairwise_median": None,
            "nearest_neighbor_mean": None,
            "duplicate_pair_count": 0,
            "duplicate_pair_fraction": None,
            "effective_dimension": effective_dimension(distance),
        }
    upper = distance[np.triu_indices(count, k=1)]
    masked = distance.copy()
    np.fill_diagonal(masked, np.inf)
    nearest = np.min(masked, axis=1)
    duplicate_count = int(np.sum(upper <= duplicate_tolerance))
    return {
        "pair_count": int(len(upper)),
        "pairwise_mean": float(np.mean(upper)),
        "pairwise_median": float(np.median(upper)),
        "nearest_neighbor_mean": float(np.mean(nearest)),
        "duplicate_pair_count": duplicate_count,
        "duplicate_pair_fraction": float(duplicate_count / len(upper)),
        "effective_dimension": effective_dimension(distance),
    }


def empirical_energy(
    full_distance: np.ndarray, left: list[int], right: list[int]
) -> float:
    cross = full_distance[np.ix_(left, right)]
    within_left = full_distance[np.ix_(left, left)]
    within_right = full_distance[np.ix_(right, right)]
    return float(
        2.0 * np.mean(cross) - np.mean(within_left) - np.mean(within_right)
    )


def medoid_index(distance: np.ndarray, indices: list[int]) -> int:
    submatrix = distance[np.ix_(indices, indices)]
    return indices[int(np.argmin(np.sum(submatrix, axis=1)))]


def align_to(anchor: Shape, shape: Shape) -> tuple[np.ndarray, int]:
    _, _, rotation, tie_count = shape_rotation_metrics(anchor, shape)
    angles = 2.0 * math.pi * np.arange(len(anchor.support), dtype=float) / len(anchor.support)
    return rotated_support(shape, angles, rotation), tie_count


def optional_numeric_summary(shapes: Iterable[Shape], key: str) -> dict[str, Any] | None:
    values = []
    for shape in shapes:
        value = shape.row.get(key)
        if isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value):
            values.append(float(value))
    if not values:
        return None
    return {
        "observed": len(values),
        "mean": float(np.mean(values)),
        "sum": float(np.sum(values)),
    }


def compute_acceptance(shapes: list[Shape]) -> dict[str, Any]:
    accepted = [shape.row.get("accepted") for shape in shapes]
    observed = [value for value in accepted if isinstance(value, bool)]
    result: dict[str, Any] = {
        "accepted_observed": len(observed),
        "accepted_fraction": (
            float(sum(observed) / len(observed)) if observed else None
        ),
    }
    for key in ("attempts", "rejections", "generation_ms", "validation_ms", "target_ms"):
        result[key] = optional_numeric_summary(shapes, key)
    return result


def combinatorial_summary(shapes: list[Shape]) -> dict[str, Any]:
    keys = sorted(
        {
            key
            for shape in shapes
            for key in shape.row
            if key.startswith("combinatorial_") or key in {"f_vector", "facet_count", "pair_bucket"}
        }
    )
    optional = {}
    for key in keys:
        values = {
            json.dumps(shape.row[key], sort_keys=True, separators=(",", ":"))
            for shape in shapes
            if key in shape.row
        }
        optional[key] = {"observed": sum(key in shape.row for shape in shapes), "unique": len(values)}
    return {
        "planar_fixed_side_count_type": f"convex-{shapes[0].side_count}-gon",
        "type_count_within_stratum": 1,
        "optional_fields": optional,
    }


def provenance_summary(shapes: list[Shape]) -> dict[str, Any]:
    values = sorted(
        {
            json.dumps(shape.row["provenance"], sort_keys=True, separators=(",", ":"))
            for shape in shapes
            if "provenance" in shape.row
        }
    )
    return {
        "observed": sum("provenance" in shape.row for shape in shapes),
        "distinct_descriptions": values,
        "naturalness_is_measured": False,
    }


def build_atlas(
    shapes: list[Shape], baseline_law: str, central_fraction: float, duplicate_tolerance: float
) -> dict[str, Any]:
    by_side: dict[int, list[Shape]] = defaultdict(list)
    for shape in shapes:
        by_side[shape.side_count].append(shape)
    strata = []
    global_laws = sorted({shape.law for shape in shapes})
    issues: list[str] = []

    for side_count in sorted(by_side):
        group = sorted(by_side[side_count], key=lambda shape: shape.sample_id)
        l2, linf = distance_matrices(group)
        indices_by_law = {
            law: [i for i, shape in enumerate(group) if shape.law == law]
            for law in global_laws
            if any(shape.law == law for shape in group)
        }
        baseline_indices = indices_by_law.get(baseline_law, [])
        baseline_medoid = medoid_index(l2, baseline_indices) if baseline_indices else None
        radius = None
        anchor: Shape | None = None
        baseline_centroid = None
        if baseline_medoid is not None:
            anchor = group[baseline_medoid]
            distances = sorted(float(l2[baseline_medoid, i]) for i in baseline_indices)
            radius_index = max(0, math.ceil(central_fraction * len(distances)) - 1)
            radius = distances[radius_index]
            baseline_centroid = np.mean(
                np.stack([align_to(anchor, group[i])[0] for i in baseline_indices]),
                axis=0,
            )
        else:
            issues.append(
                f"side_count={side_count}: baseline law {baseline_law!r} is absent; "
                "baseline comparisons are unavailable"
            )

        laws = []
        for law, indices in sorted(indices_by_law.items()):
            law_shapes = [group[i] for i in indices]
            law_l2 = l2[np.ix_(indices, indices)]
            law_linf = linf[np.ix_(indices, indices)]
            law_issues = []
            if len(indices) < SMALL_SAMPLE:
                message = (
                    f"side_count={side_count}, law={law!r}: n={len(indices)} is below "
                    f"the declared small-sample boundary {SMALL_SAMPLE}"
                )
                law_issues.append(message)
                issues.append(message)
            comparison: dict[str, Any]
            if baseline_medoid is None or anchor is None or baseline_centroid is None:
                comparison = {
                    "baseline_available": False,
                    "energy_like_l2_v_statistic": None,
                    "centroid_distance_baseline_medoid_gauge": None,
                    "centroid_alignment_ambiguous_count": None,
                    "outside_baseline_central_body_count": None,
                    "outside_baseline_central_body_fraction": None,
                }
            else:
                alignments = [align_to(anchor, group[i]) for i in indices]
                aligned = np.stack([item[0] for item in alignments])
                centroid_distance = float(
                    np.sqrt(np.mean((np.mean(aligned, axis=0) - baseline_centroid) ** 2))
                )
                outside = sum(float(l2[baseline_medoid, i]) > float(radius) for i in indices)
                comparison = {
                    "baseline_available": True,
                    "energy_like_l2_v_statistic": empirical_energy(l2, baseline_indices, indices),
                    "centroid_distance_baseline_medoid_gauge": centroid_distance,
                    "centroid_alignment_ambiguous_count": int(
                        sum(tie_count > 1 for _, tie_count in alignments)
                    ),
                    "outside_baseline_central_body_count": int(outside),
                    "outside_baseline_central_body_fraction": float(outside / len(indices)),
                }
            laws.append(
                {
                    "law": law,
                    "count": len(indices),
                    "sample_status": "small-sample" if len(indices) < SMALL_SAMPLE else "descriptive",
                    "within_l2": within_metrics(law_l2, duplicate_tolerance),
                    "within_linf": within_metrics(law_linf, duplicate_tolerance),
                    "baseline_comparison": comparison,
                    "compute_acceptance": compute_acceptance(law_shapes),
                    "combinatorial_breadth": combinatorial_summary(law_shapes),
                    "naturalness_provenance": provenance_summary(law_shapes),
                    "issues": law_issues,
                }
            )
        strata.append(
            {
                "side_count": side_count,
                "count": len(group),
                "baseline_law": baseline_law,
                "baseline_count": len(baseline_indices),
                "baseline_medoid_sample_id": (
                    group[baseline_medoid].sample_id if baseline_medoid is not None else None
                ),
                "central_body_rule": {
                    "center": "baseline empirical L2 medoid",
                    "central_fraction": central_fraction,
                    "radius_order_statistic": "sorted baseline medoid distances at index ceil(q*m)-1",
                    "radius": radius,
                    "outside_rule": "distance to baseline medoid > radius",
                },
                "laws": laws,
            }
        )

    missing_laws = {
        str(side_count): sorted(set(global_laws) - {shape.law for shape in group})
        for side_count, group in sorted(by_side.items())
        if set(global_laws) - {shape.law for shape in group}
    }
    accepted_row_side_count_allocation = {}
    baseline_counts = {
        side_count: sum(
            shape.law == baseline_law and shape.side_count == side_count for shape in shapes
        )
        for side_count in sorted(by_side)
    }
    baseline_total = sum(baseline_counts.values())
    for law in global_laws:
        counts = {
            side_count: sum(shape.law == law and shape.side_count == side_count for shape in shapes)
            for side_count in sorted(by_side)
        }
        total = sum(counts.values())
        if total and baseline_total:
            tv = 0.5 * sum(
                abs(counts[n] / total - baseline_counts[n] / baseline_total)
                for n in sorted(by_side)
            )
        else:
            tv = None
        accepted_row_side_count_allocation[law] = {
            "accepted_shape_row_counts_by_side_count": {
                str(key): value for key, value in counts.items()
            },
            "accepted_row_side_count_allocation_tv_from_baseline": tv,
        }
    return {
        "schema": "generator-quality-atlas-report-v1",
        "row_schema": SCHEMA,
        "baseline_law": baseline_law,
        "rows": len(shapes),
        "laws": global_laws,
        "side_counts": sorted(by_side),
        "missing_laws_by_side_count": missing_laws,
        "accepted_row_side_count_allocation": {
            "interpretation": (
                "Describes the analyzed accepted-shape rows only; imposed allocation, "
                "side-count applicability, and bounded rejection all affect these counts. "
                "It is not an estimate of a natural generator-law side-count distribution."
            ),
            "by_population": accepted_row_side_count_allocation,
        },
        "steiner_grid_approximation_diagnostic": {
            "comparison": "Euclidean error between the declared-grid support integral and the exact polygon Steiner formula",
            "median": float(np.median([shape.steiner_grid_error for shape in shapes])),
            "maximum": float(max(shape.steiner_grid_error for shape in shapes)),
        },
        "quality_dimensions": [
            "controlled-transfer similarity",
            "coverage expansion",
            "within-law diversity",
            "combinatorial breadth",
            "compute/acceptance",
            "naturalness/provenance",
        ],
        "no_combined_quality_score": True,
        "strata": strata,
        "issues": issues,
    }


def json_value(value: Any) -> str:
    if value is None:
        return "NA"
    if isinstance(value, float):
        return f"{value:.10g}"
    return str(value)


def write_table(report: dict[str, Any], path: Path) -> None:
    fields = [
        "side_count",
        "law",
        "count",
        "sample_status",
        "pairwise_l2_mean",
        "nearest_neighbor_l2_mean",
        "effective_dimension",
        "duplicate_pair_fraction",
        "baseline_centroid_distance",
        "energy_like_l2_v_statistic",
        "outside_baseline_count",
        "outside_baseline_fraction",
        "central_body_radius",
        "accepted_fraction",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t")
        writer.writeheader()
        for stratum in report["strata"]:
            for law in stratum["laws"]:
                within = law["within_l2"]
                comparison = law["baseline_comparison"]
                row = {
                    "side_count": stratum["side_count"],
                    "law": law["law"],
                    "count": law["count"],
                    "sample_status": law["sample_status"],
                    "pairwise_l2_mean": within["pairwise_mean"],
                    "nearest_neighbor_l2_mean": within["nearest_neighbor_mean"],
                    "effective_dimension": within["effective_dimension"]["participation_ratio"],
                    "duplicate_pair_fraction": within["duplicate_pair_fraction"],
                    "baseline_centroid_distance": comparison["centroid_distance_baseline_medoid_gauge"],
                    "energy_like_l2_v_statistic": comparison["energy_like_l2_v_statistic"],
                    "outside_baseline_count": comparison["outside_baseline_central_body_count"],
                    "outside_baseline_fraction": comparison["outside_baseline_central_body_fraction"],
                    "central_body_radius": stratum["central_body_rule"]["radius"],
                    "accepted_fraction": law["compute_acceptance"]["accepted_fraction"],
                }
                writer.writerow({key: json_value(value) for key, value in row.items()})


def write_outputs(report: dict[str, Any], out_dir: Path, config: dict[str, Any]) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    payload = dict(report)
    payload["configuration"] = config
    payload["metric_contract"] = {
        "translation": "quotiented by the exact polygon Steiner formula derived from the actual support function",
        "scale": "quotiented by division of vertices by sqrt(polygon area)",
        "rotation": "global grid scan followed by continuous local refinement; support integrals remain sampled on the declared grid",
        "reflection": "not quotiented",
        "l2": "sqrt(mean squared support difference) at its best grid rotation",
        "linf": "maximum support difference at its independently best grid rotation",
        "stratification": "all shape distances and law comparisons are within side_count",
        "angular_approximation": (
            "The exact polygon Steiner point is used; its declared-grid support-integral "
            "error is reported diagnostically. Support integrals use a uniform angular "
            "grid and rotation minimization uses grid-seeded local refinement. L2 and "
            "L-infinity values are numerical approximations, not certified bounds."
        ),
        "small_sample_boundary": SMALL_SAMPLE,
    }
    report_path = out_dir / "report.json"
    report_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    write_table(payload, out_dir / "atlas.tsv")


def regular_polygon(side_count: int) -> np.ndarray:
    angles = 2.0 * math.pi * np.arange(side_count) / side_count
    return np.column_stack((np.cos(angles), np.sin(angles)))


def transform_polygon(
    vertices: np.ndarray,
    stretch: float,
    shear: float,
    rotation: float,
    scale: float,
    translation: tuple[float, float],
) -> np.ndarray:
    deformation = np.array([[math.exp(stretch), shear], [0.0, math.exp(-stretch)]])
    cosine, sine = math.cos(rotation), math.sin(rotation)
    rotation_matrix = np.array([[cosine, -sine], [sine, cosine]])
    return scale * (vertices @ deformation.T @ rotation_matrix.T) + np.asarray(translation)


def synthetic_rows() -> list[dict[str, Any]]:
    rows = []
    law_parameters = {
        "baseline": [0.08] * 8,
        "narrow": [0.055, 0.062, 0.070, 0.076, 0.086, 0.092, 0.099, 0.107],
        "broad": [-0.65, -0.46, -0.30, -0.14, 0.18, 0.34, 0.51, 0.72],
    }
    for side_count in (5, 6):
        base = regular_polygon(side_count)
        for law, stretches in law_parameters.items():
            for index, stretch in enumerate(stretches):
                # Baseline rows deliberately differ only by quotiented transformations.
                shear = 0.03 if law == "baseline" else (0.25 * stretch if law == "broad" else 0.03)
                vertices = transform_polygon(
                    base,
                    stretch=stretch,
                    shear=shear,
                    rotation=(index * 17 % 256) * 2.0 * math.pi / 256.0,
                    scale=0.7 + 0.13 * index,
                    translation=(10.0 - 0.7 * index, -4.0 + 0.3 * index),
                )
                rows.append(
                    {
                        "schema": SCHEMA,
                        "sample_id": f"synthetic/{law}/n={side_count}/i={index}",
                        "law": law,
                        "side_count": side_count,
                        "vertices": vertices.tolist(),
                        "accepted": True,
                        "attempts": index + 1,
                        "rejections": index,
                        "generation_ms": 0.1 * (index + 1),
                        "combinatorial_type": f"convex-{side_count}-gon",
                        "provenance": "deterministic affine synthetic control; not a natural generator",
                    }
                )
    return rows


def write_jsonl(rows: Iterable[dict[str, Any]], path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--baseline-law", default="baseline")
    parser.add_argument("--support-grid", type=int, default=256)
    parser.add_argument("--steiner-grid", type=int, default=4096)
    parser.add_argument("--central-fraction", type=float, default=0.9)
    parser.add_argument("--duplicate-tolerance", type=float, default=1e-9)
    parser.add_argument("--write-synthetic-fixture", type=Path)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    if args.support_grid < 32:
        raise SystemExit("--support-grid must be at least 32")
    if args.steiner_grid < args.support_grid:
        raise SystemExit("--steiner-grid must be at least --support-grid")
    if not 0.0 < args.central_fraction <= 1.0:
        raise SystemExit("--central-fraction must be in (0, 1]")
    if args.duplicate_tolerance < 0.0:
        raise SystemExit("--duplicate-tolerance must be nonnegative")
    if args.write_synthetic_fixture is not None:
        write_jsonl(synthetic_rows(), args.write_synthetic_fixture)
    if args.input is None:
        if args.write_synthetic_fixture is None:
            raise SystemExit("provide --input and/or --write-synthetic-fixture")
        return
    shapes = load_shapes(args.input, args.support_grid, args.steiner_grid)
    report = build_atlas(
        shapes, args.baseline_law, args.central_fraction, args.duplicate_tolerance
    )
    config = {
        "input": str(args.input),
        "input_sha256": hashlib.sha256(args.input.read_bytes()).hexdigest(),
        "baseline_law": args.baseline_law,
        "support_grid": args.support_grid,
        "steiner_grid": args.steiner_grid,
        "central_fraction": args.central_fraction,
        "duplicate_tolerance": args.duplicate_tolerance,
    }
    write_outputs(report, args.out_dir, config)


if __name__ == "__main__":
    main()
